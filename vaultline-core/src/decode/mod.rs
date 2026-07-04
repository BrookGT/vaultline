//! VRL wire decode pipeline.


pub mod context;
pub mod leb;
pub mod pipeline;
pub mod stage;
pub mod tag_tables;
pub mod tlv;

pub use pipeline::{decode_bundle, scan_bundle_offsets};
