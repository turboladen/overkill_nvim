use super::{Object, ObjectType};
use log::debug;
use std::{convert::TryFrom, mem::ManuallyDrop, ptr::NonNull};

#[derive(Debug)]
#[repr(C)]
pub struct Array {
    pub items: NonNull<Object>,
    pub size: usize,
    pub capacity: usize,
}

impl Array {
    pub fn as_slice(&self) -> &[Object] {
        unsafe { std::slice::from_raw_parts(self.items.as_ref(), self.size) }
    }
}

impl Clone for Array {
    fn clone(&self) -> Self {
        let mut dst = ManuallyDrop::new(Vec::with_capacity(self.size));

        unsafe {
            std::ptr::copy(self.items.as_ref(), dst.as_mut_ptr(), self.size);
            dst.set_len(self.size);
        }

        dst.shrink_to_fit();

        Self {
            items: NonNull::new(dst.as_mut_ptr()).unwrap(),
            size: self.size,
            capacity: self.size,
        }
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        debug!("Droppping Array...");
        unsafe { Vec::from_raw_parts(self.items.as_mut(), self.size, self.capacity) };
    }
}

impl From<Vec<Object>> for Array {
    fn from(vec: Vec<Object>) -> Self {
        let mut vec = ManuallyDrop::new(vec);
        vec.shrink_to_fit();

        Self {
            items: NonNull::new(vec.as_mut_ptr()).unwrap(),
            size: vec.len(),
            capacity: vec.len(),
        }
    }
}

impl From<Array> for Vec<Object> {
    fn from(array: Array) -> Self {
        debug!("Vec::from(Array)");
        let mut vec = Vec::with_capacity(array.capacity);

        for object in array.as_slice() {
            vec.push(object.clone());
        }
        vec
    }
}

impl<'a> From<&'a Array> for &'a [Object] {
    fn from(array: &'a Array) -> Self {
        array.as_slice()
    }
}

impl TryFrom<Object> for Array {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type {
            ObjectType::kObjectTypeArray => {
                Ok(unsafe { ManuallyDrop::into_inner(value.data.array) })
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::vim::{ObjectData, ObjectType, String as LuaString};
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
                    string: ManuallyDrop::new(LuaString::from(CString::new("first one").unwrap())),
                },
            ),
            Object::new(
                ObjectType::kObjectTypeString,
                ObjectData {
                    string: ManuallyDrop::new(LuaString::from(CString::new("second one").unwrap())),
                },
            ),
        ];

        let array = Array::from(vec);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);
        assert_eq!(out_vec[0].object_type, ObjectType::kObjectTypeString);
        assert_eq!(
            CString::from(ManuallyDrop::into_inner(unsafe {
                out_vec[0].data.string.clone()
            })),
            CString::new("first one").unwrap()
        );
        assert_eq!(out_vec[1].object_type, ObjectType::kObjectTypeString);
        assert_eq!(
            CString::from(ManuallyDrop::into_inner(unsafe {
                out_vec[1].data.string.clone()
            })),
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
                    string: ManuallyDrop::new(LuaString::from(CString::new("first one").unwrap())),
                },
            ),
            Object::new(ObjectType::kObjectTypeBoolean, ObjectData { boolean: true }),
        ];
        let inner2_array = Array::from(inner2_vec);

        let vec = vec![
            Object::new(
                ObjectType::kObjectTypeArray,
                ObjectData {
                    array: ManuallyDrop::new(inner1_array),
                },
            ),
            Object::new(
                ObjectType::kObjectTypeArray,
                ObjectData {
                    array: ManuallyDrop::new(inner2_array),
                },
            ),
        ];

        let array = Array::from(vec);
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
                CString::from(ManuallyDrop::into_inner(unsafe {
                    out_vec_inner2[0].data.string.clone()
                })),
                CString::new("first one").unwrap()
            );
            assert_eq!(
                out_vec_inner2[1].object_type,
                ObjectType::kObjectTypeBoolean
            );
            assert!(unsafe { out_vec_inner2[1].data.boolean });
        }
    }

    #[test]
    fn test_clone() {
        let original_array = {
            let original_vec = vec![
                Object::new(
                    ObjectType::kObjectTypeString,
                    ObjectData {
                        string: ManuallyDrop::new(LuaString::from(
                            CString::new("first one").unwrap(),
                        )),
                    },
                ),
                Object::new(
                    ObjectType::kObjectTypeString,
                    ObjectData {
                        string: ManuallyDrop::new(LuaString::from(
                            CString::new("second one").unwrap(),
                        )),
                    },
                ),
            ];
            Array::from(original_vec)
        };

        // Clone happens here
        let cloned = original_array.clone();
        {
            let mut cloned_vec = Vec::from(cloned);

            let first_element = cloned_vec.remove(0);
            assert_eq!(
                CString::new("first one").unwrap(),
                CString::from(ManuallyDrop::into_inner(unsafe {
                    first_element.data.string
                })),
            );

            let second_element = cloned_vec.remove(0);
            assert_eq!(
                CString::new("second one").unwrap(),
                CString::from(ManuallyDrop::into_inner(unsafe {
                    second_element.data.string
                })),
            );
        }

        // Make sure we can still access the original's values

        {
            let mut original_vec = Vec::from(original_array);

            let first_element = original_vec.remove(0);
            assert_eq!(
                CString::new("first one").unwrap(),
                CString::from(ManuallyDrop::into_inner(unsafe {
                    first_element.data.string
                })),
            );

            let second_element = original_vec.remove(0);
            assert_eq!(
                CString::new("second one").unwrap(),
                CString::from(ManuallyDrop::into_inner(unsafe { second_element.data.string })),
            );
        }
    }
}
