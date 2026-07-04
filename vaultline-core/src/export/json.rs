//! JSON export for rotation snapshot summaries.


use alloc::string::String;
use alloc::format;
use crate::ledger::bundle::LedgerBundle;

pub fn summary_json(bundle: &LedgerBundle) -> String {
    format!(
        r#"{{"epoch":{},"records":{},"seals":{},"xrefs":{}}}"#,
        bundle.epoch(),
        bundle.records.len(),
        bundle.seals.len(),
        bundle.xrefs.entries().len()
    )
}
