//!
//! This module contains function wrappers for neovim functions defined in
//! `neovim/src/nvim/api/vim.c`.
//!
use super::{mode, Buffer, Error, Mode};
use neovim_sys::{
    api::vim::{self, LuaError, LuaString, Object, ObjectType},
    option::{self, OptionFlags},
};
use std::{convert::TryFrom, ffi::CStr};

/// # Errors
///
/// * If `name` can't be converted to a `LuaString`.
/// * If nvim set an error on the call.
///
pub fn nvim_get_global_option(name: &str) -> Result<Object, Error> {
    let api_name = LuaString::new(name)?;
    let mut out_err = LuaError::default();

    let option = unsafe { vim::nvim_get_option(api_name, &mut out_err) };

    if out_err.is_err() {
        Err(Error::from(out_err))
    } else {
        Ok(option)
    }
}

/// # Errors
///
/// * If `name` can't be converted to a `LuaString`.
/// * If nvim set an error on the call.
///
pub fn nvim_set_global_option(name: &str, value: Object) -> Result<(), Error> {
    _set_option(name, OptionFlags::OptGlobal as i32, value)
}

fn _set_option(name: &str, scope: i32, value: Object) -> Result<(), Error> {
    let name_ptr = LuaString::new(name).unwrap();

    let maybe_err = match value.object_type() {
        ObjectType::kObjectTypeBoolean => unsafe {
            option::set_option_value(
                name_ptr.as_ptr(),
                if value.as_boolean_unchecked() { 1 } else { 0 },
                std::ptr::null(),
                scope,
            )
        },
        ObjectType::kObjectTypeInteger => unsafe {
            option::set_option_value(
                name_ptr.as_ptr(),
                value.as_integer_unchecked(),
                std::ptr::null(),
                scope,
            )
        },
        ObjectType::kObjectTypeString => {
            let s = value.as_string_unchecked();

            unsafe {
                option::set_string_option_direct(
                    name_ptr.as_ptr(),
                    -1, // <- -1 means use the name to look up the value
                    s.to_string_lossy().as_ref().as_ptr(),
                    scope,
                    0,
                );
                std::ptr::null()
            }
        }
        ObjectType::kObjectTypeArray => {
            eprintln!("Got an array");
            std::ptr::null()
        }
        ObjectType::kObjectTypeDictionary => {
            eprintln!("Got a dict");
            std::ptr::null()
        }
        ObjectType::kObjectTypeFloat => {
            eprintln!("Got a float");
            std::ptr::null()
        }
        ObjectType::kObjectTypeNil => {
            eprintln!("Got nil");
            std::ptr::null()
        }
    };

    if maybe_err.is_null() {
        Ok(())
    } else {
        let e = unsafe { CStr::from_ptr(maybe_err) };
        Err(Error::Raw(e.to_string_lossy().into()))
    }
}

/// # Errors
///
/// * If `name` can't be converted to a `LuaString`.
/// * If nvim set an error on the call.
///
pub fn nvim_get_var(name: &str) -> Result<Object, Error> {
    _get_var(name, |api_name, out_err| unsafe {
        vim::nvim_get_var(api_name, out_err)
    })
}

/// # Errors
///
/// * If `name` can't be converted to a `LuaString`.
/// * If nvim set an error on the call.
///
pub fn nvim_get_vvar(name: &str) -> Result<Object, Error> {
    _get_var(name, |api_name, out_err| unsafe {
        vim::nvim_get_vvar(api_name, out_err)
    })
}

/// # Errors
///
/// * If `name` can't be converted to a `LuaString`.
/// * If nvim set an error on the call.
///
fn _get_var<F>(name: &str, getter: F) -> Result<Object, Error>
where
    F: Fn(LuaString, &mut LuaError) -> Object,
{
    let mut out_err = LuaError::default();
    let api_name = LuaString::new(name)?;

    let object = getter(api_name, &mut out_err);

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
pub fn nvim_set_var(name: &str, value: Object) -> Result<(), Error> {
    let mut out_err = LuaError::default();
    let api_name = LuaString::new(name)?;

    unsafe {
        vim::nvim_set_var(api_name, value, &mut out_err);
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
pub fn nvim_set_vvar(name: &str, value: Object) -> Result<(), Error> {
    _set_var(name, value, |api_name, value, out_err| unsafe {
        vim::nvim_set_vvar(api_name, value, out_err);
    })
}

fn _set_var<F>(name: &str, value: Object, setter: F) -> Result<(), Error>
where
    F: Fn(LuaString, Object, &mut LuaError),
{
    let mut out_err = LuaError::default();
    let api_name = LuaString::new(name)?;

    setter(api_name, value, &mut out_err);

    if out_err.is_err() {
        Err(Error::from(out_err))
    } else {
        Ok(())
    }
}

/// # Errors
///
/// * If `keys` can't be converted to a `LuaString`.
/// * If nvim set an error to `v:errmsg`.
///
pub fn nvim_feedkeys(keys: &str, mode: Mode, escape_csi: bool) -> Result<(), Error> {
    let api_keys = LuaString::new(keys)?;
    let api_mode = LuaString::from(mode);

    unsafe {
        vim::nvim_feedkeys(api_keys, api_mode, escape_csi);
    }

    match nvim_get_vvar("errmsg") {
        Ok(errmsg) => match errmsg.object_type() {
            ObjectType::kObjectTypeNil => Ok(()),
            ObjectType::kObjectTypeString if errmsg.as_string_unchecked().is_empty() => Ok(()),
            ObjectType::kObjectTypeString => {
                Err(Error::VErrMsg(errmsg.as_string_unchecked().clone()))
            }
            t => {
                eprintln!("Got unexpected v:errmsg object: {:?}", errmsg);
                Err(Error::ObjectError(vim::object::Error::TypeError {
                    expected: ObjectType::kObjectTypeString,
                    actual: t,
                }))
            }
        },
        Err(_) => Ok(()),
    }
}

/// Gets the current buffer (number).
///
#[must_use]
pub fn nvim_get_current_buf() -> Buffer {
    unsafe { vim::nvim_get_current_buf() }
}

/// # Errors
///
/// If the `Dictionary` returned from nvim doesn't contain both a `mode` and `blocking` key.
///
pub fn nvim_get_mode() -> Result<mode::CurrentMode, mode::CurrentModeError> {
    let d = unsafe { vim::nvim_get_mode() };

    mode::CurrentMode::try_from(d)
}

//pub fn nvim_replace_termcodes(
//    string_to_convert: &str,
//    from_part: bool,
//    do_lt: bool,
//    special: bool,
//) -> String {
//    unsafe {
//        let api_string = cstr_to_string(string_to_convert.as_ptr() as *const c_char);

//        String::new(Cow::Owned(vim::nvim_replace_termcodes(
//            api_string, from_part, do_lt, special,
//        )))
//    }
//}

//pub fn nvim_exec(src: &str, output: bool) -> Result<String, Error> {
//    unsafe {
//        let api_src = cstr_to_string(src.as_ptr() as *const c_char);
//        let mut out_err = Error::default();

//        let vim_string = vim::nvim_exec(api_src, output, out_err.inner_mut());

//        if out_err.is_err() {
//            Err(out_err)
//        } else {
//            Ok(String::new(Cow::Owned(vim_string)))
//        }
//    }
//}

//pub fn nvim_set_hl(namespace_id: Integer, name: &str, val: Dictionary) -> Result<(), Error> {
//    unsafe {
//        let api_name = cstr_to_string(name.as_ptr() as *const c_char);
//        let mut out_err = Error::default();

//        vim::nvim_set_hl(namespace_id, api_name, val.inner(), out_err.inner_mut());

//        if out_err.is_err() {
//            Err(out_err)
//        } else {
//            Ok(())
//        }
//    }
//}

//pub fn nvim_get_namespaces() -> Dictionary {
//    unsafe { Dictionary::new(vim::nvim_get_namespaces()) }
//}

//pub fn nvim_create_namespace(name: &str) -> Integer {
//    unsafe {
//        let api_name = cstr_to_string(name.as_ptr() as *const c_char);

//        vim::nvim_create_namespace(api_name)
//    }
//}
