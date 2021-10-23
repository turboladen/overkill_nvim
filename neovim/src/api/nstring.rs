use neovim_sys::api::vim;
use std::{ffi::CString, fmt, mem::ManuallyDrop};

#[derive(Debug, Clone)]
pub struct NString {
    inner: vim::String,
    init_in_rust: bool,
}

impl NString {
    pub fn new(inner: vim::String) -> Self {
        Self {
            inner,
            init_in_rust: false,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.inner.data as *const u8, self.inner.size) }
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.as_slice()).unwrap()
    }

    pub fn inner(&self) -> &vim::String {
        &self.inner
    }

    pub fn to_inner(&self) -> vim::String {
        self.inner
    }
}

// TODO: Probably should switch to `std::string::String` here since we have to take ownership.
//
impl<'a> From<&'a str> for NString {
    fn from(s: &'a str) -> Self {
        let size = s.len();
        let cstring = ManuallyDrop::new(CString::new(s).unwrap());

        let inner = vim::String {
            data: ManuallyDrop::into_inner(cstring).into_raw(),
            size,
        };

        Self {
            inner,
            init_in_rust: true,
        }
    }
}

impl PartialEq for NString {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl fmt::Display for NString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Drop for NString {
    fn drop(&mut self) {
        if self.init_in_rust {
            unsafe { CString::from_raw(self.inner.data) };
        } else {
            self.inner.free()
        }
    }
}
