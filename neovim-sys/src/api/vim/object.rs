use super::{helpers, Array, Boolean, Dictionary, Float, Integer, LuaRef, String};
use log::debug;
use std::{convert::TryFrom, fmt::Debug, mem::ManuallyDrop};

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
        debug!("Cloning Object...");
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
            ObjectType::kObjectTypeString => {
                debug!("Cloning Object::String...");
                Self {
                object_type: self.object_type,
                data: ObjectData {
                    string: unsafe { self.data.string.clone() },
                },
            }},
            ObjectType::kObjectTypeArray => Self {
                object_type: self.object_type,
                data: ObjectData {
                    array: unsafe { self.data.array.clone() },
                },
            },
            ObjectType::kObjectTypeDictionary => Self {
                object_type: self.object_type,
                data: ObjectData {
                    dictionary: unsafe { self.data.dictionary.clone() },
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

impl TryFrom<Object> for Boolean {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type {
            ObjectType::kObjectTypeBoolean => Ok(unsafe { value.data.boolean }),
            _ => Err(()),
        }
    }
}

impl TryFrom<Object> for Integer {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type {
            ObjectType::kObjectTypeInteger => Ok(unsafe { value.data.integer }),
            _ => Err(()),
        }
    }
}

impl TryFrom<Object> for Float {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type {
            ObjectType::kObjectTypeFloat => Ok(unsafe { value.data.floating }),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    pub string: ManuallyDrop<String>,
    pub array: ManuallyDrop<Array>,
    pub dictionary: ManuallyDrop<Dictionary>,
    pub luaref: LuaRef,
}
