//! Constant time lookup map implementations where keys implement
//! the [`Ordinal`](crate::Ordinal) trait.

pub(crate) mod array;
pub(crate) mod init;
pub(crate) mod init_array;
pub(crate) mod init_iter;
pub(crate) mod iter;
pub(crate) mod map;

pub use array::OrdinalArrayMap;
pub use init::OrdinalInitMap;
pub use init_array::OrdinalInitArrayMap;
pub use init_iter::InitIter;
pub use init_iter::InitIterMut;
pub use iter::Iter;
pub use iter::IterMut;
pub use iter::Keys;
pub use iter::Values;
pub use map::OrdinalMap;
