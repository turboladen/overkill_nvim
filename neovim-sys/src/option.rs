#![allow(clippy::module_name_repetitions)]

use std::{
    ffi::c_void,
    os::raw::{c_char, c_int, c_long},
};

#[derive(Debug, Clone, Copy)]
pub enum OptionFlag {
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

#[derive(Debug, Clone, Copy)]
pub enum SOpt {
    Bool = 0x01,
    Num = 0x02,
    String = 0x04,
    Global = 0x08,
    Win = 0x10,
    Buf = 0x20,
    Unset = 0x40,
}

#[derive(Debug, Clone, Copy)]
pub enum SReq {
    Global = 0,
    Win = 1,
    Buf = 2,
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

    /// Returns flags. If 0, it's a hidden or unknown option.
    ///
    pub fn get_option_value_strict(
        name: *const c_char,
        numval: *mut i64,
        stringval: *mut *const c_char,
        opt_type: c_int,
        from: *const c_void,
    ) -> c_int;
}
