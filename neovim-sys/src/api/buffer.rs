use super::vim::{NvimError, Object, String};

extern "C" {
    pub fn nvim_buf_get_option(buffer: Buffer, name: String, err: *mut NvimError) -> Object;
}

pub type Buffer = u64;
