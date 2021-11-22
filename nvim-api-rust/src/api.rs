//!
//! This module contains types and functions that give access to neovim similar to neovim's lua
//! API.
pub mod buffer;
pub mod vim;

pub(crate) mod error;
pub(crate) mod mode;
pub(crate) mod rust_object;

pub use self::{error::Error, mode::Mode, rust_object::RustObject};
pub use neovim_sys::api::{
    buffer::Buffer,
    vim::{Boolean, Float, Integer, LuaRef, LuaString, Object},
};