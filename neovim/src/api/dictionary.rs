use super::Object;
use neovim_sys::api::vim;

#[derive(Default, Debug, Clone)]
pub struct Dictionary {
    inner: vim::Dictionary,
}

impl Dictionary {
    pub fn new(inner: vim::Dictionary) -> Self {
        Self { inner }
    }

    // pub fn get<'b>(&'b self, key: &str) -> Option<Object<'b>> {
    //     self.iter().find(|(k, _)| k.as_str() == key).map(|(_, v)| v)
    // }

    // pub fn iter(&self) -> DictionaryIter<'_> {
    //     DictionaryIter {
    //         kv_iter: self.kvs_as_slice().iter(),
    //     }
    // }

    fn kvs_as_slice(&self) -> &[vim::KeyValuePair] {
        unsafe { std::slice::from_raw_parts(self.inner.items, self.inner.size) }
    }

    pub fn inner(&self) -> &vim::Dictionary {
        &self.inner
    }

    pub fn to_inner(&self) -> vim::Dictionary {
        self.inner.clone()
    }
}

impl PartialEq for Dictionary {
    fn eq(&self, other: &Self) -> bool {
        self.kvs_as_slice()
            .iter()
            .map(|kv| KeyValuePair::new(kv.clone()))
            .eq(other
                .kvs_as_slice()
                .iter()
                .map(|kv| KeyValuePair::new(kv.clone())))
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        self.inner.free()
    }
}

#[derive(Debug, Clone)]
pub struct KeyValuePair {
    inner: vim::KeyValuePair,
}

impl KeyValuePair {
    pub fn new(inner: vim::KeyValuePair) -> Self {
        Self { inner }
    }

    pub fn key(&self) -> &str {
        std::str::from_utf8(self.inner.key.as_bytes()).unwrap()
    }

    pub fn value(&self) -> Object {
        Object::new(self.inner.value.clone())
    }
}

impl PartialEq for KeyValuePair {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key() && self.value() == other.value()
    }
}

// pub struct DictionaryIter<'a> {
//     kv_iter: std::slice::Iter<'a, vim::KeyValuePair>,
// }

// impl<'a> Iterator for DictionaryIter<'a> {
//     type Item = (String<'a>, Object<'a>);

//     fn next(&mut self) -> Option<Self::Item> {
//         self.kv_iter
//             .next()
//             .map(|kv| (String::new(Cow::Borrowed(&kv.key)), Object::from(&kv.value)))
//     }
// }
