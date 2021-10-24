use super::{helpers, Object};

#[derive(Debug, Copy)]
#[repr(C)]
pub struct Array {
    pub items: *mut Object,
    pub size: usize,
    pub capacity: usize,
}

impl Array {
    pub fn as_slice(&self) -> &[Object] {
        unsafe { std::slice::from_raw_parts(self.items, self.size) }
    }

    pub fn free(self) {
        unsafe { helpers::api_free_array(self) }
    }
}

impl Default for Array {
    fn default() -> Self {
        Self {
            items: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }
}

impl Clone for Array {
    fn clone(&self) -> Self {
        let mut new_array = Array::default();

        new_array.size = self.size;
        new_array.capacity = self.capacity;

        unsafe {
            self.items.copy_to(new_array.items, self.size);
        }

        new_array
    }
}
