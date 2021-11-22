use super::VimOption;
use crate::api::Error;
use neovim_sys::api::vim::Object;

pub trait Global: VimOption
where
    Object: From<Self::Value>,
    Error: From<<Self::Value as TryFrom<Object>>::Error>,
{
    /// Calls `nvim_get_option()`.
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
    fn get_as_value() -> Result<Self::Value, Error> {
        Self::get().and_then(|object| Self::Value::try_from(object).map_err(Error::from))
    }

    /// Calls `nvim_set_option()`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set(value: Object) -> Result<(), Error> {
        crate::api::vim::nvim_set_option(Self::SHORT_NAME, value)
    }

    /// Calls `nvim_set_option()`, but handles converting the `value` param from a `Self::Value`
    /// type to a nvim `Object`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set_as_value(value: Self::Value) -> Result<(), Error> {
        Self::set(Object::from(value))
    }
}

impl Global for super::PasteToggle {}
impl Global for super::SmartCase {}
