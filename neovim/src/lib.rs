pub mod api;
pub mod eval;
pub mod typval;

pub use neovim_sys as sys;

use self::api::{Dictionary, Object, String};
use neovim_sys::api::helpers::cstr_to_string;
use std::{borrow::Cow, os::raw::c_char};
use sys::api::{buffer::Buffer, vim::Integer};

pub fn nvim_get_var(name: &str) -> Object {
    unsafe {
        let api_name = cstr_to_string(name.as_ptr() as *const c_char);
        let mut out_err = sys::api::vim::Error::default();

        Object::from(sys::api::vim::nvim_get_var(api_name, &mut out_err))
    }
}

pub fn nvim_buf_get_var(name: &str) -> Object {
    unsafe {
        let api_name = cstr_to_string(name.as_ptr() as *const c_char);
        let mut out_err = sys::api::vim::Error::default();

        Object::from(sys::api::vim::nvim_buf_get_var(api_name, &mut out_err))
    }
}

pub fn nvim_feedkeys(keys: &str, mode: &str, escape_csi: bool) {
    unsafe {
        let api_keys = cstr_to_string(keys.as_ptr() as *const c_char);
        let api_mode = cstr_to_string(mode.as_ptr() as *const c_char);

        sys::api::vim::nvim_feedkeys(api_keys, api_mode, escape_csi);
    }
}

pub fn nvim_get_current_buf() -> Buffer {
    unsafe { sys::api::vim::nvim_get_current_buf() }
}

pub fn nvim_buf_get_option(buffer: Buffer, name: &str) -> Object {
    unsafe {
        let api_name = cstr_to_string(name.as_ptr() as *const c_char);
        let mut out_err = sys::api::vim::Error::default();

        Object::from(sys::api::buffer::nvim_buf_get_option(
            buffer,
            api_name,
            &mut out_err,
        ))
    }
}

pub enum Mode {
    Normal,
    Insert,
    Replace,
    Visual,
    VisualLine,
    VisualBlock,
    Command,
    Select,
    SelectLine,
    SelectBlock,
    Terminal,
}

impl From<&str> for Mode {
    fn from(mode: &str) -> Self {
        match mode {
            "n" => Mode::Normal,
            "i" => Mode::Insert,
            "R" => Mode::Replace,
            "v" => Mode::Visual,
            "V" => Mode::VisualLine,
            "<C-v>" => Mode::VisualBlock,
            "c" => Mode::Command,
            "s" => Mode::Select,
            "S" => Mode::SelectLine,
            "<C-s>" => Mode::SelectBlock,
            "t" => Mode::Terminal,
            m => {
                // error!("unknown mode {}, falling back to Mode::Normal", m);
                Mode::Normal
            }
        }
    }
}

pub fn nvim_get_mode() -> Mode {
    // @returns Dictionary { "mode": String, "blocking": Boolean }
    //
    let d = Dictionary::new(unsafe { sys::api::vim::nvim_get_mode() });

    if let Some(Object::String(mode)) = d.get("mode") {
        Mode::from(mode.as_str())
    } else {
        Mode::Normal
    }
}

pub fn nvim_replace_termcodes(
    string_to_convert: &str,
    from_part: bool,
    do_lt: bool,
    special: bool,
) -> String {
    unsafe {
        let api_string = cstr_to_string(string_to_convert.as_ptr() as *const c_char);

        String::new(Cow::Owned(sys::api::vim::nvim_replace_termcodes(
            api_string, from_part, do_lt, special,
        )))
    }
}

pub fn nvim_exec(src: &str, output: bool) -> String {
    unsafe {
        let api_src = cstr_to_string(src.as_ptr() as *const c_char);
        let mut out_err = sys::api::vim::Error::default();

        String::new(Cow::Owned(sys::api::vim::nvim_exec(
            api_src,
            output,
            &mut out_err,
        )))
    }
}

pub fn nvim_set_hl(namespace_id: Integer, name: &str, val: Dictionary) {
    unsafe {
        let api_name = cstr_to_string(name.as_ptr() as *const c_char);
        let mut out_err = sys::api::vim::Error::default();

        sys::api::vim::nvim_set_hl(namespace_id, api_name, val.inner(), &mut out_err)
    }
}

pub fn nvim_get_namespaces() -> Dictionary {
    unsafe { Dictionary::new(sys::api::vim::nvim_get_namespaces()) }
}

pub fn nvim_create_namespace(name: &str) -> Integer {
    unsafe {
        let api_name = cstr_to_string(name.as_ptr() as *const c_char);

        sys::api::vim::nvim_create_namespace(api_name)
    }
}
