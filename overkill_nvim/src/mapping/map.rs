//! Overkill API for getting and setting mappings.
//!
use super::MapMode;
use nvim_api::api::{
    keymap::{get_maps, set_buf_map, set_buf_noremap, set_map, set_noremap, SpecialArguments},
    Buffer, Dictionary,
};

macro_rules! def_bool_return_self_meth {
    ($field:ident) => {
        /// Enable this option.
        ///
        #[must_use]
        pub const fn $field(self) -> Self {
            let arguments = self.arguments.$field();

            let mut s = self;
            s.arguments = arguments;
            s
        }
    };
}

/// This type provides the main API for getting and setting mappings.
///
#[derive(Debug, Clone, Copy)]
pub struct Mapper {
    mode: MapMode,
    arguments: SpecialArguments,
}

impl Mapper {
    /// Basic constructor.
    ///
    #[must_use]
    pub fn new(mode: MapMode) -> Self {
        Self {
            mode,
            arguments: SpecialArguments::default(),
        }
    }

    /// # Panics
    ///
    /// This panics if `lhs` or `rhs` can't be converted to internal C-Strings (so don't pass any
    /// `\0` chars!)
    ///
    pub fn map(&self, lhs: &str, rhs: &str) {
        set_map(self.mode.as_str(), lhs, rhs, Some(self.arguments)).unwrap();
    }

    /// Yields `self` to `f`, but captures `self`'s arguments before, then sets them back after `f`
    /// is called. It's helpful for reusing a `Mapper` for different purposes.
    ///
    /// ```no_run
    /// use overkill_nvim::mapping::{MapMode, Mapper};
    ///
    /// let mut normal_mapper = Mapper::new(MapMode::Normal);
    ///
    /// // Use this group for normal+silent mappings.
    /// normal_mapper.group(|mapper| {
    ///     let mapper = mapper.silent();
    ///     mapper.map("<F2>", "ihello<ESC>");
    ///     mapper.map("<F4>", "cchello<ESC>");
    /// });
    ///
    /// // Use this group for normal+unique mappings.
    /// normal_mapper.group(|mapper| {
    ///     let mapper = mapper.unique();
    ///     mapper.noremap("<F3>", ":echo 'hi'<CR>");
    /// });
    ///
    /// ```
    ///
    pub fn group<F>(&mut self, f: F)
    where
        F: Fn(Self),
    {
        let args_before = self.arguments;

        f(*self);

        self.arguments = args_before;
    }

    /// # Panics
    ///
    /// This panics if `lhs` or `rhs` can't be converted to internal C-Strings (so don't pass any
    /// `\0` chars!)
    ///
    pub fn noremap(&self, lhs: &str, rhs: &str) {
        set_noremap(self.mode.as_str(), lhs, rhs, Some(self.arguments)).unwrap();
    }

    /// # Panics
    ///
    /// This panics if `lhs` or `rhs` can't be converted to internal C-Strings (so don't pass any
    /// `\0` chars!)
    ///
    pub fn buf_map(&self, buffer: Buffer, lhs: &str, rhs: &str) {
        set_buf_map(buffer, self.mode.as_str(), lhs, rhs, Some(self.arguments)).unwrap();
    }

    /// # Panics
    ///
    /// This panics if `lhs` or `rhs` can't be converted to internal C-Strings (so don't pass any
    /// `\0` chars!)
    ///
    pub fn buf_noremap(&self, buffer: Buffer, lhs: &str, rhs: &str) {
        set_buf_noremap(buffer, self.mode.as_str(), lhs, rhs, Some(self.arguments)).unwrap();
    }

    def_bool_return_self_meth!(buffer);
    def_bool_return_self_meth!(expr);
    def_bool_return_self_meth!(nowait);
    def_bool_return_self_meth!(script);
    def_bool_return_self_meth!(silent);
    def_bool_return_self_meth!(unique);

    /// # Panics
    ///
    /// This will panic if the call to `nvim_api::keymaps::get_maps()` fails.
    ///
    #[must_use]
    pub fn list_all(&self) -> Vec<Mapping> {
        get_maps(self.mode.as_str())
            .unwrap_or_else(|_| panic!("Unable to retrive maps for mode '{}'", self.mode.as_str()))
            .into_iter()
            .map(Mapping::from)
            .collect()
    }
}

/// `Mapper::list_all()` calls an internal call that returns an array of dictionaries, where each
/// dictionary contains information about a mapping; this struct represents that dictionary.
///
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone)]
pub struct Mapping {
    mode: MapMode,
    lhs: String,
    rhs: String,
    sid: i64,
    lnum: i64,
    buffer: bool,
    expr: bool,
    noremap: bool,
    nowait: bool,
    script: bool,
    silent: bool,
}

impl Mapping {
    /// The `MapMode` for this mapping.
    ///
    #[must_use]
    pub const fn mode(&self) -> MapMode {
        self.mode
    }

    /// The left-hand side of the mapping.
    ///
    #[must_use]
    pub const fn lhs(&self) -> &String {
        &self.lhs
    }

    /// The right-hand side of the mapping.
    ///
    #[must_use]
    pub const fn rhs(&self) -> &String {
        &self.rhs
    }

    /// The `<SID>` the mapping was defined by.
    ///
    #[must_use]
    pub const fn sid(&self) -> i64 {
        self.sid
    }

    /// The line number from script `<SID>` where the mapping was defined.
    ///
    #[must_use]
    pub const fn lnum(&self) -> i64 {
        self.lnum
    }

    /// Is this a `<buffer>` mapping?
    ///
    #[must_use]
    pub const fn buffer(&self) -> bool {
        self.buffer
    }

    /// Is this an `<expr>` mapping?
    ///
    #[must_use]
    pub const fn expr(&self) -> bool {
        self.expr
    }

    /// Was this mapping made with `noremap`?
    ///
    #[must_use]
    pub const fn noremap(&self) -> bool {
        self.noremap
    }

    /// Is this an `<expr>` mapping?
    ///
    #[must_use]
    pub const fn nowait(&self) -> bool {
        self.nowait
    }

    /// Is this an `<expr>` mapping?
    ///
    #[must_use]
    pub const fn script(&self) -> bool {
        self.script
    }

    /// Is this an `<expr>` mapping?
    ///
    #[must_use]
    pub const fn silent(&self) -> bool {
        self.silent
    }
}

impl From<Dictionary> for Mapping {
    fn from(value: Dictionary) -> Self {
        Self {
            mode: MapMode::from(value.get_as_string("mode").unwrap()),
            lhs: value.get_as_string("lhs").unwrap().to_string(),
            rhs: value.get_as_string("rhs").unwrap().to_string(),
            lnum: value.get_as_integer("lnum").unwrap(),
            sid: value.get_as_integer("sid").unwrap(),
            buffer: value.get_as_boolean("buffer").unwrap(),
            expr: value.get_as_boolean("expr").unwrap(),
            noremap: value.get_as_boolean("noremap").unwrap(),
            nowait: value.get_as_boolean("nowait").unwrap(),
            script: value.get_as_boolean("script").unwrap(),
            silent: value.get_as_boolean("silent").unwrap(),
        }
    }
}
