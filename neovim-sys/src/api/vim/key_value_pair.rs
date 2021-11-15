use super::{LuaString, Object};
use std::fmt;

#[derive(Clone, PartialEq)]
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

impl fmt::Debug for KeyValuePair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyValuePair")
            .field("key", &self.key.to_string_lossy())
            .field("value", &self.value)
            .finish()
    }
}
