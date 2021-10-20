pub use neovim_sys::api::vim::{Boolean, Float, Integer, LuaRef};

use super::{Array, Dictionary, String};
use neovim_sys::api::vim::{self, ObjectData, ObjectType};
use std::{borrow::Cow, };

#[derive(Debug)]
pub enum Object<'a> {
    Nil,
    Boolean(Boolean),
    Integer(Integer),
    Float(Float),
    String(String<'a>),
    Array(Array<'a>),
    Dictionary(Dictionary),
    LuaRef(LuaRef),
    // Buffer,
    // Window,
    // Tabpage,
}

impl<'a> Object<'a> {
    pub fn into_vim(self) -> vim::Object {
        match self {
            Self::Nil => vim::Object {
                object_type: ObjectType::kObjectTypeNil,
                data: ObjectData { boolean: false },
            },
            Self::Boolean(boolean) => vim::Object {
                object_type: ObjectType::kObjectTypeBoolean,
                data: ObjectData { boolean },
            },
            Self::Integer(integer) => vim::Object {
                object_type: ObjectType::kObjectTypeInteger,
                data: ObjectData { integer },
            },
            Self::Float(floating) => vim::Object {
                object_type: ObjectType::kObjectTypeFloat,
                data: ObjectData { floating },
            },
            Self::String(string) => vim::Object {
                object_type: ObjectType::kObjectTypeString,
                data: ObjectData {
                    string: string.as_inner(),
                },
            },
            Self::Array(array) => vim::Object {
                object_type: ObjectType::kObjectTypeArray,
                data: ObjectData {
                    array: array.into_inner(),
                },
            },
            Self::Dictionary(dictionary) => vim::Object {
                object_type: ObjectType::kObjectTypeDictionary,
                data: ObjectData {
                    dictionary: dictionary.into_inner(),
                },
            },
            Self::LuaRef(luaref) => vim::Object {
                object_type: ObjectType::kObjectTypeLuaRef,
                data: ObjectData { luaref },
            },
        }
    }
}

impl<'a> From<vim::Object> for Object<'a> {
    fn from(api_object: vim::Object) -> Self {
        unsafe {
            match api_object.object_type {
                ObjectType::kObjectTypeNil => Self::Nil,
                ObjectType::kObjectTypeBoolean => Self::Boolean(api_object.data.boolean),
                ObjectType::kObjectTypeInteger => Self::Integer(api_object.data.integer),
                ObjectType::kObjectTypeFloat => Self::Float(api_object.data.floating),
                ObjectType::kObjectTypeString => {
                    Self::String(String::new(Cow::Owned(api_object.data.string)))
                }
                ObjectType::kObjectTypeArray => {
                    Self::Array(Array::new(Cow::Owned(api_object.data.array)))
                }
                ObjectType::kObjectTypeDictionary => {
                    Self::Dictionary(Dictionary::new(api_object.data.dictionary))
                }
                ObjectType::kObjectTypeLuaRef => Self::LuaRef(api_object.data.luaref),
            }
        }
    }
}

impl<'a, 'b: 'a> From<&'b vim::Object> for Object<'a> {
    fn from(api_object: &'b vim::Object) -> Self {
        unsafe {
            match api_object.object_type {
                ObjectType::kObjectTypeNil => Self::Nil,
                ObjectType::kObjectTypeBoolean => Self::Boolean(api_object.data.boolean),
                ObjectType::kObjectTypeInteger => Self::Integer(api_object.data.integer),
                ObjectType::kObjectTypeFloat => Self::Float(api_object.data.floating),
                ObjectType::kObjectTypeString => {
                    Self::String(String::new(Cow::Borrowed(&api_object.data.string)))
                }
                ObjectType::kObjectTypeArray => {
                    Self::Array(Array::new(Cow::Borrowed(&api_object.data.array)))
                }
                ObjectType::kObjectTypeDictionary => {
                    Self::Dictionary(Dictionary::new(api_object.data.dictionary))
                }
                ObjectType::kObjectTypeLuaRef => Self::LuaRef(api_object.data.luaref),
            }
        }
    }
}
