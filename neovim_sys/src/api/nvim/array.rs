//!
//! This module contains functionality for dealing with neovim's Lua `Array` type.
//!

use super::{collection::Collection, Object, ObjectType};
use std::{
    convert::TryFrom,
    mem::{self, ManuallyDrop},
    ptr,
};

///
/// An `Array` is a wrapper for neovim's Lua `Array`, where each element is an `Object`.
///
pub type Array = Collection<Object>;

impl TryFrom<Object> for Array {
    type Error = super::object::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type() {
            ObjectType::kObjectTypeArray => {
                let data = value.data();
                let size = data.array().size;
                let mut dst = ManuallyDrop::new(Vec::with_capacity(size));

                unsafe {
                    ptr::copy(data.array().items, dst.as_mut_ptr(), size);
                    dst.set_len(size);
                }

                let ptr = dst.as_mut_ptr();

                let a = Self {
                    items: ptr,
                    size,
                    capacity: size,
                };
                mem::forget(value);
                Ok(a)
            }
            t => Err(Self::Error::TypeError {
                expected: ObjectType::kObjectTypeArray,
                actual: t,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::nvim::NvimString;
    use approx::assert_ulps_eq;

    #[test]
    fn test_new() {
        let subject = Array::new([]);
        assert_eq!(subject.len(), 0);

        let subject = Array::new([Object::from(4.2)]);
        assert_eq!(subject.len(), 1);

        let subject = Array::new([Object::from(4.2), Object::new_nil()]);
        assert_eq!(subject.len(), 2);
    }

    #[test]
    fn test_vec_from_bool() {
        let array = Array::new([Object::from(true), Object::from(false)]);
        assert_eq!(array.len(), 2);
        assert_eq!(array.capacity(), 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert!(out_vec[0].try_as_boolean().unwrap());
        assert!(!out_vec[1].try_as_boolean().unwrap());
    }

    #[test]
    fn test_from_vec_int() {
        let array = Array::new([
            Object::from(i64::max_value()),
            Object::from(i64::min_value()),
        ]);
        assert_eq!(array.len(), 2);
        assert_eq!(array.capacity(), 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_eq!(out_vec[0].try_as_integer().unwrap(), i64::max_value());
        assert_eq!(out_vec[1].try_as_integer().unwrap(), i64::min_value());
    }

    #[test]
    fn test_from_vec_floats() {
        let array = Array::new([Object::from(f64::MAX), Object::from(f64::MIN)]);
        assert_eq!(array.len(), 2);
        assert_eq!(array.capacity(), 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_ulps_eq!(out_vec[0].try_as_float().unwrap(), f64::MAX);
        assert_ulps_eq!(out_vec[1].try_as_float().unwrap(), f64::MIN);
    }

    #[test]
    fn test_vec_strings() {
        let array = Array::new([
            Object::from(NvimString::new_unchecked("first one")),
            Object::from(NvimString::new_unchecked("second one")),
        ]);
        assert_eq!(array.len(), 2);
        assert_eq!(array.capacity(), 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_eq!(
            out_vec[0].try_as_string().unwrap(),
            &NvimString::new_unchecked("first one")
        );
        assert_eq!(
            out_vec[1].try_as_string().unwrap(),
            &NvimString::new_unchecked("second one")
        );
    }

    #[test]
    fn test_from_vec_of_vecs() {
        let inner1_array = Array::new([Object::from(42), Object::from(42.42)]);

        let inner2_array = Array::new([
            Object::from(NvimString::new_unchecked("first one")),
            Object::from(true),
        ]);

        let array = Array::new([Object::from(inner1_array), Object::from(inner2_array)]);
        assert_eq!(array.len(), 2);
        assert_eq!(array.capacity(), 2);

        let mut out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        {
            let out_vec_inner1: Vec<Object> =
                out_vec.remove(0).try_as_array().unwrap().clone().into();
            assert_eq!(out_vec_inner1.len(), 2);
            assert_eq!(out_vec_inner1.capacity(), 2);
            assert_eq!(out_vec_inner1[0].try_as_integer().unwrap(), 42);
            assert_ulps_eq!(out_vec_inner1[1].try_as_float().unwrap(), 42.42);
        }

        {
            let out_vec_inner2: Vec<Object> =
                out_vec.remove(0).try_as_array().unwrap().clone().into();
            assert_eq!(out_vec_inner2.len(), 2);
            assert_eq!(out_vec_inner2.capacity(), 2);

            assert_eq!(
                out_vec_inner2[0].try_as_string().unwrap(),
                &NvimString::new_unchecked("first one")
            );
            assert!(out_vec_inner2[1].try_as_boolean().unwrap());
        }
    }

    #[test]
    fn test_clone() {
        let original_array = {
            Array::new([
                Object::from(NvimString::new_unchecked("first one")),
                Object::from(NvimString::new_unchecked("second one")),
            ])
        };

        // Clone happens here
        let cloned = original_array.clone();
        assert_eq!(cloned.size, 2);
        assert_eq!(cloned.capacity, 2);

        {
            let mut cloned_vec = Vec::from(cloned);
            assert_eq!(cloned_vec.len(), 2);
            assert_eq!(cloned_vec.capacity(), 2);

            let first_element = cloned_vec.remove(0);

            let actual = first_element.try_as_string().unwrap();
            assert_eq!(actual, &NvimString::new_unchecked("first one"));

            let second_element = cloned_vec.remove(0);
            assert_eq!(
                second_element.try_as_string().unwrap(),
                &NvimString::new_unchecked("second one"),
            );
        }

        // Make sure we can still access the original's values
        {
            let mut original_vec = Vec::from(original_array);
            assert_eq!(original_vec.len(), 2);
            assert_eq!(original_vec.capacity(), 2);

            let first_element = original_vec.remove(0);

            assert_eq!(
                first_element.try_as_string().unwrap(),
                &NvimString::new_unchecked("first one")
            );

            let second_element = original_vec.remove(0);

            assert_eq!(
                second_element.try_as_string().unwrap(),
                &NvimString::new_unchecked("second one")
            );
        }
    }
}
