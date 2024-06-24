use crate::Ordinal;
use crate::__macro_refs::ordinal_size_sum;

impl Ordinal for bool {
    const ORDINAL_SIZE: usize = 2;

    fn ordinal(&self) -> usize {
        *self as usize
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        match ordinal {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
}

impl Ordinal for u8 {
    const ORDINAL_SIZE: usize = u8::MAX as usize + 1;

    #[inline]
    fn ordinal(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        u8::try_from(ordinal).ok()
    }
}

impl Ordinal for i8 {
    const ORDINAL_SIZE: usize = (i8::MAX as i16 - i8::MIN as i16 + 1) as usize;

    #[inline]
    fn ordinal(&self) -> usize {
        self.abs_diff(i8::MIN) as usize
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        i8::MIN.checked_add_unsigned(u8::try_from(ordinal).ok()?)
    }
}

impl Ordinal for u16 {
    const ORDINAL_SIZE: usize = u16::MAX as usize + 1;

    #[inline]
    fn ordinal(&self) -> usize {
        *self as usize
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        u16::try_from(ordinal).ok()
    }
}

impl Ordinal for i16 {
    const ORDINAL_SIZE: usize = (i16::MAX as isize - i16::MIN as isize + 1) as usize;

    #[inline]
    fn ordinal(&self) -> usize {
        self.abs_diff(i16::MIN) as usize
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        i16::MIN.checked_add_unsigned(u16::try_from(ordinal).ok()?)
    }
}

/// Accessing this type is compile time error on 32-bit platforms.
impl Ordinal for u32 {
    const ORDINAL_SIZE: usize = ordinal_size_sum([u32::MAX as usize, 1]);

    #[inline]
    fn ordinal(&self) -> usize {
        const { Self::ORDINAL_SIZE };
        *self as usize
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        u32::try_from(ordinal).ok()
    }
}

impl Ordinal for i32 {
    const ORDINAL_SIZE: usize = u32::ORDINAL_SIZE;

    #[inline]
    fn ordinal(&self) -> usize {
        const { Self::ORDINAL_SIZE };
        self.abs_diff(i32::MIN) as usize
    }

    #[inline]
    fn from_ordinal(ordinal: usize) -> Option<Self> {
        const { Self::ORDINAL_SIZE };
        i32::MIN.checked_add_unsigned(u32::try_from(ordinal).ok()?)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::util::test_ordinal;
    use crate::Ordinal;

    #[test]
    fn test_bool() {
        test_ordinal::<bool>([false, true]);
    }

    #[test]
    fn test_u8() {
        test_ordinal::<u8>(0..=255);
    }

    #[test]
    fn test_i8() {
        test_ordinal::<i8>(i8::MIN..=i8::MAX);
    }

    #[test]
    fn test_u16() {
        test_ordinal::<u16>(0..=u16::MAX);
    }

    #[test]
    fn test_i16() {
        test_ordinal::<i16>(i16::MIN..=i16::MAX);
    }

    #[test]
    fn test_u32() {
        if cfg!(target_pointer_width = "64") {
            let mut iter = u32::all_values();
            assert_eq!(u32::MAX as usize + 1, iter.len());
            assert_eq!(Some(0), iter.next());
            assert_eq!(Some(1), iter.next());
            assert_eq!(Some(u32::MAX), iter.next_back());
        }
    }

    #[test]
    fn test_i32() {
        if cfg!(target_pointer_width = "64") {
            let mut iter = i32::all_values();
            assert_eq!(u32::MAX as usize + 1, iter.len());
            assert_eq!(Some(i32::MIN), iter.next());
            assert_eq!(Some(i32::MAX), iter.next_back());
        }
    }
}
