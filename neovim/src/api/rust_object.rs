pub use neovim_sys::api::vim::{Boolean, Float, Integer, LuaRef};

use super::{Array, Dictionary, Object, NString};
use neovim_sys::api::vim::{self, ObjectType};

#[derive(Debug, Clone, PartialEq)]
pub enum RustObject {
    Nil,
    Boolean(Boolean),
    Integer(Integer),
    Float(Float),
    String(NString),
    Array(Array),
    Dictionary(Dictionary),
    // LuaRef(LuaRef),
    // Buffer,
    // Window,
    // Tabpage,
}

impl From<Object> for RustObject {
    fn from(object: Object) -> Self {
        Self::from(object.to_inner())
    }
}

impl From<vim::Object> for RustObject {
    fn from(api_object: vim::Object) -> Self {
        unsafe {
            match api_object.object_type {
                ObjectType::kObjectTypeNil => Self::Nil,
                ObjectType::kObjectTypeBoolean => Self::Boolean(api_object.data.boolean),
                ObjectType::kObjectTypeInteger => Self::Integer(api_object.data.integer),
                ObjectType::kObjectTypeFloat => Self::Float(api_object.data.floating),
                ObjectType::kObjectTypeString => Self::String(NString::new(api_object.data.string)),
                ObjectType::kObjectTypeArray => Self::Array(Array::new(api_object.data.array)),
                ObjectType::kObjectTypeDictionary => {
                    Self::Dictionary(Dictionary::new(api_object.data.dictionary))
                }
            }
        }
    }
}

impl<'a> From<&'a vim::Object> for RustObject {
    fn from(api_object: &'a vim::Object) -> Self {
        unsafe {
            match api_object.object_type {
                ObjectType::kObjectTypeNil => Self::Nil,
                ObjectType::kObjectTypeBoolean => Self::Boolean(api_object.data.boolean),
                ObjectType::kObjectTypeInteger => Self::Integer(api_object.data.integer),
                ObjectType::kObjectTypeFloat => Self::Float(api_object.data.floating),
                ObjectType::kObjectTypeString => Self::String(NString::new(api_object.data.string)),
                ObjectType::kObjectTypeArray => Self::Array(Array::new(api_object.data.array)),
                ObjectType::kObjectTypeDictionary => {
                    Self::Dictionary(Dictionary::new(api_object.data.dictionary))
                }
            }
        }
    }
}
