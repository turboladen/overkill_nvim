use std::{
    borrow::Borrow,
    convert::TryFrom,
    ffi::{CStr, CString, NulError},
    fmt,
    os::raw::c_char,
    ptr::NonNull,
};

#[derive(Debug)]
#[repr(C)]
pub struct String {
    data: NonNull<c_char>,
    size: usize,
}

impl String {
    /// # Errors
    ///
    /// If `s` can't be converted to a `CString`.
    ///
    pub fn new<T: Into<Vec<u8>>>(s: T) -> Result<Self, NulError> {
        let cstring = CString::new(s)?;

        Ok(Self {
            size: cstring.as_bytes().len(),
            data: unsafe { NonNull::new_unchecked(cstring.into_raw()) },
        })
    }

    #[must_use]
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.data.as_ptr()) }
    }

    /// Does not contain the trailing nul byte.
    ///
    #[must_use]
    pub fn to_bytes(&self) -> &[u8] {
        self.as_c_str().to_bytes()
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.size
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        let dst = CString::new(self.to_bytes()).unwrap();
        let ptr = dst.into_raw();

        Self {
            data: NonNull::new(ptr).unwrap(),
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
        unsafe { CString::from_raw(self.data.as_mut()) };
    }
}

impl From<CString> for String {
    fn from(cstring: CString) -> Self {
        Self {
            size: cstring.as_bytes().len(),
            data: unsafe { NonNull::new_unchecked(cstring.into_raw()) },
        }
    }
}

impl TryFrom<String> for CString {
    type Error = NulError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        Self::new(string.to_bytes())
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.to_bytes().eq(other.to_bytes())
    }
}

impl Eq for String {}

impl PartialEq<String> for str {
    fn eq(&self, other: &String) -> bool {
        self.as_bytes().eq(other.to_bytes())
    }
}

impl Borrow<str> for String {
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
    fn test_from_cstring() {
        let cstring = CString::new("tacos").unwrap();
        let string = String::from(cstring.clone());

        assert_eq!(string.size, 5);
        assert_eq!(cstring.as_c_str().to_bytes().len(), 5);
        assert_eq!(string.size, cstring.as_c_str().to_bytes().len());
        assert_eq!(string.as_c_str(), cstring.as_c_str());
    }

    #[test]
    fn test_cstring_try_from() {
        let string = String::new("burritos").unwrap();
        assert_eq!(string.size, 8);

        let string_size = string.size;
        let lossy = string.as_c_str().to_string_lossy().to_string();

        let cstring = CString::try_from(string).unwrap();

        assert_eq!(cstring.as_c_str().to_bytes().len(), string_size);
        assert_eq!(cstring.as_c_str().to_string_lossy(), lossy);
    }

    #[test]
    fn test_clone() {
        let string = String::new("burritos").unwrap();
        let clone = string.clone();
        assert_eq!(clone.as_c_str(), string.as_c_str());

        // read after copy
        assert_eq!(string.size, 8);
        let cstring = CString::try_from(string).unwrap();
        assert_eq!(&cstring.into_string().unwrap(), "burritos");
    }
}
