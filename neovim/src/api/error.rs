use neovim_sys::api::vim::{self, LuaString};
use std::ffi::NulError;

/// The general error type for handling errors.
///
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// These errors are the error you see when reading neovim documention, where the type is
    /// `Error`.
    ///
    #[error("Error from neovim: {}", .0)]
    NvimError(#[from] vim::NvimError),

    /// This class of errors only happens when dealing with `neovim_sys::vim::Object`s, when the
    /// type of object isn't what was expected.
    ///
    #[error("neovim Object error: {}", .0)]
    ObjectError(#[from] vim::object::Error),

    /// Specifically related to `LuaString` errors, this can happen if you try to work with a
    /// string that has a nul byte in it.
    ///
    #[error(transparent)]
    NulError(#[from] NulError),

    /// Some neovim calls will set `v:errmsg` instead of returning an error; this variant captures
    /// those.
    ///
    #[error("v:errmsg: '{}'", .0)]
    VErrMsg(LuaString),
}
