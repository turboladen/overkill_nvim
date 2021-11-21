#![allow(clippy::module_name_repetitions)]

use std::os::raw::{c_char, c_int, c_long};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum OptionFlags {
    OptFree = 1,
    OptGlobal = 2,
    OptLocal = 4,
    OptModeline = 8,
    OptWinOnly = 16,
    OptNoWin = 32,
    OptOneColumn = 64,
    OptNoRedraw = 128,
    OptSkipRtp = 256,
}

extern "C" {
    // FWIW, using this instead of nvim_set_option() because that requires a `channel_id`, which is
    // the result of starting a session, which I'd like to avoid.
    pub fn set_option_value(
        name: *const c_char,
        number: c_long,
        string: *const c_char,
        opt_flags: c_int,
    ) -> *const c_char;

    pub fn set_string_option_direct(
        name: *const c_char,
        option_index: c_int,
        value: *const u8,
        opt_flags: c_int,
        set_sid: c_int,
    );
}
