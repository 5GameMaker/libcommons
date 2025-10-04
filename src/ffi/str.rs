use std::{
    borrow::Borrow,
    ffi::c_char,
    fmt::{Debug, Display},
    marker::PhantomData,
    mem::{forget, transmute},
    ptr::null_mut,
    slice,
};

unsafe extern "C" fn __libcommons_rust_drop(string: *mut FfiString) {
    unsafe {
        let string = string.as_mut().unwrap();
        drop(String::from_raw_parts(
            string.buf,
            string.len,
            string.capacity,
        ));
    }
}

/// An FFI-compatible string slice.
///
/// This cannot be passed via FFI on its own. Use [FfiStr::as_ptr] to get a C-compatible wide pointer.
///
/// See `libcommons.h`.
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash)]
pub struct FfiStr {
    inner: str,
}
impl FfiStr {
    /// Get an [str] reference from this [FfiStr].
    ///
    /// Since all ffistrs must be valid UTF-8, this reference
    /// can be cheaply passed to
    pub const fn as_str(&self) -> &str {
        &self.inner
    }

    /// Get this string's underlying bytes.
    pub const fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    /// Convert an [str] to [FfiStr].
    pub const fn from_str(str: &str) -> &Self {
        unsafe { transmute(str) }
    }

    /// Create an [FfiStr] referencing the buffer.
    ///
    /// ## Safety
    /// Provided pointer must be valid for the entire time
    /// that this [FfiStr] will be used.
    pub const unsafe fn from_raw_parts(data: *const u8, len: usize) -> &'static Self {
        unsafe { transmute(str::from_utf8_unchecked(slice::from_raw_parts(data, len))) }
    }
    /// Create an [FfiStr] from bytes.
    ///
    /// ## Safety
    /// Provided buffer must contain valid UTF-8 data.
    pub const unsafe fn from_utf8_unchecked(slice: &[u8]) -> &Self {
        unsafe { transmute(str::from_utf8_unchecked(slice)) }
    }

    /// Make this [FfiStr] passable via ffi.
    pub const fn as_ptr(&self) -> FfiStrPtr<'_> {
        FfiStrPtr {
            buf: self.inner.as_ptr(),
            len: self.inner.len(),
            _phantom: PhantomData,
        }
    }

    /// Convert this [FfiStr] into an [FfiString].
    pub fn to_ffi_string(&self) -> FfiString {
        self.into()
    }
}
impl<'a> From<&'a str> for &'a FfiStr {
    fn from(value: &'a str) -> Self {
        FfiStr::from_str(value)
    }
}
impl<'a> From<FfiStrPtr<'a>> for &'a FfiStr {
    fn from(value: FfiStrPtr<'a>) -> Self {
        unsafe { FfiStr::from_raw_parts(value.buf, value.len) }
    }
}
impl<'a> From<&'a FfiStrPtr<'a>> for &'a FfiStr {
    fn from(value: &'a FfiStrPtr<'a>) -> Self {
        unsafe { FfiStr::from_raw_parts(value.buf, value.len) }
    }
}
impl AsRef<str> for FfiStr {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}
impl ToOwned for FfiStr {
    type Owned = FfiString;

    fn to_owned(&self) -> Self::Owned {
        FfiString::from(self)
    }
}

/// A wide pointer to an FFI-compatible string slice.
///
/// See `libcommons.h`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FfiStrPtr<'a> {
    buf: *const u8,
    len: usize,
    _phantom: PhantomData<&'a str>,
}
impl<'a> FfiStrPtr<'a> {
    /// Obtain the underlying [c_char] pointer.
    pub const fn as_ptr(&self) -> *const c_char {
        self.buf as *const c_char
    }

    /// Get length of this [FfiStrPtr].
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Check whether this [FfiStrPtr] is empty.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Convert this [FfiStrPtr] to [str].
    pub const fn as_str(&self) -> &str {
        unsafe {
            if self.is_empty() {
                str::from_utf8_unchecked(slice::from_raw_parts(self.buf, self.len))
            } else {
                ""
            }
        }
    }

    /// Clone this [FfiStrPtr] into an [FfiString].
    pub fn to_ffi_string(&self) -> FfiString {
        self.into()
    }
}
impl AsRef<FfiStr> for FfiStrPtr<'_> {
    fn as_ref(&self) -> &FfiStr {
        self.into()
    }
}
impl<'a> From<&'a FfiStr> for FfiStrPtr<'a> {
    fn from(value: &'a FfiStr) -> Self {
        value.as_ptr()
    }
}

/// An FFI-compatible owned string.
///
/// See `libcommons.h`.
#[repr(C)]
pub struct FfiString {
    buf: *mut u8,
    len: usize,
    capacity: usize,
    drop: Option<unsafe extern "C" fn(*mut FfiString)>,
}
impl FfiString {
    /// Create a new FfiString.
    ///
    /// This method will not allocate.
    ///
    /// ```
    /// use libcommons::ffi::str::FfiString;
    ///
    /// let mut string = FfiString::new();
    /// assert!(string.is_empty());
    /// ```
    pub const fn new() -> Self {
        Self {
            buf: null_mut(),
            len: 0,
            capacity: 0,
            drop: None,
        }
    }

    /// Create a new FfiString with specified capacity.
    ///
    /// This method will not allocate if `len` is 0.
    ///
    /// ```
    /// use libcommons::ffi::str::FfiString;
    ///
    /// let mut string = FfiString::from("Hi!");
    /// assert_eq!(string.as_bytes(), &[b'H', b'i', b'!']);
    /// ```
    pub fn with_capacity(len: usize) -> Self {
        if len == 0 {
            return Self::new();
        }

        let mut string = String::with_capacity(len);
        let ffi = Self {
            buf: string.as_mut_ptr(),
            len: string.len(),
            capacity: string.capacity(),
            drop: Some(__libcommons_rust_drop),
        };
        forget(string);
        ffi
    }

    /// Get underlying bytes.
    ///
    /// ```
    /// use libcommons::ffi::str::FfiString;
    ///
    /// let mut string = FfiString::from("Hi!");
    /// assert_eq!(string.as_bytes(), &[b'H', b'i', b'!']);
    /// ```
    pub const fn as_bytes(&self) -> &[u8] {
        if self.buf.is_null() {
            &[]
        } else {
            unsafe { slice::from_raw_parts(self.buf, self.len) }
        }
    }

    /// Get a string reference.
    ///
    /// ```
    /// use libcommons::ffi::str::FfiString;
    ///
    /// let mut string = FfiString::from("Hi!");
    /// assert_eq!(string.as_str(), "Hi!");
    /// ```
    pub const fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(self.buf, self.len)) }
    }

    /// Get an FFI string reference.
    ///
    /// ```
    /// use libcommons::ffi::str::FfiString;
    ///
    /// let mut string = FfiString::from("Hi!");
    /// string.as_ffi_str();
    /// ```
    pub const fn as_ffi_str(&self) -> &FfiStr {
        unsafe { FfiStr::from_raw_parts(self.buf, self.len) }
    }

    /// Get the length of this string in bytes.
    ///
    /// ```
    /// use libcommons::ffi::str::FfiString;
    ///
    /// let mut string = FfiString::from("Hi!");
    ///
    /// assert_eq!(string.len(), 3);
    /// ```
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Check if string is empty.
    ///
    /// ```
    /// use libcommons::ffi::str::FfiString;
    ///
    /// let mut string = FfiString::from("");
    ///
    /// assert_eq!(string.is_empty(), true);
    /// ```
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Obtain this string's capacity.
    ///
    /// ```
    /// use libcommons::ffi::str::FfiString;
    ///
    /// let string = FfiString::with_capacity(128);
    /// assert_eq!(string.capacity(), 128);
    /// ```
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Append a string slice.
    ///
    /// Will re-allocate the internal buffer if the string is
    /// too long.
    ///
    /// ```
    /// use libcommons::ffi::str::FfiString;
    ///
    /// let mut string = FfiString::new();
    ///
    /// string.push_str("Hello, ");
    /// string.push_str("world!");
    ///
    /// assert_eq!(string, "Hello, world!");
    /// ```
    pub fn push_str(&mut self, str: &str) {
        if str.is_empty() {
            return;
        }

        unsafe {
            let olen = str.len();

            if self.len + olen <= self.capacity {
                slice::from_raw_parts_mut(self.buf, self.capacity)[self.len..][..olen]
                    .copy_from_slice(str.as_bytes());
                self.len += olen;
                return;
            }

            let newsize = (self.capacity + olen)
                .checked_next_power_of_two()
                .and_then(|x| x.checked_mul(2))
                .expect("string is too long");
            let mut newstring = {
                let mut s = String::with_capacity(newsize);
                let funny_s = String::from_raw_parts(s.as_mut_ptr(), self.len + olen, s.capacity());
                forget(s);
                funny_s
            };

            if !self.buf.is_null() {
                newstring.as_bytes_mut()[0..self.len].copy_from_slice(self.as_bytes());
            }
            newstring.as_bytes_mut()[self.len..olen].copy_from_slice(str.as_bytes());

            if !self.buf.is_null()
                && let Some(drop) = self.drop
            {
                drop(self);
            }

            self.buf = newstring.as_mut_ptr();
            self.len += olen;
            self.drop = Some(__libcommons_rust_drop);
            self.capacity = newstring.capacity();

            forget(newstring);
        }
    }

    /// Append a character.
    ///
    /// Will re-allocate the internal buffer if the string is
    /// too long.
    ///
    /// ```
    /// use libcommons::ffi::str::FfiString;
    ///
    /// let mut string = FfiString::new();
    ///
    /// string.push_str("Hello, ");
    /// string.push('C');
    ///
    /// assert_eq!(string, "Hello, C");
    /// ```
    #[cfg(feature = "str")]
    pub fn push(&mut self, char: char) {
        let mut stackstr = crate::str::stack::StackString::<4>::new();
        stackstr.push(char).unwrap();
        self.push_str(&stackstr);
    }
}
impl Drop for FfiString {
    fn drop(&mut self) {
        unsafe {
            if !self.buf.is_null()
                && let Some(drop) = self.drop
            {
                drop(self);
            }
        }
    }
}
impl Default for FfiString {
    fn default() -> Self {
        Self::new()
    }
}
impl<'a> PartialEq<&'a str> for FfiString {
    fn eq(&self, other: &&'a str) -> bool {
        (&self.as_str()).eq(other)
    }
}
impl Debug for FfiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}
impl Display for FfiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}
impl From<String> for FfiString {
    fn from(mut string: String) -> Self {
        let ffi = Self {
            buf: if string.capacity() == 0 {
                null_mut()
            } else {
                string.as_mut_ptr()
            },
            len: string.len(),
            capacity: string.capacity(),
            drop: (string.capacity() != 0).then_some(__libcommons_rust_drop),
        };
        forget(string);
        ffi
    }
}
impl<'a> From<&'a str> for FfiString {
    fn from(value: &'a str) -> Self {
        String::from(value).into()
    }
}
impl<'a> From<&'a FfiStr> for FfiString {
    fn from(value: &'a FfiStr) -> Self {
        Self::from(value.as_str())
    }
}
impl<'a> From<FfiStrPtr<'a>> for FfiString {
    fn from(value: FfiStrPtr<'a>) -> Self {
        Self::from(value.as_str())
    }
}
impl<'a> From<&'a FfiStrPtr<'a>> for FfiString {
    fn from(value: &'a FfiStrPtr<'a>) -> Self {
        Self::from(value.as_str())
    }
}
impl AsRef<FfiStr> for FfiString {
    fn as_ref(&self) -> &FfiStr {
        self.as_ffi_str()
    }
}
impl AsRef<str> for FfiString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl Borrow<FfiStr> for FfiString {
    fn borrow(&self) -> &FfiStr {
        self.as_ffi_str()
    }
}
impl Borrow<str> for FfiString {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
