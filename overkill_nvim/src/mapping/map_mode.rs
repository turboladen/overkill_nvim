//! Defines a type for each of the seven map-modes (see `:help map-modes`) and their supported
//! permutations.
//!
use nvim_api::{NvimString, sys::getchar};

/// Represents any possible neovim "mode".
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapMode {
    /// Normal, visual, select, operator-pending modes; "map".
    ///
    NormalVisualSelectOperatorPending,

    /// Normal mode; "nmap".
    ///
    Normal,

    /// Visual and select modes; "vmap".
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

    /// Insert and command-line modes (ex. "map!").
    ///
    InsertAndCommandLine,

    /// Insert mode (ex. "imap").
    ///
    Insert,

    /// Insert, command-line, and lang-arg modes (ex. "lmap").
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
    pub const fn as_str(&self) -> &str {
        match self {
            MapMode::Normal => "n",
            MapMode::VisualSelect => "v",
            MapMode::Visual => "x",
            MapMode::Select => "s",
            MapMode::OperatorPending => "o",
            MapMode::Insert => "i",
            MapMode::CommandLine => "c",
            MapMode::TerminalJob => "t",
            MapMode::InsertAndCommandLine => "!",
            MapMode::LanguageMapping => "l",
            MapMode::NormalVisualSelectOperatorPending => "",
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
        Self::new(mode.as_str()).unwrap()
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
            MapMode::InsertAndCommandLine => Self::InsertCmdLine,
            MapMode::Insert => Self::Insert,
            MapMode::LanguageMapping => Self::LangMap,
            MapMode::CommandLine => Self::CmdLine,
            MapMode::TerminalJob => Self::TermFocus,
            MapMode::NormalVisualSelectOperatorPending => Self::NormalVisualSelectOpPending,
        }
    }
}
