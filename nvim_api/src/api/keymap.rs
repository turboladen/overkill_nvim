// pub use neovim_sys::getchar::Mode;

use super::Mode;
use core::fmt;
use neovim_sys::{
    api::{
        buffer::Buffer,
        nvim::{Boolean, Integer, LuaError, NvimString, Object},
        private,
    },
    getchar::{self, MapArguments, MapType},
};
use std::{ffi::CString, os::raw::c_int};

#[derive(Debug, Default, Clone, Copy)]
#[allow(clippy::struct_excessive_bools)]
pub struct Options {
    buffer: bool,
    nowait: bool,
    silent: bool,
    script: bool,
    expr: bool,
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

impl Options {
    def_bool_meth!(buffer);
    def_bool_meth!(nowait);
    def_bool_meth!(silent);
    def_bool_meth!(script);
    def_bool_meth!(expr);
    def_bool_meth!(unique);
}

impl fmt::Display for Options {
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
}

#[derive(Debug, Clone)]
pub struct Mapping {
    mode: NvimString,
    lhs: NvimString,
    rhs: NvimString,
    sid: Integer,
    lnum: Integer,
    buffer: Boolean,
    expr: Boolean,
    noremap: Boolean,
    nowait: Boolean,
    script: Boolean,
    silent: Boolean,
}

impl Mapping {
    /// Get a reference to the mapping's mode.
    #[must_use]
    pub const fn mode(&self) -> &NvimString {
        &self.mode
    }

    /// Get a reference to the mapping's lhs.
    #[must_use]
    pub const fn lhs(&self) -> &NvimString {
        &self.lhs
    }

    /// Get a reference to the mapping's rhs.
    #[must_use]
    pub const fn rhs(&self) -> &NvimString {
        &self.rhs
    }

    /// Get a reference to the mapping's sid.
    #[must_use]
    pub const fn sid(&self) -> i64 {
        self.sid
    }

    /// Get a reference to the mapping's lnum.
    #[must_use]
    pub const fn lnum(&self) -> i64 {
        self.lnum
    }

    /// Get a reference to the mapping's buffer.
    #[must_use]
    pub const fn buffer(&self) -> bool {
        self.buffer
    }

    /// Get a reference to the mapping's expr.
    #[must_use]
    pub const fn expr(&self) -> bool {
        self.expr
    }

    /// Get a reference to the mapping's noremap.
    #[must_use]
    pub const fn noremap(&self) -> bool {
        self.noremap
    }

    /// Get a reference to the mapping's nowait.
    #[must_use]
    pub const fn nowait(&self) -> bool {
        self.nowait
    }

    /// Get a reference to the mapping's script.
    #[must_use]
    pub const fn script(&self) -> bool {
        self.script
    }

    /// Get a reference to the mapping's silent.
    #[must_use]
    pub const fn silent(&self) -> bool {
        self.silent
    }
}

impl TryFrom<Object> for Mapping {
    type Error = Error;

    fn try_from(object: Object) -> Result<Self, Self::Error> {
        let value = object.into_dictionary_unchecked();

        Ok(Self {
            mode: value.get_as_string("mode").unwrap().clone(),
            lhs: value.get_as_string("lhs").unwrap().clone(),
            rhs: value.get_as_string("rhs").unwrap().clone(),
            lnum: value.get_as_integer("lnum").unwrap(),
            sid: value.get_as_integer("sid").unwrap(),
            buffer: value.get_as_boolean("buffer").unwrap(),
            expr: value.get_as_boolean("expr").unwrap(),
            noremap: value.get_as_boolean("noremap").unwrap(),
            nowait: value.get_as_boolean("nowait").unwrap(),
            script: value.get_as_boolean("script").unwrap(),
            silent: value.get_as_boolean("silent").unwrap(),
        })
    }
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
pub fn set_map(mode: Mode, lhs: &str, rhs: &str, options: Option<Options>) -> Result<(), Error> {
    let string_arg = options.map_or_else(
        || format!("{} {}", lhs, rhs),
        |o| format!("{} {} {}", o, lhs, rhs),
    );
    let cstring = CString::new(string_arg)?;
    let mut arg = cstring.into_bytes_with_nul();

    let result = unsafe {
        getchar::do_map(
            MapType::Map as c_int,
            arg.as_mut_ptr(),
            getchar::Mode::from(mode) as c_int,
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
pub fn get_maps(mode: Mode) -> Result<Vec<Mapping>, Error> {
    let maps = unsafe {
        neovim_sys::api::nvim::nvim_get_keymap(NvimString::new_unchecked(mode.abbreviation()))
    };

    let mut output = Vec::with_capacity(maps.len());

    for map in maps {
        output.push(Mapping::try_from(map)?);
    }

    Ok(output)
}

pub fn set_buf_map(
    buffer: Buffer,
    mode: Mode,
    lhs: &str,
    rhs: &str,
    options: Option<Options>,
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
            MapType::Map as c_int,
            &map_args,
            getchar::Mode::from(mode) as c_int,
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
