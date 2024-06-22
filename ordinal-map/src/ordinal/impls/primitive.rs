use crate::Ordinal;

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

    fn ordinal(&self) -> usize {
        *self as usize
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        u8::try_from(ordinal).ok()
    }
}

impl Ordinal for i8 {
    const ORDINAL_SIZE: usize = (i8::MAX as i16 - i8::MIN as i16 + 1) as usize;

    fn ordinal(&self) -> usize {
        self.abs_diff(i8::MIN) as usize
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        if ordinal < Self::ORDINAL_SIZE {
            Some((ordinal as isize + i8::MIN as isize) as i8)
        } else {
            None
        }
    }
}

impl Ordinal for u16 {
    const ORDINAL_SIZE: usize = u16::MAX as usize + 1;

    fn ordinal(&self) -> usize {
        *self as usize
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        u16::try_from(ordinal).ok()
    }
}

impl Ordinal for i16 {
    const ORDINAL_SIZE: usize = (i16::MAX as i32 - i16::MIN as i32 + 1) as usize;

    fn ordinal(&self) -> usize {
        self.abs_diff(i16::MIN) as usize
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        if ordinal < Self::ORDINAL_SIZE {
            Some((ordinal as isize + i16::MIN as isize) as i16)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::util::test_ordinal;

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
}
