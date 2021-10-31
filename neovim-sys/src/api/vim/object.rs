use super::{Array, Boolean, Dictionary, Float, Integer, LuaRef, String as LuaString};
use std::{convert::TryFrom, fmt::Debug, mem::ManuallyDrop};

#[repr(C)]
pub struct Object {
    object_type: ObjectType,
    data: ObjectData,
}

macro_rules! new_copy_type {
    ($type_variant:ident, $field_name:ident) => {
        Self {
            object_type: ObjectType::$type_variant,
            data: ObjectData { $field_name },
        }
    };
}

macro_rules! new_clone_type {
    ($type_variant:ident, $field_name:ident) => {
        Self {
            object_type: ObjectType::$type_variant,
            data: ObjectData {
                $field_name: ManuallyDrop::new($field_name),
            },
        }
    };
}

macro_rules! try_as_type {
    ($_self:expr, $object_type_variant:ident, $field_name:ident) => {
        match $_self.object_type {
            ObjectType::$object_type_variant => {
                let data = &$_self.data;
                Ok(unsafe { data.$field_name })
            }
            _ => Err(()),
        }
    };
}

macro_rules! try_as_ref_type {
    ($_self:expr, $object_type_variant:ident, $field_name:ident) => {
        match $_self.object_type {
            ObjectType::$object_type_variant => {
                let data = &$_self.data;
                Ok(unsafe { &data.$field_name })
            }
            _ => Err(()),
        }
    };
}

impl Object {
    pub fn new(object_type: ObjectType, data: ObjectData) -> Self {
        Self { object_type, data }
    }

    pub fn new_nil() -> Self {
        Self {
            object_type: ObjectType::kObjectTypeNil,
            data: ObjectData { boolean: false },
        }
    }

    pub fn object_type(&self) -> ObjectType {
        self.object_type
    }

    /// Get a reference to the object's data.
    pub fn data(&self) -> &ObjectData {
        &self.data
    }

    pub fn try_as_nil(&self) -> Result<(), ()> {
        match self.object_type {
            ObjectType::kObjectTypeNil => {
                let data = &self.data;

                if !unsafe { data.boolean } {
                    Ok(())
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }

    pub fn try_as_boolean(&self) -> Result<bool, ()> {
        try_as_type!(self, kObjectTypeBoolean, boolean)
    }

    pub fn try_as_integer(&self) -> Result<Integer, ()> {
        try_as_type!(self, kObjectTypeInteger, integer)
    }

    pub fn try_as_float(&self) -> Result<Float, ()> {
        try_as_type!(self, kObjectTypeFloat, floating)
    }

    pub fn try_as_string(&self) -> Result<&LuaString, ()> {
        try_as_ref_type!(self, kObjectTypeString, string)
    }

    pub fn try_as_array(&self) -> Result<&Array, ()> {
        try_as_ref_type!(self, kObjectTypeArray, array)
    }

    pub fn try_as_dictionary(&self) -> Result<&Dictionary, ()> {
        try_as_ref_type!(self, kObjectTypeDictionary, dictionary)
    }

    pub fn as_boolean_unchecked(&self) -> Boolean {
        unsafe { self.data.boolean }
    }

    pub fn as_integer_unchecked(&self) -> Integer {
        unsafe { self.data.integer }
    }

    pub fn as_float_unchecked(&self) -> Float {
        unsafe { self.data.floating }
    }

    pub fn as_string_unchecked(&self) -> &LuaString {
        unsafe { &self.data.string }
    }

    pub fn as_array_unchecked(&self) -> &Array {
        unsafe { &self.data.array }
    }

    pub fn as_dictionary_unchecked(&self) -> &Dictionary {
        unsafe { &self.data.dictionary }
    }

    pub fn into_boolean_unchecked(self) -> Boolean {
        unsafe { self.data.boolean }
    }

    pub fn into_integer_unchecked(self) -> Integer {
        unsafe { self.data.integer }
    }

    pub fn into_float_unchecked(self) -> Float {
        unsafe { self.data.floating }
    }

    pub fn into_string_unchecked(self) -> LuaString {
        unsafe { ManuallyDrop::into_inner(self.data.string) }
    }

    pub fn into_array_unchecked(self) -> Array {
        unsafe { ManuallyDrop::into_inner(self.data.array) }
    }

    pub fn into_dictionary_unchecked(self) -> Dictionary {
        unsafe { ManuallyDrop::into_inner(self.data.dictionary) }
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::new_nil()
    }
}

impl From<Boolean> for Object {
    fn from(boolean: Boolean) -> Self {
        new_copy_type!(kObjectTypeBoolean, boolean)
    }
}

impl From<Integer> for Object {
    fn from(integer: Integer) -> Self {
        new_copy_type!(kObjectTypeInteger, integer)
    }
}

impl From<Float> for Object {
    fn from(floating: Float) -> Self {
        new_copy_type!(kObjectTypeFloat, floating)
    }
}

impl From<LuaString> for Object {
    fn from(string: LuaString) -> Self {
        new_clone_type!(kObjectTypeString, string)
    }
}

impl From<Array> for Object {
    fn from(array: Array) -> Self {
        new_clone_type!(kObjectTypeArray, array)
    }
}

impl From<Dictionary> for Object {
    fn from(dictionary: Dictionary) -> Self {
        new_clone_type!(kObjectTypeDictionary, dictionary)
    }
}

macro_rules! copy_inner_for_clone {
    ($_self:expr, $field_name:ident) => {{
        let data = &$_self.data;
        let value = unsafe { &data.$field_name };

        Self {
            object_type: $_self.object_type,
            data: ObjectData {
                $field_name: *value,
            },
        }
    }};
}

macro_rules! clone_inner_for_clone {
    ($_self:expr, $field_name:ident) => {{
        let data = &$_self.data;
        let value = unsafe { &data.$field_name };

        Self {
            object_type: $_self.object_type,
            data: ObjectData {
                $field_name: value.clone(),
            },
        }
    }};
}

impl Clone for Object {
    fn clone(&self) -> Self {
        match self.object_type {
            ObjectType::kObjectTypeNil => Self::new_nil(),
            ObjectType::kObjectTypeBoolean => copy_inner_for_clone!(self, boolean),
            ObjectType::kObjectTypeInteger => copy_inner_for_clone!(self, integer),
            ObjectType::kObjectTypeFloat => copy_inner_for_clone!(self, floating),
            ObjectType::kObjectTypeString => clone_inner_for_clone!(self, string),
            ObjectType::kObjectTypeArray => clone_inner_for_clone!(self, array),
            ObjectType::kObjectTypeDictionary => clone_inner_for_clone!(self, dictionary),
        }
    }
}

// impl Drop for Object {
//     fn drop(&mut self) {
//         debug!("Dropping Object...{:?}", self.object_type);

//         match self.object_type {
//             ObjectType::kObjectTypeNil
//             | ObjectType::kObjectTypeBoolean
//             | ObjectType::kObjectTypeInteger
//             | ObjectType::kObjectTypeFloat => (),
//             ObjectType::kObjectTypeString => {
//                 let data = &mut self.data;

//                 debug!(
//                     "...Object::String...'{}'",
//                     unsafe { &data.string }.as_c_str().to_string_lossy()
//                 );
//                 unsafe { ManuallyDrop::drop(&mut data.string) };
//             }
//             ObjectType::kObjectTypeArray => {
//                 debug!("...Object::Array...");
//                 let data = &mut self.data;
//                 unsafe { ManuallyDrop::drop(&mut data.array) };
//             }
//             ObjectType::kObjectTypeDictionary => {
//                 debug!("...Object::Dictionary...");
//                 let data = &mut self.data;
//                 unsafe { ManuallyDrop::drop(&mut data.dictionary) };
//             }
//         }
//     }
// }

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Object");
        d.field("object_type", &self.object_type);

        match self.object_type {
            ObjectType::kObjectTypeNil => d.field("data", &"nil"),
            ObjectType::kObjectTypeBoolean => {
                let data = &self.data;
                d.field("data", unsafe { &data.boolean })
            }
            ObjectType::kObjectTypeInteger => {
                let data = &self.data;
                d.field("data", unsafe { &data.integer })
            }
            ObjectType::kObjectTypeFloat => {
                let data = &self.data;
                d.field("data", unsafe { &data.floating })
            }
            ObjectType::kObjectTypeString => {
                let data = &self.data;
                d.field("data", unsafe { &data.string })
            }
            ObjectType::kObjectTypeArray => {
                let data = &self.data;
                d.field("data", unsafe { &data.array })
            }
            ObjectType::kObjectTypeDictionary => {
                let data = &self.data;
                d.field("data", unsafe { &data.dictionary })
            }
        };

        d.finish()
    }
}

impl TryFrom<Object> for Boolean {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type {
            ObjectType::kObjectTypeBoolean => {
                let data = &value.data;
                Ok(unsafe { data.boolean })
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<Object> for Integer {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type {
            ObjectType::kObjectTypeInteger => {
                let data = &value.data;
                Ok(unsafe { data.integer })
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<Object> for Float {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type {
            ObjectType::kObjectTypeFloat => {
                let data = &value.data;
                Ok(unsafe { data.floating })
            }
            _ => Err(()),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        if self.object_type != other.object_type {
            return false;
        }

        match self.object_type {
            ObjectType::kObjectTypeNil => true,
            ObjectType::kObjectTypeBoolean => unsafe { self.data.boolean == other.data.boolean },
            ObjectType::kObjectTypeInteger => unsafe { self.data.integer == other.data.integer },
            ObjectType::kObjectTypeFloat => unsafe { self.data.floating == other.data.floating },
            ObjectType::kObjectTypeString => {
                let lhs = &self.data;
                let rhs = &self.data;
                unsafe { lhs.string.eq(&rhs.string) }
            }
            ObjectType::kObjectTypeArray => {
                let lhs = &self.data;
                let rhs = &self.data;
                unsafe { lhs.array.eq(&rhs.array) }
            }
            ObjectType::kObjectTypeDictionary => {
                let _lhs = &self.data;
                let _rhs = &self.data;
                todo!()
                // unsafe { lhs.dictionary.eq(&rhs.dictionary) }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[repr(usize)]
pub enum ObjectType {
    kObjectTypeNil = 0,
    kObjectTypeBoolean,
    kObjectTypeInteger,
    kObjectTypeFloat,
    kObjectTypeString,
    kObjectTypeArray,
    kObjectTypeDictionary,
    // kObjectTypeLuaRef,
    // EXT types, cannot be split or reordered, see #EXT_OBJECT_TYPE_SHIFT
    // kObjectTypeBuffer,
    // kObjectTypeWindow,
    // kObjectTypeTabpage,
}

#[repr(C)]
pub union ObjectData {
    pub boolean: Boolean,
    pub integer: Integer,
    pub floating: Float,
    pub string: ManuallyDrop<LuaString>,
    pub array: ManuallyDrop<Array>,
    pub dictionary: ManuallyDrop<Dictionary>,
    pub luaref: LuaRef,
}

#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;

    use super::*;

    #[test]
    fn test_from_boolean() {
        fn do_it(input: Boolean) {
            let subject = Object::from(input);
            assert_eq!(subject.object_type, ObjectType::kObjectTypeBoolean);
            assert_eq!(subject.as_boolean_unchecked(), input);
        }

        do_it(true);
        do_it(false);
    }

    #[test]
    fn test_from_integer() {
        fn do_it(input: Integer) {
            let subject = Object::from(input);
            assert_eq!(subject.object_type, ObjectType::kObjectTypeInteger);
            assert_eq!(subject.as_integer_unchecked(), input);
        }

        do_it(0);
        do_it(i64::min_value());
        do_it(i64::max_value());
    }

    #[test]
    fn test_from_float() {
        fn do_it(input: Float) {
            let subject = Object::from(input);
            assert_eq!(subject.object_type, ObjectType::kObjectTypeFloat);
            assert_ulps_eq!(subject.as_float_unchecked(), input);
        }

        do_it(0.0);
        do_it(f64::INFINITY);
        do_it(f64::NEG_INFINITY);
        do_it(f64::MIN);
        do_it(f64::MAX);

        let subject = Object::from(f64::NAN);
        assert_eq!(subject.object_type, ObjectType::kObjectTypeFloat);
        assert!(subject.as_float_unchecked().is_nan());
    }

    #[test]
    fn test_from_string() {
        fn do_it(input: &str) {
            let subject = Object::from(LuaString::new(input).unwrap());

            assert_eq!(subject.object_type, ObjectType::kObjectTypeString);
            assert_eq!(
                subject.as_string_unchecked(),
                &LuaString::new(input).unwrap()
            );
        }

        do_it("");
        do_it("one \n three");
        do_it("this is an emoji: 🌮. Tacos are cool.");
    }

    #[test]
    fn test_from_array() {
        fn do_it(input: &str) {
            let subject = Object::from(Array::new([Object::from(LuaString::new(input).unwrap())]));
            let expected = Array::new([Object::from(LuaString::new(input).unwrap())]);

            assert_eq!(subject.object_type, ObjectType::kObjectTypeArray);
            assert_eq!(subject.as_array_unchecked(), &expected);
        }

        do_it("");
        do_it("one \n three");
        do_it("this is an emoji: 🌮. Tacos are cool.");
    }
}
