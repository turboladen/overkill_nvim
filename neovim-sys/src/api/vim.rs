//!
//! This module contains wrappers for functions defined in `neovim/src/nvim/api/vim.c`.
//!
pub mod array;
pub mod collection;
pub mod dictionary;
pub mod error;
pub mod object;
pub mod string;

pub use self::{
    array::Array,
    dictionary::{KeyValuePair, Dictionary},
    error::{ErrorType, Error as LuaError},
    object::{Object, ObjectType},
    string::String as LuaString,
};

use super::buffer::Buffer;

/// Neovim defines a type `Boolean`, which is the same as a Rust `bool`.
///
pub type Boolean = bool;

/// Neovim defines a type `Integer`, which is the same as a Rust `i64`.
///
pub type Integer = i64;

/// Neovim defines a type `Float`, which is the same as a Rust `f64`.
///
pub type Float = f64;

/// Neovim defines a type `LuaRef`, which is the same as a Rust `isize`.
///
pub type LuaRef = isize;

extern "C" {
    /// Gets a global (g:) variable.
    ///
    pub fn nvim_get_var(name: LuaString, err: *mut LuaError) -> Object;

    /// Sets a global (g:) variable.
    ///
    pub fn nvim_set_var(name: LuaString, value: Object, err: *mut LuaError);

    /// Gets a v: variable.
    ///
    pub fn nvim_get_vvar(name: LuaString, err: *mut LuaError) -> Object;

    /// Sets a v: variable.
    ///
    pub fn nvim_set_vvar(name: LuaString, value: Object, err: *mut LuaError);

    /// Sends input-keys to Nvim.
    ///
    pub fn nvim_feedkeys(keys: LuaString, mode: LuaString, escape_csi: Boolean);

    /// Gets the current mode.
    ///
    pub fn nvim_get_mode() -> Dictionary;

    /// Gets the current buffer.
    ///
    pub fn nvim_get_current_buf() -> Buffer;

    /// Replaces terminal codes and keycodes in a string with the internal representation.
    ///
    pub fn nvim_replace_termcodes(
        s: LuaString,
        from_part: Boolean,
        do_lt: Boolean,
        special: Boolean,
    ) -> LuaString;

    /// Executes `Vimscript`.
    ///
    pub fn nvim_exec(src: LuaString, output: Boolean, err: *mut LuaError) -> LuaString;

    /// Sets a highlight group.
    ///
    pub fn nvim_set_hl(
        namespace_id: Integer,
        name: LuaString,
        val: Dictionary,
        err: *mut LuaError,
    );

    /// Gets existing, non-anonymous namespaces.
    ///
    pub fn nvim_get_namespaces() -> Dictionary;

    /// Creates a new namespace, or gets and existing one.
    ///
    pub fn nvim_create_namespace(name: LuaString) -> Integer;
}
