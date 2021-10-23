use super::Object;
use neovim_sys::api::vim;
use std::mem::ManuallyDrop;

#[derive(Default, Debug, Clone)]
pub struct Array {
    inner: vim::Array,
}

impl Array {
    pub fn new(inner: vim::Array) -> Self {
        Self { inner }
    }

    pub fn as_slice(&self) -> &[vim::Object] {
        unsafe {
            std::slice::from_raw_parts(self.inner.items as *const vim::Object, self.inner.size)
        }
    }

    // pub fn iter(&self) -> ArrayIter<'_> {
    //     ArrayIter {
    //         inner: self.kvs_as_slice().iter(),
    //     }
    // }

    // fn kvs_as_slice(&self) -> &[vim::KeyValuePair] {
    //     unsafe { std::slice::from_raw_parts(self.inner.items, self.inner.size) }
    // }

    pub fn inner(&self) -> &vim::Array {
        &self.inner
    }

    pub fn to_inner(&self) -> vim::Array {
        self.inner
    }

    pub fn to_vec(&self) -> Vec<Object> {
        self.as_slice()
            .iter()
            .map(|vim_object| Object::new(vim_object.clone()))
            .collect()
    }
}

impl<'a> From<&'a [Object]> for Array {
    fn from(v: &'a [Object]) -> Self {
        let size = v.len();
        let capacity = v.len();

        let boxed_slice = ManuallyDrop::new(v.to_vec().into_boxed_slice());
        let items = Box::into_raw(ManuallyDrop::into_inner(boxed_slice)) as *mut vim::Object;

        Self {
            inner: vim::Array {
                items,
                size,
                capacity,
            },
        }
    }
}

impl<'a> From<&'a [i64]> for Array {
    fn from(ints: &'a [i64]) -> Self {
        let ints: Vec<Object> = ints.iter().map(|i| Object::from(*i)).collect();
        Array::from(ints.as_slice())
    }
}

impl Drop for Array {
    fn drop(&mut self) {
        self.inner.free()
    }
}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice()
            .iter()
            .map(|o| Object::new(o.clone()))
            .eq(other.as_slice().iter().map(|o| Object::new(o.clone())))
    }
}

// pub struct ArrayIter<'a> {
//     kv_iter: std::slice::Iter<'a, vim::KeyValuePair>,
// }

// impl<'a> Iterator for ArrayIter<'a> {
//     type Item = (String<'a>, Object<'a>);

//     fn next(&mut self) -> Option<Self::Item> {
//         self.kv_iter
//             .next()
//             .map(|kv| (String::new(Cow::Borrowed(&kv.key)), Object::from(&kv.value)))
//     }
// }
