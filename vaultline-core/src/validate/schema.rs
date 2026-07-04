//! Structural schema checks for VRL bundles.


use crate::core::error::{Error, Result};
use crate::ledger::bundle::LedgerBundle;

pub fn check_schema(bundle: &LedgerBundle) -> Result<()> {
    if bundle.header.record_count as usize != bundle.records.len() {
        return Err(Error::protocol("record count mismatch"));
    }
    if bundle.header.node.is_zero() {
        return Err(Error::protocol("zero node id"));
    }
    Ok(())
}
