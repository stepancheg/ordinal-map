#![doc(hidden)]

/// Used in macros to generate `ORDINAL_SIZE` field.
pub const fn ordinal_size_product<const N: usize>(items: [usize; N]) -> usize {
    let mut r = 1usize;
    let mut i = 0;
    while i < N {
        r = match r.checked_mul(items[i]) {
            Some(size) => size,
            None => panic!("Ordinal size overflow"),
        };
        i += 1;
    }
    r
}

/// Used in macros to generate `ORDINAL_SIZE` field.
pub const fn ordinal_size_sum<const N: usize>(items: [usize; N]) -> usize {
    let mut r = 0usize;
    let mut i = 0;
    while i < N {
        r = match r.checked_add(items[i]) {
            Some(size) => size,
            None => panic!("Ordinal size overflow"),
        };
        i += 1;
    }
    r
}
