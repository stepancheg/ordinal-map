//! Total maps (maps that have a value for every possible key).

pub(crate) mod array_map;
pub(crate) mod iter;
pub(crate) mod map;

pub use crate::map::total::array_map::OrdinalTotalArrayMap;
pub use crate::map::total::iter::IntoIter;
pub use crate::map::total::iter::IntoIterArray;
pub use crate::map::total::iter::Iter;
pub use crate::map::total::iter::IterMut;
pub use crate::map::total::map::OrdinalTotalMap;
