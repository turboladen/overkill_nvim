//! Functions and supporting types from `nvim/autocmd.{c,h}`.
//!
use crate::types::CharU;
use std::os::raw::c_int;

extern "C" {
    /// Does an `augroup` with name `name` exist?
    ///
    pub fn au_has_group(name: *const CharU) -> bool;

    /// Defines an `augroup`; `:augroup {name}` or `:augroup! {name}`.
    ///
    pub fn do_augroup(name: *const CharU, del_group: c_int);
}
