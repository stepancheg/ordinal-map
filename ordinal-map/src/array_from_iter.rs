use crate::array_builder::ArrayBuilder;

pub fn array_from_iter<T, I, const S: usize>(iter: I) -> [T; S]
where
    I: IntoIterator<Item = T>,
{
    let mut builder = ArrayBuilder::new();
    for item in iter {
        builder.push(item);
    }
    builder.finish()
}

#[cfg(test)]
mod tests {
    use crate::array_from_iter::array_from_iter;

    #[test]
    fn test_array_from_iter() {
        let array = array_from_iter((0..3).map(|i| i.to_string()));
        assert_eq!(["0".to_owned(), "1".to_owned(), "2".to_owned()], array);
    }
}
