use neovim_sys::api::vim::{Dictionary, LuaString};
use std::convert::TryFrom;

/// Represents any possible neovim "mode".
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Normal mode; maps to "n".
    ///
    Normal,

    /// Insert mode; maps to "i".
    ///
    Insert,

    /// Replace mode; maps to "R".
    ///
    Replace,

    /// Visual mode; maps to "v".
    ///
    Visual,

    /// Visual-line mode; maps to "V".
    ///
    VisualLine,

    /// Visual-block mode; maps to "C-v".
    ///
    VisualBlock,

    /// Command mode; maps to "c".
    ///
    Command,

    /// Select mode; maps to "s".
    ///
    Select,

    /// Select-line mode; maps to "S.
    ///
    SelectLine,

    /// Select-block mode; maps to "C-s".
    ///
    SelectBlock,

    /// Terminal mode; maps to "t".
    ///
    Terminal,
}

impl From<&str> for Mode {
    fn from(mode: &str) -> Self {
        match mode {
            "n" => Self::Normal,
            "i" => Self::Insert,
            "R" => Self::Replace,
            "v" => Self::Visual,
            "V" => Self::VisualLine,
            "<C-v>" => Self::VisualBlock,
            "c" => Self::Command,
            "s" => Self::Select,
            "S" => Self::SelectLine,
            "<C-s>" => Self::SelectBlock,
            "t" => Self::Terminal,
            m => {
                eprintln!("unknown mode {}, falling back to Mode::Normal", m);
                Self::Normal
            }
        }
    }
}

impl From<&LuaString> for Mode {
    fn from(mode: &LuaString) -> Self {
        match mode.as_c_str().to_string_lossy().as_ref() {
            "n" => Self::Normal,
            "i" => Self::Insert,
            "R" => Self::Replace,
            "v" => Self::Visual,
            "V" => Self::VisualLine,
            "<C-v>" => Self::VisualBlock,
            "c" => Self::Command,
            "s" => Self::Select,
            "S" => Self::SelectLine,
            "<C-s>" => Self::SelectBlock,
            "t" => Self::Terminal,
            m => {
                eprintln!("unknown mode {}, falling back to Mode::Normal", m);
                Self::Normal
            }
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<Mode> for LuaString {
    fn from(mode: Mode) -> Self {
        let s = match mode {
            Mode::Normal => "n",
            Mode::Insert => "i",
            Mode::Replace => "R",
            Mode::Visual => "v",
            Mode::VisualLine => "V",
            Mode::VisualBlock => "C-v",
            Mode::Command => "c",
            Mode::Select => "s",
            Mode::SelectLine => "S",
            Mode::SelectBlock => "<C-s>",
            Mode::Terminal => "t",
        };
        Self::new(s).unwrap()
    }
}

/// Returned by `nvim_get_mode()`.
///
#[derive(Debug, Clone, Copy)]
#[allow(clippy::module_name_repetitions)]
pub struct CurrentMode {
    mode: Mode,
    blocking: bool,
}

impl CurrentMode {
    /// The current mode.
    ///
    pub const fn mode(self) -> Mode {
        self.mode
    }

    /// `true` if Nvim is waiting for input.
    ///
    pub const fn blocking(self) -> bool {
        self.blocking
    }
}

impl TryFrom<Dictionary> for CurrentMode {
    type Error = CurrentModeError;

    fn try_from(dict: Dictionary) -> Result<Self, Self::Error> {
        match (dict.get("mode"), dict.get("blocking")) {
            (Some(mode), Some(blocking)) if mode.is_string() && blocking.is_boolean() => Ok(Self {
                mode: Mode::from(mode.as_string_unchecked()),
                blocking: blocking.as_boolean_unchecked(),
            }),
            (None, Some(_)) => Err(CurrentModeError::Mode),
            (Some(_), None) => Err(CurrentModeError::Blocking),
            _ => Err(CurrentModeError::ModeAndBlocking),
        }
    }
}

/// Error type for instantiating a `CurrentMode` from the `Dictionary` returned by neovim on
/// related calls.
///
#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum CurrentModeError {
    #[error("Underlying neovim dictionary did not have the `mode` key/value pair set")]
    Mode,

    #[error("Underlying neovim dictionary did not have the `blocking` key/value pair set")]
    Blocking,

    #[error(
        "Underlying neovim dictionary did not have the `mode` or `blocking` key/value pairs set"
    )]
    ModeAndBlocking,
}

#[cfg(test)]
mod tests {
    use super::*;
    use neovim_sys::api::vim::{KeyValuePair, Object};

    #[test]
    fn try_from_dictionary_test() {
        let dict = Dictionary::new([
            KeyValuePair::new(
                LuaString::new_unchecked("mode"),
                Object::from(LuaString::new_unchecked("n")),
            ),
            KeyValuePair::new(LuaString::new_unchecked("blocking"), Object::from(false)),
        ]);
        let current_mode = CurrentMode::try_from(dict).unwrap();
        assert_eq!(current_mode.mode(), Mode::Normal);
    }
}
