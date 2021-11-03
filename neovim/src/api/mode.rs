use std::convert::TryFrom;

use neovim_sys::api::vim::{Dictionary, String as LuaString};

use super::Error;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
    Replace,
    Visual,
    VisualLine,
    VisualBlock,
    Command,
    Select,
    SelectLine,
    SelectBlock,
    Terminal,
}

impl From<&str> for Mode {
    fn from(mode: &str) -> Self {
        match mode {
            "n" => Mode::Normal,
            "i" => Mode::Insert,
            "R" => Mode::Replace,
            "v" => Mode::Visual,
            "V" => Mode::VisualLine,
            "<C-v>" => Mode::VisualBlock,
            "c" => Mode::Command,
            "s" => Mode::Select,
            "S" => Mode::SelectLine,
            "<C-s>" => Mode::SelectBlock,
            "t" => Mode::Terminal,
            m => {
                eprintln!("unknown mode {}, falling back to Mode::Normal", m);
                Mode::Normal
            }
        }
    }
}

impl From<&LuaString> for Mode {
    fn from(mode: &LuaString) -> Self {
        match mode.as_c_str().to_string_lossy().as_ref() {
            "n" => Mode::Normal,
            "i" => Mode::Insert,
            "R" => Mode::Replace,
            "v" => Mode::Visual,
            "V" => Mode::VisualLine,
            "<C-v>" => Mode::VisualBlock,
            "c" => Mode::Command,
            "s" => Mode::Select,
            "S" => Mode::SelectLine,
            "<C-s>" => Mode::SelectBlock,
            "t" => Mode::Terminal,
            m => {
                eprintln!("unknown mode {}, falling back to Mode::Normal", m);
                Mode::Normal
            }
        }
    }
}

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
        LuaString::new(s).unwrap()
    }
}

/// Returned by `nvim_get_mode()`.
///
#[derive(Debug, Clone, Copy)]
pub struct CurrentMode {
    mode: Mode,
    blocking: bool,
}

impl CurrentMode {
    /// The current mode.
    ///
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// `true` if Nvim is waiting for input.
    ///
    pub fn blocking(&self) -> bool {
        self.blocking
    }
}

impl TryFrom<Dictionary> for CurrentMode {
    type Error = Error;

    fn try_from(dict: Dictionary) -> Result<Self, Self::Error> {
        match (dict.get("mode"), dict.get("blocking")) {
            (Some(mode), Some(blocking)) => Ok(Self {
                blocking: blocking.try_as_boolean()?,
                mode: mode
                    .try_as_string()
                    .map(Mode::from)?,
            }),
            _ => Err(Error::Blargh("meow".into())),
        }
    }
}
