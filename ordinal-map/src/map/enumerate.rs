use std::marker::PhantomData;

use crate::Ordinal;

pub(crate) struct OrdinalEnumerate<K, I> {
    pub(crate) next: usize,
    pub(crate) iter: I,
    _phantom: PhantomData<K>,
}

impl<K, I> OrdinalEnumerate<K, I> {
    pub(crate) fn new(iter: I, next: usize) -> Self {
        OrdinalEnumerate {
            next,
            iter,
            _phantom: PhantomData,
        }
    }
}

impl<K: Ordinal, I: Iterator> Iterator for OrdinalEnumerate<K, I> {
    type Item = (K, I::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let v = self.iter.next()?;
        let k = K::from_ordinal(self.next).unwrap();
        self.next += 1;
        Some((k, v))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<K: Ordinal, I: ExactSizeIterator> ExactSizeIterator for OrdinalEnumerate<K, I> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K: Ordinal, I: ExactSizeIterator + DoubleEndedIterator> DoubleEndedIterator
    for OrdinalEnumerate<K, I>
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let v = self.iter.next_back()?;
        let k = K::from_ordinal(self.next + self.iter.len()).unwrap();
        Some((k, v))
    }
}

impl<K, I: Clone> Clone for OrdinalEnumerate<K, I> {
    #[inline]
    fn clone(&self) -> Self {
        OrdinalEnumerate {
            next: self.next,
            iter: self.iter.clone(),
            _phantom: PhantomData,
        }
    }
}
