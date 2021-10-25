use super::{helpers, Object, ObjectType};
use std::{
    convert::TryFrom,
    mem::{ManuallyDrop, MaybeUninit},
};

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

impl From<&Array> for &[Object] {
    fn from(array: &Array) -> Self {
        unsafe { std::slice::from_raw_parts(array.items, array.size) }
    }
}

impl TryFrom<Object> for Array {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type {
            ObjectType::kObjectTypeArray => Ok(unsafe { value.data.array }),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::vim::{ObjectData, ObjectType};
    use approx::assert_ulps_eq;
    use std::ffi::CString;

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

    #[test]
    fn test_from_vec_int() {
        let vec = vec![
            Object::new(
                ObjectType::kObjectTypeInteger,
                ObjectData {
                    integer: i64::max_value(),
                },
            ),
            Object::new(
                ObjectType::kObjectTypeInteger,
                ObjectData {
                    integer: i64::min_value(),
                },
            ),
        ];

        let array = Array::from(vec);
        assert!(!array.items.is_null());
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);
        assert_eq!(out_vec[0].object_type, ObjectType::kObjectTypeInteger);
        assert_eq!(unsafe { out_vec[0].data.integer }, i64::max_value());
        assert_eq!(out_vec[1].object_type, ObjectType::kObjectTypeInteger);
        assert_eq!(unsafe { out_vec[1].data.integer }, i64::min_value());
    }

    #[test]
    fn test_from_vec_floats() {
        let vec = vec![
            Object::new(
                ObjectType::kObjectTypeFloat,
                ObjectData { floating: f64::MAX },
            ),
            Object::new(
                ObjectType::kObjectTypeFloat,
                ObjectData { floating: f64::MAX },
            ),
        ];

        let array = Array::from(vec);
        assert!(!array.items.is_null());
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);
        assert_eq!(out_vec[0].object_type, ObjectType::kObjectTypeFloat);
        assert_ulps_eq!(unsafe { out_vec[0].data.floating }, f64::MAX);
        assert_eq!(out_vec[1].object_type, ObjectType::kObjectTypeFloat);
        assert_ulps_eq!(unsafe { out_vec[1].data.floating }, f64::MAX);
    }

    #[test]
    fn test_from_vec_strings() {
        let vec = vec![
            Object::new(
                ObjectType::kObjectTypeString,
                ObjectData {
                    string: CString::new("first one").unwrap().into(),
                },
            ),
            Object::new(
                ObjectType::kObjectTypeString,
                ObjectData {
                    string: CString::new("second one").unwrap().into(),
                },
            ),
        ];

        let array = Array::from(vec);
        assert!(!array.items.is_null());
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);
        assert_eq!(out_vec[0].object_type, ObjectType::kObjectTypeString);
        assert_eq!(
            CString::from(unsafe { out_vec[0].data.string }),
            CString::new("first one").unwrap()
        );
        assert_eq!(out_vec[1].object_type, ObjectType::kObjectTypeString);
        assert_eq!(
            CString::from(unsafe { out_vec[1].data.string }),
            CString::new("second one").unwrap()
        );
    }

    #[test]
    fn test_from_vec_of_vecs() {
        let inner1_vec = vec![
            Object::new(ObjectType::kObjectTypeInteger, ObjectData { integer: 42 }),
            Object::new(ObjectType::kObjectTypeFloat, ObjectData { floating: 42.42 }),
        ];
        let inner1_array = Array::from(inner1_vec);

        let inner2_vec = vec![
            Object::new(
                ObjectType::kObjectTypeString,
                ObjectData {
                    string: CString::new("first one").unwrap().into(),
                },
            ),
            Object::new(ObjectType::kObjectTypeBoolean, ObjectData { boolean: true }),
        ];
        let inner2_array = Array::from(inner2_vec);

        let vec = vec![
            Object::new(
                ObjectType::kObjectTypeArray,
                ObjectData {
                    array: inner1_array,
                },
            ),
            Object::new(
                ObjectType::kObjectTypeArray,
                ObjectData {
                    array: inner2_array,
                },
            ),
        ];

        let array = Array::from(vec);
        assert!(!array.items.is_null());
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let mut out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        {
            let out_vec_inner1: Vec<Object> = Array::try_from(out_vec.remove(0)).unwrap().into();
            assert_eq!(out_vec_inner1.len(), 2);
            assert_eq!(out_vec_inner1.capacity(), 2);
            assert_eq!(
                out_vec_inner1[0].object_type,
                ObjectType::kObjectTypeInteger
            );
            assert_eq!(unsafe { out_vec_inner1[0].data.integer }, 42);
            assert_eq!(out_vec_inner1[1].object_type, ObjectType::kObjectTypeFloat);
            assert_ulps_eq!(unsafe { out_vec_inner1[1].data.floating }, 42.42);
        }

        {
            let out_vec_inner2: Vec<Object> = Array::try_from(out_vec.remove(0)).unwrap().into();
            assert_eq!(out_vec_inner2.len(), 2);
            assert_eq!(out_vec_inner2.capacity(), 2);

            assert_eq!(out_vec_inner2[0].object_type, ObjectType::kObjectTypeString);
            assert_eq!(
                CString::from(unsafe { out_vec_inner2[0].data.string }),
                CString::new("first one").unwrap()
            );
            assert_eq!(
                out_vec_inner2[1].object_type,
                ObjectType::kObjectTypeBoolean
            );
            assert!(unsafe { out_vec_inner2[1].data.boolean });
        }
    }
}
