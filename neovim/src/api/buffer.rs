//!
//! This module contains function wrappers for neovim functions defined in
//! `neovim/src/nvim/api/buffer.c`.
//!
use super::{Buffer, Error};
use neovim_sys::api::{
    self,
    vim::{LuaString, NvimError, Object},
};

/// # Errors
///
/// * If `name` can't be converted to a `LuaString`.
/// * If nvim set an error on the call.
///
pub fn nvim_buf_get_var(buffer: Buffer, name: &str) -> Result<Object, Error> {
    let mut out_err = NvimError::default();
    let api_name = LuaString::new(name)?;

    let object = unsafe { api::buffer::nvim_buf_get_var(buffer, api_name, &mut out_err) };

    if out_err.is_err() {
        Err(Error::from(out_err))
    } else {
        Ok(object)
    }
}

/// # Errors
///
/// * If `name` can't be converted to a `LuaString`.
/// * If nvim set an error on the call.
///
pub fn nvim_buf_set_var(buffer: Buffer, name: &str, value: Object) -> Result<(), Error> {
    let mut out_err = NvimError::default();
    let api_name = LuaString::new(name)?;

    unsafe {
        api::buffer::nvim_buf_set_var(buffer, api_name, value, &mut out_err);
    }

    if out_err.is_err() {
        Err(Error::from(out_err))
    } else {
        Ok(())
    }
}

/// # Errors
///
/// * If `name` can't be converted to a `LuaString`.
/// * If nvim set an error on the call.
///
pub fn nvim_buf_get_option(buffer: Buffer, name: &str) -> Result<Object, Error> {
    let api_name = LuaString::new(name)?;
    let mut out_err = NvimError::default();

    let object = unsafe { api::buffer::nvim_buf_get_option(buffer, api_name, &mut out_err) };

    if out_err.is_err() {
        Err(Error::from(out_err))
    } else {
        Ok(object)
    }
}
