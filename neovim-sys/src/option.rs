//! Types and functions required for getting and setting options.
//!
#![allow(clippy::module_name_repetitions)]

use std::{
    ffi::c_void,
    os::raw::{c_char, c_int, c_long},
};

/// From `option.h`: Flags for option-setting functions.
///
#[derive(Debug, Clone, Copy)]
pub enum OptionFlags {
    /// Free old value if it was allocated.
    ///
    OptFree = 1,

    /// Use global value.
    ///
    OptGlobal = 2,

    /// Use local value.
    ///
    OptLocal = 4,

    /// Option in modeline.
    ///
    OptModeline = 8,

    /// Only set window-local options.
    ///
    OptWinOnly = 16,

    /// Don't set window-local options.
    ///
    OptNoWin = 32,

    /// List options one-per-line.
    ///
    OptOneColumn = 64,

    /// Ignore redraw flags on option.
    ///
    OptNoRedraw = 128,

    /// "skiprtp" in 'sessionoptions'.
    ///
    OptSkipRtp = 256,
}

/// From `option_defs.h`: Return value for `get_option_value_strict()`.
///
#[derive(Debug, Clone, Copy)]
pub enum SOpt {
    /// Boolean option.
    ///
    Bool = 0x01,

    /// Number option.
    ///
    Num = 0x02,

    /// String option.
    ///
    String = 0x04,

    /// Option has a global value.
    ///
    Global = 0x08,

    /// Option has a window-local value.
    ///
    Win = 0x10,

    /// Option has a buffer-local value.
    ///
    Buf = 0x20,

    /// Option does not have a local value set.
    ///
    Unset = 0x40,
}

/// From `option_defs.h`: option types for various functions in `option`.
///
#[derive(Debug, Clone, Copy)]
pub enum SReq {
    /// Request global option value.
    ///
    Global = 0,

    /// Request window-local option value.
    ///
    Win = 1,

    /// Request buffer-local option value.
    ///
    Buf = 2,
}

extern "C" {
    /// FWIW, using this instead of nvim_set_option() because that requires a `channel_id`, which is
    /// the result of starting a session, which I'd like to avoid.
    ///
    /// * `name` is the option name.
    /// * `number` is the value to set if the option is number-based (including `bool`).
    /// * `string` is the value to set if the option is string-based.
    /// * `opt_flags` should be either `OptionFlag::Global`, `OptionFlag::Local`, or both.
    ///
    /// Retruns `null` on success, or an error message on fail.
    ///
    pub fn set_option_value(
        name: *const c_char,
        number: c_long,
        string: *const c_char,
        opt_flags: c_int,
    ) -> *const c_char;

    /// Sets a string option without checking the effects. The string value is copied, so
    /// passing a reference is ok.
    ///
    /// * `name` is the option name.
    /// * `option_index` is the index of the option in the internal, global `options` array, so isn't
    ///   useful here. As such, this should always be `-1` to force the call to look up the option
    ///   using `name`.
    /// * `value` is the new value, in bytes.
    /// * `opt_flags` is one of `OptionFlag`, but should only be `OptionFlag::OptFree`,
    ///   `OptionFlag::Local`, or `OptionFlag::Global`.
    /// * When `set_sid` is `globals::Sid::None`, then the `SID` won't be set; otherwise it's
    ///   set to the given `set_sid`.
    ///
    pub fn set_string_option_direct(
        name: *const c_char,
        option_index: c_int,
        value: *const u8,
        opt_flags: c_int,
        set_sid: c_int,
    );

    /// While the typical `:set` command can return either the global _or_ local value,
    /// depending on context, this call always returns the option from the `opt_type`
    /// scope. The return value is `SOpt` flags; if 0, it's a hidden or unknown option.
    ///
    pub fn get_option_value_strict(
        name: *const c_char,
        numval: *mut i64,
        stringval: *mut *const c_char,
        opt_type: c_int,
        from: *const c_void,
    ) -> c_int;
}
