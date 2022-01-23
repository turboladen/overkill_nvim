use super::NvimOptionError;
use nvim_api::{Object, Integer};
use std::convert::TryFrom;

/// Represents an option value for `'conceallevel'`.
///
#[derive(Debug, Clone, Copy)]
pub enum ConcealLevelValue {
    /// `0`
    NormalText,

    /// `1`
    OneCharReplacement,

    /// `2`
    HiddenUnlessCustomReplacement,

    /// `3`
    Hidden,
}

impl From<ConcealLevelValue> for Integer {
    fn from(value: ConcealLevelValue) -> Self {
        match value {
            ConcealLevelValue::NormalText => 0,
            ConcealLevelValue::OneCharReplacement => 1,
            ConcealLevelValue::HiddenUnlessCustomReplacement => 2,
            ConcealLevelValue::Hidden => 3,
        }
    }
}

impl TryFrom<Integer> for ConcealLevelValue {
    type Error = NvimOptionError;

    fn try_from(value: Integer) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NormalText),
            1 => Ok(Self::OneCharReplacement),
            2 => Ok(Self::HiddenUnlessCustomReplacement),
            3 => Ok(Self::Hidden),
            _ => Err(NvimOptionError::UnexpectedOptionValue(Object::from(value))),
        }
    }
}
