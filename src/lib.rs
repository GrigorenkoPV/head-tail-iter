#![no_std]
#![deny(unsafe_code, missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(test)]
mod tests;

use core::iter::FusedIterator;

/// An iterator that splits a slice into its head and tail.
/// Then, the tail into *its* head and tail, and so on.
///
/// Iterator's element type is thus `(&'a T, &'a [T])`.
///
/// To construct the iterator, use [`HeadTailIterator::head_tail_pairs()`].
///
/// # Example
///
/// ```rust
/// # use std::fmt::Write;
/// use head_tail_iter::HeadTailIterator;
///
/// let mut s = String::new();
/// for x in [0, 1, 2, 3].head_tail_pairs() {
///     writeln!(&mut s, "{:?}", x);
/// }
/// assert_eq!(s, "\
/// (0, [1, 2, 3])
/// (1, [2, 3])
/// (2, [3])
/// (3, [])
/// ");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeadTailIter<'a, T> {
    tail: &'a [T],
}

impl<'a, T> Iterator for HeadTailIter<'a, T> {
    type Item = (&'a T, &'a [T]);

    fn next(&mut self) -> Option<Self::Item> {
        let (head, tail) = self.tail.split_first()?;
        self.tail = tail;
        Some((head, tail))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.len();
        (n, Some(n))
    }
    fn count(self) -> usize {
        self.len()
    }
    fn last(self) -> Option<Self::Item> {
        self.tail.last().map(|t| (t, &[] as &[T]))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n > self.tail.len() {
            self.tail = &[];
            None
        } else {
            self.tail = &self.tail[n..];
            self.next()
        }
    }
}

impl<T> FusedIterator for HeadTailIter<'_, T> {}
impl<T> ExactSizeIterator for HeadTailIter<'_, T> {
    fn len(&self) -> usize {
        self.tail.len()
    }
}

/// A trait that allows you to get a [`HeadTailIter`]
/// by calling [`.head_tail_pairs()`](HeadTailIterator::head_tail_pairs`).
pub trait HeadTailIterator {
    /// The type parameter for the returned [`HeadTailIter`].
    type Item;

    /// Create an iterator that yields head & tail of a given slice,
    /// then head & tail of the tail from the previous step,
    /// and so on, until there are no more elements left.
    ///
    /// Note that at the last step
    /// the head is the last element of the initial slice,
    /// and the tail is an empty slice.
    ///
    /// This method is implemented for slices, and thus works for arrays, vectors,
    /// and anything else that [derefs](core::ops::Deref) to a slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use head_tail_iter::HeadTailIterator;
    /// for (head, tail) in vec![3, 2, 1, 0].head_tail_pairs() {
    ///     assert_eq!(*head, tail.len());
    /// }
    /// ```
    #[must_use]
    fn head_tail_pairs(&self) -> HeadTailIter<'_, Self::Item>;
}

impl<T> HeadTailIterator for [T] {
    type Item = T;

    #[inline]
    fn head_tail_pairs(&self) -> HeadTailIter<'_, Self::Item> {
        HeadTailIter { tail: self }
    }
}

impl<'a, T> From<&'a T> for HeadTailIter<'a, <T as HeadTailIterator>::Item>
where
    T: HeadTailIterator + ?Sized,
{
    #[inline]
    fn from(value: &'a T) -> Self {
        value.head_tail_pairs()
    }
}
