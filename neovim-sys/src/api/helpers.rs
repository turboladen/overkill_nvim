use super::vim::{Array, Dictionary, Object, String};
use std::os::raw::c_char;

extern "C" {
    pub fn cstr_to_string(cstr: *const c_char) -> String;
    // pub fn kv_push(cstr: *const c_char) -> String;

    pub(super) fn copy_object(object: Object) -> Object;
    pub(super) fn copy_dictionary(dictionary: Dictionary) -> Dictionary;
    pub(super) fn copy_array(array: Array) -> Array;
    pub(super) fn copy_string(string: String) -> String;

    pub(super) fn api_free_object(object: Object);
    pub(super) fn api_free_dictionary(dictionary: Dictionary);
    pub(super) fn api_free_array(array: Array);
    pub(super) fn api_free_string(string: String);
}
