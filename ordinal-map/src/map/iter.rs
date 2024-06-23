use crate::map::InitIter;
use crate::map::InitIterMut;
use crate::Ordinal;

pub struct Iter<'a, K, V> {
    iter: InitIter<'a, K, Option<V>>,
}

impl<'a, K: Ordinal, V> Iter<'a, K, V> {
    pub(crate) fn new(iter: InitIter<'a, K, Option<V>>) -> Self {
        Iter { iter }
    }
}

impl<'a, K: Ordinal, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next()?;
            if let (k, Some(v)) = next {
                return Some((k, v));
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.iter.len()))
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next_back()?;
            if let (k, Some(v)) = next {
                return Some((k, v));
            }
        }
    }
}

pub struct IterMut<'a, K, V> {
    iter: InitIterMut<'a, K, Option<V>>,
}

impl<'a, K: Ordinal, V> IterMut<'a, K, V> {
    pub(crate) fn new(iter: InitIterMut<'a, K, Option<V>>) -> Self {
        IterMut { iter }
    }
}

impl<'a, K: Ordinal, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next()?;
            if let (k, Some(v)) = next {
                return Some((k, v));
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.iter.len()))
    }
}

impl<'a, K: Ordinal, V> DoubleEndedIterator for IterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next_back()?;
            if let (k, Some(v)) = next {
                return Some((k, v));
            }
        }
    }
}
