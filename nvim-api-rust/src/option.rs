pub mod buf;
pub mod global;

pub use self::global::Global;

use crate::{api::Error, key_code::KeyCode};
use neovim_sys::api::vim::{LuaString, Object};
use std::{borrow::Cow, convert::TryFrom};

/// The trait that all options implement, allowing to define each option's long name (ex.
/// `autoindent`) and short name (ex. `ai`), as well as what type of value they expect. While vim's
/// docs say that options can be a a) `boolean`, b) `integer`, or c) `string`, 1) that's not quite
/// accurate and 2) we can do a bit better with `string`-related values by using Rust types.
///
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

#[allow(clippy::fallible_impl_from)]
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
        match value.as_string_unchecked().to_string_lossy() {
            Cow::Borrowed("single") => Ok(Self::Single),
            Cow::Borrowed("double") => Ok(Self::Double),
            s => Err(VimOptionError::UnexpectedDictionaryKey(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PasteToggle;

impl VimOption for PasteToggle {
    type Value = KeyCode;

    const SHORT_NAME: &'static str = "pt";
    const LONG_NAME: &'static str = "pastetoggle";
}

// #[derive(Debug, Clone, Copy)]
// pub enum Background {
//     Light,
//     Dark,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum Backspace {
//     Indent,
//     Eol,
//     Start,
//     NoStop,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum BackupCopy {
//     Yes,
//     No,
//     Auto,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum BackupCopyBreakLink {
//     BreakSymLink,
//     BreakHardLink,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum BellOff {
//     All,
//     Backspace,
//     Cursor,
//     Complete,
//     Copy,
//     CtrlG,
//     Error,
//     Esc,
//     Ex,
//     Hangul,
//     InsertMode,
//     Lang,
//     Mess,
//     ShowMatch,
//     Operator,
//     Register,
//     Shell,
//     Spell,
//     WildMode,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum BreakIndentOptItem {
//     Min(u16),
//     Shift(u16),
//     Sbr,
//     List(u16),
//     ListMinusOne,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum BrowseDir {
//     Last,
//     Buffer,
//     Current,
// }
