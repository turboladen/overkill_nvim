use super::{helpers, Object};
use std::mem::{ManuallyDrop, MaybeUninit};

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
        let mut new_items = MaybeUninit::uninit();

        new_items.write(self.items);

        Array {
            size: self.size,
            capacity: self.capacity,
            items: unsafe { new_items.assume_init() },
        }
    }
}

impl From<Vec<Object>> for Array {
    fn from(vec: Vec<Object>) -> Self {
        let mut vec = ManuallyDrop::new(vec);
        vec.shrink_to_fit();

        Self {
            items: vec.as_mut_ptr(),
            size: vec.len(),
            capacity: vec.len(),
        }
    }
}

impl From<Array> for Vec<Object> {
    fn from(array: Array) -> Self {
        unsafe { Vec::from_raw_parts(array.items, array.size, array.capacity) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::vim::{ObjectData, ObjectType};

    #[test]
    fn test_from_vec_bool() {
        let vec = vec![
            Object::new(ObjectType::kObjectTypeBoolean, ObjectData { boolean: true }),
            Object::new(
                ObjectType::kObjectTypeBoolean,
                ObjectData { boolean: false },
            ),
        ];

        let array = Array::from(vec);
        assert!(!array.items.is_null());
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);
        assert_eq!(out_vec[0].object_type, ObjectType::kObjectTypeBoolean);
        assert!(unsafe { out_vec[0].data.boolean });
        assert_eq!(out_vec[1].object_type, ObjectType::kObjectTypeBoolean);
        assert!(unsafe { !out_vec[1].data.boolean });
    }
}
