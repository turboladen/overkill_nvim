use neovim_sys::api::vim::{
    Array, Boolean, Dictionary, Float, Integer, LuaString, Object, ObjectType,
};

#[derive(Debug, Clone, PartialEq)]
pub enum RustObject {
    Nil,
    Boolean(Boolean),
    Integer(Integer),
    Float(Float),
    String(LuaString),
    Array(Array),
    Dictionary(Dictionary),
    // LuaRef(LuaRef),
    // Buffer,
    // Window,
    // Tabpage,
}

impl From<Object> for RustObject {
    fn from(api_object: Object) -> Self {
        match api_object.object_type() {
            ObjectType::kObjectTypeNil => Self::Nil,
            ObjectType::kObjectTypeBoolean => {
                Self::Boolean(api_object.into_boolean_unchecked())
            }
            ObjectType::kObjectTypeInteger => Self::Integer(api_object.into_integer_unchecked()),
            ObjectType::kObjectTypeFloat => Self::Float(api_object.into_float_unchecked()),
            ObjectType::kObjectTypeString => Self::String(api_object.into_string_unchecked()),
            ObjectType::kObjectTypeArray => Self::Array(api_object.into_array_unchecked()),
            ObjectType::kObjectTypeDictionary => {
                Self::Dictionary(api_object.into_dictionary_unchecked())
            }
        }
    }
}

// impl<'a> From<&'a vim::Object> for RustObject {
//     fn from(api_object: &'a vim::Object) -> Self {
//         unsafe {
//             match api_object.object_type {
//                 ObjectType::kObjectTypeNil => Self::Nil,
//                 ObjectType::kObjectTypeBoolean => Self::Boolean(api_object.data.boolean),
//                 ObjectType::kObjectTypeInteger => Self::Integer(api_object.data.integer),
//                 ObjectType::kObjectTypeFloat => Self::Float(api_object.data.floating),
//                 ObjectType::kObjectTypeString => {
//                     Self::String(CString::from_raw(api_object.data.string.data))
//                 }
//                 ObjectType::kObjectTypeArray => Self::Array(Array::from(api_object.data.array)),
//                 ObjectType::kObjectTypeDictionary => {
//                     Self::Dictionary(Dictionary::from(api_object.data.dictionary))
//                 }
//             }
//         }
//     }
// }
