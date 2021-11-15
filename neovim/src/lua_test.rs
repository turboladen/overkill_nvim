#![allow(clippy::missing_panics_doc)]

use super::api;
use crate::api::{LuaString, Mode, Object, RustObject};
use neovim_sys::api::vim::{Array, Dictionary, KeyValuePair, ObjectType};

fn _test_nvim_setget_var(var: &str, value: Object, expected_object_variant: &RustObject) -> bool {
    if let Err(e) = self::api::nvim_set_var(var, value) {
        eprintln!("Error setting var: {}", e);
        return false;
    }

    match self::api::nvim_get_var(var).map(RustObject::from) {
        Ok(ref t) => {
            if t == expected_object_variant {
                true
            } else {
                eprintln!("Got unexpected value type: {:?}", t);
                false
            }
        }
        Err(e) => {
            eprintln!("Got error during test: {}", e);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn test_nvim_set_var() -> bool {
    // nil
    {
        let var = "nvim_set_var_test_nil";
        let value = Object::new_nil();

        if !_test_nvim_setget_var(var, value, &RustObject::Nil) {
            return false;
        }
    }

    // bool
    {
        let var = "nvim_set_var_test_bool";
        let value = Object::from(true);

        if !_test_nvim_setget_var(var, value, &RustObject::Boolean(true)) {
            return false;
        }
    }

    // Integer
    {
        let var = "nvim_set_var_test_integer";
        let value = Object::from(42);

        if !_test_nvim_setget_var(var, value, &RustObject::Integer(42)) {
            return false;
        }
    }

    // Float
    {
        let var = "nvim_set_var_test_float";
        let value = Object::from(123.456);

        if !_test_nvim_setget_var(var, value, &RustObject::Float(123.456)) {
            return false;
        }
    }

    // String
    {
        let var = "nvim_set_var_test_string";
        let string = LuaString::new("this is a test").unwrap();
        let value = Object::from(string);

        if !_test_nvim_setget_var(
            var,
            value,
            &RustObject::String(LuaString::new("this is a test").unwrap()),
        ) {
            return false;
        }
    }

    // Array
    {
        fn make_subject() -> Array {
            let o = Object::from(4242);
            Array::new([o])
        }
        let value = Object::from(make_subject());
        let var = "nvim_set_var_test_array";

        if !_test_nvim_setget_var(var, value, &RustObject::Array(make_subject())) {
            return false;
        }
    }

    // Dictionary
    {
        fn make_subject() -> Dictionary {
            let key = LuaString::new("meow").unwrap();
            let value = Object::from(4242);
            Dictionary::new([KeyValuePair::new(key, value)])
        }
        let value = Object::from(make_subject());
        let var = "nvim_set_var_test_dictionary";

        if !_test_nvim_setget_var(var, value, &RustObject::Dictionary(make_subject())) {
            return false;
        }
    }

    true
}

#[no_mangle]
pub extern "C" fn test_nvim_set_vvar() -> bool {
    let vvar = "warningmsg";
    let string = LuaString::new("meow").unwrap();
    let value = Object::from(string);

    if let Err(e) = self::api::nvim_set_vvar(vvar, value) {
        eprintln!("Error setting vvar: {}", e);
    }

    match self::api::nvim_get_vvar(vvar) {
        Ok(object) if object.object_type() == ObjectType::kObjectTypeString => {
            let string = object.as_string_unchecked();

            if string != &LuaString::new("meow").unwrap() {
                eprintln!(
                    "FAIL! Expected 'meow', got '{}'",
                    string.as_c_str().to_string_lossy()
                );
                return false;
            }
        }
        Ok(t) => {
            eprintln!("Got unexpected value type: {:?}", t);
            return false;
        }
        Err(e) => {
            eprintln!("Got error during test: {}", e);
            return false;
        }
    }

    true
}

#[no_mangle]
pub extern "C" fn test_nvim_buf_set_var() -> bool {
    let mut result = true;

    let var = "nvim_rs_buf_set_get_var";

    // Dictionary
    {
        fn make_subject() -> Dictionary {
            let key = LuaString::new("meow").unwrap();
            let value = Object::from(4242);
            Dictionary::new([KeyValuePair::new(key, value)])
        }
        let value = Object::from(make_subject());

        if let Err(e) = self::api::nvim_buf_set_var(0, var, value) {
            eprintln!("Error setting var: {}", e);
        }

        match self::api::nvim_buf_get_var(0, var) {
            Ok(object) if object.object_type() == ObjectType::kObjectTypeDictionary => {
                let dict = object.as_dictionary_unchecked();
                if dict != &make_subject() {
                    eprintln!("FAIL! Expected 'this is a test', got '{:?}'", dict);
                    result = false;
                }
            }
            Ok(t) => {
                eprintln!("Got unexpected value type: {:?}", t);
                result = false;
            }
            Err(e) => {
                eprintln!("Got error during test: {}", e);
                result = false;
            }
        }
    }
    result
}

#[no_mangle]
pub extern "C" fn test_nvim_get_current_buf() -> bool {
    self::api::nvim_get_current_buf() == 1
}

#[no_mangle]
pub extern "C" fn test_nvim_feedkeys() -> bool {
    match self::api::nvim_feedkeys("j", Mode::Normal, false) {
        Ok(()) => true,
        Err(e) => {
            eprintln!("Got error during test: {}", e);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn test_nvim_get_mode() -> bool {
    match self::api::nvim_get_mode() {
        Ok(current_mode) => match current_mode.mode() {
            Mode::Normal => true,
            m => {
                eprintln!("FAIL! Expected 'n', got '{:?}'", m);
                false
            }
        },
        Err(e) => {
            eprintln!("Got error during test: {}", e);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn test_nvim_get_option() -> bool {
    match self::api::nvim_get_mode() {
        Ok(current_mode) => match current_mode.mode() {
            Mode::Normal => true,
            m => {
                eprintln!("FAIL! Expected 'n', got '{:?}'", m);
                false
            }
        },
        Err(e) => {
            eprintln!("Got error during test: {}", e);
            false
        }
    }
}
