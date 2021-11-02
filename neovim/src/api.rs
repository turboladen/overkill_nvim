pub(crate) mod error;
pub(crate) mod mode;
pub(crate) mod rust_object;

pub use self::{error::Error, mode::Mode, rust_object::RustObject};
pub use neovim_sys::api::{
    buffer::Buffer,
    vim::{Boolean, Float, Integer, LuaRef, Object, String as LuaString},
};

use neovim_sys::api::{
    buffer,
    vim::{self, NvimError, ObjectType},
};

pub fn nvim_get_var(name: &str) -> Result<Object, Error> {
    let mut out_err = NvimError::default();
    let api_name = LuaString::new(name)?;

    let object = unsafe { vim::nvim_get_var(api_name, &mut out_err) };

    if out_err.is_err() {
        Err(Error::from(out_err))
    } else {
        Ok(object)
    }
}

pub fn nvim_set_var(name: &str, value: Object) -> Result<(), Error> {
    let mut out_err = NvimError::default();
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

pub fn nvim_get_vvar(name: &str) -> Result<Object, Error> {
    let mut out_err = NvimError::default();
    let api_name = LuaString::new(name)?;

    let object = unsafe { vim::nvim_get_vvar(api_name, &mut out_err) };

    if out_err.is_err() {
        Err(Error::from(out_err))
    } else {
        Ok(object)
    }
}

pub fn nvim_set_vvar(name: &str, value: Object) -> Result<(), Error> {
    let mut out_err = NvimError::default();
    let api_name = LuaString::new(name)?;

    unsafe {
        vim::nvim_set_vvar(api_name, value, &mut out_err);
    }

    if out_err.is_err() {
        Err(Error::from(out_err))
    } else {
        Ok(())
    }
}

pub fn nvim_buf_get_var(buffer: Buffer, name: &str) -> Result<Object, Error> {
    let mut out_err = NvimError::default();
    let api_name = LuaString::new(name)?;

    let object = unsafe { buffer::nvim_buf_get_var(buffer, api_name, &mut out_err) };

    if out_err.is_err() {
        Err(Error::from(out_err))
    } else {
        Ok(object)
    }
}

pub fn nvim_buf_set_var(buffer: Buffer, name: &str, value: Object) -> Result<(), Error> {
    let mut out_err = NvimError::default();
    let api_name = LuaString::new(name)?;

    unsafe {
        buffer::nvim_buf_set_var(buffer, api_name, value, &mut out_err);
    }

    if out_err.is_err() {
        Err(Error::from(out_err))
    } else {
        Ok(())
    }
}

pub fn nvim_feedkeys(keys: &str, mode: Mode, escape_csi: bool) -> Result<(), Error> {
    let api_keys = LuaString::new(keys)?;
    let api_mode = LuaString::from(mode);

    unsafe {
        vim::nvim_feedkeys(api_keys, api_mode, escape_csi);
    }

    let errmsg = nvim_get_vvar("errmsg")?;

    match errmsg.object_type() {
        ObjectType::kObjectTypeNil => Ok(()),
        ObjectType::kObjectTypeString if errmsg.as_string_unchecked().is_empty() => Ok(()),
        ObjectType::kObjectTypeString => Err(Error::VErrMsg(errmsg.as_string_unchecked().clone())),
        _ => {
            eprintln!("Got unexpected v:errmsg object: {:?}", errmsg);
            Err(Error::VErrMsg(LuaString::new("Uhohhhh").unwrap()))
        }
    }
}

pub fn nvim_get_current_buf() -> Buffer {
    unsafe { vim::nvim_get_current_buf() }
}

//pub fn nvim_buf_get_option(buffer: Buffer, name: &str) -> Result<Object, Error> {
//    unsafe {
//        let api_name = cstr_to_string(name.as_ptr() as *const c_char);
//        let mut out_err = Error::default();

//        let vim_object = buffer::nvim_buf_get_option(buffer, api_name, out_err.inner_mut());

//        if out_err.is_err() {
//            Err(out_err)
//        } else {
//            Ok(Object::from(vim_object))
//        }
//    }
//}

//pub fn nvim_get_mode() -> Mode {
//    // @returns Dictionary { "mode": String, "blocking": Boolean }
//    //
//    let d = Dictionary::new(unsafe { vim::nvim_get_mode() });

//    let mode = if let Some(Object::String(mode)) = d.get("mode") {
//        Mode::from(mode.as_str())
//    } else {
//        Mode::Normal
//    };

//    mode
//}

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
