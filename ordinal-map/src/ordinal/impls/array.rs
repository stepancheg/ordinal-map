use crate::array_builder::ArrayBuilder;
use crate::Ordinal;

impl<T: Ordinal, const N: usize> Ordinal for [T; N] {
    const ORDINAL_SIZE: usize = {
        let mut s = 1usize;
        let mut i = 0;
        while i < N {
            s = s.checked_mul(T::ORDINAL_SIZE).unwrap();
            i += 1;
        }
        s
    };

    fn ordinal(&self) -> usize {
        let mut r = 0usize;
        for v in self {
            r = r.checked_mul(T::ORDINAL_SIZE).unwrap();
            r += v.ordinal();
        }
        r
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        let mut n = ordinal;
        let mut array = ArrayBuilder::new();
        for _ in 0..N {
            let Some(i) = n.checked_rem(T::ORDINAL_SIZE) else {
                return None;
            };
            array.push(T::from_ordinal(i).unwrap());
            n /= T::ORDINAL_SIZE;
        }
        if n != 0 {
            None
        } else {
            let mut array = array.finish();
            array.reverse();
            Some(array)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::tests::util::test_ordinal;

    #[test]
    fn test_array_of_empty() {
        test_ordinal::<[Infallible; 0]>([[]]);
        test_ordinal::<[Infallible; 1]>([]);
        test_ordinal::<[Infallible; 2]>([]);
    }

    #[test]
    fn test_array_0() {
        test_ordinal::<[u16; 0]>([[]]);
    }

    #[test]
    fn test_array_1() {
        test_ordinal([[false], [true]]);
    }

    #[test]
    fn test_array_2() {
        test_ordinal([[false, false], [false, true], [true, false], [true, true]]);
        // Should be compatible with ordinal for tuple.
        test_ordinal([(false, false), (false, true), (true, false), (true, true)]);
    }
}
