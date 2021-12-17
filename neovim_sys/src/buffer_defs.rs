//! This module contains types and functions defined in `nvim/buffer_defs.h`.
//!

/// Represents a file buffer. You should really only ever have a handle to one of these (never
/// owning it).
///
// From `file_buffer` in nvim/buffer_defs.h.
//
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct buf_T {
    _inner: [u8; 0],
}
