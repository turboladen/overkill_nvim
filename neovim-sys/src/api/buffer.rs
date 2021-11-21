//!
//! This module contains function wrappers that are defined in `neovim/src/nvim/api/buffer.c`.
//!
use super::vim::{LuaError, LuaString, Object};

extern "C" {
    /// Gets a buffer-scoped (b:) variable.
    ///
    pub fn nvim_buf_get_var(buffer: Buffer, name: LuaString, err: *mut LuaError) -> Object;

    /// Sets a buffer-scoped (b:) variable.
    ///
    pub fn nvim_buf_set_var(buffer: Buffer, name: LuaString, value: Object, err: *mut LuaError);

    /// Gets a buffer option value.
    ///
    pub fn nvim_buf_get_option(buffer: Buffer, name: LuaString, err: *mut LuaError) -> Object;
}

/// The buffer number.
///
pub type Buffer = crate::types::handle_T;
