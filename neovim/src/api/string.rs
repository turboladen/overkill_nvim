use neovim_sys::api::vim;
use std::{borrow::Cow, os::raw::c_char};

#[derive(Debug, Clone)]
pub struct String<'a> {
    inner: Cow<'a, vim::String>,
}

impl<'a> String<'a> {
    pub fn new(inner: Cow<'a, vim::String>) -> Self {
        Self { inner }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.inner.data as *const u8, self.inner.size) }
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.as_slice()).unwrap()
    }

    pub fn as_inner(&self) -> vim::String {
        *self.inner.clone()
    }

    pub fn into_inner(self) -> vim::String {
        match self.inner {
            Cow::Owned(s) => s,
            Cow::Borrowed(s) => s.to_owned(),
        }
    }
}

impl<'a> From<&'a str> for String<'a> {
    fn from(s: &'a str) -> Self {
        let size = s.len();

        let mut bytes: Vec<c_char> = s.to_owned().bytes().map(|b| b as c_char).collect();
        let slice = bytes.as_mut_slice();
        let data = slice.as_mut_ptr();

        Self {
            inner: Cow::Owned(vim::String { data, size }),
        }
    }
}

impl<'a> Drop for String<'a> {
    fn drop(&mut self) {
        if let Cow::Owned(inner) = self.inner {
            inner.free()
        }
    }
}
