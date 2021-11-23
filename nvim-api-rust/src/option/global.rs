use super::VimOption;
use crate::api::Error;
use neovim_sys::api::vim::Object;

pub trait Global: VimOption
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
    fn get_global() -> Result<Self::Value, Error> {
        crate::api::vim::nvim_get_global_option(Self::SHORT_NAME)
            .and_then(|object| Self::Value::try_from(object).map_err(Error::from))
    }

    /// Calls `nvim_set_option()`, but handles converting the `value` param from a `Self::Value`
    /// type to a nvim `Object`.
    ///
    /// # Errors
    ///
    /// Errors if nvim errors on the call.
    ///
    fn set_global(value: Self::Value) -> Result<(), Error> {
        crate::api::vim::nvim_set_global_option(Self::SHORT_NAME, Object::from(value))
    }
}

impl Global for super::BreakIndent {}
impl Global for super::CmdHeight {}
impl Global for super::ColorColumn {}
impl Global for super::CursorLine {}
impl Global for super::IncCommand {}
impl Global for super::LineBreak {}
impl Global for super::List {}
impl Global for super::ListChars {}
impl Global for super::Number {}
impl Global for super::PasteToggle {}
impl Global for super::ScrollOff {}
impl Global for super::ShowTabline {}
impl Global for super::SmartCase {}
impl Global for super::Spell {}
impl Global for super::SynMaxCol {}
impl Global for super::TermGuiColors {}
