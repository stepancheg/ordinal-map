use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::slice;

use crate::set::iter::Iter;
use crate::set::set_mut::OrdinalSetMut;
use crate::set::set_ref::OrdinalSetRef;
use crate::Ordinal;

#[derive(Clone)]
enum SetImpl {
    Small(u64),
    Large(Box<[u64]>),
}

/// Default set implementation.
///
/// All operations are constant time
/// (provided that [`T::ordinal()`](Ordinal::ordinal) is constant time).
///
/// This map allocates memory when the number of elements is greater than 64.
/// When the number of elements is known to be less than or equal to 64,
/// consider using [`Set64`](crate::set::OrdinalSet64) instead.
pub struct OrdinalSet<T> {
    set: SetImpl,
    _phantom: PhantomData<T>,
}

impl<T: Ordinal> OrdinalSet<T> {
    const IS_SMALL: bool = T::ORDINAL_SIZE <= u64::BITS as usize;

    /// Create a new empty set.
    #[inline]
    pub fn new() -> Self {
        OrdinalSet {
            set: if Self::IS_SMALL {
                SetImpl::Small(0)
            } else {
                SetImpl::Large(Vec::<u64>::new().into_boxed_slice())
            },
            _phantom: PhantomData,
        }
    }

    #[inline]
    fn as_ref(&self) -> OrdinalSetRef<T> {
        match (Self::IS_SMALL, &self.set) {
            (true, SetImpl::Small(set)) => OrdinalSetRef::new(slice::from_ref(set)),
            (false, SetImpl::Large(set)) => OrdinalSetRef::new(set),
            _ => unreachable!(),
        }
    }

    /// Check if the set contains an element.
    #[inline]
    pub fn contains(&self, ordinal: &T) -> bool {
        self.as_ref().contains(ordinal)
    }

    /// Insert an element into the set, returning `true` if the element was not already present.
    #[inline]
    pub fn insert(&mut self, ordinal: T) -> bool {
        match (Self::IS_SMALL, &mut self.set) {
            (true, SetImpl::Small(set)) => OrdinalSetMut::new(slice::from_mut(set)).insert(ordinal),
            (false, SetImpl::Large(set)) => {
                if set.is_empty() {
                    *set = vec![
                        0;
                        (T::ORDINAL_SIZE.checked_add(u64::BITS as usize).unwrap() - 1)
                            / u64::BITS as usize
                    ]
                    .into_boxed_slice();
                }
                OrdinalSetMut::new(set).insert(ordinal)
            }
            _ => unreachable!(),
        }
    }

    /// Iterate over the elements of the set.
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        self.as_ref().iter()
    }
}

impl<T: Ordinal> Default for OrdinalSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ordinal> FromIterator<T> for OrdinalSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = OrdinalSet::new();
        for ordinal in iter {
            set.insert(ordinal);
        }
        set
    }
}

impl<T> Clone for OrdinalSet<T> {
    fn clone(&self) -> Self {
        OrdinalSet {
            set: self.set.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T: Ordinal + Debug> Debug for OrdinalSet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.as_ref(), f)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::num::NonZeroU16;

    use crate::set::OrdinalSet;
    use crate::tests::util::Example4;

    #[quickcheck]
    fn qc_insert_small(values: Vec<Example4>, check: Vec<Example4>) {
        let mut set: OrdinalSet<Example4> = OrdinalSet::new();
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
        let mut set: OrdinalSet<NonZeroU16> = OrdinalSet::new();
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
        let set = OrdinalSet::from_iter(values.iter().copied());
        values.sort();
        values.dedup();
        set.iter().collect::<Vec<_>>() == values
    }

    #[quickcheck]
    fn qc_iter_large(mut values: Vec<NonZeroU16>) -> bool {
        let set = OrdinalSet::from_iter(values.iter().copied());
        values.sort();
        values.dedup();
        set.iter().collect::<Vec<_>>() == values
    }
}
