use neovim_sys::api::vim::ObjectType;

use super::api;
use crate::api::{Boolean, LuaString, Object, RustObject};

// #[no_mangle]
// pub extern "C" fn nvim_get_current_buf_test() -> Boolean {
//     self::api::nvim_get_current_buf() == 1
// }

#[no_mangle]
pub extern "C" fn test_set_get_var() -> Boolean {
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
                if f != 123.456 {
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
        // let s = RustObject::Integer(4242);
        // let o = Object::from(s);
        // let items = [o];
        // let value = Array::from(items.as_ref());

        // if let Err(e) = self::api::nvim_set_var(var, value) {
        //     eprintln!("Error setting var: {}", e);
        // }

        // match self::api::nvim_get_var(var) {
        //     Object::Array(a) => {
        //         if a.as_slice() != items.as_slice() {
        //             result = false;
        //         }
        //     }
        //     t => {
        //         eprintln!("Got unexpected value type: {:?}", t);
        //         result = false;
        //     }
        // }
    }

    // Dictionary
    //     if var.is_null() {
    //         return false;
    //     }

    //     let var = unsafe { CStr::from_ptr(var) };
    //     // let expected = Array::new(Cow::Borrowed(unsafe { expected.as_ref().unwrap() }));
    //     let expected = Dictionary::new(Cow::Owned(expected));

    //     match self::api::nvim_get_var(var.to_str().unwrap()) {
    //         Object::Dictionary(d) => d == expected,
    //         t => {
    //             eprintln!("Got unexpected value type: {:?}", t);
    //             false
    //         }
    //     }
    // }

    result
}
