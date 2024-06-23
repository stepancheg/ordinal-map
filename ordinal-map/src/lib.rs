#![deny(rustdoc::broken_intra_doc_links)]
#![deny(missing_docs)]

//! The library provides [`Ordinal`](crate::Ordinal) trait to map types to `usize` values,
//! proc-macro to derive `Ordinal` trait for structs and enums,
//! and [`map`](crate::map) and [`set`](crate::set) implementations
//! that use these types as keys efficiently.
//!
//! # Example
//!
//! ```
//! use ordinal_map::map::total::OrdinalTotalMap;
//! use ordinal_map::map::OrdinalMap;
//! #[derive(ordinal_map::Ordinal)]
//! enum ErrorCategory {
//!     Network,
//!     Disk,
//!     Logic,
//! }
//!
//! fn classify_error(error: &str) -> ErrorCategory {
//!     // ...
//! #  ErrorCategory::Network
//! }
//!
//! let mut error_counts: OrdinalTotalMap<ErrorCategory, u64> = OrdinalTotalMap::default();
//!
//! # let errors = [""; 0];
//!
//! for error in &errors {
//!     let category = classify_error(error);
//!     error_counts[category] += 1;
//! }
//! ```

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
