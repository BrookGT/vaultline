//! Credential rotation seals and verification.


pub mod algo;
pub mod chain;
pub mod envelope;
pub mod verify;

pub use envelope::SealEnvelope;
pub use verify::{verify_deferred_stub, verify_seal};
