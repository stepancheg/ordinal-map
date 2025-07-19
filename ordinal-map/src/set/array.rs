use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::set::set_mut::OrdinalSetMut;
use crate::set::set_ref::OrdinalSetRef;
use crate::Ordinal;

/// Compute the size of `S` parameter for [`OrdinalArraySet`](crate::set::OrdinalArraySet).
/// This is simply `(T::ORDINAL_SIZE + 63) / 64`.
pub const fn ordinal_array_set_s<T: Ordinal>() -> usize {
    T::ORDINAL_SIZE.div_ceil(u64::BITS as usize)
}

/// Set of ordinals implemented as an array of words.
///
/// # Size parameter
///
/// Parameter `S` must be explicitly specified as
/// [`ordinal_array_set_s::<T>()`](ordinal_array_set_s)
/// due to limitations of const generics in stable Rust.
///
/// If this is not convenient, consider using:
/// - [`OrdinalSet64`](crate::set::OrdinalSet64) for types where `T::ORDINAL_SIZE <= 64`.
/// - [`OrdinalSet`](crate::set::OrdinalSet) which allocates storage dynamically.
///
/// # Example
///
/// ```
/// use ordinal_map::set::ordinal_array_set_s;
/// use ordinal_map::set::OrdinalArraySet;
/// let mut set: OrdinalArraySet<u8, { ordinal_array_set_s::<u8>() }> = OrdinalArraySet::new();
///
/// set.insert(17);
/// ```
pub struct OrdinalArraySet<T, const S: usize> {
    words: [u64; S],
    _phantom: PhantomData<T>,
}

impl<T: Ordinal, const S: usize> OrdinalArraySet<T, S> {
    const ASSERT: () = assert!(S == ordinal_array_set_s::<T>());

    /// Create a new empty set.
    #[inline]
    pub fn new() -> Self {
        const { Self::ASSERT };
        OrdinalArraySet::default()
    }

    #[inline]
    fn as_ref(&self) -> OrdinalSetRef<T> {
        const { Self::ASSERT };
        OrdinalSetRef::new(&self.words)
    }

    #[inline]
    fn as_mut(&mut self) -> OrdinalSetMut<T> {
        const { Self::ASSERT };
        OrdinalSetMut::new(&mut self.words)
    }

    /// Check if the set contains an ordinal.
    #[inline]
    pub fn contains(&self, ordinal: &T) -> bool {
        self.as_ref().contains(ordinal)
    }

    /// Insert an ordinal into the set, returning `true` if the ordinal was not already present.
    #[inline]
    pub fn insert(&mut self, ordinal: T) -> bool {
        self.as_mut().insert(ordinal)
    }
}

impl<T, const S: usize> Default for OrdinalArraySet<T, S> {
    #[inline]
    fn default() -> Self {
        OrdinalArraySet {
            words: [0; S],
            _phantom: PhantomData,
        }
    }
}

impl<T: Ordinal, const S: usize> FromIterator<T> for OrdinalArraySet<T, S> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = OrdinalArraySet::new();
        for value in iter {
            set.insert(value);
        }
        set
    }
}

impl<T, const S: usize> Clone for OrdinalArraySet<T, S> {
    fn clone(&self) -> Self {
        OrdinalArraySet {
            words: self.words,
            _phantom: PhantomData,
        }
    }
}

impl<T: Ordinal + Debug, const S: usize> Debug for OrdinalArraySet<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.as_ref(), f)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::set::array::ordinal_array_set_s;
    use crate::set::OrdinalArraySet;

    #[quickcheck]
    fn qc_insert(values: Vec<i8>, check: Vec<i8>) {
        let mut control: HashSet<i8> = HashSet::new();
        let mut set: OrdinalArraySet<i8, { ordinal_array_set_s::<i8>() }> = OrdinalArraySet::new();

        for &value in &values {
            assert_eq!(control.insert(value), set.insert(value));
        }

        for &value in &check {
            assert_eq!(control.contains(&value), set.contains(&value));
        }
    }
}
