//! Export VRL bundles to wire and summary formats.


pub mod json;
pub mod report;
pub mod wire;

pub use wire::encode_bundle;
