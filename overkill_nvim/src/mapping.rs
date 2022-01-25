//! This module contains types and functions for mapping commands.
//!

pub mod map_mode;
pub mod mapper;

pub use self::{map_mode::MapMode, mapper::Mapper};

use nvim_api::Dictionary;

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
