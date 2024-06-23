use std::marker::PhantomData;

use crate::set::set_ref::OrdinalSetRef;
use crate::Ordinal;

pub(crate) struct OrdinalSetMut<'a, T> {
    words: &'a mut [u64],
    _phantom: PhantomData<T>,
}

impl<'a, T: Ordinal> OrdinalSetMut<'a, T> {
    #[inline]
    pub(crate) fn new(words: &'a mut [u64]) -> Self {
        debug_assert!(words.len() == (T::ORDINAL_SIZE + 63) / 64);
        OrdinalSetMut {
            words,
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn as_ref(&self) -> OrdinalSetRef<T> {
        OrdinalSetRef::new(&self.words)
    }

    #[inline]
    pub(crate) fn insert(&mut self, value: T) -> bool {
        let prev = self.as_ref().contains(&value);
        self.words[value.ordinal() / u64::BITS as usize] |=
            1 << (value.ordinal() % u64::BITS as usize);
        !prev
    }
}
