use super::{KeyValuePair, LuaString, Object, ObjectType};
use std::{
    borrow::Borrow,
    convert::TryFrom,
    fmt,
    mem::{self, ManuallyDrop, MaybeUninit},
    ptr::{addr_of_mut, NonNull},
    slice,
};

#[repr(C)]
pub struct Dictionary {
    items: NonNull<KeyValuePair>,
    size: usize,
    capacity: usize,
}

impl Dictionary {
    pub fn new<T: Into<Vec<KeyValuePair>>>(vec: T) -> Self {
        let mut vec: Vec<KeyValuePair> = vec.into();

        let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();

        // Initializing the `size` field
        // Using `write` instead of assignment via `=` to not call `drop` on the
        // old, uninitialized value.
        unsafe {
            addr_of_mut!((*ptr).size).write(vec.len());
            addr_of_mut!((*ptr).capacity).write(vec.capacity());
        }

        let new_items = unsafe { NonNull::new_unchecked(vec.as_mut_ptr()) };

        unsafe {
            // Initializing the `list` field
            // If there is a panic here, then the `String` in the `name` field leaks.
            addr_of_mut!((*ptr).items).write(new_items);
        }

        mem::forget(vec);

        unsafe { uninit.assume_init() }
    }

    #[must_use]
    pub fn as_slice(&self) -> &[KeyValuePair] {
        unsafe { std::slice::from_raw_parts(&*self.items.as_ref(), self.size) }
    }

    /// Get a reference to the array's size.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.size
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a reference to the array's capacity.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    #[must_use]
    pub fn iter(&self) -> slice::Iter<'_, KeyValuePair> {
        self.as_slice().iter()
    }

    #[inline]
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&Object>
    where
        LuaString: Borrow<Q>,
        Q: PartialEq<LuaString>,
    {
        self.iter().find_map(|kv| {
            if k == kv.key() {
                Some(kv.value())
            } else {
                None
            }
        })
    }
}

impl Clone for Dictionary {
    fn clone(&self) -> Self {
        Self::new(self.as_slice())
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe { Vec::from_raw_parts(self.items.as_mut(), self.size, self.capacity) };
    }
}

impl fmt::Debug for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

// impl From<Vec<KeyValuePair>> for Dictionary {
//     fn from(vec: Vec<KeyValuePair>) -> Self {
//         let mut vec = ManuallyDrop::new(vec);
//         vec.shrink_to_fit();

//         Self {
//             items: NonNull::new(vec.as_mut_ptr()).unwrap(),
//             size: vec.len(),
//             capacity: vec.len(),
//         }
//     }
// }

impl TryFrom<Object> for Dictionary {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type() {
            ObjectType::kObjectTypeDictionary => {
                let data = value.data();
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

impl From<Dictionary> for Vec<KeyValuePair> {
    fn from(dictionary: Dictionary) -> Self {
        let v = unsafe {
            Self::from_raw_parts(
                dictionary.items.as_ptr(),
                dictionary.size,
                dictionary.capacity,
            )
        };
        std::mem::forget(dictionary);

        v
    }
}

impl<'a> From<&'a Dictionary> for &'a [KeyValuePair] {
    fn from(dict: &'a Dictionary) -> Self {
        dict.as_slice()
    }
}

impl PartialEq for Dictionary {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::{Dictionary, KeyValuePair, Object, TryFrom};
    use crate::api::vim::LuaString;
    use approx::assert_ulps_eq;
    use log::debug;

    #[test]
    fn test_from_vec_of_bool_values() {
        let array = Dictionary::new([
            KeyValuePair::new(LuaString::new("one").unwrap(), Object::from(true)),
            KeyValuePair::new(LuaString::new("two").unwrap(), Object::from(false)),
        ]);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_eq!(out_vec[0].key(), &LuaString::new("one").unwrap());
        assert!(out_vec[0].value().try_as_boolean().unwrap());

        assert_eq!(out_vec[1].key(), &LuaString::new("two").unwrap());
        assert!(!out_vec[1].value().try_as_boolean().unwrap());
    }

    #[test]
    fn test_from_vec_of_string_values() {
        let array = Dictionary::new([
            KeyValuePair::new(
                LuaString::new("one").unwrap(),
                Object::from(LuaString::new("first one").unwrap()),
            ),
            KeyValuePair::new(
                LuaString::new("two").unwrap(),
                Object::from(LuaString::new("second one").unwrap()),
            ),
        ]);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_eq!(
            out_vec[0].value().try_as_string().unwrap(),
            &LuaString::new("first one").unwrap()
        );

        assert_eq!(
            out_vec[1].value().try_as_string().unwrap(),
            &LuaString::new("second one").unwrap()
        );
    }

    #[test]
    fn test_from_vec_of_vecs() {
        let inner1_dictionary = Dictionary::new([
            KeyValuePair::new(LuaString::new("inner one one").unwrap(), Object::from(42)),
            KeyValuePair::new(
                LuaString::new("inner one two").unwrap(),
                Object::from(42.42),
            ),
        ]);

        let inner2_dictionary = Dictionary::new([
            KeyValuePair::new(
                LuaString::new("inner two one").unwrap(),
                Object::from(LuaString::new("first one").unwrap()),
            ),
            KeyValuePair::new(LuaString::new("inner two two").unwrap(), Object::from(true)),
        ]);

        let dictionary = Dictionary::new([
            KeyValuePair::new(
                LuaString::new("outer 1").unwrap(),
                Object::from(inner1_dictionary),
            ),
            KeyValuePair::new(
                LuaString::new("outer 2").unwrap(),
                Object::from(inner2_dictionary),
            ),
        ]);
        assert_eq!(dictionary.size, 2);
        assert_eq!(dictionary.capacity, 2);

        let mut out_vec = Vec::from(dictionary);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        // Validate the Dictionary value
        {
            let kvp1 = out_vec.remove(0);
            assert_eq!(kvp1.key(), &LuaString::new("outer 1").unwrap());

            // Validate the Dictionary value
            {
                let inner_dict1 = Dictionary::try_from(kvp1.value().clone()).unwrap();
                let mut inner_vec1 = Vec::from(inner_dict1);

                let inner_kvp1 = inner_vec1.remove(0);
                assert_eq!(inner_kvp1.key(), &LuaString::new("inner one one").unwrap());
                assert_eq!(inner_kvp1.value().try_as_integer().unwrap(), 42);

                let inner_kvp2 = inner_vec1.remove(0);
                assert_eq!(inner_kvp2.key(), &LuaString::new("inner one two").unwrap());
                assert_ulps_eq!(inner_kvp2.value().try_as_float().unwrap(), 42.42);
            }
        }

        // Validate the Dictionary value
        {
            let kvp2 = out_vec.remove(0);
            assert_eq!(kvp2.key(), &LuaString::new("outer 2").unwrap());

            // Validate the Dictionary value
            {
                let inner_dict2 = kvp2.value().try_as_dictionary().unwrap();
                let mut inner_vec2 = Vec::from(inner_dict2.clone());

                let inner_kvp1 = inner_vec2.remove(0);
                assert_eq!(inner_kvp1.key(), &LuaString::new("inner two one").unwrap());
                assert_eq!(
                    inner_kvp1.value().try_as_string().unwrap(),
                    &LuaString::new("first one").unwrap()
                );

                let inner_kvp2 = inner_vec2.remove(0);
                assert_eq!(inner_kvp2.key(), &LuaString::new("inner two two").unwrap());
                assert!(inner_kvp2.value().try_as_boolean().unwrap());
            }
        }
    }

    #[test]
    fn test_clone() {
        let original_dict = Dictionary::new([KeyValuePair::new(
            LuaString::new("the key").unwrap(),
            Object::from(LuaString::new("the value").unwrap()),
        )]);

        // Clone happens here
        let cloned = original_dict.clone();
        {
            let mut cloned_vec = Vec::from(cloned);

            let first_element = cloned_vec.remove(0);
            debug!("removed first element from dict vec");
            assert_eq!(first_element.key(), &LuaString::new("the key").unwrap());
            assert_eq!(
                first_element.value().try_as_string().unwrap(),
                &LuaString::new("the value").unwrap()
            );
        }

        // Make sure we can still access the original's values
        {
            let mut original_vec = Vec::from(original_dict);

            let first_element = original_vec.remove(0);
            assert_eq!(first_element.key(), &LuaString::new("the key").unwrap(),);
            assert_eq!(
                first_element.value().try_as_string().unwrap(),
                &LuaString::new("the value").unwrap(),
            );
        }
    }
}
