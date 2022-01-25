//! Typedefs from `neovim/src/nvim/types.h`.
//!
use std::os::raw::c_int;

pub(crate) type CharU = u8;
pub(crate) type handle_T = c_int;
