/// Two-way map from a type to `usize` range from `0..ORDINAL_SIZE`.
///
/// Two way ordinal mapping means:
/// - the type has `ORDINAL_SIZE` possible values
/// - each value has a unique ordinal number in the range `0..ORDINAL_SIZE`
/// - each ordinal number corresponds to a unique value
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
/// are compatible with `Ord` and `PartialOrd`, meaning `a < b <=> a.ordinal() < b.ordinal()`
/// **with the exception** of derive on enums with explicit discriminants
/// (derive ignores them and assigns ordinal numbers in order of declaration).
///
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
/// - [`Iter`](crate::OrdinalValues) to iterate over all possible values.
/// - [`map`](crate::map) module for constant time lookup maps.
/// - [`set`](crate::set) module for constant time lookup sets.
pub trait Ordinal: Sized {
    /// Number of possible values.
    ///
    /// It is compile-time error if the number of possible values is greater than `usize::MAX`.
    const ORDINAL_SIZE: usize;
    /// Index of the ordinal.
    ///
    /// # Example
    ///
    /// ```
    /// use std::num::NonZeroI16;
    ///
    /// use ordinal_map::Ordinal;
    /// assert_eq!(0, None::<u16>.ordinal());
    /// assert_eq!(1, Some::<u16>(0).ordinal());
    /// assert_eq!(2, Some::<u16>(1).ordinal());
    /// ```
    fn ordinal(&self) -> usize;
    /// Returns the ordinal from the index.
    ///
    /// # Example
    ///
    /// ```
    /// use ordinal_map::Ordinal;
    ///
    /// assert_eq!(i16::MIN, i16::from_ordinal(0).unwrap());
    /// assert_eq!(i16::MIN + 1, i16::from_ordinal(1).unwrap());
    /// ```
    fn from_ordinal(ordinal: usize) -> Option<Self>;

    /// Iterate over all possible values.
    ///
    /// Values are returned in order of their ordinal numbers.
    ///
    /// # Example
    ///
    /// ```
    /// use ordinal_map::Ordinal;
    /// let mut iter = i16::all_values();
    /// assert_eq!(Some(i16::MIN), iter.next());
    /// assert_eq!(Some(i16::MAX), iter.next_back());
    /// ```
    fn all_values() -> crate::OrdinalValues<Self> {
        crate::OrdinalValues::new()
    }
}
