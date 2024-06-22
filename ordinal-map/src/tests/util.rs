#![cfg(test)]

use std::fmt::Debug;

use crate::Ordinal;

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
