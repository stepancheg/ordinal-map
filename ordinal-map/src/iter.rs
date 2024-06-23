use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
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

impl<T: Ordinal + Debug> Debug for Iter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut iter = self.clone();
        let Some(first) = iter.next() else {
            return write!(f, "[]");
        };
        if self.range.end == T::ORDINAL_SIZE {
            write!(f, "[{:?}..]", first)
        } else {
            let Some(last) = iter.next_back() else {
                return write!(f, "[{:?}]", first);
            };
            write!(f, "[{:?}..={:?}]", first, last)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_iter_debug() {
        let mut iter = crate::Iter::<u8>::new();
        assert_eq!("[0..]", format!("{:?}", iter));
        iter.next().unwrap();
        assert_eq!("[1..]", format!("{:?}", iter));
        iter.next_back().unwrap();
        assert_eq!("[1..=254]", format!("{:?}", iter));
        assert_eq!(254, iter.len());
        iter.nth(252).unwrap();
        assert_eq!("[254]", format!("{:?}", iter));
    }
}
