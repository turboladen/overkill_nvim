//!
//! This module contains types and functions for working with neovim Lua `Object`s.
//!
use super::{Array, Boolean, Dictionary, Float, Integer, LuaRef, NvimString};
use std::{borrow::Cow, convert::TryFrom, fmt::Debug, mem::ManuallyDrop, num::NonZeroI64};

/// An error that can only happen when dealing wit `Object`s.
///
#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum Error {
    /// Captures errors where an `Object`'s internal `data` doesn't match its `object_type`. This
    /// really shouldn't happen.
    ///
    #[error("Unexpected value as Object")]
    Value,

    /// Captures cases where one `ObjectType` was expected on an `Object`, but another was found.
    ///
    #[error("Object expected to be '{expected:?}', but was '{actual:?}'")]
    TypeError {
        /// The expected `ObjectType`.
        ///
        expected: ObjectType,

        /// The actual `ObjectType`.
        ///
        actual: ObjectType,
    },

    /// Error that occurs when converting integer types fails.
    ///
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
}

/// Wrapper for neovim's Lua `Object`, which can be a:
///
/// - `nil`
/// - `boolean` (same as Rust's `bool`)
/// - `integer` (same as Rust's `i64`)
/// - `float` (same as Rust's `f64`)
/// - `string` (similar to Rust's `CString`; wrapped by this crate's `NvimString`)
/// - `array` (wrapped by this crate's `Array`)
/// - `dictionary` (wrapped by this crate's `Dictionary`)
///
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
            ObjectType::$object_type_variant => Ok($_self.data.$field_name()),
            _ => Err(Error::TypeError {
                expected: ObjectType::$object_type_variant,
                actual: $_self.object_type,
            }),
        }
    };
}

macro_rules! try_as_ref_type {
    ($_self:expr, $object_type_variant:ident, $field_name:ident) => {
        match $_self.object_type {
            ObjectType::$object_type_variant => Ok($_self.data.$field_name()),
            _ => Err(Error::TypeError {
                expected: ObjectType::$object_type_variant,
                actual: $_self.object_type,
            }),
        }
    };
}

impl Object {
    /// Convenience constructor for a `nil` `Object`.
    ///
    #[must_use]
    #[inline]
    pub fn new_nil() -> Self {
        Self {
            object_type: ObjectType::kObjectTypeNil,
            data: ObjectData { boolean: false },
        }
    }

    /// Accessor to the internal `ObjectType`.
    ///
    #[must_use]
    #[inline]
    pub const fn object_type(&self) -> ObjectType {
        self.object_type
    }

    /// Get a reference to the object's data.
    #[must_use]
    pub(crate) const fn data(&self) -> &ObjectData {
        &self.data
    }

    /// Tries to extract a `()`.
    ///
    /// # Errors
    ///
    /// If the wrapped type is not nil.
    ///
    pub fn try_as_nil(&self) -> Result<(), Error> {
        match self.object_type {
            ObjectType::kObjectTypeNil => {
                // Nils have the data union set to 0.
                if self.data.integer() == 0 {
                    Ok(())
                } else {
                    Err(Error::Value)
                }
            }
            _ => Err(Error::TypeError {
                expected: ObjectType::kObjectTypeNil,
                actual: self.object_type,
            }),
        }
    }

    /// Tries to extract a reference to the inner `Boolean`.
    ///
    /// # Errors
    ///
    /// If the wrapped type is not a `Boolean`.
    ///
    pub fn try_as_boolean(&self) -> Result<bool, Error> {
        try_as_type!(self, kObjectTypeBoolean, boolean)
    }

    /// Tries to extract a reference to the inner `Integer`.
    ///
    /// # Errors
    ///
    /// If the wrapped type is not a `Integer`.
    ///
    pub fn try_as_integer(&self) -> Result<Integer, Error> {
        try_as_type!(self, kObjectTypeInteger, integer)
    }

    /// Tries to extract a reference to the inner `Float`.
    ///
    /// # Errors
    ///
    /// If the wrapped type is not a `Float`.
    ///
    pub fn try_as_float(&self) -> Result<Float, Error> {
        try_as_type!(self, kObjectTypeFloat, float)
    }

    /// Tries to extract a reference to the inner `NvimString`.
    ///
    /// # Errors
    ///
    /// If the wrapped type is not a `NvimString`.
    ///
    pub fn try_as_string(&self) -> Result<&NvimString, Error> {
        try_as_ref_type!(self, kObjectTypeString, string)
    }

    /// Tries to extract a reference to the inner `Array`.
    ///
    /// # Errors
    ///
    /// If the wrapped type is not a `Array`.
    ///
    pub fn try_as_array(&self) -> Result<&Array, Error> {
        try_as_ref_type!(self, kObjectTypeArray, array)
    }

    /// Tries to extract a reference to the inner `Dictionary`.
    ///
    /// # Errors
    ///
    /// If the wrapped type is not a `Dictionary`.
    ///
    pub fn try_as_dictionary(&self) -> Result<&Dictionary, Error> {
        try_as_ref_type!(self, kObjectTypeDictionary, dictionary)
    }

    /// Counterpart to `try_as_boolean()`, but does not check `self`'s `object_type`, thus calling
    /// this if `self`'s internal data represents another type will give unexpected results.
    ///
    #[must_use]
    #[inline]
    pub fn as_boolean_unchecked(&self) -> Boolean {
        self.data.boolean()
    }

    /// Counterpart to `try_as_integer()`, but does not check `self`'s `object_type`, thus calling
    /// this if `self`'s internal data represents another type will give unexpected results.
    ///
    #[must_use]
    #[inline]
    pub fn as_integer_unchecked(&self) -> Integer {
        self.data.integer()
    }

    /// Counterpart to `try_as_float()`, but does not check `self`'s `object_type`, thus calling
    /// this if `self`'s internal data represents another type will give unexpected results.
    ///
    #[must_use]
    #[inline]
    pub fn as_float_unchecked(&self) -> Float {
        self.data.float()
    }

    /// Counterpart to `try_as_string()`, but does not check `self`'s `object_type`, thus calling
    /// this if `self`'s internal data represents another type will give unexpected results.
    ///
    #[must_use]
    #[inline]
    pub fn as_string_unchecked(&self) -> &NvimString {
        self.data.string()
    }

    /// Counterpart to `try_as_array()`, but does not check `self`'s `object_type`, thus calling
    /// this if `self`'s internal data represents another type will give unexpected results.
    ///
    #[must_use]
    #[inline]
    pub fn as_array_unchecked(&self) -> &Array {
        self.data.array()
    }

    /// Counterpart to `try_as_dictionary()`, but does not check `self`'s `object_type`, thus calling
    /// this if `self`'s internal data represents another type will give unexpected results.
    ///
    #[must_use]
    #[inline]
    pub fn as_dictionary_unchecked(&self) -> &Dictionary {
        self.data.dictionary()
    }

    /// Similar to `as_boolean_unchecked()`, where it does not check `self`'s `object_type` (thus
    /// calling this if `self`'s internal data represents another type will give unexpected
    /// results), but instead of taking a reference to `self`, this consumes `self` and returns the
    /// value as a `Boolean`.
    ///
    /// This is useful for when you only care about the inner type/value of the `Object`.
    ///
    #[inline]
    #[must_use]
    pub fn into_boolean_unchecked(self) -> Boolean {
        self.data.boolean()
    }

    /// Similar to `as_integer_unchecked()`, where it does not check `self`'s `object_type` (thus
    /// calling this if `self`'s internal data represents another type will give unexpected
    /// results), but instead of taking a reference to `self`, this consumes `self` and returns the
    /// value as a `Integer`.
    ///
    /// This is useful for when you only care about the inner type/value of the `Object`.
    ///
    #[inline]
    #[must_use]
    pub fn into_integer_unchecked(self) -> Integer {
        self.data.integer()
    }

    /// Similar to `as_float_unchecked()`, where it does not check `self`'s `object_type` (thus
    /// calling this if `self`'s internal data represents another type will give unexpected
    /// results), but instead of taking a reference to `self`, this consumes `self` and returns the
    /// value as a `Float`.
    ///
    /// This is useful for when you only care about the inner type/value of the `Object`.
    ///
    #[inline]
    #[must_use]
    pub fn into_float_unchecked(self) -> Float {
        self.data.float()
    }

    /// Similar to `as_string_unchecked()`, where it does not check `self`'s `object_type` (thus
    /// calling this if `self`'s internal data represents another type will give unexpected
    /// results), but instead of taking a reference to `self`, this consumes `self` and returns the
    /// value as a `NvimString`.
    ///
    /// This is useful for when you only care about the inner type/value of the `Object`.
    ///
    #[must_use]
    pub fn into_string_unchecked(self) -> NvimString {
        let s = NvimString {
            data: unsafe { self.data.string.data },
            size: unsafe { self.data.string.size },
        };
        std::mem::forget(self);
        s
    }

    /// Similar to `as_array_unchecked()`, where it does not check `self`'s `object_type` (thus
    /// calling this if `self`'s internal data represents another type will give unexpected
    /// results), but instead of taking a reference to `self`, this consumes `self` and returns the
    /// value as a `Array`.
    ///
    /// This is useful for when you only care about the inner type/value of the `Object`.
    ///
    #[must_use]
    pub fn into_array_unchecked(self) -> Array {
        let a = Array {
            items: unsafe { self.data.array.items },
            size: unsafe { self.data.array.size },
            capacity: unsafe { self.data.array.capacity },
        };
        std::mem::forget(self);
        a
    }

    /// Similar to `as_dictionary_unchecked()`, where it does not check `self`'s `object_type` (thus
    /// calling this if `self`'s internal data represents another type will give unexpected
    /// results), but instead of taking a reference to `self`, this consumes `self` and returns the
    /// value as a `Dictionary`.
    ///
    /// This is useful for when you only care about the inner type/value of the `Object`.
    ///
    #[must_use]
    pub fn into_dictionary_unchecked(self) -> Dictionary {
        let d = Dictionary {
            items: unsafe { self.data.dictionary.items },
            size: unsafe { self.data.dictionary.size },
            capacity: unsafe { self.data.array.capacity },
        };
        std::mem::forget(self);
        d
    }

    /// Convenience method for checking if `self` has `ObjectType::kObjectTypeNil`.
    ///
    #[inline]
    #[must_use]
    pub fn is_nil(&self) -> bool {
        self.object_type == ObjectType::kObjectTypeNil
    }

    /// Convenience method for checking if `self` has `ObjectType::kObjectTypeBoolean`.
    ///
    #[inline]
    #[must_use]
    pub fn is_boolean(&self) -> bool {
        self.object_type == ObjectType::kObjectTypeBoolean
    }

    /// Convenience method for checking if `self` has `ObjectType::kObjectTypeInteger`.
    ///
    #[inline]
    #[must_use]
    pub fn is_integer(&self) -> bool {
        self.object_type == ObjectType::kObjectTypeInteger
    }

    /// Convenience method for checking if `self` has `ObjectType::kObjectTypeString`.
    ///
    #[inline]
    #[must_use]
    pub fn is_string(&self) -> bool {
        self.object_type == ObjectType::kObjectTypeString
    }

    /// Convenience method for checking if `self` has `ObjectType::kObjectTypeArray`.
    ///
    #[inline]
    #[must_use]
    pub fn is_array(&self) -> bool {
        self.object_type == ObjectType::kObjectTypeArray
    }

    /// Convenience method for checking if `self` has `ObjectType::kObjectTypeDictionary`.
    ///
    #[inline]
    #[must_use]
    pub fn is_dictionary(&self) -> bool {
        self.object_type == ObjectType::kObjectTypeDictionary
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::new_nil()
    }
}

impl From<Boolean> for Object {
    #[inline]
    fn from(boolean: Boolean) -> Self {
        new_copy_type!(kObjectTypeBoolean, boolean)
    }
}

impl From<Integer> for Object {
    #[inline]
    fn from(integer: Integer) -> Self {
        new_copy_type!(kObjectTypeInteger, integer)
    }
}

macro_rules! impl_from_int {
    ($int_type:ident) => {
        impl From<$int_type> for Object {
            fn from(i: $int_type) -> Self {
                Self::from(Integer::from(i))
            }
        }
    };
}

impl_from_int!(i8);
impl_from_int!(u8);
impl_from_int!(i16);
impl_from_int!(u16);
impl_from_int!(i32);
impl_from_int!(u32);
impl_from_int!(NonZeroI64);

impl From<Float> for Object {
    #[inline]
    fn from(float: Float) -> Self {
        new_copy_type!(kObjectTypeFloat, float)
    }
}

impl From<NvimString> for Object {
    fn from(string: NvimString) -> Self {
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

impl TryFrom<String> for Object {
    type Error = std::ffi::NulError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::from(NvimString::new(value)?))
    }
}

impl<'a> TryFrom<&'a str> for Object {
    type Error = std::ffi::NulError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Self::from(NvimString::new(value)?))
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Object {
    type Error = std::ffi::NulError;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(Self::from(NvimString::new(value.as_bytes())?))
    }
}

macro_rules! copy_inner_for_clone {
    ($_self:expr, $field_name:ident) => {{
        let value = $_self.data.$field_name();

        Self {
            object_type: $_self.object_type,
            data: ObjectData { $field_name: value },
        }
    }};
}

macro_rules! clone_inner_for_clone {
    ($_self:expr, $field_name:ident) => {{
        let value = $_self.data.$field_name();

        Self {
            object_type: $_self.object_type,
            data: ObjectData {
                $field_name: ManuallyDrop::new(value.clone()),
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
            ObjectType::kObjectTypeFloat => copy_inner_for_clone!(self, float),
            ObjectType::kObjectTypeString => clone_inner_for_clone!(self, string),
            ObjectType::kObjectTypeArray => clone_inner_for_clone!(self, array),
            ObjectType::kObjectTypeDictionary => clone_inner_for_clone!(self, dictionary),
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        match self.object_type {
            ObjectType::kObjectTypeNil
            | ObjectType::kObjectTypeBoolean
            | ObjectType::kObjectTypeInteger
            | ObjectType::kObjectTypeFloat => (),
            ObjectType::kObjectTypeString => {
                let data = &mut self.data;
                unsafe { ManuallyDrop::drop(&mut data.string) };
            }
            ObjectType::kObjectTypeArray => {
                let data = &mut self.data;
                unsafe { ManuallyDrop::drop(&mut data.array) };
            }
            ObjectType::kObjectTypeDictionary => {
                let data = &mut self.data;
                unsafe { ManuallyDrop::drop(&mut data.dictionary) };
            }
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Object");
        d.field("object_type", &self.object_type);

        match self.object_type {
            ObjectType::kObjectTypeNil => d.field("data", &"nil"),
            ObjectType::kObjectTypeBoolean => d.field("data", &self.data.boolean()),
            ObjectType::kObjectTypeInteger => d.field("data", &self.data.integer()),
            ObjectType::kObjectTypeFloat => d.field("data", &self.data.float()),
            ObjectType::kObjectTypeString => d.field("data", &self.data.string().to_string_lossy()),
            ObjectType::kObjectTypeArray => d.field("data", self.data.array()),
            ObjectType::kObjectTypeDictionary => d.field("data", self.data.dictionary()),
        };

        d.finish()
    }
}

impl TryFrom<Object> for Boolean {
    type Error = Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type {
            ObjectType::kObjectTypeBoolean => Ok(value.data.boolean()),
            _ => Err(Error::TypeError {
                expected: ObjectType::kObjectTypeBoolean,
                actual: value.object_type,
            }),
        }
    }
}

impl TryFrom<Object> for Integer {
    type Error = Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type {
            ObjectType::kObjectTypeInteger => Ok(value.data.integer()),
            _ => Err(Error::TypeError {
                expected: ObjectType::kObjectTypeInteger,
                actual: value.object_type,
            }),
        }
    }
}

macro_rules! impl_try_from_for_int {
    ($int:ty) => {
        impl TryFrom<Object> for $int {
            type Error = Error;

            fn try_from(value: Object) -> Result<Self, Self::Error> {
                match value.object_type {
                    ObjectType::kObjectTypeInteger => {
                        Self::try_from(value.data.integer()).map_err(Error::from)
                    }
                    _ => Err(Error::TypeError {
                        expected: ObjectType::kObjectTypeInteger,
                        actual: value.object_type,
                    }),
                }
            }
        }
    };
}

impl_try_from_for_int!(i8);
impl_try_from_for_int!(u8);
impl_try_from_for_int!(i16);
impl_try_from_for_int!(u16);
impl_try_from_for_int!(i32);
impl_try_from_for_int!(u32);
impl_try_from_for_int!(u64);
impl_try_from_for_int!(u128);
impl_try_from_for_int!(isize);
impl_try_from_for_int!(usize);
impl_try_from_for_int!(NonZeroI64);

impl TryFrom<Object> for Float {
    type Error = Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.object_type {
            ObjectType::kObjectTypeFloat => Ok(value.data.float()),
            _ => Err(Error::TypeError {
                expected: ObjectType::kObjectTypeFloat,
                actual: value.object_type,
            }),
        }
    }
}

impl TryFrom<Object> for String {
    type Error = Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        Ok(NvimString::try_from(value)?.to_string_lossy().to_string())
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        if self.object_type != other.object_type {
            return false;
        }

        match self.object_type {
            ObjectType::kObjectTypeNil => true,
            ObjectType::kObjectTypeBoolean => self.data.boolean() == other.data.boolean(),
            ObjectType::kObjectTypeInteger => self.data.integer() == other.data.integer(),
            ObjectType::kObjectTypeFloat => self.data.float() == other.data.float(),
            ObjectType::kObjectTypeString => self.data.string() == other.data.string(),
            ObjectType::kObjectTypeArray => self.data.array() == other.data.array(),
            ObjectType::kObjectTypeDictionary => self.data.dictionary() == other.data.dictionary(),
        }
    }
}

/// Used by `Object` to communicate which type of `Object` it is.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types, clippy::module_name_repetitions)]
#[repr(C)]
pub enum ObjectType {
    /// Nil!
    ///
    kObjectTypeNil = 0,

    /// Boolean
    ///
    kObjectTypeBoolean,

    /// Integer, the same as an `i64`.
    ///
    kObjectTypeInteger,

    /// Float, the same as a `f64`.
    ///
    kObjectTypeFloat,

    /// String, wrapped by `NvimString`.
    ///
    kObjectTypeString,

    /// Array
    ///
    kObjectTypeArray,

    /// Dictionary
    kObjectTypeDictionary,
    // kObjectTypeLuaRef,
    // EXT types, cannot be split or reordered, see #EXT_OBJECT_TYPE_SHIFT
    // kObjectTypeBuffer,
    // kObjectTypeWindow,
    // kObjectTypeTabpage,
}

/// Holds the data for an `Object`.
///
#[allow(clippy::module_name_repetitions)]
#[repr(C)]
pub(crate) union ObjectData {
    boolean: Boolean,
    integer: Integer,
    float: Float,
    string: ManuallyDrop<NvimString>,
    array: ManuallyDrop<Array>,
    dictionary: ManuallyDrop<Dictionary>,
    luaref: LuaRef,
}

impl ObjectData {
    pub(crate) fn boolean(&self) -> Boolean {
        unsafe { self.boolean }
    }

    pub(crate) fn integer(&self) -> Integer {
        unsafe { self.integer }
    }

    pub(crate) fn float(&self) -> Float {
        unsafe { self.float }
    }

    pub(crate) fn string(&self) -> &NvimString {
        unsafe { &self.string }
    }

    pub(crate) fn array(&self) -> &Array {
        unsafe { &self.array }
    }

    pub(crate) fn dictionary(&self) -> &Dictionary {
        unsafe { &self.dictionary }
    }
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
            let subject = Object::from(NvimString::new(input).unwrap());

            assert_eq!(subject.object_type, ObjectType::kObjectTypeString);
            assert_eq!(
                subject.as_string_unchecked(),
                &NvimString::new(input).unwrap()
            );
        }

        do_it("");
        do_it("one \n three");
        do_it("this is an emoji: \u{1f32e}. Tacos are cool.");
    }

    #[test]
    fn test_from_array() {
        fn do_it(input: &str) {
            let subject = Object::from(Array::new([Object::from(NvimString::new(input).unwrap())]));
            let expected = Array::new([Object::from(NvimString::new(input).unwrap())]);

            assert_eq!(subject.object_type, ObjectType::kObjectTypeArray);
            assert_eq!(subject.as_array_unchecked(), &expected);
        }

        do_it("");
        do_it("one \n three");
        do_it("this is an emoji: \u{1f32e}. Tacos are cool.");
    }
}
