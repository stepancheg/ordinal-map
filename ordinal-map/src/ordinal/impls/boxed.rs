use crate::Ordinal;

impl<T: Ordinal> Ordinal for Box<T> {
    const ORDINAL_SIZE: usize = T::ORDINAL_SIZE;

    fn ordinal(&self) -> usize {
        (**self).ordinal()
    }

    fn from_ordinal(ordinal: usize) -> Option<Self> {
        Some(Box::new(T::from_ordinal(ordinal)?))
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::util::test_ordinal;

    #[test]
    fn test_box() {
        test_ordinal([Box::new(false), Box::new(true)])
    }
}
