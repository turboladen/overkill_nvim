use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

#[derive(Debug, Copy)]
#[repr(C)]
pub struct String {
    pub data: *mut c_char,
    pub size: usize,
}

impl String {
    pub fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.data) }
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

impl From<CString> for String {
    fn from(cstring: CString) -> Self {
        Self {
            size: cstring.as_bytes().len(),
            data: cstring.into_raw(),
        }
    }
}

impl From<String> for CString {
    fn from(string: String) -> Self {
        unsafe { CString::from_raw(string.data) }
    }
}
