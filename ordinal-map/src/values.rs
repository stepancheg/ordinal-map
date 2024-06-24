use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::ops::Range;

use crate::Ordinal;

/// Iterator over [`Ordinal`] values.
///
/// This iterator is created by [`Ordinal::all_values`](Ordinal::all_values).
///
/// # Example
///
/// ```
/// use ordinal_map::Ordinal;
///
/// assert_eq!(vec![false, true], bool::all_values().collect::<Vec<_>>());
/// ```
pub struct OrdinalValues<T> {
    range: Range<usize>,
    _phantom: PhantomData<T>,
}

impl<T: Ordinal> OrdinalValues<T> {
    /// Create a new iterator.
    #[inline]
    pub(crate) fn new() -> Self {
        OrdinalValues {
            range: 0..T::ORDINAL_SIZE,
            _phantom: PhantomData,
        }
    }
}

impl<T: Ordinal> Iterator for OrdinalValues<T> {
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

impl<T: Ordinal> ExactSizeIterator for OrdinalValues<T> {
    #[inline]
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl<T: Ordinal> DoubleEndedIterator for OrdinalValues<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let next = self.range.next_back()?;
        Some(T::from_ordinal(next).unwrap())
    }
}

impl<T> Clone for OrdinalValues<T> {
    #[inline]
    fn clone(&self) -> Self {
        OrdinalValues {
            range: self.range.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T: Ordinal + Debug> Debug for OrdinalValues<T> {
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
    use crate::Ordinal;

    #[test]
    fn test_iter_debug() {
        let mut iter = u8::all_values();
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
