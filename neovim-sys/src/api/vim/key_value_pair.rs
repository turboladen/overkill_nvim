use super::{Object, String as LuaString};

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct KeyValuePair {
    key: LuaString,
    value: Object,
}

impl KeyValuePair {
    pub fn new(key: LuaString, value: Object) -> Self {
        Self { key, value }
    }

    /// Get a reference to the key value pair's key.
    pub fn key(&self) -> &LuaString {
        &self.key
    }

    /// Get a reference to the key value pair's value.
    pub fn value(&self) -> &Object {
        &self.value
    }
}
