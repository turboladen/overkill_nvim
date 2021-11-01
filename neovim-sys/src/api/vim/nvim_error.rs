use std::{ffi::CStr, fmt, os::raw::c_char};

#[derive(thiserror::Error)]
#[repr(C)]
pub struct NvimError {
    error_type: ErrorType,
    msg: *const c_char,
}

impl NvimError {
    pub fn is_err(&self) -> bool {
        !matches!(self.error_type, ErrorType::kErrorTypeNone)
    }
}

impl Default for NvimError {
    fn default() -> Self {
        Self {
            error_type: ErrorType::kErrorTypeNone,
            msg: std::ptr::null(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum ErrorType {
    kErrorTypeNone = -1,
    kErrorTypeException,
    kErrorTypeValidation,
}

impl fmt::Display for NvimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_string = match self.error_type {
            ErrorType::kErrorTypeNone => return Ok(()),
            ErrorType::kErrorTypeException => "Exception",
            ErrorType::kErrorTypeValidation => "Validation",
        };

        if self.msg.is_null() {
            write!(f, "{}: (null msg)", type_string)
        } else {
            let msg = unsafe { CStr::from_ptr(self.msg) };
            write!(f, "{}: {}", type_string, msg.to_string_lossy())
        }
    }
}

impl fmt::Debug for NvimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = unsafe { CStr::from_ptr(self.msg) }.to_string_lossy();

        f.debug_struct("NvimError")
            .field("error_type", &self.error_type)
            .field("msg", &msg)
            .finish()
    }
}
