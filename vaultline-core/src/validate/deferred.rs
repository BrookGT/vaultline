//! Deferred verification scheduling checks.


use crate::core::error::Result;
use crate::core::types::{Epoch, RecordKind};
use crate::ledger::bundle::LedgerBundle;
use crate::record::deferred::DeferredVerify;
use crate::record::parser::parse_payload;

pub fn validate_deferred(bundle: &LedgerBundle, now: Epoch) -> Result<Vec<u32>> {
    let mut due = Vec::new();
    for record in &bundle.records {
        if record.kind != RecordKind::DeferredVerify {
            continue;
        }
        if let Ok(crate::record::parser::ParsedPayload::Deferred(d)) = parse_payload(record) {
            if d.is_due(now) {
                due.push(d.target_record);
            }
        }
    }
    Ok(due)
}
