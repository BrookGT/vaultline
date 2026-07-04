//! VRL record types and parsers.


pub mod credential;
pub mod deferred;
pub mod layout_tables;
pub mod entry;
pub mod event;
pub mod parser;
pub mod policy;
pub mod revocation;
pub mod writer;

pub use entry::RecordEntry;
pub use parser::{parse_payload, ParsedPayload};
