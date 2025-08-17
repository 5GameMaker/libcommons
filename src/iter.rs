use std::{
    iter::FusedIterator,
    mem::{MaybeUninit, swap},
};

/// Libcommons iterator extensions.
pub trait IterExt: Iterator {
    fn pre<const LEN: usize>(self) -> PreIter<LEN, Self>
    where
        Self: Sized;
}
impl<I, It> IterExt for I
where
    I: Iterator<Item = It>,
{
    /// Wrap this iterator in [crate::iter::PreIter].
    ///
    /// Prefetches first `LEN` elements. May be useful for
    /// making sure iterators that expect for multiple fetches
    /// to succeed won't randomly fail when they're not supposed to.
    ///
    /// ## Resumable iterators
    /// For iterators that may start returning [Some] after returning
    /// [None], this will act like [std::iter::Fuse] where if the
    /// iterator returns [None], no new elements will be fetched.
    ///
    /// ## 0-lengthed buffer
    /// If `LEN` is 0, this iterator will have no effect and will
    /// simply call [std::iter::Iterator::next] on the iterator.
    fn pre<const LEN: usize>(self) -> PreIter<LEN, Self>
    where
        Self: Sized,
    {
        PreIter::new(self)
    }
}

/// Prefetched iterator.
///
/// Prefetches first `LEN` elements. May be useful for
/// making sure iterators that expect for multiple fetches
/// to succeed won't randomly fail when they're not supposed to.
///
/// ## Resumable iterators
/// For iterators that may start returning [Some] after returning
/// [None], this will act like [std::iter::Fuse] where if the
/// iterator returns [None], no new elements will be fetched.
///
/// This has no effect if `LEN` is 0.
///
/// ## 0-lengthed buffer
/// If `LEN` is 0, this iterator will have no effect and will
/// simply call [std::iter::Iterator::next] on the iterator.
///
/// ```
/// use libcommons::prelude::*;
///
/// struct I(u32);
/// impl I {
///     pub fn num(&self) -> u32 {
///         self.0
///     }
/// }
/// impl Iterator for I {
///     type Item = u32;
///     fn next(&mut self) -> Option<u32> {
///         let n = self.0;
///         self.0 += 1;
///         if self.0 - 1 == 4 {
///             return None;
///         }
///         Some(n)
///     }
/// }
///
/// let mut iter = I(0);
///
/// assert_eq!(iter.next(), Some(0));
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), None);
/// assert_eq!(iter.next(), Some(5));
///
/// let iter = I(0);
/// let mut iter = iter.pre::<3>();
///
/// assert_eq!(iter.next(), Some(0));
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), None);
/// assert_eq!(iter.next(), None);
/// ```
pub struct PreIter<const LEN: usize, I: Iterator + ?Sized> {
    buf: [MaybeUninit<I::Item>; LEN],
    len: usize,
    iter: I,
}
impl<const LEN: usize, I, It> PreIter<LEN, I>
where
    I: Iterator<Item = It>,
{
    /// Create a new buffered iterator.
    ///
    /// `LEN` values will be attempted to be prefetched.
    pub fn new(iter: I) -> Self {
        let mut iter = Self {
            iter,
            buf: [const { MaybeUninit::uninit() }; LEN],
            len: 0,
        };

        for i in 0..LEN {
            match iter.iter.next() {
                Some(x) => {
                    iter.buf[i].write(x);
                    iter.len += 1;
                }
                None => break,
            }
        }

        iter
    }

    /// Obtain a reference to the internal iterator
    pub fn inner_iter(&self) -> &I {
        &self.iter
    }

    /// Obtain a mutable reference to the internal iterator
    pub fn inner_iter_mut(&mut self) -> &mut I {
        &mut self.iter
    }
}
impl<const LEN: usize, I, It> Iterator for PreIter<LEN, I>
where
    I: Iterator<Item = It> + ?Sized,
{
    type Item = It;

    fn next(&mut self) -> Option<Self::Item> {
        if LEN == 0 {
            return self.iter.next();
        }

        if self.len == 0 {
            return None;
        }

        let fetch = self.len == LEN;

        let item = unsafe {
            let mut iter = MaybeUninit::uninit();
            swap(&mut iter, &mut self.buf[0]);
            for i in 0..LEN - 1 {
                self.buf.swap(i, i + 1);
            }
            self.len -= 1;
            iter.assume_init()
        };

        if fetch {
            if let Some(x) = self.iter.next() {
                self.buf.last_mut().unwrap().write(x);
                self.len += 1;
            }
        }

        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.iter.size_hint();
        (hint.0 + self.len, hint.1.map(|x| x + self.len))
    }
}
impl<const LEN: usize, I: Iterator + ?Sized> FusedIterator for PreIter<LEN, I> {}
impl<const LEN: usize, I: Iterator + ?Sized + ExactSizeIterator> ExactSizeIterator
    for PreIter<LEN, I>
{
    fn len(&self) -> usize {
        self.len + self.iter.len()
    }
}
impl<const LEN: usize, I: Iterator + Clone> Clone for PreIter<LEN, I>
where
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        Self {
            len: self.len,
            iter: self.iter.clone(),
            buf: unsafe {
                let mut buf = [const { MaybeUninit::uninit() }; LEN];
                for i in 0..self.len {
                    buf[i].write(self.buf[i].assume_init_ref().clone());
                }
                buf
            },
        }
    }
}
impl<const LEN: usize, I: Iterator + Copy> Copy for PreIter<LEN, I> where I::Item: Copy {}
