use std::marker::PhantomData;

use crate::set::Iter;
use crate::Ordinal;

pub(crate) struct OrdinalSetRef<'a, T> {
    words: &'a [u64],
    _phantom: PhantomData<T>,
}

impl<'a, T: Ordinal> OrdinalSetRef<'a, T> {
    #[inline]
    pub(crate) fn iter(self) -> Iter<'a, T> {
        Iter::new(self)
    }

    #[inline]
    pub(crate) fn new(words: &'a [u64]) -> Self {
        debug_assert!(words.len() == 0 || words.len() == (T::ORDINAL_SIZE + 63) / 64);
        OrdinalSetRef {
            words,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn contains(&self, ordinal: &T) -> bool {
        let Some(word) = self.words.get(ordinal.ordinal() / u64::BITS as usize) else {
            return false;
        };
        word & (1 << (ordinal.ordinal() % u64::BITS as usize)) != 0
    }
}

impl<'a, T> Clone for OrdinalSetRef<'a, T> {
    fn clone(&self) -> Self {
        OrdinalSetRef {
            words: self.words,
            _phantom: PhantomData,
        }
    }
}
