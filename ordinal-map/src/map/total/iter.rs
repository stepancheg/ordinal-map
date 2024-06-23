use std::array;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::slice;
use std::vec;

use crate::map::enumerate::OrdinalEnumerate;
use crate::Ordinal;

/// Iterator created from
/// [`InitMap`](crate::map::OrdinalTotalMap) and
/// [`InitArrayMap`](crate::map::OrdinalTotalArrayMap).
pub struct TotalIter<'a, K, V> {
    iter: OrdinalEnumerate<K, slice::Iter<'a, V>>,
}

impl<'a, K, V> TotalIter<'a, K, V> {
    pub(crate) fn new(iter: slice::Iter<'a, V>, next: usize) -> Self {
        TotalIter {
            iter: OrdinalEnumerate::new(iter, next),
        }
    }
}

impl<'a, K: Ordinal, V> Iterator for TotalIter<'a, K, V> {
    type Item = (K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K: Ordinal, V> ExactSizeIterator for TotalIter<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for TotalIter<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> Clone for TotalIter<'a, K, V> {
    fn clone(&self) -> Self {
        TotalIter {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, K: Ordinal + Debug, V: Debug> Debug for TotalIter<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// Mutable iterator created from
/// [`InitMap`](crate::map::OrdinalTotalMap) and
/// [`InitArrayMap`](crate::map::OrdinalTotalArrayMap).
pub struct TotalIterMut<'a, K, V> {
    iter: OrdinalEnumerate<K, slice::IterMut<'a, V>>,
}

impl<'a, K: Ordinal, V> TotalIterMut<'a, K, V> {
    #[inline]
    pub(crate) fn new(iter: slice::IterMut<'a, V>) -> Self {
        TotalIterMut {
            iter: OrdinalEnumerate::new(iter, 0),
        }
    }

    pub(crate) fn iter(&self) -> TotalIter<K, V> {
        TotalIter::new(self.iter.iter.as_slice().iter(), self.iter.next)
    }
}

impl<'a, K: Ordinal, V> Iterator for TotalIterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K: Ordinal, V> ExactSizeIterator for TotalIterMut<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for TotalIterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K: Ordinal + Debug, V: Debug> Debug for TotalIterMut<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

pub struct TotalIntoIterArray<K, V, const S: usize> {
    iter: OrdinalEnumerate<K, array::IntoIter<V, S>>,
}

impl<K, V, const S: usize> TotalIntoIterArray<K, V, S> {
    #[inline]
    pub(crate) fn new(iter: array::IntoIter<V, S>) -> Self {
        TotalIntoIterArray {
            iter: OrdinalEnumerate::new(iter, 0),
        }
    }

    pub(crate) fn iter(&self) -> TotalIter<K, V> {
        TotalIter::new(self.iter.iter.as_slice().iter(), self.iter.next)
    }
}

impl<K: Ordinal, V, const S: usize> Iterator for TotalIntoIterArray<K, V, S> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<K: Ordinal, V, const S: usize> ExactSizeIterator for TotalIntoIterArray<K, V, S> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K: Ordinal, V, const S: usize> DoubleEndedIterator for TotalIntoIterArray<K, V, S> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<K, V: Clone, const S: usize> Clone for TotalIntoIterArray<K, V, S> {
    fn clone(&self) -> Self {
        TotalIntoIterArray {
            iter: self.iter.clone(),
        }
    }
}

impl<K: Ordinal + Debug, V: Debug, const S: usize> Debug for TotalIntoIterArray<K, V, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// Iterator created from [`InitMap`](crate::map::OrdinalTotalMap).
pub struct TotalIntoIter<K, V> {
    iter: OrdinalEnumerate<K, vec::IntoIter<V>>,
}

impl<K, V> TotalIntoIter<K, V> {
    #[inline]
    pub(crate) fn new(iter: vec::IntoIter<V>) -> Self {
        TotalIntoIter {
            iter: OrdinalEnumerate::new(iter, 0),
        }
    }

    pub(crate) fn iter(&self) -> TotalIter<K, V> {
        TotalIter::new(self.iter.iter.as_slice().iter(), self.iter.next)
    }
}

impl<K: Ordinal, V> Iterator for TotalIntoIter<K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<K: Ordinal, V> ExactSizeIterator for TotalIntoIter<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K: Ordinal, V> DoubleEndedIterator for TotalIntoIter<K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<K: Ordinal + Debug, V: Debug> Debug for TotalIntoIter<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<K, V: Clone> Clone for TotalIntoIter<K, V> {
    fn clone(&self) -> Self {
        TotalIntoIter {
            iter: self.iter.clone(),
        }
    }
}
