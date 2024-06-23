//! Constant time lookup set implementations where entries implement
//! the [`Ordinal`](crate::Ordinal) trait.

pub(crate) mod set;
pub(crate) mod set64;

pub use set::Iter;
pub use set::Set;
pub use set64::Iter64;
pub use set64::Set64;
