use std::borrow::Cow;

use neovim_sys::api::vim::{self, Array, Boolean, Float, Integer, LuaRef, ObjectType};

pub enum Object {
    Nil,
    Boolean(Boolean),
    Integer(Integer),
    Float(Float),
    String(String),
    Array(Array),
    Dictionary(Dictionary),
    LuaRef(LuaRef),
    // Buffer,
    // Window,
    // Tabpage,
}

impl From<vim::Object> for Object {
    fn from(api_object: vim::Object) -> Self {
        unsafe {
            match api_object.object_type {
                ObjectType::kObjectTypeNil => Self::Nil,
                ObjectType::kObjectTypeBoolean => Self::Boolean(api_object.data.boolean),
                ObjectType::kObjectTypeInteger => Self::Integer(api_object.data.integer),
                ObjectType::kObjectTypeFloat => Self::Float(api_object.data.floating),
                ObjectType::kObjectTypeString => Self::String(String::new(api_object.data.string)),
                ObjectType::kObjectTypeArray => Self::Array(api_object.data.array),
                ObjectType::kObjectTypeDictionary => Self::Dictionary(Dictionary {
                    inner: api_object.data.dictionary,
                }),
                ObjectType::kObjectTypeLuaRef => Self::LuaRef(api_object.data.luaref),
            }
        }
    }
}

pub struct Dictionary {
    inner: vim::Dictionary,
}

impl Dictionary {
    pub fn new(inner: vim::Dictionary) -> Self {
        Self { inner }
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        self.kvs_as_slice()
            .iter()
            .map(|kv| KeyValuePair::new(Cow::Borrowed(kv)))
            .find(|kv| kv.key() == key)
            .map(|kv| kv.value())
    }

    fn kvs_as_slice(&self) -> &[vim::KeyValuePair] {
        unsafe { std::slice::from_raw_parts(self.inner.items, self.inner.size) }
    }

}

pub struct KeyValuePair<'a> {
    inner: Cow<'a, vim::KeyValuePair>,
}

impl<'a> KeyValuePair<'a> {
    pub fn new(inner: Cow<'a, vim::KeyValuePair>) -> Self {
        Self { inner }
    }

    pub fn key(&self) -> &str {
        String::new(self.inner.key).as_str()
    }

    pub fn value(&self) -> Object {
        todo!()
    }
}

pub struct String {
    inner: vim::String,
}

impl String {
    pub fn new(inner: vim::String) -> Self {
        Self { inner }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.inner.data as *const u8, self.inner.size) }
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.as_slice()).unwrap()
    }
}
