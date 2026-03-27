//! RAII wrapper for PK-allocated arrays returned by enquiry functions.

use std::fmt;
use std::ops::Deref;
use std::os::raw::c_void;

use parasolid_sys::*;

/// Owned handle to a PK-allocated array.
///
/// Many Parasolid enquiry functions (e.g. `PK_BODY_ask_faces`) allocate an
/// output array internally and return a pointer + count. The caller must free
/// this memory via `PK_MEMORY_free`. `PkArray` wraps this pattern with RAII:
/// the array is freed automatically when the `PkArray` is dropped.
///
/// # Usage
///
/// ```ignore
/// let mut n = 0;
/// let mut ptr = std::ptr::null_mut();
/// pk_call!(PK_BODY_ask_faces(body, &mut n, &mut ptr));
/// let faces = unsafe { PkArray::from_raw(ptr, n) };
/// // Use faces as a slice...
/// for &face_tag in faces.iter() { /* ... */ }
/// // Automatically freed when `faces` goes out of scope.
/// ```
pub struct PkArray<T> {
    ptr: *mut T,
    len: usize,
    // Structural !Send + !Sync: PK-allocated memory is owned by the session.
    // Crossing thread boundaries risks use-after-free after PK_SESSION_stop.
    _not_send: std::marker::PhantomData<*mut ()>,
}

impl<T: Copy> PkArray<T> {
    /// Wrap a raw PK-allocated array.
    ///
    /// # Safety
    ///
    /// - `ptr` must have been allocated by Parasolid (via `PK_MEMORY_alloc` or
    ///   an enquiry function), or be null.
    /// - `len` must be the element count returned by the PK function.
    /// - The caller must not free `ptr` manually after calling this.
    /// - The `PkArray` must be dropped before the Parasolid session is stopped.
    ///   `PK_SESSION_stop` may free all FMALLO-allocated memory; dropping a
    ///   `PkArray` after that point is undefined behavior.
    #[inline]
    pub unsafe fn from_raw(ptr: *mut T, len: i32) -> Self {
        PkArray {
            ptr,
            len: len.max(0) as usize,
            _not_send: std::marker::PhantomData,
        }
    }

    /// Copy the array contents into an owned `Vec`.
    pub fn to_vec(&self) -> Vec<T> {
        if self.ptr.is_null() || self.len == 0 {
            return Vec::new();
        }
        self.as_slice().to_vec()
    }
}

impl<T> PkArray<T> {
    /// Returns the number of elements.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the array is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// View the contents as a slice.
    #[inline]
    fn as_slice(&self) -> &[T] {
        if self.ptr.is_null() || self.len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
        }
    }
}

impl<T> Deref for PkArray<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> Drop for PkArray<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let _ = PK_MEMORY_free(self.ptr as *mut c_void);
            }
            self.ptr = std::ptr::null_mut();
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for PkArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.as_slice().iter()).finish()
    }
}

// PkArray is structurally !Send and !Sync via PhantomData<*mut ()>.
// PK-allocated memory is owned by the session and freed by PK_SESSION_stop().
// Sending to another thread could allow use-after-free after session stop.
