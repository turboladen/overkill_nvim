use super::{NvimOptionError, StringFlags};
use nvim_api::{Object, NvimString};
use std::{convert::TryFrom, num::NonZeroI64};

/// Allows for setting `'colorcolumn'` in a typed manner. When used with `StringFlags`, you can set
/// multiple values:
///
/// ```compile_fail
/// use overkill_nvim::option::{StringFlags, ColorColumn, ColorColumnValue, VimOption};
///
/// ColorColumn::set(StringFlags::new(vec![
///     ColorColumnValue::Absolute(80),
///     ColorColumnValue::Absolute(120),
/// ]))
/// .ok();
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorColumnValue {
    /// Sets the option using an absolute number (ex `:set colorcolumn=3`).
    ///
    Absolute(u32),

    /// Sets the option using a number relative to `'textwidth'` (ex `:set colorcolumn=+3` or `:set
    /// colorcolumn=-3`).
    ///
    Offset(NonZeroI64),
}

impl From<ColorColumnValue> for NvimString {
    fn from(value: ColorColumnValue) -> Self {
        let s = match value {
            ColorColumnValue::Absolute(column) => column.to_string(),
            ColorColumnValue::Offset(offset) => {
                let i = offset.get();

                if i.is_positive() {
                    format!("+{}", offset)
                } else {
                    i.to_string()
                }
            }
        };

        Self::new_unchecked(s)
    }
}

impl<'a> TryFrom<&'a str> for ColorColumnValue {
    type Error = NvimOptionError;

    fn try_from(item: &'a str) -> Result<Self, Self::Error> {
        if let Some(positive) = item.strip_prefix('+') {
            let offset = match positive.parse::<NonZeroI64>() {
                Ok(s) => s,
                Err(_) => {
                    return Err(NvimOptionError::UnexpectedOptionValue(
                        Object::try_from(item).unwrap(),
                    ))
                }
            };

            Ok(Self::Offset(offset))
        } else if item.starts_with('-') {
            Ok(Self::Offset(item.parse::<NonZeroI64>().map_err(|_| {
                NvimOptionError::UnexpectedOptionValue(Object::try_from(item).unwrap())
            })?))
        } else {
            Ok(Self::Absolute(item.parse::<u32>().map_err(|_| {
                NvimOptionError::UnexpectedOptionValue(Object::try_from(item).unwrap())
            })?))
        }
    }
}

impl TryFrom<NvimString> for StringFlags<ColorColumnValue> {
    type Error = NvimOptionError;

    fn try_from(string: NvimString) -> Result<Self, Self::Error> {
        let s = string.to_string_lossy();
        let split = s.split(',');
        let mut inner = Vec::with_capacity(split.size_hint().0);

        for item in split {
            inner.push(ColorColumnValue::try_from(item)?);
        }

        Ok(Self::new(inner))
    }
}
