use crate::api::Error;
use neovim_sys::api::{buffer::Buffer, Object};

pub trait GlobalOption
where
    Object: TryFrom<<Self as GlobalOption>::Value>,
{
    type Value;

    const SHORT_NAME: &'static str;
    const LONG_NAME: &'static str;

    /// Calls `nvim_buf_get_var()`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn get() -> Result<Object, Error> {
        crate::api::vim::nvim_get_option(Self::SHORT_NAME)
    }

    /// Calls `nvim_get_var()`, but handles converting the resulting nvim Object into
    /// `Self::Value` type.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn get_as_value() -> Result<Self::Value, Error>;

    /// Calls `nvim_set_var()`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set(value: Object) -> Result<(), Error> {
        crate::api::vim::nvim_set_option(Self::SHORT_NAME, value)
    }

    /// Calls `nvim_buf_set_var()`, but handles converting the `value` param from a `Self::Value`
    /// type to a nvim `Object`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set_as_value(buffer: Buffer, value: Self::Value) -> Result<(), Error>;
}
