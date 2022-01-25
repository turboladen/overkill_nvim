use super::NvimOptionError;
use nvim_api::{NvimString, Object};
use std::num::NonZeroU8;

/// Represents an option value for `'signcolumn'`.
///
#[derive(Debug, Clone, Copy)]
pub enum SignColumnValue {
    /// `"auto"`
    ///
    Auto,

    /// `"auto:[1-9]"`
    ///
    AutoWithMax(NonZeroU8),

    /// `"auto:[1-9]-[2-9]"`
    ///
    AutoWithMinMax((NonZeroU8, NonZeroU8)),

    /// `"no"`
    ///
    No,

    /// `"yes"`
    ///
    Yes,

    /// `"yes:[1-9]"`
    ///
    YesWithMax(NonZeroU8),

    /// `"number"`
    ///
    Number,
}

impl From<SignColumnValue> for NvimString {
    fn from(value: SignColumnValue) -> Self {
        match value {
            SignColumnValue::Auto => Self::new_unchecked("auto"),
            SignColumnValue::AutoWithMax(v) => Self::new_unchecked(format!("auto:{}", v)),
            SignColumnValue::AutoWithMinMax((min, max)) => {
                Self::new_unchecked(format!("auto:{}-{}", min, max))
            }
            SignColumnValue::No => Self::new_unchecked("no"),
            SignColumnValue::Yes => Self::new_unchecked("yes"),
            SignColumnValue::YesWithMax(v) => Self::new_unchecked(format!("yes:{}", v)),
            SignColumnValue::Number => Self::new_unchecked("number"),
        }
    }
}

impl TryFrom<NvimString> for SignColumnValue {
    type Error = NvimOptionError;

    fn try_from(value: NvimString) -> Result<Self, Self::Error> {
        let string = value.to_string_lossy();
        let mut split = string.split(':');

        let value = match (split.next().unwrap(), split.next()) {
            ("auto", Some(v)) if v.contains('-') => {
                let mut min_max = v.split('-');
                let min = min_max.next().unwrap().parse::<NonZeroU8>().unwrap();
                let max = min_max.next().unwrap().parse::<NonZeroU8>().unwrap();
                Self::AutoWithMinMax((min, max))
            }
            ("auto", Some(v)) => {
                let max = v.parse::<NonZeroU8>().unwrap();
                Self::AutoWithMax(max)
            }
            ("auto", None) => Self::Auto,
            ("no", None) => Self::No,
            ("yes", Some(v)) => {
                let max = v.parse::<NonZeroU8>().unwrap();
                Self::YesWithMax(max)
            }
            ("yes", None) => Self::Yes,
            ("number", None) => Self::Number,
            _ => return Err(NvimOptionError::UnexpectedOptionValue(Object::from(value))),
        };

        Ok(value)
    }
}
