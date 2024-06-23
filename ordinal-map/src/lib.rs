#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_docs)]

//! Provide [`Ordinal`] trait to map types to `usize` values,
//! proc-macro to derive `Ordinal` trait for structs and enums,
//! and [`map`] and [`set`] implementations that use these types as keys efficiently.

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod __macro_refs;
mod iter;
pub mod map;
mod ordinal;
pub mod set;
pub(crate) mod tests;

pub use iter::Iter;
pub use ordinal::traits::Ordinal;
pub use ordinal_map_derive::Ordinal;
pub(crate) mod array_builder;
