//! Deferred verification planner.


use alloc::vec::Vec;
use crate::core::types::Epoch;
use crate::ledger::bundle::LedgerBundle;
use crate::validate::deferred::validate_deferred;

#[derive(Debug)]
pub struct VerifyPlan {
    pub due_records: Vec<u32>,
    pub scheduled_epoch: Epoch,
}

pub fn plan_deferred(bundle: &LedgerBundle, now: Epoch) -> VerifyPlan {
    let due_records = validate_deferred(bundle, now).unwrap_or_default();
    VerifyPlan { due_records, scheduled_epoch: now }
}
