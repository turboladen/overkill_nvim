#![allow(missing_docs, clippy::missing_panics_doc)]

use crate::api::{self, Mode, Object, RustObject};
use neovim_sys::api::nvim::{Array, Dictionary, KeyValuePair, NvimString, ObjectType};
use nvim_api_test::nvim_test;
use std::borrow::Borrow;

fn _test_nvim_setget_var(var: &str, value: Object, expected_object_variant: &RustObject) {
    self::api::nvim::nvim_set_var(var, value).unwrap();

    let t = self::api::nvim::nvim_get_var(var)
        .map(RustObject::from)
        .unwrap();

    assert_eq!(
        t, *expected_object_variant,
        "Got unexpected value type: {:?}",
        t
    );
}

#[nvim_test]
fn test_nvim_set_var() {
    // nil
    {
        let value = Object::new_nil();
        _test_nvim_setget_var("nvim_set_var_test_nil", value, &RustObject::Nil);
    }

    // bool
    {
        let var = "nvim_set_var_test_bool";
        let value = Object::from(true);

        _test_nvim_setget_var(var, value, &RustObject::Boolean(true));
    }

    // Integer
    {
        let var = "nvim_set_var_test_integer";
        let value = Object::from(42);

        _test_nvim_setget_var(var, value, &RustObject::Integer(42));
    }

    // Float
    {
        let var = "nvim_set_var_test_float";
        let value = Object::from(123.456);

        _test_nvim_setget_var(var, value, &RustObject::Float(123.456));
    }

    // String
    {
        let var = "nvim_set_var_test_string";
        let string = NvimString::new_unchecked("this is a test");
        let value = Object::from(string);

        _test_nvim_setget_var(
            var,
            value,
            &RustObject::String(NvimString::new_unchecked("this is a test")),
        );
    }

    // Array
    {
        fn make_subject() -> Array {
            let o = Object::from(4242);
            Array::new([o])
        }
        let value = Object::from(make_subject());
        let var = "nvim_set_var_test_array";

        _test_nvim_setget_var(var, value, &RustObject::Array(make_subject()));
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

        _test_nvim_setget_var(var, value, &RustObject::Dictionary(make_subject()));
    }
}

#[nvim_test]
fn test_nvim_set_vvar() {
    let vvar = "warningmsg";
    let string = NvimString::new_unchecked("meow");
    let value = Object::from(string);

    self::api::nvim::nvim_set_vvar(vvar, value).unwrap();

    let object = self::api::nvim::nvim_get_vvar(vvar).unwrap();
    assert_eq!(object.object_type(), ObjectType::kObjectTypeString);

    let string = object.into_string_unchecked();
    assert_eq!(string, NvimString::new_unchecked("meow"));
}

#[nvim_test]
fn test_nvim_buf_set_var() {
    fn make_subject() -> Dictionary {
        let key = NvimString::new_unchecked("meow");
        let value = Object::from(4242);
        Dictionary::new([KeyValuePair::new(key, value)])
    }

    let value = Object::from(make_subject());
    let var = "nvim_rs_buf_set_get_var";

    self::api::buffer::nvim_buf_set_var(0, var, value).unwrap();

    let object = self::api::buffer::nvim_buf_get_var(0, var).unwrap();
    assert_eq!(object.object_type(), ObjectType::kObjectTypeDictionary);

    let dict = object.into_dictionary_unchecked();
    assert_eq!(dict, make_subject());
}

#[nvim_test]
fn test_nvim_get_current_buf() {
    assert_eq!(self::api::nvim::nvim_get_current_buf(), 1);
}

#[nvim_test]
fn test_nvim_feedkeys() {
    self::api::nvim::nvim_feedkeys("j", Mode::Normal, false).unwrap();
}

#[nvim_test]
fn test_nvim_get_mode() {
    let current_mode = self::api::nvim::nvim_get_mode().unwrap();

    assert_eq!(current_mode.mode(), Mode::Normal);
}

#[nvim_test]
fn test_nvim_set_global_option() {
    // Boolean option
    {
        let option_name = "autoread";

        let value = self::api::nvim::nvim_get_global_option(option_name).unwrap();
        assert!(value.into_boolean_unchecked());

        self::api::nvim::nvim_set_global_option(option_name, false).unwrap();

        let value = self::api::nvim::nvim_get_global_option(option_name).unwrap();
        assert!(!value.as_boolean_unchecked());
    }

    // Integer option
    {
        let option_name = "aleph";

        let value = self::api::nvim::nvim_get_global_option(option_name).unwrap();
        assert_eq!(value.into_integer_unchecked(), 224);

        self::api::nvim::nvim_set_global_option(option_name, 225).unwrap();

        let value = self::api::nvim::nvim_get_global_option(option_name).unwrap();
        assert_eq!(value.into_integer_unchecked(), 225);
    }

    // String option
    {
        let option_name = "pastetoggle";

        let value = self::api::nvim::nvim_get_global_option(option_name).unwrap();
        let expected = "";

        assert_eq!(Borrow::<str>::borrow(value.as_string_unchecked()), expected);

        let expected_in = NvimString::new_unchecked("<F8>");
        let expected = NvimString::new_unchecked("<F8>");

        self::api::nvim::nvim_set_global_option(option_name, expected_in).unwrap();

        let value = self::api::nvim::nvim_get_global_option(option_name).unwrap();
        assert_eq!(value.into_string_unchecked(), expected);
    }
}