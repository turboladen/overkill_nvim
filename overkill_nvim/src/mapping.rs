use nvim_api::api::{Boolean, Integer, NvimString, Object};

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

impl From<Object> for Mapping {
    fn from(object: Object) -> Self {
        let value = object.into_dictionary_unchecked();

        Self {
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
        }
    }
}
