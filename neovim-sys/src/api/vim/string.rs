//!
//! This module contains types and functions for working with neovim Lua `String`s.
//!
use std::{
    borrow::{Borrow, Cow},
    convert::TryFrom,
    ffi::{CStr, CString, NulError},
    fmt,
    mem::{self, MaybeUninit},
    os::raw::c_char,
    ptr::addr_of_mut,
};

/// Very similar to Rust's `CString`, this type represents a `String` in neovim's Lua interface.
/// Named `String` here, but is exported as `LuaString`, just to save on confusion with Rust's
/// `String`.
///
#[derive(Debug)]
#[repr(C)]
pub struct String {
    // This must not contain the nul byte.
    pub(super) data: *mut c_char,
    pub(super) size: usize,
}

impl String {
    /// # Errors
    ///
    /// If `s` can't be converted to a `CString`.
    ///
    pub fn new<T: Into<Vec<u8>>>(s: T) -> Result<Self, NulError> {
        let cstring = CString::new(s)?;
        let mut vec = cstring.into_bytes();

        let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();

        // Initializing the `size` field
        // Using `write` instead of assignment via `=` to not call `drop` on the
        // old, uninitialized value.
        unsafe {
            addr_of_mut!((*ptr).size).write(vec.len());
        }

        let new_data = vec.as_mut_ptr().cast::<c_char>();

        unsafe {
            // Initializing the `list` field
            // If there is a panic here, then the `String` in the `name` field leaks.
            addr_of_mut!((*ptr).data).write(new_data);
        }
        mem::forget(vec);

        Ok(unsafe { uninit.assume_init() })
    }

    /// Just like, `CStr`, this wraps the underlying raw C-string with a safe wrapper.
    ///
    #[must_use]
    #[inline]
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.data) }
    }

    /// Just like `CStr`, if the underlying data is valid UTF-8, it'll return a borrowed `Cow<'_,
    /// str>`; if not, it replaces the non-UTF-8 bytes with a replacement character and returns an
    /// owned `Cow<'_, str>`.
    ///
    #[must_use]
    #[inline]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.as_c_str().to_string_lossy()
    }

    /// Does not contain the trailing nul byte.
    ///
    #[must_use]
    #[inline]
    pub fn to_bytes(&self) -> &[u8] {
        self.as_c_str().to_bytes()
    }

    /// The number of bytes the string contains.
    ///
    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.size
    }

    /// Is this a 0-length string?
    ///
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        let dst = CString::new(self.to_bytes()).unwrap();
        let ptr = dst.into_raw();

        Self {
            data: ptr,
            size: self.size,
        }
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_c_str().to_string_lossy())
    }
}

impl Drop for String {
    fn drop(&mut self) {
        if !self.data.is_null() {
            let _v = unsafe { Vec::from_raw_parts(self.data, self.size, self.size + 1) };
        }
    }
}

impl TryFrom<CString> for String {
    type Error = NulError;

    #[inline]
    fn try_from(cstring: CString) -> Result<Self, Self::Error> {
        Self::new(cstring.into_bytes())
    }
}

impl TryFrom<String> for CString {
    type Error = NulError;

    #[inline]
    fn try_from(string: String) -> Result<Self, Self::Error> {
        Self::new(string.to_bytes())
    }
}

impl PartialEq for String {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes().eq(other.to_bytes())
    }
}

impl Eq for String {}

impl PartialEq<String> for str {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.as_bytes().eq(other.to_bytes())
    }
}

impl Borrow<str> for String {
    #[inline]
    fn borrow(&self) -> &str {
        std::str::from_utf8(self.to_bytes()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_c_str() {
        let subject = String::new("things are cool").unwrap();
        let cstring = CString::new("things are cool").unwrap();
        assert_eq!(subject.as_c_str(), cstring.as_c_str());
    }

    #[test]
    fn test_partial_eq() {
        let lhs = String::new("meow meow stuff").unwrap();
        let rhs = String::new("meow meow stuff").unwrap();
        assert_eq!(lhs, rhs);

        let lhs = String::new("meow meow stuff").unwrap();
        let rhs = String::new("meow stuff").unwrap();
        assert_ne!(lhs, rhs);
    }

    #[test]
    fn test_try_from_cstring() {
        let cstring = CString::new("tacos").unwrap();
        let lua_string = String::try_from(cstring.clone()).unwrap();

        assert_eq!(lua_string.size, 5);
        assert_eq!(cstring.as_c_str().to_bytes().len(), 5);
        assert_eq!(lua_string.size, cstring.as_c_str().to_bytes().len());
        assert_eq!(lua_string.as_c_str(), cstring.as_c_str());
    }

    #[test]
    fn test_cstring_try_from() {
        let lua_string = String::new("burritos").unwrap();
        assert_eq!(lua_string.size, 8);

        let string_size = lua_string.size;
        let lossy = lua_string.as_c_str().to_string_lossy().to_string();

        let cstring = CString::try_from(lua_string).unwrap();

        assert_eq!(cstring.as_c_str().to_bytes().len(), string_size);
        assert_eq!(cstring.as_c_str().to_string_lossy(), lossy);
    }

    #[test]
    fn test_clone() {
        let lua_string = String::new("burritos").unwrap();
        let clone = lua_string.clone();
        assert_eq!(clone.as_c_str(), lua_string.as_c_str());

        // read after copy
        assert_eq!(lua_string.size, 8);
        let cstring = CString::try_from(lua_string).unwrap();
        assert_eq!(&cstring.into_string().unwrap(), "burritos");
    }
}
