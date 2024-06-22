use std::marker::PhantomData;

use crate::Ordinal;

/// Iterator over [`Set64`].
pub struct Iter64<T> {
    set: u64,
    _phantom: PhantomData<T>,
}

impl<T: Ordinal> Iterator for Iter64<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let ordinal = self.set.trailing_zeros();
        if ordinal == u64::BITS {
            None
        } else {
            self.set &= !(1 << ordinal);
            Some(T::from_ordinal(ordinal as usize).unwrap())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Ordinal> ExactSizeIterator for Iter64<T> {
    fn len(&self) -> usize {
        self.set.count_ones() as usize
    }
}

impl<T: Ordinal> DoubleEndedIterator for Iter64<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let ordinal = (u64::BITS - 1).checked_sub(self.set.leading_zeros())?;
        self.set &= !(1 << ordinal);
        Some(T::from_ordinal(ordinal as usize).unwrap())
    }
}

/// Set for implementations of [`Ordinal`](crate::Ordinal) with a maximum ordinal size of 64.
///
/// This is implemented using a single `u64` bitset.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Set64<T: Ordinal> {
    set: u64,
    _phantom: PhantomData<T>,
}

impl<T: Ordinal> Set64<T> {
    const ASSERT: () = {
        assert!(T::ORDINAL_SIZE <= u64::BITS as usize);
    };

    #[inline]
    pub fn new() -> Self {
        const { Self::ASSERT };
        Set64 {
            set: 0,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn insert(&mut self, ordinal: &T) -> bool {
        const { Self::ASSERT };
        let r = !self.contains(ordinal);
        self.set |= 1 << ordinal.ordinal();
        r
    }

    #[inline]
    pub fn all() -> Self {
        const { Self::ASSERT };
        Set64 {
            set: (1 << T::ORDINAL_SIZE) - 1,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter64<T> {
        const { Self::ASSERT };
        Iter64 {
            set: self.set,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn contains(&self, ordinal: &T) -> bool {
        const { Self::ASSERT };
        self.set & (1 << ordinal.ordinal()) != 0
    }
}

impl<T: Ordinal> FromIterator<T> for Set64<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Set64::new();
        for ordinal in iter {
            set.set |= 1 << ordinal.ordinal();
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use crate::set::Set64;
    use crate::tests::util::test_exact_size_iterator;
    use crate::tests::util::Example4;

    // Fails at compilation time (as expected).
    // #[test]
    // fn test_more_than_64() {
    //     Set64::<u8>::new();
    // }

    #[test]
    fn test_all() {
        let set = Set64::<Example4>::all();
        assert_eq!(set.set, 0b1111);
    }

    #[quickcheck]
    fn qc_iterator(mut values: Vec<Example4>) -> bool {
        let set = Set64::from_iter(values.iter().copied());
        values.sort();
        values.dedup();
        set.iter().collect::<Vec<_>>() == values
    }

    #[quickcheck]
    fn qc_double_ended_iterator(mut values: Vec<Example4>) -> bool {
        let set = Set64::from_iter(values.iter().copied());
        values.sort();
        values.dedup();
        set.iter().rev().collect::<Vec<_>>() == values.into_iter().rev().collect::<Vec<_>>()
    }

    #[quickcheck]
    fn qc_exact_size_iterator(values: Vec<Example4>) {
        test_exact_size_iterator(Set64::from_iter(values).iter());
    }
}
