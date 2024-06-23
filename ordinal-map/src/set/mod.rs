//! Constant time lookup set implementations where entries implement
//! the [`Ordinal`](crate::Ordinal) trait.

pub(crate) mod array;
pub(crate) mod set;
pub(crate) mod set64;
pub(crate) mod set_mut;
pub(crate) mod set_ref;

pub use array::OrdinalArraySet;
pub use set::Iter;
pub use set::OrdinalSet;
pub use set64::Iter64;
pub use set64::OrdinalSet64;
