//! Composable validation rule runner.


use crate::core::error::Result;
use crate::core::Limits;
use crate::core::types::Epoch;
use crate::ledger::bundle::LedgerBundle;
use crate::validate::chain::validate_rotation_chain;
use crate::validate::deferred::validate_deferred;
use crate::validate::quota::enforce_quotas;
use crate::validate::schema::check_schema;
use crate::xref::resolver::XrefResolver;

#[derive(Debug)]
pub struct ValidationReport {
    pub schema_ok: bool,
    pub xref_ok: bool,
    pub chain_ok: bool,
    pub deferred_due: usize,
}

pub fn validate_bundle(bundle: &LedgerBundle, limits: &Limits, now: Epoch) -> Result<ValidationReport> {
    check_schema(bundle)?;
    let xref_ok = XrefResolver::new(bundle).validate_all().is_ok();
    validate_rotation_chain(bundle)?;
    let deferred_due = validate_deferred(bundle, now)?;
    enforce_quotas(bundle, limits)?;
    Ok(ValidationReport {
        schema_ok: true,
        xref_ok,
        chain_ok: true,
        deferred_due: deferred_due.len(),
    })
}
