use std::{borrow::Borrow, fmt::Debug, hint::black_box, mem::MaybeUninit, ops::{Deref, DerefMut}};

/// Validate an object.
///
/// Invalid objects are that existence of which violates its logical
/// contraints, i.e. having an object of a non-zero type with a value
/// of zero.
///
/// Implementors of [Drop] and [Validate] should not rely on [Validate::is_valid]
/// in the implementation of [Drop::drop] as those objects are not
/// supposed to be created normally, much less be dropped.
///
/// ## Safety
///
/// While this trait may be implemented for any type, it must be implemented in a way
/// that in safe code [is_valid][Validate::is_valid] always returns `true`. `false` is
/// only ever returned if an object's state was violated. By
/// [create_invalid][HasInvalidStates::create_invalid] or similar methods.
pub unsafe trait Validate {
    /// Validate this object.
    ///
    /// Returns `true` if the object is valid and `false` if it isn't.
    ///
    /// Invalid objects are that existence of which violates its logical
    /// contraints, i.e. having an object of a non-zero type with a value
    /// of zero.
    ///
    /// Implementors of [Drop] and [Validate] should not rely on [Validate::is_valid]
    /// in the implementation of [Drop::drop] as those objects are not
    /// supposed to be created normally, much less be dropped.
    fn is_valid(&self) -> bool;
}
/// Whether an object has no invalid states.
///
/// This guaranteed that you can allocate an array of the size of
/// this object, fill it with random bytes, transmute it, and it
/// will never create an invalid object, and that calling
/// [Validate::is_valid] on it will always return `true`.
///
/// This trait is implemented for all primitive types.
///
/// ## Safety
///
/// This trait must only be implemented for types for which this constraint
/// applies. Implementing both [NoInvalidStates] and [HasInvalidStates] is unsound.
pub unsafe trait NoInvalidStates {}
unsafe impl<T: ?Sized + NoInvalidStates> Validate for T {
    fn is_valid(&self) -> bool { true }
}
/// Whether an object any invalid states.
///
/// This trait is implemented for all non-zero types and references,
/// as well as some convenient wrappers like [AlwaysSome].
///
/// ## Safety
///
/// This trait must be implemented following the constraints of [Validate].
///
/// Implementing both [NoInvalidStates] and [HasInvalidStates] is unsound.
pub unsafe trait HasInvalidStates: Validate {
    /// Create an invalid instance.
    ///
    /// For this object, [Validate::is_valid] is guaranteed to return
    /// `false`.
    ///
    /// ## Safety
    ///
    /// Invalid objects must be handled with caution. Not doing so may
    /// easily lead to undefined behavior.
    unsafe fn create_invalid() -> MaybeUninit<Self>
    where Self: Sized;
}

/// An [Option] that is always [Some].
///
/// Not really useful for anything as this type is equivalent to `T`.
///
/// Implements [Validate].
#[derive(Debug, PartialEq, Eq)]
pub struct AlwaysSome<T>(Option<T>);
impl<T> AlwaysSome<T> {
    /// Create a new [AlwaysSome] from [Option].
    ///
    /// ## Const
    ///
    /// This function is not `const` as Rust cannot evaludate `Drop`
    /// for an `Option` in a const fn.
    pub fn new(opt: Option<T>) -> Option<AlwaysSome<T>> { opt.map(|x| AlwaysSome(Some(x))) }
    /// Create a new [AlwaysSome] from `T`.
    pub const fn new_some(v: T) -> Self { Self(Some(v)) }
}
impl<T> Borrow<Option<T>> for AlwaysSome<T> {
    fn borrow(&self) -> &Option<T> {
        &self.0
    }
}
impl<T> AsRef<Option<T>> for AlwaysSome<T> {
    fn as_ref(&self) -> &Option<T> {
        &self.0
    }
}
impl<T> AsRef<T> for AlwaysSome<T> {
    fn as_ref(&self) -> &T {
        self.0.as_ref().unwrap()
    }
}
impl<T> AsMut<T> for AlwaysSome<T> {
    fn as_mut(&mut self) -> &mut T {
        self.0.as_mut().unwrap()
    }
}
impl<T> Deref for AlwaysSome<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}
impl<T> DerefMut for AlwaysSome<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap()
    }
}
unsafe impl<T> Validate for AlwaysSome<T> {
    fn is_valid(&self) -> bool {
        black_box(&self.0).is_some()
    }
}
unsafe impl<T> HasInvalidStates for AlwaysSome<T> {
    unsafe fn create_invalid() -> MaybeUninit<Self>
    where Self: Sized
    {
        MaybeUninit::new(AlwaysSome(None))
    }
}
