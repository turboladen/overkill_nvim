//! Defines a type for each of the seven map-modes (see `:help map-modes`) and their supported
//! permutations.
//!
use nvim_api::{sys::getchar, NvimString};

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
            "" => Self::NormalVisualSelectOperatorPending,
            "n" => Self::Normal,
            "v" => Self::VisualSelect,
            "x" => Self::Visual,
            "s" => Self::Select,
            "o" => Self::OperatorPending,
            "i" => Self::Insert,
            "!" => Self::InsertAndCommandLine,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(MapMode::Normal.as_str(), "n");
        assert_eq!(MapMode::VisualSelect.as_str(), "v");
        assert_eq!(MapMode::Visual.as_str(), "x");
        assert_eq!(MapMode::Select.as_str(), "s");
        assert_eq!(MapMode::OperatorPending.as_str(), "o");
        assert_eq!(MapMode::Insert.as_str(), "i");
        assert_eq!(MapMode::CommandLine.as_str(), "c");
        assert_eq!(MapMode::TerminalJob.as_str(), "t");
        assert_eq!(MapMode::InsertAndCommandLine.as_str(), "!");
        assert_eq!(MapMode::LanguageMapping.as_str(), "l");
        assert_eq!(MapMode::NormalVisualSelectOperatorPending.as_str(), "");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(MapMode::from("n"), MapMode::Normal);
        assert_eq!(MapMode::from("v"), MapMode::VisualSelect);
        assert_eq!(MapMode::from("x"), MapMode::Visual);
        assert_eq!(MapMode::from("s"), MapMode::Select);
        assert_eq!(MapMode::from("o"), MapMode::OperatorPending);
        assert_eq!(MapMode::from("i"), MapMode::Insert);
        assert_eq!(MapMode::from("c"), MapMode::CommandLine);
        assert_eq!(MapMode::from("t"), MapMode::TerminalJob);
        assert_eq!(MapMode::from("!"), MapMode::InsertAndCommandLine);
        assert_eq!(MapMode::from("l"), MapMode::LanguageMapping);
        assert_eq!(
            MapMode::from(""),
            MapMode::NormalVisualSelectOperatorPending
        );
        assert_eq!(MapMode::from("anything else"), MapMode::Normal);
    }
}
