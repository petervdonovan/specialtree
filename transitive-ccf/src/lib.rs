//! CCF (Canonically Constructible From) analysis algorithms and data types
//!
//! This crate provides algorithms for analyzing transitive relationships in
//! Canonically Constructible From (CCF) relations, which are core to the
//! meta-language framework's type system.

pub mod analysis;
pub mod types;

#[cfg(test)]
mod tests;

pub use analysis::*;
pub use types::*;
