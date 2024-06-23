use std::marker::PhantomData;
use std::ops::Range;

use crate::Ordinal;

/// Iterate over all values of an ordinal.
pub struct Iter<T> {
    range: Range<usize>,
    _phantom: PhantomData<T>,
}

impl<T: Ordinal> Iter<T> {
    /// Create a new iterator.
    #[inline]
    pub fn new() -> Self {
        Iter {
            range: 0..T::ORDINAL_SIZE,
            _phantom: PhantomData,
        }
    }
}

impl<T: Ordinal> Iterator for Iter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.range.next()?;
        Some(T::from_ordinal(next).unwrap())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<T: Ordinal> ExactSizeIterator for Iter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl<T: Ordinal> DoubleEndedIterator for Iter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let next = self.range.next_back()?;
        Some(T::from_ordinal(next).unwrap())
    }
}

impl<T> Clone for Iter<T> {
    #[inline]
    fn clone(&self) -> Self {
        Iter {
            range: self.range.clone(),
            _phantom: PhantomData,
        }
    }
}
