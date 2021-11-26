use nvim_api_rs::sys::api::vim::{LuaString, Object};
use std::{borrow::Cow, convert::TryFrom};
use super::VimOptionError;

#[derive(Debug, Clone, Copy)]
pub enum IncCommandValue {
    NoSplit,
    Split,
}

impl From<IncCommandValue> for Object {
    fn from(value: IncCommandValue) -> Self {
        match value {
            IncCommandValue::NoSplit => Self::from(LuaString::new_unchecked("nosplit")),
            IncCommandValue::Split => (Self::from(LuaString::new_unchecked("split"))),
        }
    }
}

impl TryFrom<Object> for IncCommandValue {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.as_string_unchecked().to_string_lossy() {
            Cow::Borrowed("nosplit") => Ok(Self::NoSplit),
            Cow::Borrowed("split") => Ok(Self::Split),
            _ => Err(VimOptionError::UnexpectedOptionValue(value)),
        }
    }
}
