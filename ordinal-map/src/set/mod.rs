//! Constant time lookup set implementations where entries implement
//! the [`Ordinal`](crate::Ordinal) trait.

pub(crate) mod array;
pub(crate) mod iter;
pub(crate) mod set;
pub(crate) mod set64;
pub(crate) mod set_mut;
pub(crate) mod set_ref;

pub use array::ordinal_array_set_s;
pub use array::OrdinalArraySet;
pub use iter::Iter;
pub use set::OrdinalSet;
pub use set64::Iter64;
pub use set64::OrdinalSet64;
