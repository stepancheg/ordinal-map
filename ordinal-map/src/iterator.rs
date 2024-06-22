use std::marker::PhantomData;
use std::ops::Range;

use crate::Ordinal;

pub struct Iter<T> {
    range: Range<usize>,
    _phantom: PhantomData<T>,
}

impl<T: Ordinal> Iter<T> {
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
        T::from_ordinal(next)
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
