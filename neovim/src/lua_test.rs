use super::api;
use crate::api::{Boolean, LuaString, Object, RustObject};
use approx::ulps_ne;
use neovim_sys::api::vim::{Array, Dictionary, KeyValuePair, ObjectType};

#[no_mangle]
pub extern "C" fn test_nvim_set_var() -> Boolean {
    let mut result = true;

    let var = "nvim_rs_set_get_var";

    // nil
    {
        let value = Object::new_nil();

        if let Err(e) = self::api::nvim_set_var(var, value) {
            eprintln!("Error setting var: {}", e);
        }

        match self::api::nvim_get_var(var).map(RustObject::from) {
            Ok(RustObject::Nil) => (),
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

    // // bool
    {
        let value = Object::from(true);

        if let Err(e) = self::api::nvim_set_var(var, value) {
            eprintln!("Error setting var: {}", e);
        }

        match self::api::nvim_get_var(var).map(RustObject::from) {
            Ok(RustObject::Boolean(b)) => {
                if !b {
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

    // Integer
    {
        let value = Object::from(42);

        if let Err(e) = self::api::nvim_set_var(var, value) {
            eprintln!("Error setting var: {}", e);
        }

        match self::api::nvim_get_var(var).map(RustObject::from) {
            Ok(RustObject::Integer(i)) => {
                if i != 42 {
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

    // Float
    {
        let value = Object::from(123.456);

        if let Err(e) = self::api::nvim_set_var(var, value) {
            eprintln!("Error setting var: {}", e);
        }

        match self::api::nvim_get_var(var).map(RustObject::from) {
            Ok(RustObject::Float(f)) => {
                if ulps_ne!(f, 123.456) {
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

    // String
    {
        let string = LuaString::new("this is a test").unwrap();
        let value = Object::from(string);

        if let Err(e) = self::api::nvim_set_var(var, value) {
            eprintln!("Error setting var: {}", e);
        }

        match self::api::nvim_get_var(var) {
            Ok(object) if object.object_type() == ObjectType::kObjectTypeString => {
                let string = object.as_string_unchecked();
                if string != &LuaString::new("this is a test").unwrap() {
                    eprintln!(
                        "FAIL! Expected 'this is a test', got '{}'",
                        string.as_c_str().to_string_lossy()
                    );
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

    // Array
    {
        fn make_subject() -> Array {
            let o = Object::from(4242);
            Array::new([o])
        }
        let value = Object::from(make_subject());

        if let Err(e) = self::api::nvim_set_var(var, value) {
            eprintln!("Error setting var: {}", e);
        }

        match self::api::nvim_get_var(var) {
            Ok(object) if object.object_type() == ObjectType::kObjectTypeArray => {
                let array = object.as_array_unchecked();
                if array != &make_subject() {
                    eprintln!("FAIL! Expected 'this is a test', got '{:?}'", array);
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

    // Dictionary
    {
        fn make_subject() -> Dictionary {
            let key = LuaString::new("meow").unwrap();
            let value = Object::from(4242);
            Dictionary::new([KeyValuePair::new(key, value)])
        }
        let value = Object::from(make_subject());

        if let Err(e) = self::api::nvim_set_var(var, value) {
            eprintln!("Error setting var: {}", e);
        }

        match self::api::nvim_get_var(var) {
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
pub extern "C" fn test_nvim_set_vvar() -> Boolean {
    let mut result = true;

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

    result
}

#[no_mangle]
pub extern "C" fn test_nvim_buf_set_var() -> Boolean {
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
pub extern "C" fn test_nvim_get_current_buf() -> Boolean {
    self::api::nvim_get_current_buf() == 1
}
