use super::vim::{Array, Dictionary, Object, String};
use std::os::raw::c_char;

extern "C" {
    pub fn cstr_to_string(cstr: *const c_char) -> String;
    // pub fn kv_push(cstr: *const c_char) -> String;

    // pub fn copy_object(object: Object) -> Object;
    pub fn copy_dictionary(dictionary: Dictionary) -> Dictionary;
    pub fn copy_array(array: Array) -> Array;
    pub fn copy_string(string: String) -> String;

    pub fn api_free_object(object: Object);
    // pub fn api_free_dictionary(dictionary: Dictionary);
    pub fn api_free_array(array: Array);
    pub fn api_free_string(string: String);
}
