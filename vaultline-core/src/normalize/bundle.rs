//! Normalize full VRL bundles for canonical comparison.


use crate::core::error::Result;
use crate::ledger::bundle::LedgerBundle;
use crate::validate::rules::validate_bundle;
use crate::core::{Epoch, Limits};

pub fn normalize_bundle(bundle: &LedgerBundle) -> Result<LedgerBundle> {
    let limits = Limits::default();
    let _ = validate_bundle(bundle, &limits, Epoch(0))?;
    let mut out = bundle.clone();
    out.records.sort_by_key(|r| r.id.0);
    Ok(out)
}
