pub use neovim_sys::api::vim::{Boolean, Float, Integer, LuaRef};

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
                ObjectType::kObjectTypeDictionary => Self::Dictionary(Dictionary {
                    inner: api_object.data.dictionary,
                }),
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
                ObjectType::kObjectTypeDictionary => Self::Dictionary(Dictionary {
                    inner: api_object.data.dictionary,
                }),
                ObjectType::kObjectTypeLuaRef => Self::LuaRef(api_object.data.luaref),
            }
        }
    }
}

#[derive(Default)]
pub struct Dictionary {
    inner: vim::Dictionary,
}

impl Dictionary {
    pub fn new(inner: vim::Dictionary) -> Self {
        Self { inner }
    }

    pub fn get<'b>(&'b self, key: &str) -> Option<Object<'b>> {
        self.iter().find(|(k, _)| k.as_str() == key).map(|(_, v)| v)
    }

    pub fn iter(&self) -> DictionaryIter<'_> {
        DictionaryIter {
            kv_iter: self.kvs_as_slice().iter(),
        }
    }

    fn kvs_as_slice(&self) -> &[vim::KeyValuePair] {
        unsafe { std::slice::from_raw_parts(self.inner.items, self.inner.size) }
    }

    pub fn inner(&self) -> vim::Dictionary {
        self.inner
    }

    pub fn inner_ref(&self) -> &vim::Dictionary {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut vim::Dictionary {
        &mut self.inner
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        self.inner.free()
    }
}

pub struct DictionaryIter<'a> {
    kv_iter: std::slice::Iter<'a, vim::KeyValuePair>,
}

impl<'a> Iterator for DictionaryIter<'a> {
    type Item = (String<'a>, Object<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.kv_iter
            .next()
            .map(|kv| (String::new(Cow::Borrowed(&kv.key)), Object::from(&kv.value)))
    }
}

#[derive(Clone)]
pub struct String<'a> {
    inner: Cow<'a, vim::String>,
}

impl<'a> String<'a> {
    pub fn new(inner: Cow<'a, vim::String>) -> Self {
        Self { inner }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.inner.data as *const u8, self.inner.size) }
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.as_slice()).unwrap()
    }
}

impl<'a> Drop for String<'a> {
    fn drop(&mut self) {
        self.inner.free()
    }
}
