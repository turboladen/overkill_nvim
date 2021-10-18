
use neovim_sys::api::vim;
use std::borrow::Cow;


#[derive(Clone)]
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
}

impl<'a> Drop for String<'a> {
    fn drop(&mut self) {
        self.inner.free()
    }
}
