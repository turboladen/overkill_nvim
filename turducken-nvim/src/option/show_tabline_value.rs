use super::VimOptionError;
use nvim_api_rs::sys::api::vim::Object;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
pub enum ShowTablineValue {
    Never,
    OnlyIfTabPages,
    Always,
}

impl From<ShowTablineValue> for Object {
    fn from(value: ShowTablineValue) -> Self {
        match value {
            ShowTablineValue::Never => Self::from(0),
            ShowTablineValue::OnlyIfTabPages => Self::from(1),
            ShowTablineValue::Always => Self::from(2),
        }
    }
}

impl TryFrom<Object> for ShowTablineValue {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.as_integer_unchecked() {
            0 => Ok(Self::Never),
            1 => Ok(Self::OnlyIfTabPages),
            2 => Ok(Self::Always),
            _ => Err(VimOptionError::UnexpectedOptionValue(value)),
        }
    }
}
