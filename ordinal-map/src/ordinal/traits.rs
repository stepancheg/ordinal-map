/// Type that can be converted to `usize` range from `0..ORDINAL_SIZE`.
pub trait Ordinal: Sized {
    const ORDINAL_SIZE: usize;
    /// Index of the ordinal.
    fn ordinal(&self) -> usize;
    fn from_ordinal(ordinal: usize) -> Option<Self>;
}
