//!
//! # overkill-nvim
//!
//! This crate provides something like a framework for building neovim plugins.
//!
#![deny(unused_extern_crates)]
#![warn(
    box_pointers,
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    missing_copy_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_qualifications
)]

pub mod key_code;
pub mod mapping;
// pub mod mode;
pub mod option;

#[cfg(feature = "lua_test")]
pub mod lua_test;

pub use nvim_api as api;
pub use nvim_api::api::{Array, Boolean, Dictionary, Float, Integer, NvimString, Object};
