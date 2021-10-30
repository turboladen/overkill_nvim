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

        let ptr = dst.as_mut_ptr();

        Self {
            items: NonNull::new(ptr).unwrap(),
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
        debug!("Dictionary::try_from(Object)");

        match value.object_type {
            ObjectType::kObjectTypeDictionary => {
                let data = &value.data;
                // Move all the value (Object) data into the new Dictionary.
                // Since we moved the data, don't call drop for the Object.
                let size = unsafe { &data.dictionary }.size;
                let mut dst = ManuallyDrop::new(Vec::with_capacity(size));

                unsafe {
                    std::ptr::copy(data.dictionary.items.as_ref(), dst.as_mut_ptr(), size);
                    dst.set_len(size);
                }

                let ptr = dst.as_mut_ptr();

                let d = Self {
                    items: NonNull::new(ptr).unwrap(),
                    size,
                    capacity: size,
                };
                std::mem::forget(value);
                Ok(d)
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Dictionary, KeyValuePair, Object, TryFrom};
    use crate::api::vim::String as LuaString;
    use approx::assert_ulps_eq;
    use log::debug;
    use std::ffi::CString;

    #[test]
    fn test_from_vec_of_bool_values() {
        let vec = vec![
            KeyValuePair {
                key: LuaString::new("one").unwrap(),
                value: Object::new_boolean(true),
            },
            KeyValuePair {
                key: LuaString::new("two").unwrap(),
                value: Object::new_boolean(false),
            },
        ];

        let array = Dictionary::from(vec);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_eq!(out_vec[0].key, LuaString::new("one").unwrap());
        assert!(out_vec[0].value.try_as_boolean().unwrap());

        assert_eq!(out_vec[1].key, LuaString::new("two").unwrap());
        assert!(!out_vec[1].value.try_as_boolean().unwrap());
    }

    #[test]
    fn test_from_vec_of_string_values() {
        let vec = vec![
            KeyValuePair {
                key: LuaString::new("one").unwrap(),
                value: Object::new_string(LuaString::new("first one").unwrap()),
            },
            KeyValuePair {
                key: CString::new("two").unwrap().into(),
                value: Object::new_string(LuaString::new("second one").unwrap()),
            },
        ];

        let array = Dictionary::from(vec);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_eq!(
            out_vec[0].value.try_as_cloned_string().unwrap(),
            LuaString::new("first one").unwrap()
        );

        assert_eq!(
            out_vec[1].value.try_as_cloned_string().unwrap(),
            LuaString::new("second one").unwrap()
        );
    }

    #[test]
    fn test_from_vec_of_vecs() {
        let inner1_dictionary = {
            let inner1_vec = vec![
                KeyValuePair::new(
                    LuaString::new("inner one one").unwrap(),
                    Object::new_integer(42),
                ),
                KeyValuePair::new(
                    LuaString::new("inner one two").unwrap(),
                    Object::new_float(42.42),
                ),
            ];
            Dictionary::from(inner1_vec)
        };

        let inner2_dictionary = {
            let inner2_vec = vec![
                KeyValuePair::new(
                    LuaString::new("inner two one").unwrap(),
                    Object::new_string(LuaString::new("first one").unwrap()),
                ),
                KeyValuePair::new(
                    LuaString::new("inner two two").unwrap(),
                    Object::new_boolean(true),
                ),
            ];
            Dictionary::from(inner2_vec)
        };

        let vec = vec![
            KeyValuePair::new(
                LuaString::new("outer 1").unwrap(),
                Object::new_dictionary(inner1_dictionary),
            ),
            KeyValuePair::new(
                LuaString::new("outer 2").unwrap(),
                Object::new_dictionary(inner2_dictionary),
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
            assert_eq!(kvp1.key, LuaString::new("outer 1").unwrap());

            // Validate the Dictionary value
            {
                let inner_dict1 = Dictionary::try_from(kvp1.value.clone()).unwrap();
                let mut inner_vec1 = Vec::from(inner_dict1);

                let inner_kvp1 = inner_vec1.remove(0);
                assert_eq!(inner_kvp1.key, LuaString::new("inner one one").unwrap());
                assert_eq!(inner_kvp1.value.try_as_integer().unwrap(), 42);

                let inner_kvp2 = inner_vec1.remove(0);
                assert_eq!(inner_kvp2.key, LuaString::new("inner one two").unwrap());
                assert_ulps_eq!(inner_kvp2.value.try_as_float().unwrap(), 42.42);
            }
        }

        // Validate the Dictionary value
        {
            let kvp2 = out_vec.remove(0);
            assert_eq!(kvp2.key, LuaString::new("outer 2").unwrap());

            // Validate the Dictionary value
            {
                let inner_dict2 = kvp2.value.try_as_cloned_dictionary().unwrap();
                let mut inner_vec2 = Vec::from(inner_dict2);

                let inner_kvp1 = inner_vec2.remove(0);
                assert_eq!(inner_kvp1.key, LuaString::new("inner two one").unwrap());
                assert_eq!(
                    inner_kvp1.value.try_as_cloned_string().unwrap(),
                    LuaString::new("first one").unwrap()
                );

                let inner_kvp2 = inner_vec2.remove(0);
                assert_eq!(inner_kvp2.key, LuaString::new("inner two two").unwrap());
                assert!(inner_kvp2.value.try_as_boolean().unwrap());
            }
        }
    }

    #[test]
    fn test_clone() {
        crate::init_logger();
        let original_dict = {
            let original_vec = vec![KeyValuePair::new(
                LuaString::new("the key").unwrap(),
                Object::new_string(LuaString::new("the value").unwrap()),
            )];
            Dictionary::from(original_vec)
        };

        // Clone happens here
        let cloned = original_dict.clone();
        {
            let mut cloned_vec = Vec::from(cloned);

            let first_element = cloned_vec.remove(0);
            debug!("removed first element from dict vec");
            assert_eq!(first_element.key, LuaString::new("the key").unwrap());
            assert_eq!(
                first_element.value.try_as_cloned_string().unwrap(),
                LuaString::new("the value").unwrap()
            );
        }

        // Make sure we can still access the original's values
        {
            let mut original_vec = Vec::from(original_dict);

            let first_element = original_vec.remove(0);
            assert_eq!(
                first_element.value.try_as_cloned_string().unwrap(),
                LuaString::new("first one").unwrap(),
            );

            let second_element = original_vec.remove(0);
            assert_eq!(
                second_element.value.try_as_cloned_string().unwrap(),
                LuaString::new("second one").unwrap(),
            );
        }
    }
}
