use std::cmp::Ordering;

use crate::Ordinal;

impl Ordinal for Ordering {
    const ORDINAL_SIZE: usize = 3;

    fn ordinal(&self) -> usize {
        match self {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => 2,
        }
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        match ordinal {
            0 => Some(Ordering::Less),
            1 => Some(Ordering::Equal),
            2 => Some(Ordering::Greater),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::tests::util::test_ordinal;

    #[test]
    fn test_ordering() {
        test_ordinal([Ordering::Less, Ordering::Equal, Ordering::Greater]);
    }
}
