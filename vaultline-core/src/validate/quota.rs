//! Bundle size and record quota enforcement.


use crate::core::error::{Error, Result};
use crate::core::Limits;
use crate::ledger::bundle::LedgerBundle;

pub fn enforce_quotas(bundle: &LedgerBundle, limits: &Limits) -> Result<()> {
    let total: usize = bundle.records.iter().map(|r| r.payload.len()).sum();
    if total > limits.max_bundle {
        return Err(Error::CapacityLimit);
    }
    if bundle.xrefs.entries().len() > limits.max_xref {
        return Err(Error::CapacityLimit);
    }
    Ok(())
}
