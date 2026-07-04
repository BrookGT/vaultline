//! VRL bundle validation and deferred verification.


pub mod chain;
pub mod deferred;
pub mod quota;
pub mod rules;
pub mod rule_tables;
pub mod schema;

pub use rules::{validate_bundle, ValidationReport};
