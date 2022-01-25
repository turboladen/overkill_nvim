//! Overkill API for getting and setting mappings.
//!
use super::{MapMode, Mapping};
use nvim_api::{
    keymap::{get_maps, set_buf_map, set_buf_noremap, set_map, set_noremap, SpecialArguments},
    Buffer,
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
    /// `\0` chars!).
    ///
    pub fn map(&self, lhs: &str, rhs: &str) {
        if self.arguments.any_set() {
            set_map(self.mode.as_str(), lhs, rhs, Some(self.arguments)).unwrap();
        } else {
            set_map(self.mode.as_str(), lhs, rhs, None).unwrap();
        }
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

    /// Gets a list of all mappings for the `Mapper`'s `mode`.
    ///
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

    /// Gets a list of mappings that match `Mapper`'s `mode` and `arguments`.
    /// Note that this doesn't check for `unique` or `noremap`.
    ///
    /// # Panics
    ///
    /// This will panic if the call to `nvim_api::keymaps::get_maps()` fails.
    ///
    #[must_use]
    pub fn list(&self) -> Vec<Mapping> {
        get_maps(self.mode.as_str())
            .unwrap_or_else(|_| panic!("Unable to retrive maps for mode '{}'", self.mode.as_str()))
            .into_iter()
            .map(Mapping::from)
            .filter(|mapping| {
                mapping.mode() == self.mode
                    && mapping.buffer() == self.arguments.is_buffer()
                    && mapping.expr() == self.arguments.is_expr()
                    && mapping.nowait() == self.arguments.is_nowait()
                    && mapping.script() == self.arguments.is_script()
                    && mapping.silent() == self.arguments.is_silent()
            })
            .collect()
    }
}
