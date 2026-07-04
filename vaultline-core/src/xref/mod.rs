//! Cross-reference resolution for VRL bundles.


pub mod graph;
pub mod link;
pub mod resolver;
pub mod table;

pub use resolver::XrefResolver;
pub use table::{XrefEntry, XrefTable};
