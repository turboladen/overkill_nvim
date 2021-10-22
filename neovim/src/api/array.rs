use super::Object;
use neovim_sys::api::vim;

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
        self.inner.clone()
    }
}

// impl<'a, 'b: 'a> From<&'a [Object<'b>]> for Array<'a> {
//     fn from(v: &'a [Object<'b>]) -> Self {
//         let size = v.len();
//         let capacity = v.len();
//         let mut vim_vec: Vec<vim::Object> =
//             v.to_owned().into_iter().map(|o| o.into_vim()).collect();
//         let slice = vim_vec.as_mut_slice();
//         let items = slice.as_mut_ptr();

//         Self {
//             inner: Cow::Owned(vim::Array {
//                 items,
//                 size,
//                 capacity,
//             }),
//         }
//     }
// }

// impl<'b, 'a: 'b> From<Vec<&'b Object<'a>>> for Array<'b> {
//     fn from(v: Vec<&'b Object<'a>>) -> Self {
//         let size = v.len();
//         let capacity = v.capacity();
//         let mut vim_vec: Vec<vim::Object> = v.into_iter().map(|o| o.clone().as_inner()).collect();
//         let slice = vim_vec.as_mut_slice();
//         let items = slice.as_mut_ptr();

//         Self {
//             inner: Cow::Owned(vim::Array {
//                 items,
//                 size,
//                 capacity,
//             }),
//         }
//     }
// }
// impl<'a> From<Vec<Object<'a>>> for Array<'a> {
//     fn from(v: Vec<Object<'a>>) -> Self {
//         let size = v.len();
//         eprintln!("vec size: {}", size);
//         let capacity = v.capacity();
//         eprintln!("vec capacity: {}", capacity);
//         // let mut vim_vec: Vec<vim::Object> = v.into_iter().map(|o| o.into_inner()).collect();
//         // let slice = vim_vec.as_mut_slice();
//         // let items = slice.as_mut_ptr();
//         let vim_vec: Vec<vim::Object> = v.into_iter().map(|o| o.into_inner()).collect();
//         let mut slice = vim_vec.into_boxed_slice();
//         let items = slice.as_mut_ptr();

//         Self {
//             inner: Cow::Owned(vim::Array {
//                 items,
//                 size,
//                 capacity,
//             }),
//         }
//     }
// }
impl<'a> From<&'a [Object]> for Array {
    fn from(v: &'a [Object]) -> Self {
        let size = v.len();
        eprintln!("vec size: {}", size);
        let capacity = v.len();
        eprintln!("vec capacity: {}", capacity);
        // let mut vim_vec: Vec<vim::Object> = v.into_iter().map(|o| o.into_inner()).collect();
        // let slice = vim_vec.as_mut_slice();
        // let items = slice.as_mut_ptr();
        let mut vim_vec: Vec<vim::Object> = v.to_owned().iter().map(|o| o.to_inner()).collect();
        let slice = vim_vec.as_mut_slice();
        for o in slice.iter() {
            eprintln!("Object type: {:?}", o.object_type);
            eprintln!("Object type: {}", o.object_type as u32);
            eprintln!("Object: {:?}", o);
        }
        let items = slice.as_mut_ptr();
        eprintln!("items is null? {}", items.is_null());

        Self {
            inner: vim::Array {
                items,
                size,
                capacity,
            },
        }
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
