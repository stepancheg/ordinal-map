use std::iter;
use std::marker::PhantomData;
use std::slice;

use crate::Ordinal;

/// Iterator created from
/// [`InitMap`](crate::map::OrdinalInitMap) and
/// [`InitArrayMap`](crate::map::OrdinalInitArrayMap).
pub struct InitIter<'a, K, V> {
    iter: iter::Enumerate<slice::Iter<'a, V>>,
    _phantom: PhantomData<K>,
}

impl<'a, K, V> InitIter<'a, K, V> {
    pub(crate) fn new(iter: iter::Enumerate<slice::Iter<'a, V>>) -> Self {
        InitIter {
            iter,
            _phantom: PhantomData,
        }
    }
}

impl<'a, K: Ordinal, V> Iterator for InitIter<'a, K, V> {
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(i, v)| (K::from_ordinal(i).unwrap(), v))
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
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .map(|(i, v)| (K::from_ordinal(i).unwrap(), v))
    }
}

/// Mutable iterator created from
/// [`InitMap`](crate::map::OrdinalInitMap) and
/// [`InitArrayMap`](crate::map::OrdinalInitArrayMap).
pub struct InitIterMut<'a, K, V> {
    iter: iter::Enumerate<slice::IterMut<'a, V>>,
    _phantom: PhantomData<K>,
}

impl<'a, K, V> InitIterMut<'a, K, V> {
    pub(crate) fn new(iter: iter::Enumerate<slice::IterMut<'a, V>>) -> Self {
        InitIterMut {
            iter,
            _phantom: PhantomData,
        }
    }
}

impl<'a, K: Ordinal, V> Iterator for InitIterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(i, v)| (K::from_ordinal(i).unwrap(), v))
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
        self.iter
            .next_back()
            .map(|(i, v)| (K::from_ordinal(i).unwrap(), v))
    }
}
