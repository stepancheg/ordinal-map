/// Type that can be converted to `usize` range from `0..ORDINAL_SIZE`.
///
/// This type is implemented for
/// - small integer types
/// - tuples
/// - some builtin types like [`Option`] and [`Result`]
/// - and it can be derived with the `#[derive(Ordinal)]` attribute for structs and enums
///
/// # Relation to `Ord` and `PartialOrd`
///
/// Implementations provided in this crate and generated with `#[derive(Ordinal)]`
/// are compatible with `Ord` and `PartialOrd`, meaning `a < b <=> a.ordinal() < b.ordinal()`.
/// This is not enforced by the trait itself, but it is a good practice to follow.
///
/// # Derive
///
/// `#[derive(Ordinal)]` works for arbitrary structs and enums.
/// Generated implementation is compatible with `#[derive(PartialOrd)]`.
///
/// ```
/// use ordinal_map::Ordinal;
/// #[derive(Ordinal)]
/// enum Color {
///     Red,
///     Cyan,
///     Blue,
/// }
///
/// #[derive(Ordinal)]
/// struct Bright(bool);
///
/// #[derive(Ordinal)]
/// enum MaybeColor {
///     Default,
///     Colored(Color, Bright),
///     Invisible,
/// }
///
/// assert_eq!(0, MaybeColor::Default.ordinal());
/// assert_eq!(1, MaybeColor::Colored(Color::Red, Bright(false)).ordinal());
/// assert_eq!(2, MaybeColor::Colored(Color::Red, Bright(true)).ordinal());
/// assert_eq!(3, MaybeColor::Colored(Color::Cyan, Bright(false)).ordinal());
/// assert_eq!(4, MaybeColor::Colored(Color::Cyan, Bright(true)).ordinal());
/// ```
///
/// # See also
///
/// - [`Iter`](crate::Iter) to iterate over all possible values.
/// - [`map`](crate::map) module for constant time lookup maps.
/// - [`set`](crate::set) module for constant time lookup sets.
pub trait Ordinal: Sized {
    /// Number of possible values.
    ///
    /// It is compile-time error if the number of possible values is greater than `usize::MAX`.
    const ORDINAL_SIZE: usize;
    /// Index of the ordinal.
    fn ordinal(&self) -> usize;
    /// Returns the ordinal from the index.
    fn from_ordinal(ordinal: usize) -> Option<Self>;
}
