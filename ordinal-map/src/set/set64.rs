use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::slice;

use crate::set::set_ref::OrdinalSetRef;
use crate::Ordinal;

/// Iterator over [`OrdinalSet64`].
pub struct Iter64<T> {
    set: u64,
    _phantom: PhantomData<T>,
}

impl<T: Ordinal> Iterator for Iter64<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let ordinal = self.set.trailing_zeros();
        if ordinal == u64::BITS {
            None
        } else {
            self.set &= !(1 << ordinal);
            Some(T::from_ordinal(ordinal as usize).unwrap())
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Ordinal> ExactSizeIterator for Iter64<T> {
    #[inline]
    fn len(&self) -> usize {
        self.set.count_ones() as usize
    }
}

impl<T: Ordinal> DoubleEndedIterator for Iter64<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let ordinal = (u64::BITS - 1).checked_sub(self.set.leading_zeros())?;
        self.set &= !(1 << ordinal);
        Some(T::from_ordinal(ordinal as usize).unwrap())
    }
}

impl<T> Clone for Iter64<T> {
    #[inline]
    fn clone(&self) -> Self {
        Iter64 {
            set: self.set,
            _phantom: PhantomData,
        }
    }
}

impl<T: Ordinal + Debug> Debug for Iter64<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

/// Set for implementations of [`Ordinal`](crate::Ordinal) with a maximum ordinal size of 64.
///
/// This is implemented using a single `u64` value to store the set of elements.
/// To store set of arbitrary size, consider using [`OrdinalSet`](crate::set::OrdinalSet).
#[derive(Eq, PartialEq)]
pub struct OrdinalSet64<T> {
    set: u64,
    _phantom: PhantomData<T>,
}

impl<T: Ordinal> OrdinalSet64<T> {
    const ASSERT: () = {
        assert!(T::ORDINAL_SIZE <= u64::BITS as usize);
    };

    /// Create a new empty set.
    #[inline]
    pub const fn new() -> Self {
        const { Self::ASSERT };
        OrdinalSet64 {
            set: 0,
            _phantom: PhantomData,
        }
    }

    fn as_ref(&self) -> OrdinalSetRef<'_, T> {
        OrdinalSetRef::new(slice::from_ref(&self.set))
    }

    /// Create a set containing all possible elements of [`K`](Ordinal).
    #[inline]
    pub fn all() -> Self {
        const { Self::ASSERT };
        OrdinalSet64 {
            set: (1 << T::ORDINAL_SIZE) - 1,
            _phantom: PhantomData,
        }
    }

    /// Insert an element into the set, returning `true` if the element was not already present.
    #[inline]
    pub fn insert(&mut self, ordinal: T) -> bool {
        const { Self::ASSERT };
        let r = !self.contains(&ordinal);
        self.set |= 1 << ordinal.ordinal();
        r
    }

    /// Iterate over the elements of the set.
    #[inline]
    pub fn iter(&self) -> Iter64<T> {
        const { Self::ASSERT };
        Iter64 {
            set: self.set,
            _phantom: PhantomData,
        }
    }

    /// Check if the set contains an element.
    #[inline]
    pub fn contains(&self, ordinal: &T) -> bool {
        const { Self::ASSERT };
        self.set & (1 << ordinal.ordinal()) != 0
    }
}

impl<T: Ordinal> Default for OrdinalSet64<T> {
    fn default() -> Self {
        OrdinalSet64::new()
    }
}

impl<T: Ordinal> FromIterator<T> for OrdinalSet64<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = OrdinalSet64::new();
        for ordinal in iter {
            set.set |= 1 << ordinal.ordinal();
        }
        set
    }
}

impl<T> Clone for OrdinalSet64<T> {
    fn clone(&self) -> Self {
        OrdinalSet64 {
            set: self.set,
            _phantom: PhantomData,
        }
    }
}

impl<T: Ordinal + Debug> Debug for OrdinalSet64<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.as_ref(), f)
    }
}

#[cfg(test)]
mod tests {
    use crate::set::OrdinalSet64;
    use crate::tests::util::test_exact_size_iterator;
    use crate::tests::util::Example4;

    // Fails at compilation time (as expected).
    // #[test]
    // fn test_more_than_64() {
    //     Set64::<u8>::new();
    // }

    #[test]
    fn test_all() {
        let set = OrdinalSet64::<Example4>::all();
        assert_eq!(set.set, 0b1111);
    }

    #[quickcheck]
    fn qc_iterator(mut values: Vec<Example4>) -> bool {
        let set = OrdinalSet64::from_iter(values.iter().copied());
        values.sort();
        values.dedup();
        set.iter().collect::<Vec<_>>() == values
    }

    #[quickcheck]
    fn qc_double_ended_iterator(mut values: Vec<Example4>) -> bool {
        let set = OrdinalSet64::from_iter(values.iter().copied());
        values.sort();
        values.dedup();
        set.iter().rev().collect::<Vec<_>>() == values.into_iter().rev().collect::<Vec<_>>()
    }

    #[quickcheck]
    fn qc_exact_size_iterator(values: Vec<Example4>) {
        test_exact_size_iterator(OrdinalSet64::from_iter(values).iter());
    }
}
