use neovim_sys::api::nvim::{
    Array, Boolean, Dictionary, Float, Integer, NvimString, Object, ObjectType,
};

/// While the internal object, `neovim_sys::vim::Object` is just fine to work with, this type
/// provides a Rust-ier interface to that object.
///
#[derive(Debug, Clone, PartialEq)]
pub enum RustObject {
    /// Nil! Represents a `neovim_sys::vim::Object` where its `object_type()` is
    /// `ObjectType::kObjectTypeNil`.
    ///
    Nil,

    /// Represents a `neovim_sys::vim::Object` where its `object_type()` is
    /// `ObjectType::kObjectTypeBoolean`.
    ///
    Boolean(Boolean),

    /// Represents a `neovim_sys::vim::Object` where its `object_type()` is
    /// `ObjectType::kObjectTypeInteger`.
    ///
    Integer(Integer),

    /// Represents a `neovim_sys::vim::Object` where its `object_type()` is
    /// `ObjectType::kObjectTypeFloat`.
    ///
    Float(Float),

    /// Represents a `neovim_sys::vim::Object` where its `object_type()` is
    /// `ObjectType::kObjectTypeString`.
    ///
    String(NvimString),

    /// Represents a `neovim_sys::vim::Object` where its `object_type()` is
    /// `ObjectType::kObjectTypeArray`.
    ///
    Array(Array),

    /// Represents a `neovim_sys::vim::Object` where its `object_type()` is
    /// `ObjectType::kObjectTypeDictionary`.
    ///
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
            ObjectType::kObjectTypeBoolean => Self::Boolean(api_object.into_boolean_unchecked()),
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
