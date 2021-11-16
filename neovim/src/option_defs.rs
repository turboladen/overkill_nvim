use crate::api::Error;
use neovim_sys::api::{
    buffer::Buffer,
    vim::{Integer, LuaString, Object},
};
use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
    path::PathBuf,
};

pub trait BufOption
where
    Object: TryFrom<<Self as BufOption>::Value>,
{
    type Value;

    const SHORT_NAME: &'static str;
    const LONG_NAME: &'static str;

    /// Calls `nvim_buf_get_var()`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn get(buffer: Buffer) -> Result<Object, Error> {
        crate::api::nvim_buf_get_option(buffer, Self::SHORT_NAME)
    }

    /// Calls `nvim_buf_get_var()`, but handles converting the resulting nvim Object into
    /// `Self::Value` type.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn get_as_value(buffer: Buffer) -> Result<Self::Value, Error>;

    /// Calls `nvim_buf_set_var()`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set(buffer: Buffer, value: Object) -> Result<(), Error> {
        crate::api::nvim_buf_set_var(buffer, Self::SHORT_NAME, value)
    }

    /// Calls `nvim_buf_set_var()`, but handles converting the `value` param from a `Self::Value`
    /// type to a nvim `Object`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set_as_value(buffer: Buffer, value: Self::Value) -> Result<(), Error>;
}

#[derive(Debug, Clone, Copy)]
pub struct Aleph;

impl BufOption for Aleph {
    type Value = Integer;

    const SHORT_NAME: &'static str = "al";
    const LONG_NAME: &'static str = "aleph";

    fn get_as_value(buffer: Buffer) -> Result<Self::Value, Error> {
        Self::get(buffer).map(|object| object.as_integer_unchecked())
    }

    fn set_as_value(buffer: Buffer, value: Self::Value) -> Result<(), Error> {
        Self::set(buffer, Object::from(value))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AllowRevIns;

impl BufOption for AllowRevIns {
    type Value = bool;

    const SHORT_NAME: &'static str = "ari";
    const LONG_NAME: &'static str = "allowrevins";

    fn get_as_value(buffer: Buffer) -> Result<Self::Value, Error> {
        Self::get(buffer).map(|object| object.as_boolean_unchecked())
    }

    fn set_as_value(buffer: Buffer, value: Self::Value) -> Result<(), Error> {
        Self::set(buffer, Object::from(value))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AmbiWidth;

impl BufOption for AmbiWidth {
    type Value = AmbiWidthOption;

    const SHORT_NAME: &'static str = "ambw";
    const LONG_NAME: &'static str = "ambiwidth";

    fn get_as_value(buffer: Buffer) -> Result<Self::Value, Error> {
        Self::get(buffer).and_then(|object| object.try_into())
    }

    fn set_as_value(buffer: Buffer, value: Self::Value) -> Result<(), Error> {
        let o = Object::try_from(value)?;
        Self::set(buffer, o)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AmbiWidthOption {
    Single,
    Double,
}

impl TryFrom<AmbiWidthOption> for Object {
    type Error = Error;

    fn try_from(value: AmbiWidthOption) -> Result<Self, Self::Error> {
        match value {
            AmbiWidthOption::Single => Ok(Self::from(LuaString::new("single")?)),
            AmbiWidthOption::Double => Ok(Self::from(LuaString::new("double")?)),
        }
    }
}

impl TryFrom<Object> for AmbiWidthOption {
    type Error = Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.as_string_unchecked().as_c_str().to_string_lossy() {
            Cow::Borrowed("single") => Ok(Self::Single),
            Cow::Borrowed("double") => Ok(Self::Double),
            s => Err(Error::Blargh(format!(
                "Got unexpected value for ambiwidth: '{}'",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub enum VimOption {
    AmbiWidth(AmbiWidth),
    AntiAlias(bool),
    AutoChdir(bool),
    AutoShellDir(bool),
    Arabic(bool),
    ArabicShape(bool),
    AutoIndent(bool),
    AutoRead(bool),
    AutoWrite(bool),
    AutoWriteAll(bool),
    Background(Background),
    Backspace(Vec<Backspace>),
    Backup(bool),
    BackupCopy(BackupCopy, Option<BackupCopyBreakLink>),
    BackupDir(Vec<PathBuf>),
    BackupExt(String),
    BackupSkip(Vec<PathBuf>),
    BaloonDelay(u16),
    BaloonEval(bool),
    BaloonEvalTerm(bool),
    BaloonExpr(String),
    BellOff(Vec<BellOff>),
    Binary(bool),
    Bomb(bool),
    BreakAt(String),
    BreakIndent(bool),
    BreakIndentOpt(Vec<BreakIndentOptItem>),
    BrowseDir(BrowseDir),
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
