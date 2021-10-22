use neovim_sys::api::vim::{self, ErrorType};
use std::{ffi::CStr, fmt};

#[derive(Default)]
pub struct Error {
    inner: vim::Error,
}

impl Error {
    pub fn is_err(&self) -> bool {
        !matches!(self.inner.error_type, ErrorType::kErrorTypeNone)
    }

    pub fn inner_mut(&mut self) -> &mut vim::Error {
        &mut self.inner
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_string = match self.inner.error_type {
            ErrorType::kErrorTypeNone => return Ok(()),
            ErrorType::kErrorTypeException => "Exception",
            ErrorType::kErrorTypeValidation => "Validation",
        };

        if self.inner.msg.is_null() {
            write!(f, "{}: (null msg)", type_string)
        } else {
            let msg = unsafe { CStr::from_ptr(self.inner.msg) };
            write!(f, "{}: {}", type_string, msg.to_string_lossy())
        }
    }
}
