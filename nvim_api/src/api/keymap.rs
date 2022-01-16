use core::fmt;
use neovim_sys::{
    api::{
        buffer::Buffer,
        nvim::{object, Dictionary, LuaError, NvimString},
        private,
    },
    getchar::{self, MapArguments, MapType, UnexpectedMode},
};
use std::{ffi::CString, os::raw::c_int, str::FromStr};

/// Correlates to Section 1.2 in `map.txt` (`:map-arguements`).
///
#[derive(Debug, Default, Clone, Copy)]
#[allow(clippy::struct_excessive_bools)]
pub struct SpecialArguments {
    buffer: bool,
    expr: bool,
    nowait: bool,
    script: bool,
    silent: bool,
    unique: bool,
}

macro_rules! def_bool_meth {
    ($field:ident) => {
        #[must_use]
        pub const fn $field(self) -> Self {
            let mut s = self;
            s.$field = true;
            s
        }
    };
}

impl SpecialArguments {
    def_bool_meth!(buffer);
    def_bool_meth!(expr);
    def_bool_meth!(nowait);
    def_bool_meth!(script);
    def_bool_meth!(silent);
    def_bool_meth!(unique);
}

impl fmt::Display for SpecialArguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.buffer {
            f.write_str("<buffer>")?;
        }
        if self.nowait {
            f.write_str("<nowait>")?;
        }
        if self.silent {
            f.write_str("<silent>")?;
        }
        if self.script {
            f.write_str("<script>")?;
        }
        if self.expr {
            f.write_str("<expr>")?;
        }
        if self.unique {
            f.write_str("<unique>")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    // 1
    #[error("Invalid arguments for mapping/abbreviation: '{}'", _0)]
    InvalidArguments(String),

    /// When deleting a map, if it's not found, you get this.
    ///
    // 2
    #[error("Mapping/abbreviation not found: '{}'", _0)]
    NoMatches(String),

    /// When a mapping/abbreviation is already set, but is set to `<unique>`, you get this.
    ///
    // 5
    #[error("Mapping/abbreviation must be unit: '{}'", _0)]
    EntryNotUnique(String),

    // In case we get a value we don't recognize
    #[error("Unknown error code from neovim: '{}'", _0)]
    Unknown(c_int),

    #[error(transparent)]
    NulError(#[from] std::ffi::NulError),

    #[error(transparent)]
    LuaError(#[from] LuaError),

    #[error(transparent)]
    ObjectError(#[from] object::Error),

    #[error(transparent)]
    UnexpectedMode(#[from] UnexpectedMode),
}

/// Defines a mapping for `mode` that maps `lhs` to `rhs`.
///
/// # Errors
///
/// This will error if:
///
/// - `lhs` and `rhs` can't be coerced to a `CString`.
/// - nvim returns an error.
///
pub fn set_map(
    mode: &str,
    lhs: &str,
    rhs: &str,
    special_arguments: Option<SpecialArguments>,
) -> Result<(), Error> {
    map(MapType::Map, mode, lhs, rhs, special_arguments)
}

/// Defines a `noremap` mapping for `mode` that maps `lhs` to `rhs`.
///
/// # Errors
///
/// This will error if:
///
/// - `lhs` and `rhs` can't be coerced to a `CString`.
/// - nvim returns an error.
///
pub fn set_noremap(
    mode: &str,
    lhs: &str,
    rhs: &str,
    special_arguments: Option<SpecialArguments>,
) -> Result<(), Error> {
    map(MapType::NoRemap, mode, lhs, rhs, special_arguments)
}

fn map(
    map_type: MapType,
    mode: &str,
    lhs: &str,
    rhs: &str,
    special_arguments: Option<SpecialArguments>,
) -> Result<(), Error> {
    let string_arg = special_arguments.map_or_else(
        || format!("{} {}", lhs, rhs),
        |o| format!("{} {} {}", o, lhs, rhs),
    );
    let cstring = CString::new(string_arg)?;
    let mut arg = cstring.into_bytes_with_nul();

    let result = unsafe {
        getchar::do_map(
            map_type as c_int,
            arg.as_mut_ptr(),
            getchar::Mode::from_str(mode)? as c_int,
            false,
        )
    };

    match result {
        0 => Ok(()),
        1 => Err(Error::InvalidArguments(rhs.to_string())),
        2 => Err(Error::NoMatches(lhs.to_string())),
        5 => Err(Error::EntryNotUnique(lhs.to_string())),
        v => Err(Error::Unknown(v)),
    }
}
/// # Errors
///
/// This will error if a `Mapping` can't be built from any of the `Dictionary`s returned by the
/// call to `nvim_get_keymap()`.
///
pub fn get_maps(mode: &str) -> Result<Vec<Dictionary>, Error> {
    let maps = unsafe { neovim_sys::api::nvim::nvim_get_keymap(NvimString::new(mode)?) };
    let mut output = Vec::with_capacity(maps.len());

    for object in maps {
        output.push(object.try_into_dictionary()?);
    }

    Ok(output)
}

/// Similar to `nvim_buf_set_keymap()`.
///
/// # Errors
///
/// This will error if:
///
/// - `lhs` and `rhs` can't be coerced to a `CString`.
/// - `lhs` and `rhs` can't be parsed into valid mapping arguments.
/// - nvim returns an error.
///
pub fn set_buf_map(
    buffer: Buffer,
    mode: &str,
    lhs: &str,
    rhs: &str,
    options: Option<SpecialArguments>,
) -> Result<(), Error> {
    buf_map(MapType::Map, buffer, mode, lhs, rhs, options)
}

/// Similar to `nvim_buf_set_keymap()`, but passing `{noremap = true}` with the options.
///
/// # Errors
///
/// This will error if:
///
/// - `lhs` and `rhs` can't be coerced to a `CString`.
/// - `lhs` and `rhs` can't be parsed into valid mapping arguments.
/// - nvim returns an error.
///
pub fn set_buf_noremap(
    buffer: Buffer,
    mode: &str,
    lhs: &str,
    rhs: &str,
    options: Option<SpecialArguments>,
) -> Result<(), Error> {
    buf_map(MapType::NoRemap, buffer, mode, lhs, rhs, options)
}

fn buf_map(
    map_type: MapType,
    buffer: Buffer,
    mode: &str,
    lhs: &str,
    rhs: &str,
    options: Option<SpecialArguments>,
) -> Result<(), Error> {
    let is_unmap = false;
    let mut map_args = MapArguments::new();

    {
        let string_arg = options.map_or_else(
            || format!("{} {}", lhs, rhs),
            |o| format!("{} {} {}", o, lhs, rhs),
        );
        let cstring = CString::new(string_arg)?;
        let args = cstring.into_bytes_with_nul();

        unsafe { getchar::str_to_mapargs(args.as_ptr(), is_unmap, &mut map_args) };
    };

    let mut out_err = LuaError::default();
    let buf = unsafe { private::find_buffer_by_handle(buffer, &mut out_err) };

    if out_err.is_err() {
        return Err(Error::from(out_err));
    }

    let result = unsafe {
        getchar::buf_do_map(
            map_type as c_int,
            &map_args,
            getchar::Mode::from_str(mode)? as c_int,
            is_unmap,
            buf,
        )
    };

    match result {
        0 => Ok(()),
        1 => Err(Error::InvalidArguments(rhs.to_string())),
        2 => Err(Error::NoMatches(lhs.to_string())),
        5 => Err(Error::EntryNotUnique(lhs.to_string())),
        v => Err(Error::Unknown(v)),
    }
}
