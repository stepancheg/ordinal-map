use std::marker::PhantomData;

use crate::Ordinal;

enum SetImpl {
    Small(u64),
    Large(Box<[u64]>),
}

impl SetImpl {}

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

    /// Check if the set contains an element.
    #[inline]
    pub fn contains(&self, ordinal: &T) -> bool {
        match (Self::IS_SMALL, &self.set) {
            (true, SetImpl::Small(set)) => set & (1 << ordinal.ordinal()) != 0,
            (false, SetImpl::Large(set)) => {
                let Some(bits) = set.get(ordinal.ordinal() / u64::BITS as usize) else {
                    return false;
                };
                bits & (1 << (ordinal.ordinal() % u64::BITS as usize)) != 0
            }
            _ => unreachable!(),
        }
    }

    // TODO: add iterator.
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
