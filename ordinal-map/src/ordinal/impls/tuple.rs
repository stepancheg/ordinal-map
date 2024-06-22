use crate as ordinal_map;

ordinal_map_derive::impl_ordinal_for_tuple! {}

#[cfg(test)]
mod tests {
    use crate::tests::util::test_ordinal;

    #[test]
    fn test_tuple_0() {
        test_ordinal([()]);
    }

    #[test]
    fn test_tuple_1() {
        test_ordinal((0..=255u8).map(|i| (i,)));
    }

    #[test]
    fn test_tuple_2() {
        test_ordinal((0..=255u8).flat_map(|i| [false, true].map(move |b| (i, b))));
    }

    #[test]
    fn test_tuple_3() {
        test_ordinal((0..=255u8).flat_map(|i| {
            [false, true]
                .into_iter()
                .flat_map(move |b| [None, Some(false), Some(true)].map(move |o| (i, b, o)))
        }));
    }
}
