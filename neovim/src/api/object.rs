use std::convert::TryFrom;

use super::{Array, Dictionary, NString, RustObject};
use neovim_sys::api::vim::{self, ObjectType};

#[derive(Debug)]
pub struct Object {
    inner: vim::Object,
}

impl Object {
    pub fn new(inner: vim::Object) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &vim::Object {
        &self.inner
    }

    pub fn to_inner(&self) -> vim::Object {
        self.inner.clone()
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        Object::new(self.inner.clone())
    }
}

impl From<bool> for Object {
    fn from(boolean: bool) -> Self {
        Self {
            inner: vim::Object {
                object_type: ObjectType::kObjectTypeBoolean,
                data: vim::ObjectData { boolean },
            },
        }
    }
}

impl From<i64> for Object {
    fn from(integer: i64) -> Self {
        Self {
            inner: vim::Object {
                object_type: ObjectType::kObjectTypeInteger,
                data: vim::ObjectData { integer },
            },
        }
    }
}

impl TryFrom<Object> for i64 {
    type Error = ();

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value.inner.object_type {
            ObjectType::kObjectTypeInteger => Ok(unsafe { value.inner.data.integer }),
            _ => Err(()),
        }
    }
}

impl From<f64> for Object {
    fn from(floating: f64) -> Self {
        Self {
            inner: vim::Object {
                object_type: ObjectType::kObjectTypeFloat,
                data: vim::ObjectData { floating },
            },
        }
    }
}

impl From<NString> for Object {
    fn from(string: NString) -> Self {
        Self {
            inner: vim::Object {
                object_type: ObjectType::kObjectTypeString,
                data: vim::ObjectData {
                    string: string.to_inner(),
                },
            },
        }
    }
}

impl From<Array> for Object {
    fn from(array: Array) -> Self {
        Self {
            inner: vim::Object {
                object_type: ObjectType::kObjectTypeArray,
                data: vim::ObjectData {
                    array: array.to_inner(),
                },
            },
        }
    }
}

impl From<Dictionary> for Object {
    fn from(dictionary: Dictionary) -> Self {
        Self {
            inner: vim::Object {
                object_type: ObjectType::kObjectTypeDictionary,
                data: vim::ObjectData {
                    dictionary: dictionary.to_inner(),
                },
            },
        }
    }
}

impl From<RustObject> for Object {
    fn from(rust_object: RustObject) -> Self {
        match rust_object {
            RustObject::Nil => Self::new(vim::Object {
                object_type: ObjectType::kObjectTypeNil,
                data: vim::ObjectData { boolean: false },
            }),
            RustObject::Boolean(boolean) => Object::from(boolean),
            RustObject::Integer(integer) => Object::from(integer),
            RustObject::Float(floating) => Object::from(floating),
            RustObject::String(string) => Object::from(string),
            RustObject::Array(array) => Object::from(array),
            RustObject::Dictionary(dictionary) => Object::from(dictionary),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self.inner.object_type, other.inner.object_type) {
            (ObjectType::kObjectTypeNil, ObjectType::kObjectTypeNil) => true,
            (ObjectType::kObjectTypeBoolean, ObjectType::kObjectTypeBoolean) => unsafe {
                self.inner.data.boolean == other.inner.data.boolean
            },
            (ObjectType::kObjectTypeInteger, ObjectType::kObjectTypeInteger) => unsafe {
                self.inner.data.integer == other.inner.data.integer
            },
            (ObjectType::kObjectTypeFloat, ObjectType::kObjectTypeFloat) => unsafe {
                self.inner.data.floating == other.inner.data.floating
            },
            (ObjectType::kObjectTypeString, ObjectType::kObjectTypeString) => unsafe {
                self.inner.data.string.as_bytes() == other.inner.data.string.as_bytes()
            },
            (ObjectType::kObjectTypeArray, ObjectType::kObjectTypeArray) => {
                let (lhs, rhs) = unsafe {
                    let l = Array::new(self.inner.data.array);
                    let r = Array::new(other.inner.data.array);
                    (l, r)
                };
                lhs == rhs
            }
            (ObjectType::kObjectTypeDictionary, ObjectType::kObjectTypeDictionary) => {
                let (lhs, rhs) = unsafe {
                    let l = Dictionary::new(self.inner.data.dictionary);
                    let r = Dictionary::new(other.inner.data.dictionary);
                    (l, r)
                };
                lhs == rhs
            }
            _ => false,
        }
    }
}
