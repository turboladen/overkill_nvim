use log::debug;
use super::{Dictionary, Object, String as LuaString};

#[derive(Debug)]
#[repr(C)]
pub struct KeyValuePair {
    pub key: LuaString,
    pub value: Object,
}

impl KeyValuePair {
    pub fn new(key: LuaString, value: Object) -> Self {
        Self { key, value }
    }
}

impl Drop for KeyValuePair {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.key.data.as_mut());
            // std::ptr::drop_in_place(self.value);
        }
    }
}

impl Clone for KeyValuePair {
    fn clone(&self) -> Self {
        debug!("Cloning KeyValuePair...");
        Self {
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

impl From<Dictionary> for Vec<KeyValuePair> {
    fn from(dict: Dictionary) -> Self {
        debug!("Vec<KeyValuePair>::from(Dictionary)");
        let v = unsafe { Vec::from_raw_parts(dict.items.as_ptr(), dict.size, dict.capacity) };
        std::mem::forget(dict);

        v
    }
}

impl<'a> From<&'a Dictionary> for &'a [KeyValuePair] {
    fn from(dict: &'a Dictionary) -> Self {
        dict.as_slice()
    }
}
