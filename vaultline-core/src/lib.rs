//! vaultline-core — VRL (Vault Rotation Ledger) parsing and validation.


#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(missing_debug_implementations)]

extern crate alloc;

pub mod buf;
pub mod catalog;
pub mod core;
pub mod decode;
pub mod export;
pub mod ledger;
pub mod normalize;
pub mod record;
pub mod runtime;
pub mod seal;
pub mod validate;
pub mod xref;

pub use core::error::{Error, Result};
pub use ledger::{build_bundle, parse_bundle, LedgerBundle};

use alloc::format;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RotationReport {
    pub node_label: String,
    pub records_parsed: u32,
    pub seals_verified: u32,
    pub deferred_pending: u32,
}

pub mod limits {
    pub use crate::core::limits::*;
}

pub mod pipeline {
    use super::*;

    pub fn ingest(wire: &[u8]) -> Result<RotationReport> {
        let bundle = parse_bundle(wire)?;
        let snap = runtime::replay_snapshot(&bundle);
        Ok(RotationReport {
            node_label: format!("{:?}", bundle.header.node),
            records_parsed: bundle.records.len() as u32,
            seals_verified: bundle.seals.len() as u32,
            deferred_pending: snap.pending_verify,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn limits_sane() {
        assert!(limits::MAX_DEPTH >= 8);
    }
}
