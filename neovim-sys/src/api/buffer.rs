use super::vim::{NvimError, Object, String};

extern "C" {
    pub fn nvim_buf_get_var(buffer: Buffer, name: self::String, err: *mut NvimError) -> Object;
    pub fn nvim_buf_set_var(buffer: Buffer, name: self::String, value: Object, err: *mut NvimError);

    pub fn nvim_buf_get_option(buffer: Buffer, name: String, err: *mut NvimError) -> Object;
}

pub type Buffer = crate::types::handle_T;
