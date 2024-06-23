use std::array;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::slice;
use std::vec;

use crate::map::enumerate::OrdinalEnumerate;
use crate::Ordinal;

/// Iterator created from
/// [`InitMap`](crate::map::OrdinalInitMap) and
/// [`InitArrayMap`](crate::map::OrdinalInitArrayMap).
pub struct InitIter<'a, K, V> {
    iter: OrdinalEnumerate<K, slice::Iter<'a, V>>,
}

impl<'a, K, V> InitIter<'a, K, V> {
    pub(crate) fn new(iter: slice::Iter<'a, V>, next: usize) -> Self {
        InitIter {
            iter: OrdinalEnumerate::new(iter, next),
        }
    }
}

impl<'a, K: Ordinal, V> Iterator for InitIter<'a, K, V> {
    type Item = (K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K: Ordinal, V> ExactSizeIterator for InitIter<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for InitIter<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K, V> Clone for InitIter<'a, K, V> {
    fn clone(&self) -> Self {
        InitIter {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, K: Ordinal + Debug, V: Debug> Debug for InitIter<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// Mutable iterator created from
/// [`InitMap`](crate::map::OrdinalInitMap) and
/// [`InitArrayMap`](crate::map::OrdinalInitArrayMap).
pub struct InitIterMut<'a, K, V> {
    iter: OrdinalEnumerate<K, slice::IterMut<'a, V>>,
}

impl<'a, K: Ordinal, V> InitIterMut<'a, K, V> {
    #[inline]
    pub(crate) fn new(iter: slice::IterMut<'a, V>) -> Self {
        InitIterMut {
            iter: OrdinalEnumerate::new(iter, 0),
        }
    }

    pub(crate) fn iter(&self) -> InitIter<K, V> {
        InitIter::new(self.iter.iter.as_slice().iter(), self.iter.next)
    }
}

impl<'a, K: Ordinal, V> Iterator for InitIterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K: Ordinal, V> ExactSizeIterator for InitIterMut<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for InitIterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<'a, K: Ordinal + Debug, V: Debug> Debug for InitIterMut<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

pub struct InitIntoIterArray<K, V, const S: usize> {
    iter: OrdinalEnumerate<K, array::IntoIter<V, S>>,
}

impl<K, V, const S: usize> InitIntoIterArray<K, V, S> {
    #[inline]
    pub(crate) fn new(iter: array::IntoIter<V, S>) -> Self {
        InitIntoIterArray {
            iter: OrdinalEnumerate::new(iter, 0),
        }
    }

    pub(crate) fn iter(&self) -> InitIter<K, V> {
        InitIter::new(self.iter.iter.as_slice().iter(), self.iter.next)
    }
}

impl<K: Ordinal, V, const S: usize> Iterator for InitIntoIterArray<K, V, S> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<K: Ordinal, V, const S: usize> ExactSizeIterator for InitIntoIterArray<K, V, S> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K: Ordinal, V, const S: usize> DoubleEndedIterator for InitIntoIterArray<K, V, S> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<K, V: Clone, const S: usize> Clone for InitIntoIterArray<K, V, S> {
    fn clone(&self) -> Self {
        InitIntoIterArray {
            iter: self.iter.clone(),
        }
    }
}

impl<K: Ordinal + Debug, V: Debug, const S: usize> Debug for InitIntoIterArray<K, V, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// Iterator created from [`InitMap`](crate::map::OrdinalInitMap).
pub struct InitIntoIter<K, V> {
    iter: OrdinalEnumerate<K, vec::IntoIter<V>>,
}

impl<K, V> InitIntoIter<K, V> {
    #[inline]
    pub(crate) fn new(iter: vec::IntoIter<V>) -> Self {
        InitIntoIter {
            iter: OrdinalEnumerate::new(iter, 0),
        }
    }

    pub(crate) fn iter(&self) -> InitIter<K, V> {
        InitIter::new(self.iter.iter.as_slice().iter(), self.iter.next)
    }
}

impl<K: Ordinal, V> Iterator for InitIntoIter<K, V> {
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

impl<K: Ordinal, V> ExactSizeIterator for InitIntoIter<K, V> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K: Ordinal, V> DoubleEndedIterator for InitIntoIter<K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<K: Ordinal + Debug, V: Debug> Debug for InitIntoIter<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<K, V: Clone> Clone for InitIntoIter<K, V> {
    fn clone(&self) -> Self {
        InitIntoIter {
            iter: self.iter.clone(),
        }
    }
}
