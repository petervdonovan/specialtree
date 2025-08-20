//! CCF (Canonically Constructible From) analysis algorithms and data types
//!
//! This crate provides algorithms for analyzing transitive relationships in
//! Canonically Constructible From (CCF) relations, which are core to the
//! meta-language framework's type system.

pub mod types;
pub mod analysis;

#[cfg(test)]
mod tests;

pub use types::*;
pub use analysis::*;
