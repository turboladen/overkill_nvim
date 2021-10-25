use super::{Object, String};

#[derive(Debug, Copy)]
#[repr(C)]
pub struct Dictionary {
    pub items: *mut KeyValuePair,
    pub size: usize,
    pub capacity: usize,
}

impl Dictionary {
    pub fn as_slice(&self) -> &[KeyValuePair] {
        unsafe { std::slice::from_raw_parts(self.items, self.size) }
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self {
            items: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }
}

impl Clone for Dictionary {
    fn clone(&self) -> Self {
        let mut new_dict = Self::default();

        new_dict.size = self.size;
        new_dict.capacity = self.capacity;

        unsafe {
            self.items.copy_to(new_dict.items, self.size);
        }

        new_dict
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct KeyValuePair {
    pub key: String,
    pub value: Object,
}
