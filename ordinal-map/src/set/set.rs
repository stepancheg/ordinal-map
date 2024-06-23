use std::marker::PhantomData;

use crate::Ordinal;

/// Iterator over elements of [`Set`].
pub struct Iter<'a, T> {
    iter: crate::Iter<T>,
    set: SliceSet<'a, T>,
}

impl<'a, T: Ordinal> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next()?;
            if self.set.contains(&next) {
                return Some(next);
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.iter.len()))
    }
}

impl<'a, T: Ordinal> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next_back()?;
            if self.set.contains(&next) {
                return Some(next);
            }
        }
    }
}

enum SetImpl {
    Small(u64),
    Large(Box<[u64]>),
}

impl SetImpl {}

struct SliceSet<'a, T> {
    words: &'a [u64],
    _phantom: PhantomData<T>,
}

impl<'a, T: Ordinal> SliceSet<'a, T> {
    #[inline]
    fn contains(&self, ordinal: &T) -> bool {
        let Some(word) = self.words.get(ordinal.ordinal() / u64::BITS as usize) else {
            return false;
        };
        word & (1 << (ordinal.ordinal() % u64::BITS as usize)) != 0
    }
}

/// Default set implementation.
///
/// All operations are constant time
/// (provided that [`T::ordinal()`](Ordinal::ordinal) is constant time).
///
/// This map allocates memory when the number of elements is greater than 64.
/// When the number of elements is known to be less than or equal to 64,
/// consider using [`Set64`](crate::set::Set64) instead.
pub struct Set<T: Ordinal> {
    set: SetImpl,
    _phantom: PhantomData<T>,
}

impl<T: Ordinal> Set<T> {
    const IS_SMALL: bool = T::ORDINAL_SIZE <= u64::BITS as usize;

    /// Create a new empty set.
    #[inline]
    pub fn new() -> Self {
        Set {
            set: if Self::IS_SMALL {
                SetImpl::Small(0)
            } else {
                SetImpl::Large(Vec::<u64>::new().into_boxed_slice())
            },
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn slice_set(&self) -> SliceSet<T> {
        match (Self::IS_SMALL, &self.set) {
            (true, SetImpl::Small(set)) => SliceSet {
                words: std::slice::from_ref(set),
                _phantom: PhantomData,
            },
            (false, SetImpl::Large(set)) => SliceSet {
                words: set,
                _phantom: PhantomData,
            },
            _ => unreachable!(),
        }
    }

    /// Check if the set contains an element.
    #[inline]
    pub fn contains(&self, ordinal: &T) -> bool {
        self.slice_set().contains(ordinal)
    }

    /// Insert an element into the set, returning `true` if the element was not already present.
    #[inline]
    pub fn insert(&mut self, ordinal: T) -> bool {
        let r = !self.contains(&ordinal);
        match (Self::IS_SMALL, &mut self.set) {
            (true, SetImpl::Small(set)) => {
                *set |= 1 << ordinal.ordinal();
            }
            (false, SetImpl::Large(set)) => {
                if set.is_empty() {
                    *set = vec![
                        0;
                        (T::ORDINAL_SIZE.checked_add(u64::BITS as usize).unwrap() - 1)
                            / u64::BITS as usize
                    ]
                    .into_boxed_slice();
                }
                set[ordinal.ordinal() / u64::BITS as usize] |=
                    1 << (ordinal.ordinal() % u64::BITS as usize);
            }
            _ => unreachable!(),
        }
        r
    }

    /// Iterate over the elements of the set.
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter {
            iter: crate::Iter::<T>::new(),
            set: self.slice_set(),
        }
    }
}

impl<T: Ordinal> Default for Set<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ordinal> FromIterator<T> for Set<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Set::new();
        for ordinal in iter {
            set.insert(ordinal);
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::num::NonZeroU16;

    use crate::set::Set;
    use crate::tests::util::Example4;

    #[quickcheck]
    fn qc_insert_small(values: Vec<Example4>, check: Vec<Example4>) {
        let mut set: Set<Example4> = Set::new();
        let mut control: HashSet<Example4> = HashSet::new();
        for value in &values {
            let control_inserted = control.insert(*value);
            let inserted = set.insert(*value);
            assert_eq!(control_inserted, inserted);
        }

        for value in &check {
            assert_eq!(control.contains(value), set.contains(value));
        }
    }

    #[quickcheck]
    fn qc_insert_large(values: Vec<NonZeroU16>, check: Vec<NonZeroU16>) {
        let mut set: Set<NonZeroU16> = Set::new();
        let mut control: HashSet<NonZeroU16> = HashSet::new();
        for value in &values {
            let control_inserted = control.insert(*value);
            let inserted = set.insert(*value);
            assert_eq!(control_inserted, inserted);
        }

        for value in &check {
            assert_eq!(control.contains(value), set.contains(value));
        }
    }

    #[quickcheck]
    fn qc_iter_small(mut values: Vec<Example4>) -> bool {
        let set = Set::from_iter(values.iter().copied());
        values.sort();
        values.dedup();
        set.iter().collect::<Vec<_>>() == values
    }

    #[quickcheck]
    fn qc_iter_large(mut values: Vec<NonZeroU16>) -> bool {
        let set = Set::from_iter(values.iter().copied());
        values.sort();
        values.dedup();
        set.iter().collect::<Vec<_>>() == values
    }
}
