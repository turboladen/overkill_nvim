use neovim_sys::api::vim::{self, String as LuaString};
use std::ffi::NulError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error from neovim: {}", .0)]
    NvimError(#[from] vim::NvimError),

    #[error(transparent)]
    NulError(#[from] NulError),

    #[error("v:errmsg: '{}'", .0)]
    VErrMsg(LuaString),
}
