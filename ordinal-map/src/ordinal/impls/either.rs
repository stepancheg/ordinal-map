#![cfg(feature = "either")]

use either::Either;

use crate::Ordinal;

impl<A: Ordinal, B: Ordinal> Ordinal for Either<A, B> {
    const ORDINAL_SIZE: usize = Result::<A, B>::ORDINAL_SIZE;

    fn ordinal(&self) -> usize {
        match self {
            Either::Left(a) => a.ordinal(),
            Either::Right(b) => A::ORDINAL_SIZE + b.ordinal(),
        }
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        if ordinal < A::ORDINAL_SIZE {
            Some(Either::Left(A::from_ordinal(ordinal).unwrap()))
        } else {
            B::from_ordinal(ordinal - A::ORDINAL_SIZE).map(Either::Right)
        }
    }
}

#[cfg(test)]
mod tests {
    use either::Either;

    use crate::tests::util::test_ordinal;

    #[test]
    fn test_either() {
        test_ordinal([Either::Left(false), Either::Left(true), Either::Right(())]);
    }
}
