use super::{StringFlags, VimOptionError};
use nvim_api_rs::sys::api::vim::Object;
use std::{convert::TryFrom, num::NonZeroI64};

impl TryFrom<Object> for StringFlags<ColorColumnValue> {
    type Error = VimOptionError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let lua_string = value.into_string_unchecked();
        let string = lua_string.to_string_lossy();
        let split = string.split(',');
        let mut inner = Vec::with_capacity(split.size_hint().0);

        for item in split {
            if let Some(positive) = item.strip_prefix('+') {
                inner.push(ColorColumnValue::Offset(
                    positive.parse::<NonZeroI64>().map_err(|_| {
                        VimOptionError::UnexpectedOptionValue(Object::from(lua_string.clone()))
                    })?,
                ));
            } else if item.starts_with('-') {
                inner.push(ColorColumnValue::Offset(
                    item.parse::<NonZeroI64>().map_err(|_| {
                        VimOptionError::UnexpectedOptionValue(Object::from(lua_string.clone()))
                    })?,
                ));
            } else {
                inner.push(ColorColumnValue::Absolute(item.parse::<u32>().map_err(
                    |_| VimOptionError::UnexpectedOptionValue(Object::from(lua_string.clone())),
                )?));
            }
        }

        Ok(Self::new(inner))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorColumnValue {
    Absolute(u32),
    Offset(NonZeroI64),
}

impl From<ColorColumnValue> for String {
    fn from(value: ColorColumnValue) -> Self {
        match value {
            ColorColumnValue::Absolute(column) => column.to_string(),
            ColorColumnValue::Offset(offset) => {
                let i = offset.get();

                if i.is_positive() {
                    format!("+{}", offset)
                } else {
                    i.to_string()
                }
            }
        }
    }
}
