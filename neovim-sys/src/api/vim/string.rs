use log::debug;
use std::{
    convert::TryFrom,
    ffi::{CStr, CString},
    mem::ManuallyDrop,
    os::raw::c_char,
    ptr::NonNull,
};

use super::{Object, ObjectType};

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
        debug!(
            "Cloning String: '{}' ({})",
            self.as_cstr().to_string_lossy(),
            self.size
        );
        let dst = CString::new(self.as_bytes()).unwrap();
        let ptr = dst.into_raw();

        Self {
            data: NonNull::new(ptr).unwrap(),
            size: self.size,
        }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        debug!(
            "Droppping String...: '{}'",
            self.as_cstr().to_string_lossy()
        );
        unsafe { CString::from_raw(self.data.as_mut()) };
    }
}

impl From<CString> for String {
    fn from(cstring: CString) -> Self {
        debug!("String::from(cstring)...: '{}'", cstring.to_str().unwrap());
        Self {
            size: cstring.as_bytes().len(),
            data: NonNull::new(cstring.into_raw()).unwrap(),
        }
    }
}

impl From<String> for CString {
    fn from(string: String) -> Self {
        debug!(
            "CString::from(string)...: '{}'",
            string.as_cstr().to_string_lossy()
        );
        CString::new(string.as_bytes()).unwrap()
    }
}

impl TryFrom<Object> for String {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        debug!("String::try_from(Object): '{}'", unsafe {
            value.data.string.as_cstr().to_string_lossy()
        });

        match value.object_type {
            ObjectType::kObjectTypeString => {
                let dst = CString::new(unsafe { value.data.string.as_bytes() }).unwrap();
                let ptr = dst.into_raw();

                let s = Self {
                    data: NonNull::new(ptr).unwrap(),
                    size: unsafe { value.data.string.size },
                };
                // Since we moved the data, don't call drop for the Object.
                std::mem::forget(value);
                Ok(s)
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cstring() {
        let cstring = CString::new("tacos").unwrap();
        let string = String::from(cstring.clone());

        assert_eq!(string.size, 5);
        assert_eq!(cstring.as_c_str().to_bytes().len(), 5);
        assert_eq!(string.size, cstring.as_c_str().to_bytes().len());
        assert_eq!(string.as_cstr(), cstring.as_c_str());
    }

    #[test]
    fn test_into_cstring() {
        let string = {
            let cstring = CString::new("burritos").unwrap();
            String::from(cstring)
        };
        assert_eq!(string.size, 8);
        let string_size = string.size;
        let lossy = string.as_cstr().to_string_lossy().to_string();

        let cstring = CString::from(string);

        assert_eq!(cstring.as_c_str().to_bytes().len(), string_size);
        assert_eq!(cstring.as_c_str().to_string_lossy(), lossy);
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
