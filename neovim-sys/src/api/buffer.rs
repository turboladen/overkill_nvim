use super::vim::{NvimError, Object, LuaString};

extern "C" {
    pub fn nvim_buf_get_var(buffer: Buffer, name: LuaString, err: *mut NvimError) -> Object;
    pub fn nvim_buf_set_var(buffer: Buffer, name: LuaString, value: Object, err: *mut NvimError);

    pub fn nvim_buf_get_option(buffer: Buffer, name: LuaString, err: *mut NvimError) -> Object;
}

pub type Buffer = crate::types::handle_T;
