//!
//! This module contains functionality that's common to both `Array` and `Dictionary`.
//!

pub mod into_iter;

pub use self::into_iter::IntoIter;

use std::{
    alloc::{self, Layout},
    fmt,
    marker::PhantomData,
    mem::{self, ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut, Index, IndexMut},
    ptr::{self, addr_of_mut, NonNull},
    slice::SliceIndex,
};

/// Base type for `Array` and `Dictionary`. Since the behavior of those types are quite similar,
/// the bulk of it is defined here.
///
#[repr(C)]
pub struct Collection<T> {
    pub(super) items: NonNull<T>,
    pub(super) size: usize,
    pub(super) capacity: usize,
    pub(super) _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Collection<T> {}
unsafe impl<T: Sync> Sync for Collection<T> {}

impl<T> Collection<T> {
    /// Basic constructor. See also `new_from()`.
    ///
    /// # Panics
    ///
    /// This will panic if `T` isn't `Sized` (which shouldn't be a problem since a `Collection`
    /// should only represent nvim things, which are all sized).
    ///
    #[must_use]
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");

        Self {
            items: NonNull::dangling(),
            size: 0,
            capacity: 0,
            _marker: PhantomData,
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.capacity == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // This can't overflow since self.cap <= isize::MAX.
            let new_cap = 2 * self.capacity;

            // `Layout::array` checks that the number of bytes is <= usize::MAX,
            // but this is redundant since old_layout.size() <= isize::MAX,
            // so the `unwrap` should never fail.
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(
            isize::try_from(new_layout.size()).is_ok(),
            "Allocation too large"
        );

        let new_ptr = if self.capacity == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            let old_ptr = self.items.as_ptr().cast::<u8>();
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.items = match NonNull::new(new_ptr.cast::<T>()) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.capacity = new_cap;
    }

    /// Instantiates a new `Self` using any pararmeter that can be converted into a `Vec<T>`.
    ///
    pub fn new_from<U: Into<Vec<T>>>(vec: U) -> Self {
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

        let new_items = unsafe { NonNull::new_unchecked(vec.as_mut_ptr()) };

        unsafe {
            // Initializing the `items` field
            // If there is a panic here, then the `String` in the `name` field leaks.
            addr_of_mut!((*ptr).items).write(new_items);
        }

        mem::forget(vec);

        unsafe { uninit.assume_init() }
    }

    /// Appends `elem` to the end of the collection. Note that for `Dictionary`s, no key-sorting is
    /// done (i.e. like a `BTreeMap` or `HashMap`); the item is simply added to the end of the
    /// collection.
    ///
    pub fn push(&mut self, elem: T) {
        if self.size == self.capacity {
            self.grow();
        }

        unsafe {
            ptr::write(self.items.as_ptr().add(self.size), elem);
        }

        // Can't fail, we'll OOM first.
        self.size += 1;
    }

    /// Removes and return the last element in the collection. If the collection is empty, it
    /// returns `None`.
    ///
    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            unsafe { Some(ptr::read(self.items.as_ptr().add(self.size))) }
        }
    }

    /// Like `Vec::insert()`, this inserts and element at a specific index, shifting the existing
    /// values towards the end of the collection.
    ///
    /// # Panics
    ///
    /// This panics if `index` is out-of-bounds.
    ///
    pub fn insert(&mut self, index: usize, elem: T) {
        // Note: `<=` because it's valid to insert after everything which would be equivalent to
        // push.
        assert!(index <= self.size, "index out of bounds");

        if self.capacity == self.size {
            self.grow();
        }

        unsafe {
            ptr::copy(
                self.items.as_ptr().add(index),
                self.items.as_ptr().add(index + 1),
                self.size - index,
            );
            ptr::write(self.items.as_ptr().add(index), elem);
            self.size += 1;
        }
    }

    /// Like `Vec::remove()`, this removes the element at `index`.
    ///
    /// # Panics
    ///
    /// This panics if `index` is out of bounds.
    ///
    pub fn remove(&mut self, index: usize) -> T {
        // Note: `<` because it's *not* valid to remove after everything
        assert!(index < self.size, "index out of bounds");

        unsafe {
            self.size -= 1;
            let result = ptr::read(self.items.as_ptr().add(index));

            ptr::copy(
                self.items.as_ptr().add(index + 1),
                self.items.as_ptr().add(index),
                self.size - index,
            );
            result
        }
    }

    /// Builds a slice of all internal items.
    ///
    #[must_use]
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        // It's important to use `as_ptr()` here instead of `as_ref()` since if the `Collection`
        // hasn't yet been written to, `self.items` will still be dangling.
        unsafe { std::slice::from_raw_parts(self.items.as_ptr(), self.size) }
    }

    /// Builds a mutable slice of all internal items.
    ///
    #[must_use]
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // It's important to use `as_ptr()` here instead of `as_ref()` since if the `Collection`
        // hasn't yet been written to, `self.items` will still be dangling.
        unsafe { std::slice::from_raw_parts_mut(self.items.as_ptr(), self.size) }
    }

    /// The number of items in the collection.
    ///
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.size
    }

    /// Is this an empty collection?
    ///
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The capacity of items in the collection. This will only differ form `len()` if the
    /// `Collection` was instantiated as such.
    ///
    #[inline]
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// A mutable pointer to the inner `items`. Note that if the `Collection` hasn't yet been
    /// written to, this pointer may still be uninitialized (dangling).
    ///
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.items.as_ptr()
    }
}

impl<T> Default for Collection<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for Collection<T>
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self::new_from(self.as_slice().to_vec())
    }
}

impl<T> Drop for Collection<T> {
    fn drop(&mut self) {
        if self.capacity != 0 {
            while self.pop().is_some() {}

            let layout = Layout::array::<T>(self.capacity).unwrap();

            unsafe {
                alloc::dealloc(self.items.as_ptr().cast::<u8>(), layout);
            }
        }
    }
}

impl<T> Deref for Collection<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> DerefMut for Collection<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> fmt::Debug for Collection<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T> From<Vec<T>> for Collection<T> {
    fn from(vec: Vec<T>) -> Self {
        Self::new_from(vec)
    }
}

impl<T> From<Collection<T>> for Vec<T> {
    fn from(collection: Collection<T>) -> Self {
        let v = unsafe {
            Self::from_raw_parts(
                collection.items.as_ptr(),
                collection.size,
                collection.capacity,
            )
        };
        std::mem::forget(collection);

        v
    }
}

impl<'a, T> From<&'a Collection<T>> for &'a [T] {
    fn from(collection: &'a Collection<T>) -> Self {
        collection.as_slice()
    }
}

impl<I, T> Index<I> for Collection<T>
where
    I: SliceIndex<[T]>,
{
    type Output = <I as SliceIndex<[T]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.deref().index(index)
    }
}

impl<I, T> IndexMut<I> for Collection<T>
where
    I: SliceIndex<[T]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.deref_mut().index_mut(index)
    }
}

impl<T> PartialEq for Collection<T>
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(self.as_slice(), other.as_slice())
    }
}

impl<T> IntoIterator for Collection<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let capacity = self.capacity;

            let mut me = ManuallyDrop::new(self);

            let start = me.as_mut_ptr();
            let end = if capacity == 0 {
                start
            } else {
                start.add(me.len())
            };

            IntoIter {
                buf: NonNull::new_unchecked(start),
                capacity,
                start,
                end,
                _marker: PhantomData,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::nvim::{Array, Dictionary, NvimString, Object};

    #[test]
    fn test_new_array() {
        let array = Array::new();
        assert_eq!(array.len(), 0);
        assert_eq!(array.capacity(), 0);
    }

    #[test]
    fn test_new_dictionary() {
        let dict = Dictionary::new();
        assert_eq!(dict.len(), 0);
        assert_eq!(dict.capacity(), 0);
    }

    #[test]
    fn test_push() {
        let mut array = Array::new();
        assert_eq!(array.len(), 0);
        assert_eq!(array.capacity(), 0);

        array.push(Object::from(true));
        assert_eq!(array.len(), 1);
        assert_eq!(array.capacity(), 1);

        array.push(Object::from(4.56));
        assert_eq!(array.len(), 2);
        assert_eq!(array.capacity(), 2);

        array.push(Object::from(-4));
        assert_eq!(array.len(), 3);
        assert_eq!(array.capacity(), 4);

        array.push(Object::from(NvimString::new_unchecked("bobo the clown")));
        assert_eq!(array.len(), 4);
        assert_eq!(array.capacity(), 4);

        array.push(Object::new_nil());
        assert_eq!(array.len(), 5);
        assert_eq!(array.capacity(), 8);
    }

    #[test]
    fn test_pop() {
        let mut array = Array::new();
        assert_eq!(array.len(), 0);
        assert_eq!(array.capacity(), 0);

        array.push(Object::from(true));
        assert_eq!(array.len(), 1);
        assert_eq!(array.capacity(), 1);

        let _x = array.pop().unwrap();
        assert_eq!(array.len(), 0);
        assert_eq!(array.capacity(), 1);

        assert!(array.pop().is_none());
    }

    #[test]
    #[should_panic]
    fn test_insert_out_of_bounds() {
        let mut array = Array::new();
        array.insert(1, Object::from(42));
    }

    #[test]
    fn test_insert() {
        let mut array = Array::new();
        array.insert(0, Object::from(1));
        assert_eq!(array[0], Object::from(1));

        // Insert at the end
        array.insert(1, Object::from(2));
        assert_eq!(array[0], Object::from(1));
        assert_eq!(array[1], Object::from(2));

        // Insert at the beginning
        array.insert(0, Object::from(3));
        assert_eq!(array[0], Object::from(3));
        assert_eq!(array[1], Object::from(1));
        assert_eq!(array[2], Object::from(2));
    }

    #[test]
    #[should_panic]
    fn test_remove_out_of_bounds() {
        let mut array = Array::new();
        array.remove(1);
    }

    #[test]
    fn test_remove() {
        let mut array = Array::new();

        array.push(Object::from(1));
        let item = array.remove(0);
        assert_eq!(item, Object::from(1));

        array.push(Object::from(2));
        let item = array.remove(0);
        assert_eq!(item, Object::from(2));

        array.push(Object::from(3));
        array.push(Object::from(4));
        let item = array.remove(1);
        assert_eq!(item, Object::from(4));
    }

    #[test]
    fn test_as_slice() {
        let mut array = Array::new();
        array.push(Object::new_nil());
        array.push(Object::from(false));

        let s = array.as_slice();
        assert_eq!(s, &[Object::new_nil(), Object::from(false)]);
    }

    #[test]
    fn test_deref() {
        let mut array = Array::new();
        array.push(Object::from(false));
        array.push(Object::from(4.567));

        let s = &*array;
        assert_eq!(s, &[Object::from(false), Object::from(4.567)]);
    }

    #[test]
    fn test_deref_mut() {
        let mut array = Array::new();
        array.push(Object::from(false));
        array.push(Object::from(4.567));

        let s = &mut *array;

        if let Some(first) = s.first_mut() {
            *first = Object::from(123);
        }
        assert_eq!(s, &[Object::from(123), Object::from(4.567)]);
    }

    #[test]
    fn test_clone() {
        let v = vec![
            Object::from(true),
            Object::from(123),
            Object::from(4.56),
            Object::from(NvimString::new_unchecked("meow")),
        ];
        let original = Array::from(v);
        let clone = original.clone();
        assert_eq!(original, clone);
    }

    #[test]
    fn test_from_vec() {
        let v = vec![Object::from(true), Object::from(123), Object::from(4.56)];
        let array = Array::from(v);
        assert_eq!(array[0], Object::from(true));
        assert_eq!(array[1], Object::from(123));
        assert_eq!(array[2], Object::from(4.56));
    }

    #[test]
    fn test_into_iter() {
        let v = vec![Object::from(1u8), Object::from(2u8), Object::from(3u8)];
        let array = Array::from(v.clone());

        let new_v: Vec<_> = array.into_iter().collect();
        assert_eq!(new_v, v);
    }
}
