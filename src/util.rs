#[cfg(feature = "extra_traits")]
pub trait Fun {
    /// Use value instead of self.
    fn instead<Y>(&self, value: Y) -> Y;
    /// Map self to value.
    fn instead_with<Y, F>(self, value: F) -> Y
    where
        F: FnOnce(Self) -> Y,
        Self: Sized;
    /// Apply a function to self.
    fn apply<F>(self, with: F) -> Self
    where
        F: FnOnce(&mut Self),
        Self: Sized;
    /// Move and apply a function to self.
    fn apply_with<F>(&mut self, with: F) -> &mut Self
    where
        F: FnOnce(&mut Self);
    /// Move and map self.
    fn apply_map<F>(self, with: F) -> Self
    where
        F: FnOnce(Self) -> Self,
        Self: Sized;
    /// Reverse [Eq].
    fn rev_eq<O>(&self, other: &O) -> bool
    where
        O: PartialEq<Self>;
    /// Reverse [Eq].
    fn rev_ne<O>(&self, other: &O) -> bool
    where
        O: PartialEq<Self>;
    /// Drop self.
    fn drop(self);
    /// Forget self.
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

#[cfg(feature = "result")]
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
#[cfg(feature = "result")]
pub type Result<T = (), E = BoxError> = std::result::Result<T, E>;
#[cfg(feature = "result")]
pub const K: Result = Ok(());
