//! Utilities.
//!
//! This is where things like box error and functional traits go.

use core::fmt;
use std::{borrow::{Borrow, BorrowMut}, hash::Hash, marker::PhantomData, mem::MaybeUninit, ops::{Deref, DerefMut}};

mod __inner {
    /// One-liner tools to make code look nicer.
    ///
    /// i.e. instead of
    #[cfg_attr(feature = "extra_traits", doc = "```")]
    #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
    /// let mut a = Some(Some("hi"));
    /// match &mut a {
    ///     None => (),
    ///     Some(x) => drop(x.take()),
    /// }
    /// ```
    /// you can do
    #[cfg_attr(feature = "extra_traits", doc = "```")]
    #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
    /// use libcommons::prelude::*;
    ///
    /// let mut a = Some(Some("hi"));
    /// match &mut a {
    ///     None => (),
    ///     Some(x) => x.take().drop(),
    /// }
    /// ```
    #[allow(unused)]
    pub trait Fun {
        /// Drop self and return another value.
        ///
        /// For a function `f`, return `f(self)`.
        ///
        #[cfg_attr(feature = "extra_traits", doc = "```")]
        #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
        /// #![allow(deprecated)]
        /// use libcommons::prelude::*;
        ///
        /// fn f(value: &mut Option<Option<i32>>) -> bool {
        ///     match value {
        ///         Some(x) => x.take().instead(true),
        ///         None => false,
        ///     }
        /// }
        ///
        /// assert_eq!(f(&mut Some(Some(1))), true);
        /// assert_eq!(f(&mut Some(None)), true);
        /// assert_eq!(f(&mut None), false);
        /// ```
        fn instead<Y>(&self, value: Y) -> Y;

        /// Map self to another value.
        ///
        /// For a function `f`, return `f(self)`.
        ///
        #[cfg_attr(feature = "extra_traits", doc = "```")]
        #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
        /// #![allow(deprecated)]
        /// use libcommons::prelude::*;
        /// use std::ops::Add;
        ///
        /// fn as_f32(v: i32) -> f32 {
        ///     v as f32
        /// }
        ///
        /// assert_eq!(1i32.add(2i32).instead_with(as_f32), 3.0f32);
        /// ```
        #[deprecated(note = "use `instead_map` instead", since = "0.6.0")]
        fn instead_with<Y, F>(self, value: F) -> Y
        where
            F: FnOnce(Self) -> Y,
            Self: Sized;

        /// Map self to another value.
        ///
        /// For a function `f`, return `f(self)`.
        ///
        #[cfg_attr(feature = "extra_traits", doc = "```")]
        #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
        /// use libcommons::prelude::*;
        /// use std::ops::Add;
        ///
        /// fn as_f32(v: i32) -> f32 {
        ///     v as f32
        /// }
        ///
        /// assert_eq!(1i32.add(2i32).instead_map(as_f32), 3.0f32);
        /// ```
        fn instead_map<Y, F>(self, value: F) -> Y
        where
            F: FnOnce(Self) -> Y,
            Self: Sized;

        /// Modify this value.
        ///
        /// For a function `f`, this calls `f(&mut self)` and returns `self`.
        ///
        /// This method takes ownership of `self`. For a `&mut self` version,
        /// use [Fun::apply_with].
        ///
        #[cfg_attr(feature = "extra_traits", doc = "```")]
        #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
        /// use libcommons::prelude::*;
        /// use std::process::Command;
        ///
        /// let mut command = Command::new("echo");
        /// command.arg("1")
        ///        .apply_with(|x: &mut std::process::Command| {
        ///            if cfg!(target_os = "windows") {
        ///                x.arg("2");
        ///            }
        ///        });
        ///
        /// if cfg!(target_os = "windows") {
        ///     assert_eq!(command.get_args().len(), 2);
        /// } else {
        ///     assert_eq!(command.get_args().len(), 1);
        /// }
        /// ```
        fn apply<F>(self, with: F) -> Self
        where
            F: FnOnce(&mut Self),
            Self: Sized;

        /// Modify this value.
        ///
        /// For a function `f`, this calls `f(&mut self)` and returns `self`.
        ///
        /// Unlike [Fun::apply], this takes a `&mut self`, which might be
        /// preferrable in some cases.
        ///
        #[cfg_attr(feature = "extra_traits", doc = "```")]
        #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
        /// use libcommons::prelude::*;
        /// use std::process::Command;
        ///
        /// let mut command = Command::new("echo");
        /// command.arg("1")
        ///        .apply_with(|x: &mut std::process::Command| {
        ///            if cfg!(target_os = "windows") {
        ///                x.arg("2");
        ///            }
        ///        });
        ///
        /// if cfg!(target_os = "windows") {
        ///     assert_eq!(command.get_args().len(), 2);
        /// } else {
        ///     assert_eq!(command.get_args().len(), 1);
        /// }
        /// ```
        fn apply_with<F>(&mut self, with: F) -> &mut Self
        where
            F: FnOnce(&mut Self);

        /// Map this value.
        ///
        /// For a function `f` that return `Self`, this calls `f(self)`.
        ///
        #[cfg_attr(feature = "extra_traits", doc = "```")]
        #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
        /// use libcommons::prelude::*;
        /// use std::process::Command;
        ///
        /// let command = Command::new("echo")
        ///     .apply_map(|mut x: std::process::Command| {
        ///         x.arg("1");
        ///         if cfg!(target_os = "windows") {
        ///             x.arg("2");
        ///         }
        ///         x
        ///     });
        ///
        /// if cfg!(target_os = "windows") {
        ///     assert_eq!(command.get_args().len(), 2);
        /// } else {
        ///     assert_eq!(command.get_args().len(), 1);
        /// }
        /// ```
        fn apply_map<F>(self, with: F) -> Self
        where
            F: FnOnce(Self) -> Self,
            Self: Sized;

        /// Check whether the other object does equals to self.
        ///
        /// For `x` as an other object, this checks if `x == self`. As such,
        /// `x` must implement [PartialEq] for `self.
        ///
        #[cfg_attr(feature = "extra_traits", doc = "```")]
        #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
        /// use libcommons::prelude::*;
        ///
        /// fn some_funky_api() -> String {
        ///     "this is a string".to_string()
        /// }
        ///
        /// assert!("this is a string".rev_eq(&some_funky_api()));
        /// ```
        fn rev_eq<O>(&self, other: &O) -> bool
        where
            O: PartialEq<Self>;

        /// Check whether the other object does not equal to self.
        ///
        /// For `x` as an other object, this checks if `x != self`. As such,
        /// `x` must implement [PartialEq] for `self.
        ///
        #[cfg_attr(feature = "extra_traits", doc = "```")]
        #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
        /// use libcommons::prelude::*;
        ///
        /// fn some_funky_api() -> String {
        ///     "this is a string".to_string()
        /// }
        ///
        /// assert!("this is another string".rev_ne(&some_funky_api()));
        /// ```
        fn rev_ne<O>(&self, other: &O) -> bool
        where
            O: PartialEq<Self>;

        /// Drop the object.
        ///
        /// This does the same thing as passing `self` to [std::mem::drop].
        ///
        #[cfg_attr(feature = "extra_traits", doc = "```")]
        #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
        /// use libcommons::prelude::*;
        ///
        /// let mut a = Some(Some("hi"));
        /// match &mut a {
        ///     None => (),
        ///     Some(x) => x.take().drop(),
        /// }
        /// assert!(a.is_some_and(|x| x.is_none()));
        /// ```
        fn drop(self);

        /// Forget the object.
        ///
        /// This does the same thing as passing `self` to [std::mem::forget].
        ///
        #[cfg_attr(feature = "extra_traits", doc = "```")]
        #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
        /// use libcommons::prelude::*;
        ///
        /// let mut dropped = false;
        ///
        /// struct S<'a> {
        ///     dropped: &'a mut bool,
        /// }
        /// impl Drop for S<'_> {
        ///     fn drop(&mut self) {
        ///         *self.dropped = true;
        ///     }
        /// }
        ///
        /// let value = S { dropped: &mut dropped };
        /// value.forget();
        /// assert!(!dropped);
        /// ```
        fn forget(self);
    }
    impl<T> Fun for T {
        fn instead<Y>(&self, value: Y) -> Y {
            value
        }

        fn instead_with<Y, F>(self, value: F) -> Y
        where
            F: FnOnce(Self) -> Y,
            Self: Sized,
        {
            value(self)
        }

        fn instead_map<Y, F>(self, value: F) -> Y
        where
            F: FnOnce(Self) -> Y,
            Self: Sized,
        {
            value(self)
        }

        fn apply<F>(mut self, with: F) -> Self
        where
            F: FnOnce(&mut Self),
            Self: Sized,
        {
            with(&mut self);
            self
        }

        fn apply_with<F>(&mut self, with: F) -> &mut Self
        where
            F: FnOnce(&mut Self),
        {
            with(self);
            self
        }

        fn apply_map<F>(self, with: F) -> Self
        where
            F: FnOnce(Self) -> Self,
            Self: Sized,
        {
            with(self)
        }

        fn rev_eq<O>(&self, other: &O) -> bool
        where
            O: PartialEq<Self>,
        {
            other.eq(self)
        }

        fn rev_ne<O>(&self, other: &O) -> bool
        where
            O: PartialEq<Self>,
        {
            other.ne(self)
        }

        fn drop(self) {}

        fn forget(self) {
            std::mem::forget(self);
        }
    }

    /// Extensions for immutable slices.
    #[allow(unused)]
    pub trait SliceExt {
        /// Obtain the underlying bytes of this slice.
        ///
        #[cfg_attr(feature = "extra_traits", doc = "```")]
        #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
        /// use libcommons::util::SliceExt;
        ///
        /// let mut nums = [1i32; 4];
        /// unsafe {
        ///     let num = 1i32.to_ne_bytes();
        ///     for c in nums.as_u8_slice().chunks(4) {
        ///         assert_eq!(c, &num);
        ///     }
        /// }
        /// ```
        ///
        /// ## Safety
        ///
        /// Since Rust objects have undefined layout, this method should not
        /// be relied on unless you really know what you're doing.
        unsafe fn as_u8_slice(&self) -> &[u8];
    }
    impl<T> SliceExt for &[T] {
        unsafe fn as_u8_slice(&self) -> &[u8] { unsafe {
            let range = self.as_ptr_range();
            let start = range.start.cast::<u8>();
            let end = range.end.cast::<u8>();
            std::slice::from_raw_parts(start, start.addr().abs_diff(end.addr()))
        } }
    }

    /// Extensions for mutable slices.
    #[allow(unused)]
    pub trait SliceMutExt: SliceExt {
        /// Obtain the underlying bytes of this slice.
        ///
        #[cfg_attr(feature = "extra_traits", doc = "```")]
        #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
        /// use libcommons::util::SliceMutExt;
        ///
        /// let mut nums = [1i32; 4];
        /// unsafe {
        ///     let num = 2i32.to_ne_bytes();
        ///     for c in nums.as_u8_slice_mut().chunks_mut(4) {
        ///         c.copy_from_slice(&num);
        ///     }
        /// }
        /// for n in nums {
        ///     assert_eq!(n, 2);
        /// }
        /// ```
        ///
        /// ## Safety
        ///
        /// With this method it is really easy to violate the integrity of
        /// an object. Use it carefully.
        unsafe fn as_u8_slice_mut(&mut self) -> &mut [u8];
    }
    impl<T> SliceExt for &mut [T] {
        unsafe fn as_u8_slice(&self) -> &[u8] { unsafe {
            let range = self.as_ptr_range();
            let start = range.start.cast::<u8>();
            let end = range.end.cast::<u8>();
            std::slice::from_raw_parts(start, start.addr().abs_diff(end.addr()))
        } }
    }
    impl<T> SliceMutExt for &mut [T] {
        unsafe fn as_u8_slice_mut(&mut self) -> &mut [u8] { unsafe {
            let range = self.as_mut_ptr_range();
            let start = range.start.cast::<u8>();
            let end = range.end.cast::<u8>();
            std::slice::from_raw_parts_mut(start, start.addr().abs_diff(end.addr()))
        } }
    }

    impl<T> SliceExt for Box<[T]> {
        unsafe fn as_u8_slice(&self) -> &[u8] { unsafe {
            let range = self.as_ptr_range();
            let start = range.start.cast::<u8>();
            let end = range.end.cast::<u8>();
            std::slice::from_raw_parts(start, start.addr().abs_diff(end.addr()))
        } }
    }
    impl<T> SliceMutExt for Box<[T]> {
        unsafe fn as_u8_slice_mut(&mut self) -> &mut [u8] { unsafe {
            let range = self.as_mut_ptr_range();
            let start = range.start.cast::<u8>();
            let end = range.end.cast::<u8>();
            std::slice::from_raw_parts_mut(start, start.addr().abs_diff(end.addr()))
        } }
    }

    impl<T, const LEN: usize> SliceExt for [T; LEN] {
        unsafe fn as_u8_slice(&self) -> &[u8] { unsafe {
            let range = self.as_ptr_range();
            let start = range.start.cast::<u8>();
            let end = range.end.cast::<u8>();
            std::slice::from_raw_parts(start, start.addr().abs_diff(end.addr()))
        } }
    }
    impl<T, const LEN: usize> SliceMutExt for [T; LEN] {
        unsafe fn as_u8_slice_mut(&mut self) -> &mut [u8] { unsafe {
            let range = self.as_mut_ptr_range();
            let start = range.start.cast::<u8>();
            let end = range.end.cast::<u8>();
            std::slice::from_raw_parts_mut(start, start.addr().abs_diff(end.addr()))
        } }
    }

    #[allow(unused)]
    pub trait ResultExt<T, E> {
        /// Deflate error.
        ///
        /// Selects for an error via a filter, if the error matches the
        /// filter, returns [Err]. Otherwise, the original value wrapped
        /// in [Ok]. This function is the opposite of [std::result::Result::flatten].
        ///
        #[cfg_attr(feature = "extra_traits", doc = "```")]
        #[cfg_attr(not(feature = "extra_traits"), doc = "```ignore")]
        /// use libcommons::util::ResultExt;
        ///
        /// #[derive(Debug, PartialEq)]
        /// enum ErrTypes {
        ///     One,
        ///     Two,
        /// }
        ///
        /// type R = Result<Result<bool, ErrTypes>, ErrTypes>;
        ///
        /// let ohno1: R = Err(ErrTypes::One).inflate(|x| Some(ErrTypes::One).take_if(|_| x == &ErrTypes::Two));
        /// let ohno2: R = Err(ErrTypes::Two).inflate(|x| Some(ErrTypes::One).take_if(|_| x == &ErrTypes::Two));
        /// let coool: R = Ok(true).inflate(|x| Some(ErrTypes::One).take_if(|_| x == &ErrTypes::Two));
        ///
        /// assert_eq!(ohno1, Ok(Err(ErrTypes::One)));
        /// assert_eq!(ohno2, Err(ErrTypes::One));
        /// assert_eq!(coool, Ok(Ok(true)));
        /// ```
        fn inflate<F>(self, filter: F) -> Result<Result<T, E>, E>
        where
            F: FnMut(&E) -> Option<E>;
    }
    impl<T, E> ResultExt<T, E> for std::result::Result<T, E> {
        fn inflate<F>(self, mut filter: F) -> Result<Result<T, E>, E>
        where
            F: FnMut(&E) -> Option<E>,
        {
            match self {
                Self::Ok(x) => Ok(Ok(x)),
                Self::Err(x) => {
                    if let Some(x) = filter(&x) {
                        Err(x)
                    } else {
                        Ok(Err(x))
                    }
                }
            }
        }
    }
}

#[cfg(feature = "extra_traits")]
pub use __inner::*;
#[allow(unused)]
#[cfg(not(feature = "extra_traits"))]
pub(crate) use __inner::*;

#[cfg(feature = "result")]
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
#[cfg(feature = "result")]
pub type Result<T = (), E = BoxError> = std::result::Result<T, E>;
#[cfg(feature = "result")]
pub const K: Result = Ok(());

/// Enforce static type checking at the point.
///
/// Used to force type coersion or to only specify a return-position generic parameter.
#[must_use]
#[inline(always)]
pub const fn t<T>(t: T) -> T { t }

/// Use `Anc` without actually requiring it.
///
/// This is identical to `(PhantomData<Anc>, T)`, but with more explicit
/// semantics.
///
/// This type is marked with `#[repr(transparent)]`, so it's completely safe
/// to 
///
/// ```rust
/// use libcommons::util::Use;
///
/// pub struct StuffDo<T> {
///     inner: T,
/// }
///
/// impl<R, T: Fn() -> R> StuffDo<Use<R, T>> {
///     pub fn new(inner: T) -> Self {
///         Self {
///             inner: Use::new(inner),
///         }
///     }
///     pub fn do_thing(&self) -> R {
///         (self.inner)()
///     }
/// }
/// ```
#[repr(transparent)]
pub struct Use<Anc, T> {
    value: T,
    _phantom: PhantomData<Anc>,
}
impl<Anc, T> Use<Anc, T> {
    #[must_use]
    #[inline(always)]
    pub fn new(value: T) -> Self { Self { value, _phantom: PhantomData } }

    #[must_use]
    #[inline(always)]
    pub fn unwrap(self) -> T { self.value }
}
impl<Anc, T> AsRef<T> for Use<Anc, T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}
impl<Anc, T> AsMut<T> for Use<Anc, T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
impl<Anc, T> Borrow<T> for Use<Anc, T> {
    fn borrow(&self) -> &T {
        &self.value
    }
}
impl<Anc, T> BorrowMut<T> for Use<Anc, T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
impl<Anc, T: fmt::Display> fmt::Display for Use<Anc, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}
impl<Anc, T: fmt::Debug> fmt::Debug for Use<Anc, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(&self.value, f)
    }
}
impl<Anc, T: Clone> Clone for Use<Anc, T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), _phantom: PhantomData }
    }
}
impl<Anc, T: Hash> Hash for Use<Anc, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}
impl<Anc, T: PartialOrd<T>> PartialOrd for Use<Anc, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        T::partial_cmp(self.borrow(), other.borrow())
    }
    fn lt(&self, other: &Self) -> bool {
        T::lt(self.borrow(), other.borrow())
    }
    fn le(&self, other: &Self) -> bool {
        T::le(self.borrow(), other.borrow())
    }
    fn gt(&self, other: &Self) -> bool {
        T::gt(self.borrow(), other.borrow())
    }
    fn ge(&self, other: &Self) -> bool {
        T::ge(self.borrow(), other.borrow())
    }
}
impl<Anc, T: PartialEq<T>> PartialEq for Use<Anc, T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<Anc, T: PartialEq<T>> PartialEq<T> for Use<Anc, T> {
    fn eq(&self, other: &T) -> bool {
        self.value == *other
    }
}
impl<Anc, T> Deref for Use<Anc, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<Anc, T> DerefMut for Use<Anc, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

/// Whether two byte arrays contain the same data.
///
/// Works in `const fn`s.
pub const fn same_bytes(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false }
    let mut i = 0usize;
    while i < a.len() {
        if a[i] != b[i] { return false }
        i += 1;
    }
    true
}

pub fn zeroed<T: Zeroable>() -> T { T::zeroed() }

/// Mark object as being safe to call [zeroed][std::mem::zeroed] on.
///
/// ## Safety
///
/// This trait must only be implemented for types for which zeroed state is
/// valid. Not satisfying this constraint could lead to undefined behavior.
pub unsafe trait Zeroable {
    /// Create a zeroed object.
    #[must_use]
    fn zeroed() -> Self
    where Self: Sized
    { unsafe { std::mem::zeroed() } }
}
macro_rules! zeroable_zero {
    ($($ty:ty),* $(,)?) => {$(
        unsafe impl Zeroable for $ty {}
    )*};
}
unsafe impl<T> Zeroable for MaybeUninit<T> {}
zeroable_zero! {
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128,
    char, bool,
}

/// Create a new zeroed buffer.
#[must_use]
pub fn new_buffer_zeroed<T: Zeroable>(len: usize) -> Box<[T]> { unsafe {
    let mut vec = Vec::<T>::new();
    vec.reserve_exact(len);
    vec.set_len(len);
    let range = vec.as_ptr_range();
    let start = range.start.cast::<u8>().cast_mut();
    let end = range.end.cast::<u8>().cast_mut();
    std::slice::from_raw_parts_mut(start, start.addr().abs_diff(end.addr())).fill(0);
    vec.into_boxed_slice()
} }
/// Create a new buffer.
///
/// Since [MaybeUninit] is zeroable, it allows for any type.
#[must_use]
pub fn new_buffer<T>(len: usize) -> Box<[MaybeUninit<T>]> { new_buffer_zeroed(len) }
/// Create a new buffer.
///
/// ## Safety
///
/// The caller must ensure that the 
#[must_use]
pub unsafe fn new_buffer_assume_init<T>(len: usize) -> Box<[T]> { unsafe { new_buffer(len).assume_init() } }
