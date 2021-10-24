use std::os::raw::c_char;

#[repr(C)]
pub struct Error {
    pub error_type: ErrorType,
    pub msg: *const c_char,
}

impl Default for Error {
    fn default() -> Self {
        Self {
            error_type: ErrorType::kErrorTypeNone,
            msg: std::ptr::null(),
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum ErrorType {
    kErrorTypeNone = -1,
    kErrorTypeException,
    kErrorTypeValidation,
}
