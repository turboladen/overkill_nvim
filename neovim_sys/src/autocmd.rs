//! Functions and supporting types from `nvim/autocmd.{c,h}`.
//!
use crate::types::CharU;
use std::os::raw::{c_char, c_int};

extern "C" {
    /// Does an `augroup` with name `name` exist?
    ///
    pub fn au_has_group(name: *const CharU) -> bool;

    /// Defines an `augroup`; `:augroup {name}` or `:augroup! {name}`.
    ///
    pub fn do_augroup(name: *const CharU, del_group: c_int);

    /// Defines an `autocmd`.
    ///
    /// - `:h autocmd-define`:
    ///   - `:autocmd [group] <event> <pattern> [++once] [++nested] <cmd>`
    /// - `:h autocmd-remove`:
    ///   - `:autocmd! [group] <event> <pattern>`
    ///   - `:autocmd! [group] * <pattern>`
    ///   - `:autocmd! [group] <event>`
    ///   - `:autocmd! [group]`
    /// - `:h autocmd-list`:
    ///   - `:autocmd [group] <event> <pattern>`
    ///   - `:autocmd [group] * <pattern>`
    ///   - `:autocmd [group] <event>`
    ///   - `:autocmd [group]`
    pub fn do_autocmd(arg_in: *const CharU, force_it: c_int);

    /// #Group#Event#pat
    /// #Group#Event
    /// #Group
    /// #Event#pat
    /// #Event
    ///
    pub fn au_exists(arg: *const c_char) -> bool;
}
