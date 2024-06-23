use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::slice;

use crate::Ordinal;

/// Iterator created from
/// [`InitMap`](crate::map::OrdinalInitMap) and
/// [`InitArrayMap`](crate::map::OrdinalInitArrayMap).
pub struct InitIter<'a, K, V> {
    iter: slice::Iter<'a, V>,
    next: usize,
    _phantom: PhantomData<K>,
}

impl<'a, K, V> InitIter<'a, K, V> {
    pub(crate) fn new(iter: slice::Iter<'a, V>) -> Self {
        InitIter {
            iter,
            next: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, K: Ordinal, V> Iterator for InitIter<'a, K, V> {
    type Item = (K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let v = self.iter.next()?;
        let k = K::from_ordinal(self.next).unwrap();
        self.next += 1;
        Some((k, v))
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
        let v = self.iter.next_back()?;
        let k = K::from_ordinal(self.iter.len()).unwrap();
        Some((k, v))
    }
}

impl<'a, K, V> Clone for InitIter<'a, K, V> {
    fn clone(&self) -> Self {
        InitIter {
            iter: self.iter.clone(),
            next: self.next,
            _phantom: PhantomData,
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
    iter: slice::IterMut<'a, V>,
    next: usize,
    _phantom: PhantomData<K>,
}

impl<'a, K, V> InitIterMut<'a, K, V> {
    #[inline]
    pub(crate) fn new(iter: slice::IterMut<'a, V>) -> Self {
        InitIterMut {
            iter,
            next: 0,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn iter(&self) -> InitIter<K, V> {
        InitIter {
            iter: self.iter.as_slice().iter(),
            next: self.next,
            _phantom: PhantomData,
        }
    }
}

impl<'a, K: Ordinal, V> Iterator for InitIterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.iter.next()?;
        let k = K::from_ordinal(self.next).unwrap();
        self.next += 1;
        Some((k, v))
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
        let v = self.iter.next_back()?;
        let k = K::from_ordinal(self.iter.len()).unwrap();
        Some((k, v))
    }
}

impl<'a, K: Ordinal + Debug, V: Debug> Debug for InitIterMut<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
