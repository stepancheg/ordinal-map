use std::array;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::slice;
use std::vec;

use crate::map::enumerate::OrdinalEnumerate;
use crate::Ordinal;

/// Iterator created from
/// [`InitMap`](crate::map::total::OrdinalTotalMap) and
/// [`InitArrayMap`](crate::map::total::OrdinalTotalArrayMap).
pub struct Iter<'a, K, V> {
    iter: OrdinalEnumerate<K, slice::Iter<'a, V>>,
}

impl<'a, K, V> Iter<'a, K, V> {
    pub(crate) fn new(iter: slice::Iter<'a, V>, next: usize) -> Self {
        Iter {
            iter: OrdinalEnumerate::new(iter, next),
        }
    }
}

impl<'a, K: Ordinal, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K: Ordinal, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for Iter<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> Clone for Iter<'a, K, V> {
    fn clone(&self) -> Self {
        Iter {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, K: Ordinal + Debug, V: Debug> Debug for Iter<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// Mutable iterator created from
/// [`InitMap`](crate::map::total::OrdinalTotalMap) and
/// [`InitArrayMap`](crate::map::total::OrdinalTotalArrayMap).
pub struct IterMut<'a, K, V> {
    iter: OrdinalEnumerate<K, slice::IterMut<'a, V>>,
}

impl<'a, K: Ordinal, V> IterMut<'a, K, V> {
    #[inline]
    pub(crate) fn new(iter: slice::IterMut<'a, V>) -> Self {
        IterMut {
            iter: OrdinalEnumerate::new(iter, 0),
        }
    }

    pub(crate) fn iter(&self) -> Iter<K, V> {
        Iter::new(self.iter.iter.as_slice().iter(), self.iter.next)
    }
}

impl<'a, K: Ordinal, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K: Ordinal, V> ExactSizeIterator for IterMut<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for IterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K: Ordinal + Debug, V: Debug> Debug for IterMut<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// Iterator created from [`TotalArrayMap`](crate::map::total::OrdinalTotalArrayMap).
pub struct IntoIterArray<K, V, const S: usize> {
    iter: OrdinalEnumerate<K, array::IntoIter<V, S>>,
}

impl<K, V, const S: usize> IntoIterArray<K, V, S> {
    #[inline]
    pub(crate) fn new(iter: array::IntoIter<V, S>) -> Self {
        IntoIterArray {
            iter: OrdinalEnumerate::new(iter, 0),
        }
    }

    pub(crate) fn iter(&self) -> Iter<K, V> {
        Iter::new(self.iter.iter.as_slice().iter(), self.iter.next)
    }
}

impl<K: Ordinal, V, const S: usize> Iterator for IntoIterArray<K, V, S> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<K: Ordinal, V, const S: usize> ExactSizeIterator for IntoIterArray<K, V, S> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K: Ordinal, V, const S: usize> DoubleEndedIterator for IntoIterArray<K, V, S> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<K, V: Clone, const S: usize> Clone for IntoIterArray<K, V, S> {
    fn clone(&self) -> Self {
        IntoIterArray {
            iter: self.iter.clone(),
        }
    }
}

impl<K: Ordinal + Debug, V: Debug, const S: usize> Debug for IntoIterArray<K, V, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// Iterator created from [`InitMap`](crate::map::total::OrdinalTotalMap).
pub struct IntoIter<K, V> {
    iter: OrdinalEnumerate<K, vec::IntoIter<V>>,
}

impl<K, V> IntoIter<K, V> {
    #[inline]
    pub(crate) fn new(iter: vec::IntoIter<V>) -> Self {
        IntoIter {
            iter: OrdinalEnumerate::new(iter, 0),
        }
    }

    pub(crate) fn iter(&self) -> Iter<K, V> {
        Iter::new(self.iter.iter.as_slice().iter(), self.iter.next)
    }
}

impl<K: Ordinal, V> Iterator for IntoIter<K, V> {
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

impl<K: Ordinal, V> ExactSizeIterator for IntoIter<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K: Ordinal, V> DoubleEndedIterator for IntoIter<K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<K: Ordinal + Debug, V: Debug> Debug for IntoIter<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<K, V: Clone> Clone for IntoIter<K, V> {
    fn clone(&self) -> Self {
        IntoIter {
            iter: self.iter.clone(),
        }
    }
}
