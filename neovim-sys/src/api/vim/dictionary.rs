//!
//! This module contains functionality for dealing with neovim's Lua `Dictionary` type.
//!

use super::{collection::Collection, LuaString, Object};
use std::borrow::Borrow;
use std::fmt;

/// Wrapper for neovim's `Dictionary` type.
///
pub type Dictionary = Collection<KeyValuePair>;

impl Dictionary {
    /// Similar to Rust's `HashMap`/`BTreeMap::get()`, this tries to get the value that's related
    /// to `k`.
    ///
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

/// Elements of a `Dictionary`.
///
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct KeyValuePair {
    key: LuaString,
    value: Object,
}

impl KeyValuePair {
    /// Basic constructor.
    ///
    #[must_use]
    pub const fn new(key: LuaString, value: Object) -> Self {
        Self { key, value }
    }

    /// A reference to the key.
    ///
    #[must_use]
    pub const fn key(&self) -> &LuaString {
        &self.key
    }

    /// A reference to the value.
    ///
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

#[cfg(test)]
mod tests {
    use super::{Dictionary, KeyValuePair, Object};
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
                let inner_dict1 = kvp1.value().as_dictionary_unchecked();
                let mut inner_vec1 = Vec::from(inner_dict1.clone());

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

    #[test]
    fn get_existing_key_test() {
        let original_dict = Dictionary::new([KeyValuePair::new(
            LuaString::new("the key").unwrap(),
            Object::from(LuaString::new("the value").unwrap()),
        )]);

        let value = original_dict.get("the key").unwrap();
        let string = value.as_string_unchecked();
        assert_eq!(string.to_string_lossy(), "the value");
    }
}
