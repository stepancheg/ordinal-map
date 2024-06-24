use crate::array_builder::ArrayBuilder;

pub(crate) fn array_as_mut<T, const S: usize>(array: &mut [T; S]) -> [&mut T; S] {
    let mut array: &mut [T] = array;

    let mut result = ArrayBuilder::new();
    while let Some((first, rest)) = array.split_first_mut() {
        result.push(first);
        array = rest;
    }
    result.finish()
}
