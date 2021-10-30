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
        debug!("Clone Array...");
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
        debug!("Vec<Object>::from(Array)");
        let v = unsafe { Vec::from_raw_parts(array.items.as_ptr(), array.size, array.capacity) };
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
        debug!("Array::try_from(Object)");

        match value.object_type {
            ObjectType::kObjectTypeArray => {
                let data = &value.data;
                let size = unsafe { &data.array }.size;
                let mut dst = ManuallyDrop::new(Vec::with_capacity(size));

                unsafe {
                    std::ptr::copy(data.array.items.as_ref(), dst.as_mut_ptr(), size);
                    dst.set_len(size);
                }

                let ptr = dst.as_mut_ptr();

                let a = Self {
                    items: NonNull::new(ptr).unwrap(),
                    size,
                    capacity: size,
                };
                std::mem::forget(value);
                Ok(a)
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::vim::String as LuaString;
    use approx::assert_ulps_eq;

    #[test]
    fn test_from_vec_bool() {
        let vec = vec![Object::new_boolean(true), Object::new_boolean(false)];

        let array = Array::from(vec);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert!(out_vec[0].try_as_boolean().unwrap());
        assert!(!out_vec[1].try_as_boolean().unwrap());
    }

    #[test]
    fn test_from_vec_int() {
        let vec = vec![
            Object::new_integer(i64::max_value()),
            Object::new_integer(i64::min_value()),
        ];

        let array = Array::from(vec);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_eq!(out_vec[0].try_as_integer().unwrap(), i64::max_value());
        assert_eq!(out_vec[1].try_as_integer().unwrap(), i64::min_value());
    }

    #[test]
    fn test_from_vec_floats() {
        let vec = vec![Object::new_float(f64::MAX), Object::new_float(f64::MIN)];

        let array = Array::from(vec);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_ulps_eq!(out_vec[0].try_as_float().unwrap(), f64::MAX);
        assert_ulps_eq!(out_vec[1].try_as_float().unwrap(), f64::MIN);
    }

    #[test]
    fn test_from_vec_strings() {
        let vec = vec![
            Object::new_string(LuaString::new("first one").unwrap()),
            Object::new_string(LuaString::new("second one").unwrap()),
        ];

        let array = Array::from(vec);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        let out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        assert_eq!(
            out_vec[0].try_as_cloned_string().unwrap(),
            LuaString::new("first one").unwrap()
        );
        assert_eq!(
            out_vec[1].try_as_cloned_string().unwrap(),
            LuaString::new("second one").unwrap()
        );
    }

    #[test]
    fn test_from_vec_of_vecs() {
        let inner1_vec = vec![Object::new_integer(42), Object::new_float(42.42)];
        debug!("TEST Array::from(inner1_vec)...");
        let inner1_array = Array::from(inner1_vec);

        let inner2_vec = vec![
            Object::new_string(LuaString::new("first one").unwrap()),
            Object::new_boolean(true),
        ];
        debug!("TEST Array::from(inner2_vec)...");
        let inner2_array = Array::from(inner2_vec);

        let vec = vec![
            Object::new_array(inner1_array),
            Object::new_array(inner2_array),
        ];

        debug!("TEST Array::from(vec)...");
        let array = Array::from(vec);
        assert_eq!(array.size, 2);
        assert_eq!(array.capacity, 2);

        debug!("TEST Vec::from(array)...");
        let mut out_vec = Vec::from(array);
        assert_eq!(out_vec.len(), 2);
        assert_eq!(out_vec.capacity(), 2);

        {
            let out_vec_inner1: Vec<Object> =
                out_vec.remove(0).try_as_cloned_array().unwrap().into();
            assert_eq!(out_vec_inner1.len(), 2);
            assert_eq!(out_vec_inner1.capacity(), 2);
            assert_eq!(out_vec_inner1[0].try_as_integer().unwrap(), 42);
            assert_ulps_eq!(out_vec_inner1[1].try_as_float().unwrap(), 42.42);
        }

        {
            // let out_vec_inner2: Vec<Object> = Array::try_from(out_vec.remove(0)).unwrap().into();
            let out_vec_inner2: Vec<Object> =
                out_vec.remove(0).try_as_cloned_array().unwrap().into();
            assert_eq!(out_vec_inner2.len(), 2);
            assert_eq!(out_vec_inner2.capacity(), 2);

            assert_eq!(
                out_vec_inner2[0].try_as_cloned_string().unwrap(),
                LuaString::new("first one").unwrap()
            );
            assert!(out_vec_inner2[1].try_as_boolean().unwrap());
        }
    }

    #[test]
    fn test_clone() {
        let original_array = {
            let original_vec = vec![
                Object::new_string(LuaString::new("first one").unwrap()),
                Object::new_string(LuaString::new("second one").unwrap()),
            ];
            debug!("TEST Array::from(Vec<Object>)...");
            Array::from(original_vec)
        };

        // Clone happens here
        debug!("TEST Array::clone()...");
        let cloned = original_array.clone();
        assert_eq!(cloned.size, 2);
        assert_eq!(cloned.capacity, 2);
        debug!("TEST------------------------------------");

        {
            debug!("TEST Vec<Object>::from(cloned Array)...");
            let mut cloned_vec = Vec::from(cloned);
            assert_eq!(cloned_vec.len(), 2);
            assert_eq!(cloned_vec.capacity(), 2);

            let first_element = cloned_vec.remove(0);

            debug!("TEST LuaString::try_from(first_element)...");
            let actual = first_element.try_as_cloned_string().unwrap();
            assert_eq!(LuaString::new("first one").unwrap(), actual);

            let second_element = cloned_vec.remove(0);
            debug!("TEST LuaString::try_from(second_element)...");
            assert_eq!(
                LuaString::new("second one").unwrap(),
                second_element.try_as_cloned_string().unwrap(),
            );
        }
        debug!("TEST------------------------------------");

        // Make sure we can still access the original's values
        {
            debug!("TEST Vec<Object>::from(original Array)...");
            let mut original_vec = Vec::from(original_array);
            assert_eq!(original_vec.len(), 2);
            assert_eq!(original_vec.capacity(), 2);

            let first_element = original_vec.remove(0);

            debug!("TEST LuaString::try_from(first_element)...");
            let actual = first_element.try_as_cloned_string().unwrap();
            assert_eq!(LuaString::new("first one").unwrap(), actual);

            let second_element = original_vec.remove(0);

            debug!("TEST LuaString::try_from(second)...");
            // let actual = LuaString::try_from(second_element).unwrap();
            let actual = second_element.try_as_cloned_string().unwrap();
            assert_eq!(LuaString::new("second one").unwrap(), actual);
        }
    }
}
