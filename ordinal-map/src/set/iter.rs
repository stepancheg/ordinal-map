use std::fmt::Debug;

use crate::set::set_ref::OrdinalSetRef;
use crate::Ordinal;

/// Iterator over elements of [`OrdinalSet`](crate::set::OrdinalSet)
/// or [`OrdinalArraySet`](crate::set::OrdinalArraySet).
pub struct Iter<'a, T> {
    iter: crate::OrdinalValues<T>,
    set: OrdinalSetRef<'a, T>,
}

impl<'a, T: Ordinal> Iter<'a, T> {
    #[inline]
    pub(crate) fn new(set: OrdinalSetRef<'a, T>) -> Self {
        Iter {
            iter: T::all_values(),
            set,
        }
    }
}

impl<'a, T: Ordinal> Iterator for Iter<'a, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next()?;
            if self.set.contains(&next) {
                return Some(next);
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.iter.len()))
    }
}

impl<'a, T: Ordinal> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next_back()?;
            if self.set.contains(&next) {
                return Some(next);
            }
        }
    }
}

impl<'a, T> Clone for Iter<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        Iter {
            iter: self.iter.clone(),
            set: self.set.clone(),
        }
    }
}

impl<'a, T: Ordinal + Debug> Debug for Iter<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}
