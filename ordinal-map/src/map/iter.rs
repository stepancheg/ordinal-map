use std::fmt::Debug;
use std::fmt::Formatter;

use crate::map::InitIter;
use crate::map::InitIterMut;
use crate::Ordinal;

/// Iterator over the entries of
/// [`OrdinalMap`](crate::map::OrdinalMap) and [`OrdinalArrayMap`](crate::map::OrdinalArrayMap).
pub struct Iter<'a, K, V> {
    iter: InitIter<'a, K, Option<V>>,
}

impl<'a, K: Ordinal, V> Iter<'a, K, V> {
    #[inline]
    pub(crate) fn new(iter: InitIter<'a, K, Option<V>>) -> Self {
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
    iter: InitIterMut<'a, K, Option<V>>,
}

impl<'a, K: Ordinal, V> IterMut<'a, K, V> {
    #[inline]
    pub(crate) fn new(iter: InitIterMut<'a, K, Option<V>>) -> Self {
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
