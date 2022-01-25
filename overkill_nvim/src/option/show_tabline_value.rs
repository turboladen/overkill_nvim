use super::NvimOptionError;
use nvim_api::{Integer, Object};
use std::convert::TryFrom;

/// Represents an option value for `'showtabline'`.
///
#[derive(Debug, Clone, Copy)]
pub enum ShowTablineValue {
    /// `0`
    ///
    Never,

    /// `1`
    ///
    OnlyIfTabPages,

    /// `2`
    ///
    Always,
}

impl From<ShowTablineValue> for Integer {
    fn from(value: ShowTablineValue) -> Self {
        match value {
            ShowTablineValue::Never => 0,
            ShowTablineValue::OnlyIfTabPages => 1,
            ShowTablineValue::Always => 2,
        }
    }
}

impl TryFrom<Integer> for ShowTablineValue {
    type Error = NvimOptionError;

    fn try_from(value: Integer) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Never),
            1 => Ok(Self::OnlyIfTabPages),
            2 => Ok(Self::Always),
            _ => Err(NvimOptionError::UnexpectedOptionValue(Object::from(value))),
        }
    }
}
