use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::mem;

use crate::map::total;
use crate::Ordinal;

/// Iterator over the entries of
/// [`OrdinalMap`](crate::map::OrdinalMap) and [`OrdinalArrayMap`](crate::map::OrdinalArrayMap).
pub struct Iter<'a, K, V> {
    iter: total::Iter<'a, K, Option<V>>,
}

impl<'a, K, V> Iter<'a, K, V> {
    #[inline]
    pub(crate) fn new(iter: total::Iter<'a, K, Option<V>>) -> Self {
        Iter { iter }
    }
}

impl<'a, K: Ordinal, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next()?;
            if let (k, Some(v)) = next {
                return Some((k, v));
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.iter.len()))
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for Iter<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next_back()?;
            if let (k, Some(v)) = next {
                return Some((k, v));
            }
        }
    }
}

impl<'a, K, V> Clone for Iter<'a, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Iter {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, K: Ordinal + Debug, V: Debug> Debug for Iter<'a, K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// Iterator over mutable references to the entries of
/// [`OrdinalMap`](crate::map::OrdinalMap) and [`OrdinalArrayMap`](crate::map::OrdinalArrayMap).
pub struct IterMut<'a, K, V> {
    iter: total::IterMut<'a, K, Option<V>>,
}

impl<'a, K: Ordinal, V> IterMut<'a, K, V> {
    #[inline]
    pub(crate) fn new(iter: total::IterMut<'a, K, Option<V>>) -> Self {
        IterMut { iter }
    }

    fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(self.iter.iter())
    }
}

impl<'a, K: Ordinal, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next()?;
            if let (k, Some(v)) = next {
                return Some((k, v));
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.iter.len()))
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for IterMut<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next_back()?;
            if let (k, Some(v)) = next {
                return Some((k, v));
            }
        }
    }
}

impl<'a, K: Ordinal + Debug, V: Debug> Debug for IterMut<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// Iterator over the keys of [`OrdinalMap`](crate::map::OrdinalMap)
/// and [`OrdinalArrayMap`](crate::map::OrdinalArrayMap).
pub struct Keys<'a, K, V> {
    iter: Iter<'a, K, V>,
}

impl<'a, K: Ordinal, V> Keys<'a, K, V> {
    #[inline]
    pub(crate) fn new(iter: Iter<'a, K, V>) -> Self {
        Keys { iter }
    }
}

impl<'a, K: Ordinal, V> Iterator for Keys<'a, K, V> {
    type Item = K;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for Keys<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|(k, _)| k)
    }
}

impl<'a, K: Ordinal, V> Clone for Keys<'a, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Keys {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, K: Ordinal + Debug> Debug for Keys<'a, K, ()> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// Iterator over the values of [`OrdinalMap`](crate::map::OrdinalMap)
/// and [`OrdinalArrayMap`](crate::map::OrdinalArrayMap).
pub struct Values<'a, K, V> {
    iter: Iter<'a, K, V>,
}

impl<'a, K: Ordinal, V> Values<'a, K, V> {
    #[inline]
    pub(crate) fn new(iter: Iter<'a, K, V>) -> Self {
        Values { iter }
    }
}

impl<'a, K: Ordinal, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for Values<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|(_, v)| v)
    }
}

impl<'a, K: Ordinal, V> Clone for Values<'a, K, V> {
    #[inline]
    fn clone(&self) -> Self {
        Values {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, K: Ordinal + Debug, V: Debug> Debug for Values<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// Iterator over mutable references to the values of
/// [`OrdinalMap`](crate::map::OrdinalMap) and [`OrdinalArrayMap`](crate::map::OrdinalArrayMap).
pub struct ValuesMut<'a, K, V> {
    iter: IterMut<'a, K, V>,
}

impl<'a, K: Ordinal, V> ValuesMut<'a, K, V> {
    #[inline]
    pub(crate) fn new(iter: IterMut<'a, K, V>) -> Self {
        ValuesMut { iter }
    }

    fn iter(&self) -> Values<'_, K, V> {
        Values {
            iter: self.iter.iter(),
        }
    }
}

impl<'a, K: Ordinal, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for ValuesMut<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|(_, v)| v)
    }
}

impl<'a, K: Ordinal + Debug, V: Debug> Debug for ValuesMut<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// Iterator that removes all key-value pairs from [`OrdinalMap`](crate::map::OrdinalMap)
/// or [`OrdinalArrayMap`](crate::map::OrdinalArrayMap).
pub struct Drain<'a, K: Ordinal, V> {
    iter: total::IterMut<'a, K, Option<V>>,
}

impl<'a, K: Ordinal, V> Drain<'a, K, V> {
    pub(crate) fn new(iter: total::IterMut<'a, K, Option<V>>) -> Self {
        Drain { iter }
    }
}

impl<'a, K: Ordinal, V> Drop for Drain<'a, K, V> {
    fn drop(&mut self) {
        for _ in self {}
    }
}

impl<'a, K: Ordinal, V> Iterator for Drain<'a, K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (k, v) = self.iter.next()?;
            if let Some(v) = mem::take(v) {
                return Some((k, v));
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.iter.len()))
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for Drain<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            let (k, v) = self.iter.next_back()?;
            if let Some(v) = mem::take(v) {
                return Some((k, v));
            }
        }
    }
}

/// Iterator created from [`OrdinalInitArrayMap`](total::OrdinalTotalArrayMap).
pub struct IntoIterArray<K, V, const S: usize> {
    iter: total::IntoIterArray<K, Option<V>, S>,
    _phantom: PhantomData<K>,
}

impl<K: Ordinal, V, const S: usize> IntoIterArray<K, V, S> {
    #[inline]
    pub(crate) fn new(iter: total::IntoIterArray<K, Option<V>, S>) -> Self {
        IntoIterArray {
            iter,
            _phantom: PhantomData,
        }
    }

    fn iter(&self) -> Iter<K, V> {
        Iter::new(self.iter.iter())
    }
}

impl<K: Ordinal, V, const S: usize> Iterator for IntoIterArray<K, V, S> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (k, v) = self.iter.next()?;
            if let Some(v) = v {
                return Some((k, v));
            }
        }
    }
}

impl<K: Ordinal, V, const S: usize> DoubleEndedIterator for IntoIterArray<K, V, S> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            let (k, v) = self.iter.next_back()?;
            if let Some(v) = v {
                return Some((k, v));
            }
        }
    }
}

impl<K: Ordinal + Debug, V: Debug, const S: usize> Debug for IntoIterArray<K, V, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<K, V: Clone, const S: usize> Clone for IntoIterArray<K, V, S> {
    fn clone(&self) -> Self {
        IntoIterArray {
            iter: self.iter.clone(),
            _phantom: PhantomData,
        }
    }
}

/// Iterator created from [`OrdinalMap`](crate::map::OrdinalMap).
pub struct IntoIter<K, V> {
    iter: total::IntoIter<K, Option<V>>,
}

impl<K, V> IntoIter<K, V> {
    pub(crate) fn new(iter: total::IntoIter<K, Option<V>>) -> Self {
        IntoIter { iter }
    }

    fn iter(&self) -> Iter<K, V> {
        Iter::new(self.iter.iter())
    }
}

impl<K: Ordinal, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (k, v) = self.iter.next()?;
            if let Some(v) = v {
                return Some((k, v));
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.iter.len()))
    }
}

impl<K: Ordinal, V> DoubleEndedIterator for IntoIter<K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            let (k, v) = self.iter.next_back()?;
            if let Some(v) = v {
                return Some((k, v));
            }
        }
    }
}

impl<K: Ordinal + Debug, V: Debug> Debug for IntoIter<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
