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
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    pub fn from_str<'a>(str: &'a str) -> &'a Self {
        unsafe { transmute(str) }
    }

    pub unsafe fn from_raw_parts(data: *const u8, len: usize) -> &'static Self {
        unsafe { transmute(str::from_utf8_unchecked(slice::from_raw_parts(data, len))) }
    }

    pub unsafe fn from_utf8_unchecked<'a>(slice: &'a [u8]) -> &'a Self {
        unsafe { transmute(str::from_utf8_unchecked(slice)) }
    }

    pub fn as_ptr(&self) -> FfiStrPtr<'_> {
        FfiStrPtr {
            buf: self.inner.as_ptr(),
            len: self.inner.len(),
            _phantom: PhantomData,
        }
    }

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
    pub fn as_ptr(&self) -> *const c_char {
        self.buf as *const c_char
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_ref(&self) -> &FfiStr {
        self.into()
    }

    pub fn as_str(&self) -> &str {
        Into::<&FfiStr>::into(self).as_str()
    }

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
        if len == 0 {}

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
    pub fn as_bytes(&self) -> &[u8] {
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
    pub fn as_str(&self) -> &str {
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
    pub fn as_ffi_str(&self) -> &FfiStr {
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

            if !self.buf.is_null() {
                if let Some(drop) = self.drop {
                    drop(self);
                }
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
            if !self.buf.is_null() {
                if let Some(drop) = self.drop {
                    drop(self);
                }
            }
        }
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
impl<'a> From<String> for FfiString {
    fn from(mut string: String) -> Self {
        let ffi = Self {
            buf: (string.capacity() != 0)
                .then_some(string.as_mut_ptr())
                .unwrap_or(null_mut()),
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
