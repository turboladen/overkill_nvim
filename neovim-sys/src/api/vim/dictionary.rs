use super::{KeyValuePair, Object, ObjectType};
use log::debug;
use std::{convert::TryFrom, mem::ManuallyDrop, ptr::NonNull};

#[derive(Debug)]
#[repr(C)]
pub struct Dictionary {
    pub items: NonNull<KeyValuePair>,
    pub size: usize,
    pub capacity: usize,
}

impl Dictionary {
    pub fn as_slice(&self) -> &[KeyValuePair] {
        unsafe { std::slice::from_raw_parts(self.items.as_ref(), self.size) }
    }
}

impl Clone for Dictionary {
    fn clone(&self) -> Self {
        debug!("Cloning Dictionary...");
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

impl Drop for Dictionary {
    fn drop(&mut self) {
        debug!("Droppping Dictionary...");
        unsafe { Vec::from_raw_parts(self.items.as_mut(), self.size, self.capacity) };
    }
}

impl From<Vec<KeyValuePair>> for Dictionary {
    fn from(vec: Vec<KeyValuePair>) -> Self {
        let mut vec = ManuallyDrop::new(vec);
        vec.shrink_to_fit();

        Self {
            items: NonNull::new(vec.as_mut_ptr()).unwrap(),
            size: vec.len(),
            capacity: vec.len(),
        }
    }
}

impl TryFrom<Object> for Dictionary {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type {
            ObjectType::kObjectTypeDictionary => {
                Ok(unsafe { ManuallyDrop::into_inner(value.data.dictionary) })
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Dictionary, KeyValuePair, ManuallyDrop, Object, TryFrom};
    use crate::api::vim::{Boolean, Float, Integer, ObjectData, ObjectType, String as LuaString};
    use approx::assert_ulps_eq;
    use std::ffi::CString;

    #[test]
    fn test_from_vec_of_bool_values() {
        let vec = vec![
            KeyValuePair {
                key: CString::new("one").unwrap().into(),
                value: Object::new(ObjectType::kObjectTypeBoolean, ObjectData { boolean: true }),
            },
            KeyValuePair {
                key: CString::new("two").unwrap().into(),
                value: Object::new(
                    ObjectType::kObjectTypeBoolean,
                    ObjectData { boolean: false },
                ),
            },
        ];

        let array = Dictionary::from(vec);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_eq!(
            out_vec[0].key.as_cstr(),
            CString::new("one").unwrap().as_c_str()
        );
        assert_eq!(out_vec[0].value.object_type, ObjectType::kObjectTypeBoolean);
        assert!(unsafe { out_vec[0].value.data.boolean });

        assert_eq!(
            out_vec[1].key.as_cstr(),
            CString::new("two").unwrap().as_c_str()
        );
        assert_eq!(out_vec[1].value.object_type, ObjectType::kObjectTypeBoolean);
        assert!(unsafe { !out_vec[1].value.data.boolean });
    }

    #[test]
    fn test_from_vec_of_string_values() {
        let vec = vec![
            KeyValuePair {
                key: CString::new("one").unwrap().into(),
                value: Object::new(
                    ObjectType::kObjectTypeString,
                    ObjectData {
                        string: ManuallyDrop::new(LuaString::from(
                            CString::new("first one").unwrap(),
                        )),
                    },
                ),
            },
            KeyValuePair {
                key: CString::new("two").unwrap().into(),
                value: Object::new(
                    ObjectType::kObjectTypeString,
                    ObjectData {
                        string: ManuallyDrop::new(LuaString::from(
                            CString::new("second one").unwrap(),
                        )),
                    },
                ),
            },
        ];

        let array = Dictionary::from(vec);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_eq!(out_vec[0].value.object_type, ObjectType::kObjectTypeString);
        assert_eq!(
            CString::from(ManuallyDrop::into_inner(unsafe {
                out_vec[0].value.data.string.clone()
            })),
            CString::new("first one").unwrap()
        );

        assert_eq!(out_vec[1].value.object_type, ObjectType::kObjectTypeString);
        assert_eq!(
            CString::from(ManuallyDrop::into_inner(unsafe {
                out_vec[1].value.data.string.clone()
            })),
            CString::new("second one").unwrap()
        );
    }

    #[test]
    fn test_from_vec_of_vecs() {
        let inner1_dictionary = {
            let inner1_vec = vec![
                KeyValuePair::new(
                    CString::new("inner one one").unwrap().into(),
                    Object::new(ObjectType::kObjectTypeInteger, ObjectData { integer: 42 }),
                ),
                KeyValuePair::new(
                    CString::new("inner one two").unwrap().into(),
                    Object::new(ObjectType::kObjectTypeFloat, ObjectData { floating: 42.42 }),
                ),
            ];
            Dictionary::from(inner1_vec)
        };

        let inner2_dictionary = {
            let inner2_vec = vec![
                KeyValuePair::new(
                    CString::new("inner two one").unwrap().into(),
                    Object::new(
                        ObjectType::kObjectTypeString,
                        ObjectData {
                            string: ManuallyDrop::new(LuaString::from(
                                CString::new("first one").unwrap(),
                            )),
                        },
                    ),
                ),
                KeyValuePair::new(
                    CString::new("inner two two").unwrap().into(),
                    Object::new(ObjectType::kObjectTypeBoolean, ObjectData { boolean: true }),
                ),
            ];
            Dictionary::from(inner2_vec)
        };

        let vec = vec![
            KeyValuePair::new(
                CString::new("outer 1").unwrap().into(),
                Object::new(
                    ObjectType::kObjectTypeDictionary,
                    ObjectData {
                        dictionary: ManuallyDrop::new(inner1_dictionary),
                    },
                ),
            ),
            KeyValuePair::new(
                CString::new("outer 2").unwrap().into(),
                Object::new(
                    ObjectType::kObjectTypeDictionary,
                    ObjectData {
                        dictionary: ManuallyDrop::new(inner2_dictionary),
                    },
                ),
            ),
        ];

        let dictionary = Dictionary::from(vec);
        assert_eq!(dictionary.size, 2);
        assert_eq!(dictionary.capacity, 2);

        let mut out_vec = Vec::from(dictionary);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        // Validate the Dictionary value
        {
            let kvp1 = out_vec.remove(0);
            assert_eq!(
                kvp1.key.as_cstr(),
                CString::new("outer 1").unwrap().as_c_str()
            );

            // Validate the Dictionary value
            {
                let inner_dict1 = Dictionary::try_from(kvp1.value).unwrap();
                let mut inner_vec1 = Vec::from(inner_dict1);

                let inner_kvp1 = inner_vec1.remove(0);
                assert_eq!(
                    inner_kvp1.key.as_cstr(),
                    CString::new("inner one one").unwrap().as_c_str()
                );
                assert_eq!(Integer::try_from(inner_kvp1.value).unwrap(), 42);

                let inner_kvp2 = inner_vec1.remove(0);
                assert_eq!(
                    inner_kvp2.key.as_cstr(),
                    CString::new("inner one two").unwrap().as_c_str()
                );
                assert_ulps_eq!(Float::try_from(inner_kvp2.value).unwrap(), 42.42);
            }
        }

        // Validate the Dictionary value
        {
            let kvp2 = out_vec.remove(0);
            assert_eq!(
                kvp2.key.as_cstr(),
                CString::new("outer 2").unwrap().as_c_str()
            );

            // Validate the Dictionary value
            {
                let inner_dict2 = Dictionary::try_from(kvp2.value).unwrap();
                let mut inner_vec2 = Vec::from(inner_dict2);

                let inner_kvp1 = inner_vec2.remove(0);
                assert_eq!(
                    inner_kvp1.key.as_cstr(),
                    CString::new("inner two one").unwrap().as_c_str()
                );
                assert_eq!(
                    // CString::from(LuaString::try_from(inner_kvp1.value).unwrap()).as_c_str(),
                    CString::from(ManuallyDrop::into_inner(unsafe { inner_kvp1.value.data.string }))
                        .as_c_str(),
                    CString::new("first one").unwrap().as_c_str()
                );

                let inner_kvp2 = inner_vec2.remove(0);
                assert_eq!(
                    inner_kvp2.key.as_cstr(),
                    CString::new("inner two two").unwrap().as_c_str()
                );
                assert!(Boolean::try_from(inner_kvp2.value).unwrap());
            }
        }
    }

    #[test]
    fn test_clone() {
        crate::init_logger();
        let original_dict = {
            let original_vec = vec![KeyValuePair::new(
                CString::new("the key").unwrap().into(),
                Object::new(
                    ObjectType::kObjectTypeString,
                    ObjectData {
                        string: ManuallyDrop::new(LuaString::from(
                            CString::new("the value").unwrap(),
                        )),
                    },
                ),
            )];
            Dictionary::from(original_vec)
        };

        // Clone happens here
        let cloned = original_dict.clone();
        {
            let mut cloned_vec = Vec::from(cloned);

            let first_element = cloned_vec.remove(0);
            debug!("removed first element from dict vec");
            assert_eq!(
                CString::from(first_element.key).as_c_str(),
                CString::new("the key").unwrap().as_c_str()
            );
            // assert_eq!(
            //     CString::from(LuaString::try_from(first_element.value).unwrap()).as_c_str(),
            //     CString::new("the value").unwrap().as_c_str()
            // );
        }

        // Make sure we can still access the original's values
        // {
        //     let mut original_vec = Vec::from(original_array);

        //     let first_element = original_vec.remove(0);
        //     assert_eq!(
        //         CString::new("first one").unwrap(),
        //         CString::from(ManuallyDrop::into_inner(unsafe {
        //             first_element.data.string
        //         })),
        //     );

        //     let second_element = original_vec.remove(0);
        //     assert_eq!(
        //         CString::new("second one").unwrap(),
        //         CString::from(ManuallyDrop::into_inner(unsafe {
        //             second_element.data.string
        //         })),
        //     );
        // }
    }
}
