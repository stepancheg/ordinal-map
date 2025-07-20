use crate::Ordinal;

const SURROGATE_START: u32 = 0xd800;

impl Ordinal for char {
    const ORDINAL_SIZE: usize = char::MAX as usize + 1 - 0x800;

    fn ordinal(&self) -> usize {
        let c = *self as u32;
        if c < SURROGATE_START {
            c as usize
        } else {
            c as usize - 0x800
        }
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        if ordinal < SURROGATE_START as usize {
            char::from_u32(ordinal as u32)
        } else {
            char::from_u32((ordinal as u32).checked_add(0x800)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::util::test_ordinal;

    #[test]
    fn test_char() {
        test_ordinal((0..=(char::MAX as u32)).filter_map(char::from_u32))
    }
}
