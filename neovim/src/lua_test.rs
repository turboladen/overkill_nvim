use super::api;
use crate::api::{Boolean, NString, RustObject};

// #[no_mangle]
// pub extern "C" fn nvim_get_current_buf_test() -> Boolean {
//     self::api::nvim_get_current_buf() == 1
// }

#[no_mangle]
pub extern "C" fn test_primitives() -> Boolean {
    let mut result = true;

    // NString
    {
        let nstring = NString::from("meow");
        if &nstring.to_string() != "meow" {
            eprintln!("Uh oh: {}", nstring);
            result = false;
        }
    }

    result
}

#[no_mangle]
pub extern "C" fn test_set_get_var() -> Boolean {
    let mut result = true;

    let var = "nvim_rs_set_get_var";

    // nil
    {
        let value = RustObject::Nil;

        if let Err(e) = self::api::nvim_set_var(var, value.into()) {
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
        let value = RustObject::Boolean(true);

        if let Err(e) = self::api::nvim_set_var(var, value.into()) {
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
        let value = RustObject::Integer(42);

        if let Err(e) = self::api::nvim_set_var(var, value.into()) {
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
        let value = RustObject::Float(123.456);

        if let Err(e) = self::api::nvim_set_var(var, value.into()) {
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
        let string = crate::api::NString::from("this is a test");
        // let value = RustObject::String(string);

        // if let Err(e) = self::api::nvim_set_var(var, value.into()) {
        //     eprintln!("Error setting var: {}", e);
        // }

        // match self::api::nvim_get_var(var).map(RustObject::from) {
        //     Ok(RustObject::String(s)) => {
        //         if s.as_str() != "this is a test" {
        //             eprintln!("FAIL! Expected 'this is a test', got '{}'", s.as_str());
        //             result = false;
        //         }
        //     }
        //     Ok(t) => {
        //         eprintln!("Got unexpected value type: {:?}", t);
        //         result = false;
        //     }
        //     Err(e) => {
        //         eprintln!("Got error during test: {}", e);
        //         result = false;
        //     }
        // }
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
