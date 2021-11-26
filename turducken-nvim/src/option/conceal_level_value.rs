use super::VimOptionError;
use nvim_api_rs::sys::api::vim::Object;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
pub enum ConcealLevelValue {
    NormalText,
    OneCharReplacement,
    HiddenUnlessCustomReplacement,
    Hidden,
}

impl From<ConcealLevelValue> for Object {
    fn from(value: ConcealLevelValue) -> Self {
        match value {
            ConcealLevelValue::NormalText => Self::from(0),
            ConcealLevelValue::OneCharReplacement => Self::from(1),
            ConcealLevelValue::HiddenUnlessCustomReplacement => Self::from(2),
            ConcealLevelValue::Hidden => Self::from(3),
        }
    }
}

impl TryFrom<Object> for ConcealLevelValue {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.as_integer_unchecked() {
            0 => Ok(Self::NormalText),
            1 => Ok(Self::OneCharReplacement),
            2 => Ok(Self::HiddenUnlessCustomReplacement),
            3 => Ok(Self::Hidden),
            _ => Err(VimOptionError::UnexpectedOptionValue(value)),
        }
    }
}
