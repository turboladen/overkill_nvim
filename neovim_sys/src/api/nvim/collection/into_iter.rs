//!
//! Borrowed from https://doc.rust-lang.org/src/alloc/vec/into_iter.rs.html
//!
use std::{
    fmt,
    marker::PhantomData,
    ptr::{self, NonNull},
    slice,
};

/// An iterator that moves out of a `Collection`.
///
/// ```
/// use neovim_sys::api::nvim::{Array, Object};
///
/// let a = Array::new_from([Object::from(0), Object::from(1), Object::from(2)]);
/// let iter: neovim_sys::api::nvim::collection::IntoIter<_> = a.into_iter();
/// ```
pub struct IntoIter<T> {
    pub(super) buf: NonNull<T>,
    pub(super) _marker: PhantomData<T>,
    pub(super) capacity: usize,
    pub(super) start: *const T,
    pub(super) end: *const T,
}

impl<T> IntoIter<T> {
    /// Returns the remaining items of this iterator as a slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use neovim_sys::api::nvim::{Array, Object};
    ///
    /// let a = Array::new_from([Object::from(1.1), Object::from(2.2), Object::from(3.3)]);
    ///
    /// let mut into_iter = a.into_iter();
    /// assert_eq!(into_iter.as_slice(), &[Object::from(1.1), Object::from(2.2), Object::from(3.3)]);
    ///
    /// let _ = into_iter.next().unwrap();
    /// assert_eq!(into_iter.as_slice(), &[Object::from(2.2), Object::from(3.3)]);
    /// ```
    ///
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.start, self.len()) }
    }

    /// Returns the remaining items of this iterator as a mutable slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use neovim_sys::api::nvim::{Array, Object};
    ///
    /// let a = Array::new_from([Object::from(1.1), Object::from(2.2), Object::from(3.3)]);
    ///
    /// let mut into_iter = a.into_iter();
    /// assert_eq!(into_iter.as_slice(), &[Object::from(1.1), Object::from(2.2), Object::from(3.3)]);
    ///
    /// into_iter.as_mut_slice()[2] = Object::from(42);
    ///
    /// let _ = into_iter.next().unwrap();
    /// assert_eq!(into_iter.as_slice(), &[Object::from(2.2), Object::from(42)]);
    /// ```
    ///
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { &mut *self.as_raw_mut_slice() }
    }

    fn as_raw_mut_slice(&mut self) -> *mut [T] {
        ptr::slice_from_raw_parts_mut(self.start as *mut T, self.len())
    }
}

impl<T> fmt::Debug for IntoIter<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.as_slice()).finish()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            let old = self.start;
            self.start = unsafe { self.start.offset(1) };

            Some(unsafe { ptr::read(old) })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.len();

        (exact, Some(exact))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        unsafe { self.end.offset_from(self.start) as usize }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut IntoIter<T>);

        impl<T> Drop for DropGuard<'_, T> {
            fn drop(&mut self) {
                let len = unsafe { self.0.end.offset_from(self.0.start) as usize };
                unsafe {
                    let _ = Vec::from_raw_parts(self.0.buf.as_ptr(), len, self.0.capacity);
                }
            }
        }

        let guard = DropGuard(self);
        // destroy the remaining elements
        unsafe {
            ptr::drop_in_place(guard.0.as_raw_mut_slice());
        }
        // now `guard` will be dropped and do the rest
    }
}
