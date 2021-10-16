use std::os::raw::c_char;

extern "C" {
    pub fn nvim_get_var(name: self::String, err: *mut Error) -> Object;
    pub fn nvim_buf_get_var(name: self::String, err: *mut Error) -> Object;

    pub fn nvim_feedkeys(keys: self::String, mode: self::String, escape_csi: Boolean);

    pub fn nvim_get_mode() -> Dictionary;
}

#[derive(Clone)]
#[repr(C)]
pub struct Object {
    pub object_type: ObjectType,
    pub data: ObjectData,
}

impl Object {
    pub fn new(object_type: ObjectType, data: ObjectData) -> Self {
        Self { object_type, data }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe { helpers::api_free_object(*self) }
    }
}

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum ObjectType {
    kObjectTypeNil = 0,
    kObjectTypeBoolean,
    kObjectTypeInteger,
    kObjectTypeFloat,
    kObjectTypeString,
    kObjectTypeArray,
    kObjectTypeDictionary,
    kObjectTypeLuaRef,
    // EXT types, cannot be split or reordered, see #EXT_OBJECT_TYPE_SHIFT
    // kObjectTypeBuffer,
    // kObjectTypeWindow,
    // kObjectTypeTabpage,
}

#[derive(Clone)]
#[repr(C)]
pub union ObjectData {
    pub boolean: Boolean,
    pub integer: Integer,
    pub floating: Float,
    pub string: String,
    pub array: Array,
    pub dictionary: Dictionary,
    pub luaref: LuaRef,
}

pub type Boolean = bool;
pub type Integer = i64;
pub type Float = f64;
pub type LuaRef = isize;

#[derive(Clone)]
#[repr(C)]
pub struct Array {
    items: *const Object,
    size: usize,
    capacity: usize,
}

impl Default for Array {
    fn default() -> Self {
        Self {
            items: std::ptr::null(),
            size: 0,
            capacity: 0,
        }
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        unsafe { helpers::api_free_array(*self) }
    }
}

#[repr(C)]
pub struct Dictionary {
    pub items: *const KeyValuePair,
    pub size: usize,
    pub capacity: usize,
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe { helpers::api_free_dictionary(*self) }
    }
}

#[repr(C)]
pub struct KeyValuePair {
    pub key: String,
    pub value: Object,
}

#[derive(Clone)]
#[repr(C)]
pub struct String {
    pub data: *const c_char,
    pub size: usize,
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe { helpers::api_free_string(*self) }
    }
}

#[repr(C)]
pub struct Error {
    error_type: ErrorType,
    msg: *const c_char,
}

impl Default for Error {
    fn default() -> Self {
        Self {
            error_type: ErrorType::kErrorTypeNone,
            msg: std::ptr::null(),
        }
    }
}

#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum ErrorType {
    kErrorTypeNone = -1,
    kErrorTypeException,
    kErrorTypeValidation,
}
