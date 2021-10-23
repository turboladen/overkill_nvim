use super::{buffer::Buffer, helpers};
use std::{fmt::Debug, os::raw::c_char};

extern "C" {
    pub fn nvim_get_var(name: self::String, err: *mut Error) -> Object;
    pub fn nvim_set_var(name: self::String, value: Object, err: *mut Error);

    pub fn nvim_buf_get_var(name: self::String, err: *mut Error) -> Object;

    pub fn nvim_feedkeys(keys: self::String, mode: self::String, escape_csi: Boolean);

    pub fn nvim_get_mode() -> Dictionary;
    pub fn nvim_get_current_buf() -> Buffer;
    pub fn nvim_replace_termcodes(
        s: String,
        from_part: Boolean,
        do_lt: Boolean,
        special: Boolean,
    ) -> String;

    pub fn nvim_exec(src: String, output: Boolean, err: *mut Error) -> String;

    pub fn nvim_set_hl(namespace_id: Integer, name: String, val: Dictionary, err: *mut Error);
    pub fn nvim_get_namespaces() -> Dictionary;
    pub fn nvim_create_namespace(name: String) -> Integer;
}

#[repr(C)]
pub struct Object {
    pub object_type: ObjectType,
    pub data: ObjectData,
}

impl Object {
    pub fn new(object_type: ObjectType, data: ObjectData) -> Self {
        Self { object_type, data }
    }

    pub fn free(self) {
        unsafe { helpers::api_free_object(self) }
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        match self.object_type {
            ObjectType::kObjectTypeNil => Self {
                object_type: self.object_type,
                data: ObjectData { boolean: false },
            },
            ObjectType::kObjectTypeBoolean => Self {
                object_type: self.object_type,
                data: ObjectData {
                    boolean: unsafe { self.data.boolean },
                },
            },
            ObjectType::kObjectTypeInteger => Self {
                object_type: self.object_type,
                data: ObjectData {
                    integer: unsafe { self.data.integer },
                },
            },
            ObjectType::kObjectTypeFloat => Self {
                object_type: self.object_type,
                data: ObjectData {
                    floating: unsafe { self.data.floating },
                },
            },
            ObjectType::kObjectTypeString => Self {
                object_type: self.object_type,
                data: ObjectData {
                    string: unsafe { helpers::copy_string(self.data.string) },
                },
            },
            ObjectType::kObjectTypeArray => Self {
                object_type: self.object_type,
                data: ObjectData {
                    array: unsafe { helpers::copy_array(self.data.array) },
                },
            },
            ObjectType::kObjectTypeDictionary => Self {
                object_type: self.object_type,
                data: ObjectData {
                    dictionary: unsafe { helpers::copy_dictionary(self.data.dictionary) },
                },
            },
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Object");
        d.field("object_type", &self.object_type);

        unsafe {
            match self.object_type {
                ObjectType::kObjectTypeNil => d.field("data", &"nil"),
                ObjectType::kObjectTypeBoolean => d.field("data", &self.data.boolean),
                ObjectType::kObjectTypeInteger => d.field("data", &self.data.integer),
                ObjectType::kObjectTypeFloat => d.field("data", &self.data.floating),
                ObjectType::kObjectTypeString => d.field("data", &self.data.string),
                ObjectType::kObjectTypeArray => d.field("data", &self.data.array),
                ObjectType::kObjectTypeDictionary => d.field("data", &self.data.dictionary),
                // ObjectType::kObjectTypeLuaRef => d.field("data", &self.data.luaref),
            };
        }

        d.finish()
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(usize)]
pub enum ObjectType {
    kObjectTypeNil = 0,
    kObjectTypeBoolean,
    kObjectTypeInteger,
    kObjectTypeFloat,
    kObjectTypeString,
    kObjectTypeArray,
    kObjectTypeDictionary,
    // kObjectTypeLuaRef,
    // EXT types, cannot be split or reordered, see #EXT_OBJECT_TYPE_SHIFT
    // kObjectTypeBuffer,
    // kObjectTypeWindow,
    // kObjectTypeTabpage,
}

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

#[derive(Debug, Copy)]
#[repr(C)]
pub struct Array {
    pub items: *mut Object,
    pub size: usize,
    pub capacity: usize,
}

impl Array {
    pub fn as_slice(&self) -> &[Object] {
        unsafe { std::slice::from_raw_parts(self.items, self.size) }
    }

    pub fn free(self) {
        unsafe { helpers::api_free_array(self) }
    }
}

impl Default for Array {
    fn default() -> Self {
        Self {
            items: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }
}

impl Clone for Array {
    fn clone(&self) -> Self {
        let mut new_array = Array::default();

        new_array.size = self.size;
        new_array.capacity = self.capacity;

        unsafe {
            self.items.copy_to(new_array.items, self.size);
        }

        new_array
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Dictionary {
    pub items: *const KeyValuePair,
    pub size: usize,
    pub capacity: usize,
}

impl Dictionary {
    pub fn as_slice(&self) -> &[KeyValuePair] {
        unsafe { std::slice::from_raw_parts(self.items, self.size) }
    }

    pub fn free(self) {
        unsafe { helpers::api_free_dictionary(self) }
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self {
            items: std::ptr::null(),
            size: 0,
            capacity: 0,
        }
    }
}

impl Clone for Dictionary {
    fn clone(&self) -> Self {
        unsafe { helpers::copy_dictionary(*self) }
    }
}

impl Copy for Dictionary {}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct KeyValuePair {
    pub key: String,
    pub value: Object,
}

#[derive(Debug, Copy)]
#[repr(C)]
pub struct String {
    pub data: *mut c_char,
    pub size: usize,
}

impl String {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data as *const u8, self.size) }
    }

    pub fn free(self) {
        unsafe { helpers::api_free_string(self) }
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        if !self.data.is_null() {
            let mut dst = Vec::with_capacity(self.size);
            unsafe {
                std::ptr::copy(self.data, dst.as_mut_ptr(), self.size);
                dst.set_len(self.size);
            }
            String {
                data: dst.as_mut_ptr(),
                size: self.size,
            }
        } else {
            String {
                data: std::ptr::null_mut(),
                size: 0,
            }
        }
    }
}

#[repr(C)]
pub struct Error {
    pub error_type: ErrorType,
    pub msg: *const c_char,
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
