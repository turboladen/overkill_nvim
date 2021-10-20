use super::{Object, String};
use neovim_sys::api::vim;
use std::borrow::Cow;

#[derive(Default, Debug)]
pub struct Dictionary {
    inner: vim::Dictionary,
}

impl Dictionary {
    pub fn new(inner: vim::Dictionary) -> Self {
        Self { inner }
    }

    pub fn get<'b>(&'b self, key: &str) -> Option<Object<'b>> {
        self.iter().find(|(k, _)| k.as_str() == key).map(|(_, v)| v)
    }

    pub fn iter(&self) -> DictionaryIter<'_> {
        DictionaryIter {
            kv_iter: self.kvs_as_slice().iter(),
        }
    }

    fn kvs_as_slice(&self) -> &[vim::KeyValuePair] {
        unsafe { std::slice::from_raw_parts(self.inner.items, self.inner.size) }
    }

    pub fn inner(&self) -> vim::Dictionary {
        self.inner
    }

    pub fn inner_ref(&self) -> &vim::Dictionary {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut vim::Dictionary {
        &mut self.inner
    }

    pub fn into_inner(self) -> vim::Dictionary {
        self.inner
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        self.inner.free()
    }
}

// impl PartialEq for Dictionary {
//     fn eq(&self, other: &Self) -> bool {
//         self.kvs_as_slice() == other.kvs_as_slice()
//     }
// }

pub struct KeyValuePair {
    inner: vim::KeyValuePair,
}


pub struct DictionaryIter<'a> {
    kv_iter: std::slice::Iter<'a, vim::KeyValuePair>,
}

impl<'a> Iterator for DictionaryIter<'a> {
    type Item = (String<'a>, Object<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.kv_iter
            .next()
            .map(|kv| (String::new(Cow::Borrowed(&kv.key)), Object::from(&kv.value)))
    }
}
