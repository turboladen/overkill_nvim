//! This module contains types and functions for mapping commands.
//!
use nvim_api::{api::NvimString, sys::getchar};

// #[derive(Debug, Clone)]
// pub struct MappingDictionary {
//     mode: NvimString,
//     lhs: NvimString,
//     rhs: NvimString,
//     sid: Integer,
//     lnum: Integer,
//     buffer: Boolean,
//     expr: Boolean,
//     noremap: Boolean,
//     nowait: Boolean,
//     script: Boolean,
//     silent: Boolean,
// }

// impl MappingDictionary {
//     /// Get a reference to the mapping's mode.
//     #[must_use]
//     pub const fn mode(&self) -> &NvimString {
//         &self.mode
//     }

//     /// Get a reference to the mapping's lhs.
//     #[must_use]
//     pub const fn lhs(&self) -> &NvimString {
//         &self.lhs
//     }

//     /// Get a reference to the mapping's rhs.
//     #[must_use]
//     pub const fn rhs(&self) -> &NvimString {
//         &self.rhs
//     }

//     /// Get a reference to the mapping's sid.
//     #[must_use]
//     pub const fn sid(&self) -> i64 {
//         self.sid
//     }

//     /// Get a reference to the mapping's lnum.
//     #[must_use]
//     pub const fn lnum(&self) -> i64 {
//         self.lnum
//     }

//     /// Get a reference to the mapping's buffer.
//     #[must_use]
//     pub const fn buffer(&self) -> bool {
//         self.buffer
//     }

//     /// Get a reference to the mapping's expr.
//     #[must_use]
//     pub const fn expr(&self) -> bool {
//         self.expr
//     }

//     /// Get a reference to the mapping's noremap.
//     #[must_use]
//     pub const fn noremap(&self) -> bool {
//         self.noremap
//     }

//     /// Get a reference to the mapping's nowait.
//     #[must_use]
//     pub const fn nowait(&self) -> bool {
//         self.nowait
//     }

//     /// Get a reference to the mapping's script.
//     #[must_use]
//     pub const fn script(&self) -> bool {
//         self.script
//     }

//     /// Get a reference to the mapping's silent.
//     #[must_use]
//     pub const fn silent(&self) -> bool {
//         self.silent
//     }
// }

// impl From<Object> for MappingDictionary {
//     fn from(object: Object) -> Self {
//         let value = object.into_dictionary_unchecked();

//         Self {
//             mode: value.get_as_string("mode").unwrap().clone(),
//             lhs: value.get_as_string("lhs").unwrap().clone(),
//             rhs: value.get_as_string("rhs").unwrap().clone(),
//             lnum: value.get_as_integer("lnum").unwrap(),
//             sid: value.get_as_integer("sid").unwrap(),
//             buffer: value.get_as_boolean("buffer").unwrap(),
//             expr: value.get_as_boolean("expr").unwrap(),
//             noremap: value.get_as_boolean("noremap").unwrap(),
//             nowait: value.get_as_boolean("nowait").unwrap(),
//             script: value.get_as_boolean("script").unwrap(),
//             silent: value.get_as_boolean("silent").unwrap(),
//         }
//     }
// }

/// Represents any possible neovim "mode".
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapMode {
    /// Normal mode; "nmap".
    ///
    Normal,

    /// Visual-select mode; "vmap".
    ///
    VisualSelect,

    /// Select mode; "smap".
    ///
    Select,

    /// Visual mode; "xmap".
    ///
    Visual,

    /// Operator-pending mode (ex. "omap").
    ///
    OperatorPending,

    /// Insert & command-line (ex. "map!").
    ///
    InsertAndCommandLine,

    /// Insert mode (ex. "imap").
    ///
    Insert,

    /// Lang-map mode (ex. "lmap").
    ///
    LanguageMapping,

    /// Command mode (ex. "cmap").
    ///
    CommandLine,

    /// Terminal-job mode (ex. "tmap").
    ///
    TerminalJob,
}

impl MapMode {
    /// The single char that represents the `MapMode` (ex. "n" is for normal mode, etc.).
    ///
    #[must_use]
    pub const fn abbreviation(&self) -> char {
        match self {
            MapMode::Normal => 'n',
            MapMode::VisualSelect => 'v',
            MapMode::Select => 's',
            MapMode::Visual => 'x',
            MapMode::OperatorPending => 'o',
            MapMode::InsertAndCommandLine => '!',
            MapMode::Insert => 'i',
            MapMode::LanguageMapping => 'l',
            MapMode::CommandLine => 'c',
            MapMode::TerminalJob => 't',
        }
    }
}

impl From<&str> for MapMode {
    fn from(mode: &str) -> Self {
        match mode {
            "n" => Self::Normal,
            "v" => Self::VisualSelect,
            "s" => Self::Select,
            "x" => Self::Visual,
            "o" => Self::OperatorPending,
            "!" => Self::InsertAndCommandLine,
            "i" => Self::Insert,
            "l" => Self::LanguageMapping,
            "c" => Self::CommandLine,
            "t" => Self::TerminalJob,
            m => {
                eprintln!("unknown mode {}, falling back to Mode::Normal", m);
                Self::Normal
            }
        }
    }
}

impl From<&NvimString> for MapMode {
    fn from(mode: &NvimString) -> Self {
        mode.as_c_str().to_string_lossy().as_ref().into()
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<MapMode> for NvimString {
    fn from(mode: MapMode) -> Self {
        let mut buf: [u8; 1] = [0; 1];
        let my_str: &str = mode.abbreviation().encode_utf8(&mut buf);
        Self::new(my_str).unwrap()
    }
}

impl From<MapMode> for getchar::Mode {
    fn from(api_mode: MapMode) -> Self {
        match api_mode {
            MapMode::Normal => Self::Normal,
            MapMode::VisualSelect => Self::VisualSelectMode,
            MapMode::Select => Self::SelectMode,
            MapMode::Visual => Self::Visual,
            MapMode::OperatorPending => Self::OpPending,
            MapMode::InsertAndCommandLine => todo!(),
            MapMode::Insert => Self::Insert,
            MapMode::LanguageMapping => Self::LangMap,
            MapMode::CommandLine => Self::CmdLine,
            MapMode::TerminalJob => Self::TermFocus,
        }
    }
}
