use super::{Array, Dictionary, RustObject};
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

impl From<RustObject> for Object {
    fn from(rust_object: RustObject) -> Self {
        let vim_object = match rust_object {
            RustObject::Nil => vim::Object {
                object_type: ObjectType::kObjectTypeNil,
                data: vim::ObjectData { boolean: false },
            },
            RustObject::Boolean(boolean) => vim::Object {
                object_type: ObjectType::kObjectTypeBoolean,
                data: vim::ObjectData { boolean },
            },
            RustObject::Integer(integer) => vim::Object {
                object_type: ObjectType::kObjectTypeInteger,
                data: vim::ObjectData { integer },
            },
            RustObject::Float(floating) => vim::Object {
                object_type: ObjectType::kObjectTypeFloat,
                data: vim::ObjectData { floating },
            },
            RustObject::String(string) => vim::Object {
                object_type: ObjectType::kObjectTypeString,
                data: vim::ObjectData {
                    string: string.to_inner(),
                },
            },
            RustObject::Array(array) => vim::Object {
                object_type: ObjectType::kObjectTypeArray,
                data: vim::ObjectData {
                    array: array.to_inner(),
                },
            },
            RustObject::Dictionary(dictionary) => vim::Object {
                object_type: ObjectType::kObjectTypeDictionary,
                data: vim::ObjectData {
                    dictionary: dictionary.to_inner(),
                },
            },
        };
        Self::new(vim_object)
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
