use std::ffi::{CString, NulError};

pub fn augroup(name: &str) -> Result<(), NulError> {
    let cstring = CString::new(name)?;
    let api_name = cstring.into_bytes_with_nul();

    unsafe { neovim_sys::autocmd::do_augroup(api_name.as_ptr(), 0) };

    Ok(())
}

pub fn remove_augroup(name: &str) -> Result<(), NulError> {
    let cstring = CString::new(name)?;
    let api_name = cstring.into_bytes_with_nul();

    unsafe { neovim_sys::autocmd::do_augroup(api_name.as_ptr(), 1) };

    Ok(())
}

pub fn augroup_defined(name: &str) -> Result<bool, NulError> {
    let cstring = CString::new(name)?;
    let api_name = cstring.into_bytes_with_nul();

    let result = unsafe { neovim_sys::autocmd::au_has_group(api_name.as_ptr()) };

    Ok(result)
}
