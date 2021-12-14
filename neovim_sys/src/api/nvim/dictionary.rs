//!
//! This module contains functionality for dealing with neovim's Lua `Dictionary` type.
//!

use super::{collection::Collection, Array, Boolean, Float, Integer, NvimString, Object};
use std::{borrow::Borrow, fmt};

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
        NvimString: Borrow<Q>,
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

    /// Convenience method for calling `get()` then forcing to a `Boolean`. Only call this if
    /// you're 100% sure the value is a `Boolean`.
    ///
    pub fn get_as_boolean<Q: ?Sized>(&self, k: &Q) -> Option<Boolean>
    where
        NvimString: Borrow<Q>,
        Q: PartialEq<NvimString>,
    {
        self.get(k).map(Object::as_boolean_unchecked)
    }

    /// Convenience method for calling `get()` then forcing to a `Integer`. Only call this if
    /// you're 100% sure the value is a `Integer`.
    ///
    pub fn get_as_integer<Q: ?Sized>(&self, k: &Q) -> Option<Integer>
    where
        NvimString: Borrow<Q>,
        Q: PartialEq<NvimString>,
    {
        self.get(k).map(Object::as_integer_unchecked)
    }

    /// Convenience method for calling `get()` then forcing to a `Float`. Only call this if
    /// you're 100% sure the value is a `Float`.
    ///
    pub fn get_as_float<Q: ?Sized>(&self, k: &Q) -> Option<Float>
    where
        NvimString: Borrow<Q>,
        Q: PartialEq<NvimString>,
    {
        self.get(k).map(Object::as_float_unchecked)
    }

    /// Convenience method for calling `get()` then forcing to a `NvimString`. Only call this if
    /// you're 100% sure the value is a `NvimString`.
    ///
    pub fn get_as_string<Q: ?Sized>(&self, k: &Q) -> Option<&NvimString>
    where
        NvimString: Borrow<Q>,
        Q: PartialEq<NvimString>,
    {
        self.get(k).map(Object::as_string_unchecked)
    }

    /// Convenience method for calling `get()` then forcing to a `Array`. Only call this if
    /// you're 100% sure the value is a `Array`.
    ///
    pub fn get_as_array<Q: ?Sized>(&self, k: &Q) -> Option<&Array>
    where
        NvimString: Borrow<Q>,
        Q: PartialEq<NvimString>,
    {
        self.get(k).map(Object::as_array_unchecked)
    }

    /// Convenience method for calling `get()` then forcing to a `Dictionary`. Only call this if
    /// you're 100% sure the value is a `Dictionary`.
    ///
    pub fn get_as_dictionary<Q: ?Sized>(&self, k: &Q) -> Option<&Self>
    where
        NvimString: Borrow<Q>,
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
    pub const fn new(key: NvimString, value: Object) -> Self {
        Self { key, value }
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
        let array = Dictionary::new([
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
        let array = Dictionary::new([
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
        let inner1_dictionary = Dictionary::new([
            KeyValuePair::new(NvimString::new_unchecked("inner one one"), Object::from(42)),
            KeyValuePair::new(
                NvimString::new_unchecked("inner one two"),
                Object::from(42.42),
            ),
        ]);

        let inner2_dictionary = Dictionary::new([
            KeyValuePair::new(
                NvimString::new_unchecked("inner two one"),
                Object::from(NvimString::new_unchecked("first one")),
            ),
            KeyValuePair::new(
                NvimString::new_unchecked("inner two two"),
                Object::from(true),
            ),
        ]);

        let dictionary = Dictionary::new([
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
        let original_dict = Dictionary::new([KeyValuePair::new(
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

    #[test]
    fn get_existing_key_test() {
        let original_dict = Dictionary::new([KeyValuePair::new(
            NvimString::new_unchecked("the key"),
            Object::from(NvimString::new_unchecked("the value")),
        )]);

        let value = original_dict.get("the key").unwrap();
        let string = value.as_string_unchecked();
        assert_eq!(string.to_string_lossy(), "the value");
    }
}
