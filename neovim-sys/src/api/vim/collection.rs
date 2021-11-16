//!
//! This module contains functionality that's common to both `Array` and `Dictionary`.
//!
use std::{
    fmt,
    mem::{self, MaybeUninit},
    ptr::addr_of_mut,
    slice,
};

/// Base type for `Array` and `Dictionary`. Since the behavior of those types are quite similar,
/// the bulk of it is defined here.
///
#[repr(C)]
pub struct Collection<T> {
    pub(super) items: *mut T,
    pub(super) size: usize,
    pub(super) capacity: usize,
}

impl<T> Collection<T> {
    /// Instantiates a new `Self` using any pararmeter that can be converted into a `Vec<T>`.
    ///
    pub fn new<U: Into<Vec<T>>>(vec: U) -> Self {
        let mut vec: Vec<T> = vec.into();

        let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();

        // Initializing the `size` field
        // Using `write` instead of assignment via `=` to not call `drop` on the
        // old, uninitialized value.
        unsafe {
            addr_of_mut!((*ptr).size).write(vec.len());
            addr_of_mut!((*ptr).capacity).write(vec.capacity());
        }

        let new_items = vec.as_mut_ptr();

        unsafe {
            // Initializing the `items` field
            // If there is a panic here, then the `String` in the `name` field leaks.
            addr_of_mut!((*ptr).items).write(new_items);
        }

        mem::forget(vec);

        unsafe { uninit.assume_init() }
    }

    /// Builds a slice of all internal items.
    ///
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.items, self.size) }
    }

    /// The number of items in the collection.
    ///
    #[must_use]
    pub const fn len(&self) -> usize {
        self.size
    }

    /// Is this an empty collection?
    ///
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The capacity of items in the collection. This will only differ form `len()` if the
    /// `Collection` was instantiated as such.
    ///
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns an iterator over `&T`.
    ///
    #[must_use]
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.as_slice().iter()
    }
}

impl<T: Clone> Clone for Collection<T> {
    fn clone(&self) -> Self {
        Self::new(self.as_slice().to_vec())
    }
}

impl<T> Drop for Collection<T> {
    fn drop(&mut self) {
        let _vec = unsafe { Vec::from_raw_parts(self.items, self.size, self.capacity) };
    }
}

impl<T: fmt::Debug> fmt::Debug for Collection<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> From<Collection<T>> for Vec<T> {
    fn from(dictionary: Collection<T>) -> Self {
        let v =
            unsafe { Self::from_raw_parts(dictionary.items, dictionary.size, dictionary.capacity) };
        std::mem::forget(dictionary);

        v
    }
}

impl<'a, T> From<&'a Collection<T>> for &'a [T] {
    fn from(dict: &'a Collection<T>) -> Self {
        dict.as_slice()
    }
}

impl<T> PartialEq for Collection<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(self.as_slice(), other.as_slice())
    }
}
