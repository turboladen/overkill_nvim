use super::VimOption;
use crate::api::error::Error;
use neovim_sys::api::{buffer::Buffer, vim::Object};
use std::convert::TryFrom;

pub trait Buf: VimOption
where
    Object: From<Self::Value>,
    Error: From<<Self::Value as TryFrom<Object>>::Error>,
{
    /// Calls `nvim_buf_get_option()`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn get(buffer: Buffer) -> Result<Object, Error> {
        crate::api::buffer::nvim_buf_get_option(buffer, Self::SHORT_NAME)
    }

    /// Calls `nvim_buf_get_option()`, but handles converting the resulting nvim Object into
    /// `Self::Value` type.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn get_as_value(buffer: Buffer) -> Result<Self::Value, Error> {
        Self::get(buffer).and_then(|object| Self::Value::try_from(object).map_err(Error::from))
    }

    /// Calls `nvim_buf_set_option()`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set(buffer: Buffer, value: Object) -> Result<(), Error> {
        crate::api::buffer::nvim_buf_set_option(buffer, Self::SHORT_NAME, value)
    }

    /// Calls `nvim_buf_set_option()`, but handles converting the `value` param from a `Self::Value`
    /// type to a nvim `Object`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set_as_value(buffer: Buffer, value: Self::Value) -> Result<(), Error> {
        Self::set(buffer, Object::from(value))
    }
}
