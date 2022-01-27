//!
//! # nvim-api
//!
//! This crate provides a Rust abstraction over `neovim-sys`, providing an API similar to neovim's
//! lua API.
//!
#![deny(unused_extern_crates)]
#![warn(
    box_pointers,
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    missing_copy_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_qualifications
)]

#[cfg(feature = "lua_test")]
pub mod lua_test;

pub mod autocmd;
pub mod buffer;
pub mod keymap;
pub mod nvim;

pub(crate) mod error;
pub(crate) mod rust_object;

pub use self::{error::Error, rust_object::RustObject};
pub use neovim_sys as sys;
pub use neovim_sys::api::{
    buffer::Buffer,
    nvim::{Array, Boolean, Dictionary, Float, Integer, LuaRef, NvimString, Object},
};
