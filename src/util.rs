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
    /// Drop self.
    fn drop(self);
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

    fn drop(self) {}
}
