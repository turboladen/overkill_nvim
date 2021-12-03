use super::NvimOptionError;
use nvim_api::{api::Object, sys::api::nvim::NvimString};
use std::{borrow::Cow, convert::TryFrom};

/// Represents an option value for `'inccommand'`.
///
#[derive(Debug, Clone, Copy)]
pub enum IncCommandValue {
    /// `"nosplit"`
    ///
    NoSplit,
    /// `"split"`
    ///
    Split,
}

impl From<IncCommandValue> for NvimString {
    fn from(value: IncCommandValue) -> Self {
        match value {
            IncCommandValue::NoSplit => Self::new_unchecked("nosplit"),
            IncCommandValue::Split => Self::new_unchecked("split"),
        }
    }
}

impl TryFrom<NvimString> for IncCommandValue {
    type Error = NvimOptionError;

    fn try_from(value: NvimString) -> Result<Self, Self::Error> {
        match value.to_string_lossy() {
            Cow::Borrowed("nosplit") => Ok(Self::NoSplit),
            Cow::Borrowed("split") => Ok(Self::Split),
            _ => Err(NvimOptionError::UnexpectedOptionValue(Object::from(value))),
        }
    }
}
