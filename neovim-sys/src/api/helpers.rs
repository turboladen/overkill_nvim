use super::vim::{Array, Dictionary, Object, String};
use std::os::raw::c_char;

extern "C" {
    pub fn cstr_to_string(cstr: *const c_char) -> String;

    pub fn api_free_string(api_string: String);
    pub fn api_free_object(api_object: Object);
    pub fn api_free_array(api_array: Array);
    pub fn api_free_dictionary(api_dictionary: Dictionary);
}
