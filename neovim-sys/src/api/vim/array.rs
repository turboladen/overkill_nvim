pub mod into_iter;

use self::into_iter::IntoIter;
use super::{Object, ObjectType};
use std::{
    convert::TryFrom,
    fmt,
    marker::PhantomData,
    mem::{self, ManuallyDrop, MaybeUninit},
    ptr::{self, addr_of_mut},
    slice,
};

#[repr(C)]
pub struct Array {
    items: *mut Object,
    size: usize,
    capacity: usize,
}

impl Array {
    pub fn new<T: Into<Vec<Object>>>(vec: T) -> Self {
        let mut vec: Vec<Object> = vec.into();

        let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();

        // Initializing the `size` field
        // Using `write` instead of assignment via `=` to not call `drop` on the
        // old, uninitialized value.
        unsafe {
            addr_of_mut!((*ptr).size).write(vec.len());
            addr_of_mut!((*ptr).capacity).write(vec.capacity());
        }

        let new_items =  vec.as_mut_ptr() ;

        unsafe {
            // Initializing the `list` field
            // If there is a panic here, then the `String` in the `name` field leaks.
            addr_of_mut!((*ptr).items).write(new_items);
        }

        mem::forget(vec);

        unsafe { uninit.assume_init() }
    }

    #[must_use]
    pub fn as_slice(&self) -> &[Object] {
        unsafe { slice::from_raw_parts(self.items, self.size) }
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
    pub fn iter(&self) -> slice::Iter<'_, Object> {
        self.as_slice().iter()
    }
}

impl IntoIterator for Array {
    type Item = Object;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let me = ManuallyDrop::new(self);
            let alloc = ptr::read(me.items);
            let begin = me.items;
            let end: *const Object = begin.add(me.len());
            let cap = me.capacity();
            IntoIter {
                buf: begin,
                phantom: PhantomData,
                cap,
                alloc,
                ptr: begin,
                end,
            }
        }
    }
}

impl Clone for Array {
    fn clone(&self) -> Self {
        Self::new(self.as_slice())
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        unsafe { Vec::from_raw_parts(self.items, self.size, self.capacity) };
    }
}

// impl From<Vec<Object>> for Array {
//     fn from(vec: Vec<Object>) -> Self {
//         let mut vec = ManuallyDrop::new(vec);
//         vec.shrink_to_fit();

//         Self {
//             items: NonNull::new(vec.as_mut_ptr()).unwrap(),
//             size: vec.len(),
//             capacity: vec.len(),
//         }
//     }
// }

impl From<Array> for Vec<Object> {
    fn from(array: Array) -> Self {
        let v = unsafe { Self::from_raw_parts(array.items, array.size, array.capacity) };
        std::mem::forget(array);

        v
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
        match value.object_type() {
            ObjectType::kObjectTypeArray => {
                let data = value.data();
                let size = unsafe { &data.array }.size;
                let mut dst = ManuallyDrop::new(Vec::with_capacity(size));

                unsafe {
                    ptr::copy(data.array.items, dst.as_mut_ptr(), size);
                    dst.set_len(size);
                }

                let ptr = dst.as_mut_ptr();
                if ptr.is_null() {
                    return Err(());
                }

                let a = Self {
                    items: ptr,
                    size,
                    capacity: size,
                };
                mem::forget(value);
                Ok(a)
            }
            _ => Err(()),
        }
    }
}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::vim::LuaString;
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
            Object::from(LuaString::new("first one").unwrap()),
            Object::from(LuaString::new("second one").unwrap()),
        ]);
        assert_eq!(array.len(), 2);
        assert_eq!(array.capacity(), 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_eq!(
            out_vec[0].try_as_string().unwrap(),
            &LuaString::new("first one").unwrap()
        );
        assert_eq!(
            out_vec[1].try_as_string().unwrap(),
            &LuaString::new("second one").unwrap()
        );
    }

    #[test]
    fn test_from_vec_of_vecs() {
        let inner1_array = Array::new([Object::from(42), Object::from(42.42)]);

        let inner2_array = Array::new([
            Object::from(LuaString::new("first one").unwrap()),
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
                &LuaString::new("first one").unwrap()
            );
            assert!(out_vec_inner2[1].try_as_boolean().unwrap());
        }
    }

    #[test]
    fn test_clone() {
        let original_array = {
            Array::new([
                Object::from(LuaString::new("first one").unwrap()),
                Object::from(LuaString::new("second one").unwrap()),
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
            assert_eq!(actual, &LuaString::new("first one").unwrap());

            let second_element = cloned_vec.remove(0);
            assert_eq!(
                second_element.try_as_string().unwrap(),
                &LuaString::new("second one").unwrap(),
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
                &LuaString::new("first one").unwrap()
            );

            let second_element = original_vec.remove(0);

            assert_eq!(
                second_element.try_as_string().unwrap(),
                &LuaString::new("second one").unwrap()
            );
        }
    }

    #[test]
    fn test_into_iter() {
        let array = Array::new([
            Object::from(true),
            Object::from(42),
            Object::from(LuaString::new("blah").unwrap()),
        ]);

        let mut iter = array.into_iter();
        let boolean = iter.next().unwrap();
        assert!(boolean.try_as_boolean().unwrap());

        let integer = iter.next().unwrap();
        assert_eq!(integer.try_as_integer().unwrap(), 42);

        let string = iter.next().unwrap();
        assert_eq!(
            string.try_as_string().unwrap(),
            &LuaString::new("blah").unwrap()
        );
    }
}
