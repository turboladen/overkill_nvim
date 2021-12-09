//! Functions here are from `nvim/api/private/*.h`. Kinda seems like maybe we shouldn't be wrapping
//! these, but they're exported, sooo....
//!
use crate::api::nvim::{LuaError, NvimString, Object};
use std::{ffi::c_void, os::raw::c_int};

extern "C" {
    /// Gets the value of a global or local (buffer, window) option.
    ///
    /// * If `opt_type` is `SReq::Win` or `SReq::Buf`, `from` must be a pointer to the window or
    /// buffer.
    /// * `opt_type` should be one `SReq`.
    /// * `name` is option name.
    /// * `error` is an out-pointer to capture any error that might occur during the call.
    ///
    pub fn get_option_from(
        from: *const c_void,
        opt_type: c_int,
        name: NvimString,
        error: *mut LuaError,
    ) -> Object;
}
