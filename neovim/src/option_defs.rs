pub mod buf_option;
pub mod global_option;

use crate::api::Error;
use neovim_sys::api::{LuaString, Object};
use std::{borrow::Cow, convert::TryFrom, path::PathBuf};

pub trait VimOption
where
    Object: From<Self::Value>,
    Error: From<<<Self as VimOption>::Value as TryFrom<Object>>::Error>,
{
    type Value: TryFrom<Object>;

    const SHORT_NAME: &'static str;
    const LONG_NAME: &'static str;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum VimOptionError {
    #[error("Unexpected key in dictionary option: '{}'", _0)]
    UnexpectedDictionaryKey(String),
}

#[derive(Debug, Clone, Copy)]
pub struct Aleph;

impl VimOption for Aleph {
    type Value = i64;

    const SHORT_NAME: &'static str = "al";
    const LONG_NAME: &'static str = "aleph";
}

#[derive(Debug, Clone, Copy)]
pub struct AllowRevIns;

impl VimOption for AllowRevIns {
    type Value = bool;

    const SHORT_NAME: &'static str = "ari";
    const LONG_NAME: &'static str = "allowrevins";
}

#[derive(Debug, Clone, Copy)]
pub struct AmbiWidth;

impl VimOption for AmbiWidth {
    type Value = AmbiWidthOption;

    const SHORT_NAME: &'static str = "ambw";
    const LONG_NAME: &'static str = "ambiwidth";
}

#[derive(Debug, Clone, Copy)]
pub enum AmbiWidthOption {
    Single,
    Double,
}

impl From<AmbiWidthOption> for Object {
    fn from(value: AmbiWidthOption) -> Self {
        match value {
            AmbiWidthOption::Single => Self::from(LuaString::new("single").unwrap()),
            AmbiWidthOption::Double => (Self::from(LuaString::new("double").unwrap())),
        }
    }
}

impl TryFrom<Object> for AmbiWidthOption {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.as_string_unchecked().as_c_str().to_string_lossy() {
            Cow::Borrowed("single") => Ok(Self::Single),
            Cow::Borrowed("double") => Ok(Self::Double),
            s => Err(VimOptionError::UnexpectedDictionaryKey(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Background {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy)]
pub enum Backspace {
    Indent,
    Eol,
    Start,
    NoStop,
}

#[derive(Debug, Clone, Copy)]
pub enum BackupCopy {
    Yes,
    No,
    Auto,
}

#[derive(Debug, Clone, Copy)]
pub enum BackupCopyBreakLink {
    BreakSymLink,
    BreakHardLink,
}

#[derive(Debug, Clone, Copy)]
pub enum BellOff {
    All,
    Backspace,
    Cursor,
    Complete,
    Copy,
    CtrlG,
    Error,
    Esc,
    Ex,
    Hangul,
    InsertMode,
    Lang,
    Mess,
    ShowMatch,
    Operator,
    Register,
    Shell,
    Spell,
    WildMode,
}

#[derive(Debug, Clone, Copy)]
pub enum BreakIndentOptItem {
    Min(u16),
    Shift(u16),
    Sbr,
    List(u16),
    ListMinusOne,
}

#[derive(Debug, Clone, Copy)]
pub enum BrowseDir {
    Last,
    Buffer,
    Current,
}
