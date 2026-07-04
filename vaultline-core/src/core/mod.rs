//! Core types, limits, and error handling.


pub mod constants;
pub mod error;
pub mod flags;
pub mod limits;
pub mod types;

pub use error::{Error, Result};
pub use limits::Limits;
pub use types::{Epoch, NodeId, RecordId, RecordKind, RotationSeq, SealId, MAGIC};
