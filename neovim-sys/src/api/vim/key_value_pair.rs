use super::{LuaString, Object};

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct KeyValuePair {
    key: LuaString,
    value: Object,
}

impl KeyValuePair {
    #[must_use]
    pub fn new(key: LuaString, value: Object) -> Self {
        Self { key, value }
    }

    /// Get a reference to the key value pair's key.
    #[must_use]
    pub const fn key(&self) -> &LuaString {
        &self.key
    }

    /// Get a reference to the key value pair's value.
    #[must_use]
    pub const fn value(&self) -> &Object {
        &self.value
    }
}
