use neovim_sys::typval::{self, ListT, ListitemS};
use std::ptr::NonNull;

pub enum TypVal {
    List(List),
}

pub struct List {
    inner: NonNull<ListT>,
}

impl List {
    pub fn new(length: usize) -> Self {
        let maybe_inner = unsafe { neovim_sys::typval::tv_list_alloc(length) };

        NonNull::new(maybe_inner)
            .map(|inner| Self { inner })
            .unwrap()
    }

    pub fn len(&self) -> isize {
        unsafe { typval::tv_list_len(self.inner.as_ref() as *const ListT) }
    }
}

pub struct ListItem {
    pub inner: NonNull<ListitemS>,
}

impl ListItem {
    // A new, uninitialized list item.
    //
    pub fn new() -> Self {
        let maybe_inner = unsafe { neovim_sys::typval::tv_list_item_alloc() };

        NonNull::new(maybe_inner)
            .map(|inner| Self { inner })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_item_new_test() {
        let _ = ListItem::new();
    }
}
