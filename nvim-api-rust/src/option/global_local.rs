use super::VimOption;
use crate::api::Error;
use neovim_sys::api::vim::Object;

pub trait GlobalLocal: VimOption
where
    Object: From<Self::Value>,
    Error: From<<Self::Value as TryFrom<Object>>::Error>,
{
    /// Calls `nvim_get_var()`, but handles converting the resulting nvim Object into
    /// `Self::Value` type.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn get() -> Result<Self::Value, Error> {
        crate::api::vim::nvim_get_global_local_option(Self::SHORT_NAME)
            .and_then(|object| Self::Value::try_from(object).map_err(Error::from))
    }

    /// Calls `nvim_set_option()`, but handles converting the `value` param from a `Self::Value`
    /// type to a nvim `Object`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set(value: Self::Value) -> Result<(), Error> {
        crate::api::vim::nvim_set_global_local_option(Self::SHORT_NAME, Object::from(value))
    }
}

impl GlobalLocal for super::AutoIndent {}
impl GlobalLocal for super::ColorColumn {}
impl GlobalLocal for super::CursorLine {}
impl GlobalLocal for super::ExpandTab {}
impl GlobalLocal for super::FoldEnable {}
impl GlobalLocal for super::ShiftWidth {}
impl GlobalLocal for super::SoftTabStop {}
impl GlobalLocal for super::Spell {}
impl GlobalLocal for super::SpellLang {}
impl GlobalLocal for super::SmartIndent {}
impl GlobalLocal for super::SwapFile {}
impl GlobalLocal for super::TabStop {}
impl GlobalLocal for super::UndoFile {}
