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

pub mod api;

pub use neovim_sys as sys;

#[cfg(feature = "lua_test")]
pub mod lua_test;
