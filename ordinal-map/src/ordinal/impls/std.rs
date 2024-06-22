use std::convert::Infallible;

use crate::Ordinal;
use crate::__macro_refs::ordinal_size_sum;

impl<A: Ordinal> Ordinal for Option<A> {
    const ORDINAL_SIZE: usize = ordinal_size_sum([A::ORDINAL_SIZE, 1]);

    fn ordinal(&self) -> usize {
        match self {
            None => 0,
            Some(a) => a.ordinal() + 1,
        }
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        if ordinal == 0 {
            Some(None)
        } else {
            A::from_ordinal(ordinal - 1).map(Some)
        }
    }
}

impl<A: Ordinal, B: Ordinal> Ordinal for Result<A, B> {
    const ORDINAL_SIZE: usize = ordinal_size_sum([A::ORDINAL_SIZE, B::ORDINAL_SIZE]);

    fn ordinal(&self) -> usize {
        match self {
            Ok(a) => a.ordinal(),
            Err(b) => A::ORDINAL_SIZE + b.ordinal(),
        }
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        if ordinal < A::ORDINAL_SIZE {
            Some(Ok(A::from_ordinal(ordinal).unwrap()))
        } else {
            B::from_ordinal(ordinal - A::ORDINAL_SIZE).map(Err)
        }
    }
}

impl Ordinal for Infallible {
    const ORDINAL_SIZE: usize = 0;

    fn ordinal(&self) -> usize {
        match *self {}
    }

    fn from_ordinal(_: usize) -> Option<Self> {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use crate::tests::util::test_ordinal;

    #[test]
    fn test_option() {
        test_ordinal([None, Some(false), Some(true)]);
    }

    #[test]
    fn test_result() {
        test_ordinal([
            Ok(false),
            Ok(true),
            Err(None),
            Err(Some(false)),
            Err(Some(true)),
        ]);
    }

    #[test]
    fn test_infallible() {
        test_ordinal::<Infallible>([]);
    }
}
