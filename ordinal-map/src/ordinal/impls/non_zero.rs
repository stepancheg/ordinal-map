use std::num::NonZeroI16;
use std::num::NonZeroI32;
use std::num::NonZeroI64;
use std::num::NonZeroI8;
use std::num::NonZeroIsize;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::num::NonZeroU64;
use std::num::NonZeroU8;
use std::num::NonZeroUsize;

use crate::Ordinal;

impl Ordinal for NonZeroU8 {
    const ORDINAL_SIZE: usize = u8::MAX as usize;

    #[inline]
    fn ordinal(&self) -> usize {
        self.get() as usize - 1
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        NonZeroU8::try_from(u8::try_from(ordinal.checked_add(1)?).ok()?).ok()
    }
}

impl Ordinal for NonZeroI8 {
    const ORDINAL_SIZE: usize = NonZeroU8::ORDINAL_SIZE;

    #[inline]
    fn ordinal(&self) -> usize {
        if self.get() > 0 {
            127 + self.get() as usize
        } else {
            self.get().abs_diff(i8::MIN) as usize
        }
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        match ordinal.checked_sub(i8::MAX as usize + 1) {
            None => NonZeroI8::new(
                i8::MIN
                    .checked_add_unsigned(u8::try_from(ordinal).unwrap())
                    .unwrap(),
            ),
            Some(rem) => NonZeroI8::new(i8::try_from(rem + 1).ok()?),
        }
    }
}

impl Ordinal for NonZeroU16 {
    const ORDINAL_SIZE: usize = u16::MAX as usize;

    #[inline]
    fn ordinal(&self) -> usize {
        self.get() as usize - 1
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        NonZeroU16::try_from(u16::try_from(ordinal.checked_add(1)?).ok()?).ok()
    }
}

impl Ordinal for NonZeroI16 {
    const ORDINAL_SIZE: usize = NonZeroU16::ORDINAL_SIZE;

    #[inline]
    fn ordinal(&self) -> usize {
        if self.get() > 0 {
            32767 + self.get() as usize
        } else {
            self.get().abs_diff(i16::MIN) as usize
        }
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        match ordinal.checked_sub(i16::MAX as usize + 1) {
            None => NonZeroI16::new(
                i16::MIN
                    .checked_add_unsigned(u16::try_from(ordinal).unwrap())
                    .unwrap(),
            ),
            Some(rem) => NonZeroI16::new(i16::try_from(rem + 1).ok()?),
        }
    }
}

impl Ordinal for NonZeroU32 {
    const ORDINAL_SIZE: usize = u32::MAX as usize;

    #[inline]
    fn ordinal(&self) -> usize {
        self.get() as usize - 1
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        NonZeroU32::try_from(u32::try_from(ordinal.checked_add(1)?).ok()?).ok()
    }
}

impl Ordinal for NonZeroI32 {
    const ORDINAL_SIZE: usize = NonZeroU32::ORDINAL_SIZE;

    #[inline]
    fn ordinal(&self) -> usize {
        if self.get() > 0 {
            2147483647 + self.get() as usize
        } else {
            self.get().abs_diff(i32::MIN) as usize
        }
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        match ordinal.checked_sub(i32::MAX as usize + 1) {
            None => NonZeroI32::new(
                i32::MIN
                    .checked_add_unsigned(u32::try_from(ordinal).unwrap())
                    .unwrap(),
            ),
            Some(rem) => NonZeroI32::new(i32::try_from(rem + 1).ok()?),
        }
    }
}

impl Ordinal for NonZeroUsize {
    const ORDINAL_SIZE: usize = usize::MAX;

    #[inline]
    fn ordinal(&self) -> usize {
        self.get() - 1
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        NonZeroUsize::new(ordinal.checked_add(1)?)
    }
}

impl Ordinal for NonZeroIsize {
    const ORDINAL_SIZE: usize = NonZeroUsize::ORDINAL_SIZE;

    #[inline]
    fn ordinal(&self) -> usize {
        if self.get() > 0 {
            usize::MAX / 2 + self.get() as usize
        } else {
            self.get().abs_diff(isize::MIN)
        }
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        match ordinal.checked_sub(isize::MAX as usize + 1) {
            None => NonZeroIsize::new(isize::MIN.checked_add_unsigned(ordinal).unwrap()),
            Some(rem) => NonZeroIsize::new(isize::try_from(rem + 1).ok()?),
        }
    }
}

/// It is compile-time error to use this impl on 32-bit platforms.
impl Ordinal for NonZeroU64 {
    const ORDINAL_SIZE: usize = if u64::MAX == usize::MAX as u64 {
        usize::MAX
    } else {
        panic!("NonZeroU64::ORDINAL_SIZE is too large for 32-bit platforms")
    };

    fn ordinal(&self) -> usize {
        const { Self::ORDINAL_SIZE };
        NonZeroUsize::new(self.get() as usize).unwrap().ordinal()
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        const { Self::ORDINAL_SIZE };
        Some(NonZeroU64::new(NonZeroUsize::from_ordinal(ordinal)?.get() as u64).unwrap())
    }
}

impl Ordinal for NonZeroI64 {
    const ORDINAL_SIZE: usize = NonZeroU64::ORDINAL_SIZE;

    fn ordinal(&self) -> usize {
        const { Self::ORDINAL_SIZE };
        NonZeroIsize::new(self.get() as isize).unwrap().ordinal()
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        const { Self::ORDINAL_SIZE };
        Some(NonZeroI64::new(NonZeroIsize::from_ordinal(ordinal)?.get() as i64).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroI16;
    use std::num::NonZeroI32;
    use std::num::NonZeroI64;
    use std::num::NonZeroI8;
    use std::num::NonZeroIsize;
    use std::num::NonZeroU16;
    use std::num::NonZeroU64;
    use std::num::NonZeroU8;
    use std::num::NonZeroUsize;

    use crate::tests::util::test_ordinal;
    use crate::tests::util::test_ordinal_some;
    use crate::tests::util::test_ordinal_value;

    #[test]
    fn test_non_zero_u8() {
        test_ordinal((1..=u8::MAX).map(|i| NonZeroU8::new(i).unwrap()));
    }

    #[test]
    fn test_non_zero_i8() {
        test_ordinal((i8::MIN..=i8::MAX).filter_map(NonZeroI8::new));
    }

    #[test]
    fn test_non_zero_u16() {
        test_ordinal((1..=u16::MAX).map(|i| NonZeroU16::new(i).unwrap()));
    }

    #[test]
    fn test_non_zero_i16() {
        test_ordinal((i16::MIN..=i16::MAX).filter_map(NonZeroI16::new));
    }

    #[test]
    fn test_non_zero_u32() {
        test_ordinal_some::<u32>();
    }

    #[test]
    fn test_non_zero_i32() {
        test_ordinal_some::<i32>();
        test_ordinal_value(NonZeroI32::new(-1));
        test_ordinal_value(NonZeroI32::new(1));
    }

    #[test]
    fn test_non_zero_u64() {
        if cfg!(target_pointer_width = "64") {
            test_ordinal_some::<NonZeroU64>();
        }
    }

    #[test]
    fn test_non_zero_i64() {
        if cfg!(target_pointer_width = "64") {
            test_ordinal_some::<NonZeroI64>();
            test_ordinal_value(NonZeroI64::new(-1));
            test_ordinal_value(NonZeroI64::new(1));
        }
    }

    #[test]
    fn test_non_zero_usize() {
        test_ordinal_some::<NonZeroUsize>();
    }

    #[test]
    fn test_non_zero_isize() {
        test_ordinal_some::<NonZeroIsize>();
        test_ordinal_value(NonZeroIsize::new(-1));
        test_ordinal_value(NonZeroIsize::new(1));
    }
}
