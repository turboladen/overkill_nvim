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
    string::String as LuaString,
};

use super::buffer::Buffer;

extern "C" {
    pub fn nvim_get_var(name: LuaString, err: *mut NvimError) -> Object;
    pub fn nvim_set_var(name: LuaString, value: Object, err: *mut NvimError);
    pub fn nvim_get_vvar(name: LuaString, err: *mut NvimError) -> Object;
    pub fn nvim_set_vvar(name: LuaString, value: Object, err: *mut NvimError);

    pub fn nvim_feedkeys(keys: LuaString, mode: LuaString, escape_csi: Boolean);

    pub fn nvim_get_mode() -> Dictionary;
    pub fn nvim_get_current_buf() -> Buffer;
    pub fn nvim_replace_termcodes(
        s: LuaString,
        from_part: Boolean,
        do_lt: Boolean,
        special: Boolean,
    ) -> LuaString;

    pub fn nvim_exec(src: LuaString, output: Boolean, err: *mut NvimError) -> LuaString;

    pub fn nvim_set_hl(
        namespace_id: Integer,
        name: LuaString,
        val: Dictionary,
        err: *mut NvimError,
    );
    pub fn nvim_get_namespaces() -> Dictionary;
    pub fn nvim_create_namespace(name: LuaString) -> Integer;
}

pub type Boolean = bool;
pub type Integer = i64;
pub type Float = f64;
pub type LuaRef = isize;
