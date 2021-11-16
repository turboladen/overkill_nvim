//!
//! This module contains the wrapper and related functions for neovim's Lua `Error`.
//!
use std::{ffi::CStr, fmt, os::raw::c_char};

/// Wrapper for neovim's Lua `Error`.
///
#[derive(thiserror::Error, Clone, Copy)]
#[repr(C)]
pub struct Error {
    error_type: ErrorType,
    msg: *mut c_char,
}

impl Error {
    /// Since an "error" can also be `None`, this is a convenience method to check if the `self` is
    /// actually an error.
    ///
    #[must_use]
    pub const fn is_err(&self) -> bool {
        !matches!(self.error_type, ErrorType::kErrorTypeNone)
    }

    /// Get a reference to the nvim error's msg.
    ///
    #[inline]
    #[must_use]
    pub fn msg(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.msg) }
    }

    /// Get a reference to the nvim error's error type.
    ///
    #[must_use]
    #[inline]
    pub const fn error_type(&self) -> ErrorType {
        self.error_type
    }
}

impl Default for Error {
    fn default() -> Self {
        Self {
            error_type: ErrorType::kErrorTypeNone,
            msg: std::ptr::null_mut(),
        }
    }
}

/// Used by `Error` to communicate which type of `Error` it is.
///
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types, clippy::module_name_repetitions)]
#[repr(C)]
pub enum ErrorType {
    /// Not an error!
    ///
    kErrorTypeNone = -1,

    /// An exception.
    ///
    kErrorTypeException,

    /// Validation error.
    ///
    kErrorTypeValidation,
}

impl fmt::Display for Error {
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

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = unsafe { CStr::from_ptr(self.msg) }.to_string_lossy();

        f.debug_struct("Error")
            .field("error_type", &self.error_type)
            .field("msg", &msg)
            .finish()
    }
}
