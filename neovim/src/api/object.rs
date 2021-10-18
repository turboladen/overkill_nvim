pub use neovim_sys::api::vim::{Boolean, Float, Integer, LuaRef};

use super::{Dictionary, String};
use neovim_sys::api::vim::{self, Array, ObjectType};
use std::borrow::Cow;

pub enum Object<'a> {
    Nil,
    Boolean(Boolean),
    Integer(Integer),
    Float(Float),
    String(String<'a>),
    Array(Array),
    Dictionary(Dictionary),
    LuaRef(LuaRef),
    // Buffer,
    // Window,
    // Tabpage,
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
                ObjectType::kObjectTypeArray => Self::Array(api_object.data.array),
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
                ObjectType::kObjectTypeArray => Self::Array(api_object.data.array),
                ObjectType::kObjectTypeDictionary => Self::Dictionary(Dictionary::new(
                    api_object.data.dictionary,
                )),
                ObjectType::kObjectTypeLuaRef => Self::LuaRef(api_object.data.luaref),
            }
        }
    }
}
