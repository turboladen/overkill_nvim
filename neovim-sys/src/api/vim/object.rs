use super::{helpers, Array, Boolean, Dictionary, Float, Integer, LuaRef, String};
use std::{fmt::Debug, };

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
