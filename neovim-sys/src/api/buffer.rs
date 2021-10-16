use super::vim::{Error, Object, String};

extern "C" {
    pub fn nvim_buf_get_option(buffer: Buffer, name: String, err: *mut Error) -> Object;
}

pub type Buffer = u64;
