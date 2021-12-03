#![allow(missing_docs, clippy::missing_panics_doc)]

use crate::api::{self, Mode, Object, RustObject};
use neovim_sys::api::nvim::{Array, Dictionary, KeyValuePair, NvimString, ObjectType};
use std::borrow::Borrow;

macro_rules! print_error_return_false {
    ($e:expr) => {{
        eprintln!("Got error during test: {}", $e);
        return false;
    }};
}

fn _test_nvim_setget_var(var: &str, value: Object, expected_object_variant: &RustObject) -> bool {
    if let Err(e) = self::api::nvim::nvim_set_var(var, value) {
        eprintln!("Error setting var: {}", e);
        return false;
    }

    match self::api::nvim::nvim_get_var(var).map(RustObject::from) {
        Ok(ref t) => {
            if t == expected_object_variant {
                true
            } else {
                eprintln!("Got unexpected value type: {:?}", t);
                false
            }
        }
        Err(e) => print_error_return_false!(e),
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
        let string = NvimString::new_unchecked("this is a test");
        let value = Object::from(string);

        if !_test_nvim_setget_var(
            var,
            value,
            &RustObject::String(NvimString::new_unchecked("this is a test")),
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
            let key = NvimString::new_unchecked("meow");
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
    let string = NvimString::new_unchecked("meow");
    let value = Object::from(string);

    if let Err(e) = self::api::nvim::nvim_set_vvar(vvar, value) {
        eprintln!("Error setting vvar: {}", e);
    }

    match self::api::nvim::nvim_get_vvar(vvar) {
        Ok(object) if object.object_type() == ObjectType::kObjectTypeString => {
            let string = object.as_string_unchecked();

            if string != &NvimString::new_unchecked("meow") {
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
        Err(e) => print_error_return_false!(e),
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
            let key = NvimString::new_unchecked("meow");
            let value = Object::from(4242);
            Dictionary::new([KeyValuePair::new(key, value)])
        }
        let value = Object::from(make_subject());

        if let Err(e) = self::api::buffer::nvim_buf_set_var(0, var, value) {
            eprintln!("Error setting var: {}", e);
        }

        match self::api::buffer::nvim_buf_get_var(0, var) {
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
            Err(e) => print_error_return_false!(e),
        }
    }
    result
}

#[no_mangle]
pub extern "C" fn test_nvim_get_current_buf() -> bool {
    self::api::nvim::nvim_get_current_buf() == 1
}

#[no_mangle]
pub extern "C" fn test_nvim_feedkeys() -> bool {
    match self::api::nvim::nvim_feedkeys("j", Mode::Normal, false) {
        Ok(()) => true,
        Err(e) => print_error_return_false!(e),
    }
}

#[no_mangle]
pub extern "C" fn test_nvim_get_mode() -> bool {
    match self::api::nvim::nvim_get_mode() {
        Ok(current_mode) => match current_mode.mode() {
            Mode::Normal => true,
            m => {
                eprintln!("FAIL! Expected 'n', got '{:?}'", m);
                false
            }
        },
        Err(e) => print_error_return_false!(e),
    }
}

#[no_mangle]
pub extern "C" fn test_nvim_set_global_option() -> bool {
    // Boolean option
    {
        let option_name = "autoread";

        match self::api::nvim::nvim_get_global_option(option_name) {
            Ok(value) => {
                if !value.as_boolean_unchecked() {
                    eprintln!(
                        "FAIL! Expected `true`, got: {}",
                        value.as_boolean_unchecked()
                    );
                    return false;
                }
            }
            Err(e) => print_error_return_false!(e),
        }

        match self::api::nvim::nvim_set_global_option(option_name, false) {
            Ok(_) => match self::api::nvim::nvim_get_global_option(option_name) {
                Ok(value) => {
                    let v = value.as_boolean_unchecked();

                    if v {
                        eprintln!("FAIL! Expected `false`, got: {}", v);
                        return false;
                    }
                }
                Err(e) => print_error_return_false!(e),
            },
            Err(e) => print_error_return_false!(e),
        }
    }

    // Integer option
    {
        let option_name = "aleph";

        match self::api::nvim::nvim_get_global_option(option_name) {
            Ok(value) => {
                if !value.as_integer_unchecked() == 224 {
                    eprintln!("FAIL! Expected 224, got: {}", value.as_integer_unchecked());
                    return false;
                }
            }
            Err(e) => print_error_return_false!(e),
        }

        match self::api::nvim::nvim_set_global_option(option_name, 225) {
            Ok(_) => match self::api::nvim::nvim_get_global_option(option_name) {
                Ok(value) => {
                    if !value.as_integer_unchecked() == 225 {
                        eprintln!("FAIL! Expected 225, got: {}", value.as_integer_unchecked());
                        return false;
                    }
                }
                Err(e) => print_error_return_false!(e),
            },
            Err(e) => print_error_return_false!(e),
        }
    }

    // String option
    {
        let option_name = "pastetoggle";

        match self::api::nvim::nvim_get_global_option(option_name) {
            Ok(value) => {
                let expected = "";

                if Borrow::<str>::borrow(value.as_string_unchecked()) != expected {
                    eprintln!(
                        "FAIL! Expected `\"{}\"`, got: `\"{}\"`",
                        expected,
                        value.as_string_unchecked()
                    );
                    return false;
                }
            }
            Err(e) => print_error_return_false!(e),
        }

        let expected_in = NvimString::new_unchecked("<F8>");
        let expected = NvimString::new_unchecked("<F8>");

        match self::api::nvim::nvim_set_global_option(option_name, expected_in) {
            Ok(_) => match self::api::nvim::nvim_get_global_option(option_name) {
                Ok(value) => {
                    if value.as_string_unchecked() != &expected {
                        eprintln!(
                            "FAIL! Expected `\"{}\"`, got: `\"{}\"`",
                            expected,
                            value.as_string_unchecked()
                        );
                        return false;
                    }
                }
                Err(e) => print_error_return_false!(e),
            },
            Err(e) => print_error_return_false!(e),
        }
    }

    true
}
