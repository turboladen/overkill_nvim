pub mod array;
pub mod dictionary;
pub mod key_value_pair;
pub mod nvim_error;
pub mod object;
pub mod string;

pub use self::{
    array::Array,
    dictionary::Dictionary,
    key_value_pair::KeyValuePair,
    nvim_error::{ErrorType, NvimError},
    object::{Object, ObjectData, ObjectType},
    string::String,
};

use super::buffer::Buffer;

extern "C" {
    pub fn nvim_get_var(name: self::String, err: *mut NvimError) -> Object;
    pub fn nvim_set_var(name: self::String, value: Object, err: *mut NvimError);
    pub fn nvim_get_vvar(name: self::String, err: *mut NvimError) -> Object;
    pub fn nvim_set_vvar(name: self::String, value: Object, err: *mut NvimError);

    pub fn nvim_feedkeys(keys: self::String, mode: self::String, escape_csi: Boolean);

    pub fn nvim_get_mode() -> Dictionary;
    pub fn nvim_get_current_buf() -> Buffer;
    pub fn nvim_replace_termcodes(
        s: String,
        from_part: Boolean,
        do_lt: Boolean,
        special: Boolean,
    ) -> String;

    pub fn nvim_exec(src: String, output: Boolean, err: *mut NvimError) -> String;

    pub fn nvim_set_hl(namespace_id: Integer, name: String, val: Dictionary, err: *mut NvimError);
    pub fn nvim_get_namespaces() -> Dictionary;
    pub fn nvim_create_namespace(name: String) -> Integer;
}

pub type Boolean = bool;
pub type Integer = i64;
pub type Float = f64;
pub type LuaRef = isize;
