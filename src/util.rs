//! Utilities.
//!
//! This is where things like box error and functional traits go.

/// One-liner tools to make code look nicer.
///
/// i.e. instead of
/// ```
/// let mut a = Some(Some("hi"));
/// match &mut a {
///     None => (),
///     Some(x) => drop(x.take()),
/// }
/// ```
/// you can do
/// ```
/// use libcommons::prelude::*;
///
/// let mut a = Some(Some("hi"));
/// match &mut a {
///     None => (),
///     Some(x) => x.take().drop(),
/// }
/// ```
#[cfg(feature = "extra_traits")]
pub trait Fun {
    /// Drop self and return another value.
    ///
    /// For a function `f`, return `f(self)`.
    ///
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
#[cfg(feature = "extra_traits")]
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

#[cfg(feature = "extra_traits")]
pub trait ResultExt<T, E> {
    /// Deflate error.
    ///
    /// Selects for an error via a filter, if the error matches the
    /// filter, returns [Err]. Otherwise, the original value wrapped
    /// in [Ok]. This function is the opposite of [std::result::Result::flatten].
    ///
    /// ```
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
#[cfg(feature = "extra_traits")]
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

#[cfg(feature = "result")]
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
#[cfg(feature = "result")]
pub type Result<T = (), E = BoxError> = std::result::Result<T, E>;
#[cfg(feature = "result")]
pub const K: Result = Ok(());
