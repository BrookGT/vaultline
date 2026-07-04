//! Replay rotation events into snapshot state.


use crate::core::types::RecordKind;
use crate::ledger::bundle::LedgerBundle;
use crate::ledger::snapshot::RotationSnapshot;
use crate::record::parser::parse_payload;

pub fn replay_snapshot(bundle: &LedgerBundle) -> RotationSnapshot {
    let mut snap = RotationSnapshot::new(bundle.header.node, bundle.header.epoch);
    for record in &bundle.records {
        match record.kind {
            RecordKind::CredentialEntry => snap.register_credential(),
            RecordKind::DeferredVerify => snap.register_deferred(),
            RecordKind::RotationEvent => {
                if let Ok(crate::record::parser::ParsedPayload::Event(ev)) = parse_payload(record) {
                    snap.advance(ev.seq);
                }
            }
            _ => {}
        }
    }
    snap
}
