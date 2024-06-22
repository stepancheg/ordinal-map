use std::marker::PhantomData;

use crate::Ordinal;

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
    pub fn all() -> Self {
        const { Self::ASSERT };
        Set64 {
            set: (1 << T::ORDINAL_SIZE) - 1,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn contains(&self, ordinal: &T) -> bool {
        const { Self::ASSERT };
        self.set & (1 << ordinal.ordinal()) != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::Set64;

    // Fails at compilation time (as expected).
    // #[test]
    // fn test_more_than_64() {
    //     Set64::<u8>::new();
    // }

    #[test]
    fn test2() {
        let _set = Set64::<bool>::new();
    }
}
