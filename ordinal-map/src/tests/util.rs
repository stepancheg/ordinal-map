#![cfg(test)]

use std::fmt::Debug;
use quickcheck::{Arbitrary, Gen};

use crate::Ordinal;
use crate as ordinal_map;

pub(crate) fn test_ordinal<T: Ordinal + Ord + Eq + Debug>(expected: impl IntoIterator<Item = T>) {
    let expected = Vec::from_iter(expected);

    // Self check.
    for (a, b) in expected.iter().zip(expected.iter().skip(1)) {
        assert!(a < b);
    }

    assert_eq!(expected.len(), T::ORDINAL_SIZE);
    assert_eq!(expected, crate::Iter::<T>::new().collect::<Vec<_>>());

    for i in 0..T::ORDINAL_SIZE {
        let t = T::from_ordinal(i).unwrap();
        assert_eq!(t, expected[i]);
        assert_eq!(i, t.ordinal());
    }

    assert_eq!(None, T::from_ordinal(T::ORDINAL_SIZE));
}

#[derive(Ordinal, Eq, PartialEq, Copy, Clone, Debug, Ord, PartialOrd)]
pub(crate) enum Example4 {
    A,
    B,
    C,
    D,
}

impl Arbitrary for Example4 {
    fn arbitrary(g: &mut Gen) -> Self {
        *g.choose(&[Example4::A, Example4::B, Example4::C, Example4::D]).unwrap()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
        Box::new(match self {
            Example4::A => vec![].into_iter(),
            Example4::B => vec![Example4::A].into_iter(),
            Example4::C => vec![Example4::B, Example4::A].into_iter(),
            Example4::D => vec![Example4::C, Example4::B, Example4::A].into_iter(),
        })
    }
}

pub(crate) fn test_exact_size_iterator<I: ExactSizeIterator>(mut iter: I) {
    let mut rem = iter.len();
    for _ in 0..rem {
        assert_eq!(rem, iter.len());
        assert!(iter.next().is_some());
        rem -= 1;
    }
    assert_eq!(0, iter.len());
    assert!(iter.next().is_none());
}
