//! Constant time lookup map implementations where keys implement
//! the [`Ordinal`](crate::Ordinal) trait.

pub(crate) mod array_map;
pub(crate) mod entry;
pub(crate) mod enumerate;
pub(crate) mod init_array_map;
pub(crate) mod init_iter;
pub(crate) mod init_map;
pub(crate) mod iter;
pub(crate) mod map;

pub use array_map::OrdinalArrayMap;
pub use entry::Entry;
pub use entry::OccupiedEntry;
pub use entry::VacantEntry;
pub use init_array_map::OrdinalInitArrayMap;
pub use init_iter::InitIntoIter;
pub use init_iter::InitIter;
pub use init_iter::InitIterMut;
pub use init_map::OrdinalInitMap;
pub use iter::Drain;
pub use iter::IntoIter;
pub use iter::IntoIterArray;
pub use iter::Iter;
pub use iter::IterMut;
pub use iter::Keys;
pub use iter::Values;
pub use map::OrdinalMap;
