use log::debug;
use std::{
    ffi::{CStr, CString},
    mem::ManuallyDrop,
    os::raw::c_char,
    ptr::NonNull,
};

#[derive(Debug)]
#[repr(C)]
pub struct String {
    pub data: NonNull<c_char>,
    pub size: usize,
}

impl String {
    pub fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.data.as_ptr()) }
    }

    /// Does not contain the trailing nul byte.
    ///
    pub fn as_bytes(&self) -> &[u8] {
        self.as_cstr().to_bytes()
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        let dst = ManuallyDrop::new(CString::new(self.as_bytes()).unwrap());

        unsafe {
            std::ptr::copy(self.data.as_ref(), dst.as_ptr() as *mut c_char, self.size);
        }
        Self {
            data: NonNull::new(dst.as_ptr() as *mut c_char).unwrap(),
            size: self.size,
        }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        debug!("Droppping String...: {}", self.as_cstr().to_string_lossy());
        unsafe { CString::from_raw(self.data.as_mut()) };
    }
}

impl From<CString> for String {
    fn from(cstring: CString) -> Self {
        debug!("String::from(cstring)...: {}", cstring.to_str().unwrap());
        Self {
            size: cstring.as_bytes().len(),
            data: NonNull::new(cstring.into_raw()).unwrap(),
        }
    }
}

impl From<String> for CString {
    fn from(string: String) -> Self {
        debug!("CString::from(string)...: {:?}", string.as_cstr());
        CString::new(string.as_bytes()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cstring() {
        let cstring = CString::new("tacos").unwrap();
        let string = String::from(cstring.clone());
        assert_eq!(string.size, cstring.as_c_str().to_bytes().len());
        assert_eq!(string.as_cstr(), cstring.as_c_str());
    }

    #[test]
    fn test_into_cstring() {
        let string = {
            let cstring = CString::new("burritos").unwrap();
            String::from(cstring)
        };
        let cstring = CString::from(string.clone());

        assert_eq!(cstring.as_c_str().to_bytes().len(), string.size);
        assert_eq!(cstring.as_c_str(), string.as_cstr());
    }

    #[test]
    fn test_clone() {
        let string = {
            let cstring = CString::new("burritos").unwrap();
            String::from(cstring)
        };
        let clone = string.clone();
        assert_eq!(clone.as_cstr(), string.as_cstr());

        // read after copy
        assert_eq!(string.size, 8);
        let cstring = CString::from(string);
        assert_eq!(&cstring.into_string().unwrap(), "burritos");
    }
}
