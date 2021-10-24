use super::helpers;
use std::os::raw::c_char;

#[derive(Debug, Copy, Eq)]
#[repr(C)]
pub struct String {
    pub data: *mut c_char,
    pub size: usize,
}

impl String {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data as *const u8, self.size) }
    }

    pub fn free(self) {
        unsafe { helpers::api_free_string(self) }
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        if !self.data.is_null() {
            let mut dst = Vec::with_capacity(self.size);
            unsafe {
                std::ptr::copy(self.data, dst.as_mut_ptr(), self.size);
                dst.set_len(self.size);
            }
            String {
                data: dst.as_mut_ptr(),
                size: self.size,
            }
        } else {
            String {
                data: std::ptr::null_mut(),
                size: 0,
            }
        }
    }
}
