//!
//! This module contains functionality for dealing with neovim's Lua `Dictionary` type.
//!

use super::{collection::Collection, Array, Boolean, Float, Integer, NvimString, Object};
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
        Q: PartialEq<NvimString>,
    {
        self.iter().find_map(|kv| {
            if k == kv.key() {
                Some(kv.value())
            } else {
                None
            }
        })
    }

    /// Similar to Rust's `HashMap`/`BTreeMap::insert()`, this sets the key/value pair. If the key
    /// already had a value, this is removed and returned. If the key didn't have a value, this returns
    /// `None`.
    ///
    #[inline]
    pub fn set<V>(&mut self, k: NvimString, v: V) -> Option<Object>
    where
        Object: From<V>,
    {
        if let Some(kv) = self.iter_mut().find(|kv| &k == kv.key()) {
            let old = kv.value().clone();
            kv.set_value(v);
            return Some(old);
        }

        self.push(KeyValuePair::new(k, v));

        None
    }

    /// Convenience method for calling `get()` then forcing to a `Boolean`. Only call this if
    /// you're 100% sure the value is a `Boolean`.
    ///
    pub fn get_as_boolean<Q: ?Sized>(&self, k: &Q) -> Option<Boolean>
    where
        Q: PartialEq<NvimString>,
    {
        self.get(k).map(Object::as_boolean_unchecked)
    }

    /// Convenience method for calling `get()` then forcing to a `Integer`. Only call this if
    /// you're 100% sure the value is a `Integer`.
    ///
    pub fn get_as_integer<Q: ?Sized>(&self, k: &Q) -> Option<Integer>
    where
        Q: PartialEq<NvimString>,
    {
        self.get(k).map(Object::as_integer_unchecked)
    }

    /// Convenience method for calling `get()` then forcing to a `Float`. Only call this if
    /// you're 100% sure the value is a `Float`.
    ///
    pub fn get_as_float<Q: ?Sized>(&self, k: &Q) -> Option<Float>
    where
        Q: PartialEq<NvimString>,
    {
        self.get(k).map(Object::as_float_unchecked)
    }

    /// Convenience method for calling `get()` then forcing to a `NvimString`. Only call this if
    /// you're 100% sure the value is a `NvimString`.
    ///
    pub fn get_as_string<Q: ?Sized>(&self, k: &Q) -> Option<&NvimString>
    where
        Q: PartialEq<NvimString>,
    {
        self.get(k).map(Object::as_string_unchecked)
    }

    /// Convenience method for calling `get()` then forcing to a `Array`. Only call this if
    /// you're 100% sure the value is a `Array`.
    ///
    pub fn get_as_array<Q: ?Sized>(&self, k: &Q) -> Option<&Array>
    where
        Q: PartialEq<NvimString>,
    {
        self.get(k).map(Object::as_array_unchecked)
    }

    /// Convenience method for calling `get()` then forcing to a `Dictionary`. Only call this if
    /// you're 100% sure the value is a `Dictionary`.
    ///
    pub fn get_as_dictionary<Q: ?Sized>(&self, k: &Q) -> Option<&Self>
    where
        Q: PartialEq<NvimString>,
    {
        self.get(k).map(Object::as_dictionary_unchecked)
    }
}

/// Elements of a `Dictionary`.
///
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct KeyValuePair {
    key: NvimString,
    value: Object,
}

impl KeyValuePair {
    /// Basic constructor.
    ///
    #[must_use]
    pub fn new<V>(key: NvimString, value: V) -> Self
    where
        Object: From<V>,
    {
        Self {
            key,
            value: Object::from(value),
        }
    }

    /// A reference to the key.
    ///
    #[inline]
    #[must_use]
    pub const fn key(&self) -> &NvimString {
        &self.key
    }

    /// A reference to the value.
    ///
    #[inline]
    #[must_use]
    pub const fn value(&self) -> &Object {
        &self.value
    }

    /// Sets the value.
    ///
    #[inline]
    pub fn set_value<V>(&mut self, value: V)
    where
        Object: From<V>,
    {
        self.value = Object::from(value);
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
    use crate::api::nvim::NvimString;
    use approx::assert_ulps_eq;
    use log::debug;

    #[test]
    fn test_from_vec_of_bool_values() {
        let array = Dictionary::new_from([
            KeyValuePair::new(NvimString::new_unchecked("one"), Object::from(true)),
            KeyValuePair::new(NvimString::new_unchecked("two"), Object::from(false)),
        ]);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_eq!(out_vec[0].key(), &NvimString::new_unchecked("one"));
        assert!(out_vec[0].value().try_as_boolean().unwrap());

        assert_eq!(out_vec[1].key(), &NvimString::new_unchecked("two"));
        assert!(!out_vec[1].value().try_as_boolean().unwrap());
    }

    #[test]
    fn test_from_vec_of_string_values() {
        let array = Dictionary::new_from([
            KeyValuePair::new(
                NvimString::new_unchecked("one"),
                Object::from(NvimString::new_unchecked("first one")),
            ),
            KeyValuePair::new(
                NvimString::new_unchecked("two"),
                Object::from(NvimString::new_unchecked("second one")),
            ),
        ]);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_eq!(
            out_vec[0].value().try_as_string().unwrap(),
            &NvimString::new_unchecked("first one")
        );

        assert_eq!(
            out_vec[1].value().try_as_string().unwrap(),
            &NvimString::new_unchecked("second one")
        );
    }

    #[test]
    fn test_from_vec_of_vecs() {
        let inner1_dictionary = Dictionary::new_from([
            KeyValuePair::new(NvimString::new_unchecked("inner one one"), Object::from(42)),
            KeyValuePair::new(
                NvimString::new_unchecked("inner one two"),
                Object::from(42.42),
            ),
        ]);

        let inner2_dictionary = Dictionary::new_from([
            KeyValuePair::new(
                NvimString::new_unchecked("inner two one"),
                Object::from(NvimString::new_unchecked("first one")),
            ),
            KeyValuePair::new(
                NvimString::new_unchecked("inner two two"),
                Object::from(true),
            ),
        ]);

        let dictionary = Dictionary::new_from([
            KeyValuePair::new(
                NvimString::new_unchecked("outer 1"),
                Object::from(inner1_dictionary),
            ),
            KeyValuePair::new(
                NvimString::new_unchecked("outer 2"),
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
            assert_eq!(kvp1.key(), &NvimString::new_unchecked("outer 1"));

            // Validate the Dictionary value
            {
                let inner_dict1 = kvp1.value().as_dictionary_unchecked();
                let mut inner_vec1 = Vec::from(inner_dict1.clone());

                let inner_kvp1 = inner_vec1.remove(0);
                assert_eq!(
                    inner_kvp1.key(),
                    &NvimString::new_unchecked("inner one one")
                );
                assert_eq!(inner_kvp1.value().try_as_integer().unwrap(), 42);

                let inner_kvp2 = inner_vec1.remove(0);
                assert_eq!(
                    inner_kvp2.key(),
                    &NvimString::new_unchecked("inner one two")
                );
                assert_ulps_eq!(inner_kvp2.value().try_as_float().unwrap(), 42.42);
            }
        }

        // Validate the Dictionary value
        {
            let kvp2 = out_vec.remove(0);
            assert_eq!(kvp2.key(), &NvimString::new_unchecked("outer 2"));

            // Validate the Dictionary value
            {
                let inner_dict2 = kvp2.value().try_as_dictionary().unwrap();
                let mut inner_vec2 = Vec::from(inner_dict2.clone());

                let inner_kvp1 = inner_vec2.remove(0);
                assert_eq!(
                    inner_kvp1.key(),
                    &NvimString::new_unchecked("inner two one")
                );
                assert_eq!(
                    inner_kvp1.value().try_as_string().unwrap(),
                    &NvimString::new_unchecked("first one")
                );

                let inner_kvp2 = inner_vec2.remove(0);
                assert_eq!(
                    inner_kvp2.key(),
                    &NvimString::new_unchecked("inner two two")
                );
                assert!(inner_kvp2.value().try_as_boolean().unwrap());
            }
        }
    }

    #[test]
    fn test_clone() {
        let original_dict = Dictionary::new_from([KeyValuePair::new(
            NvimString::new_unchecked("the key"),
            Object::from(NvimString::new_unchecked("the value")),
        )]);

        // Clone happens here
        let cloned = original_dict.clone();
        {
            let mut cloned_vec = Vec::from(cloned);

            let first_element = cloned_vec.remove(0);
            debug!("removed first element from dict vec");
            assert_eq!(first_element.key(), &NvimString::new_unchecked("the key"));
            assert_eq!(
                first_element.value().try_as_string().unwrap(),
                &NvimString::new_unchecked("the value")
            );
        }

        // Make sure we can still access the original's values
        {
            let mut original_vec = Vec::from(original_dict);

            let first_element = original_vec.remove(0);
            assert_eq!(first_element.key(), &NvimString::new_unchecked("the key"));
            assert_eq!(
                first_element.value().try_as_string().unwrap(),
                &NvimString::new_unchecked("the value"),
            );
        }
    }

    mod get_set {
        use super::*;

        #[test]
        fn test_get_existing_key() {
            let original_dict = Dictionary::new_from([KeyValuePair::new(
                NvimString::new_unchecked("the key"),
                Object::from(NvimString::new_unchecked("the value")),
            )]);

            let value = original_dict.get("the key").unwrap();
            let string = value.as_string_unchecked();
            assert_eq!(string.to_string_lossy(), "the value");
        }

        #[test]
        fn test_get_missing_key() {
            let original_dict = Dictionary::default();
            assert!(original_dict.get("the key").is_none());
        }

        #[test]
        fn test_set() {
            let mut original_dict = Dictionary::default();
            original_dict.set(NvimString::new_unchecked("the key"), 42.42);
            assert_eq!(original_dict.get("the key").unwrap(), &Object::from(42.42));
        }
    }
}
