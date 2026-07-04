//! Rotation chain continuity validation.


use crate::buf::Cursor;
use crate::core::error::{Error, Result};
use crate::core::types::RecordKind;
use crate::ledger::bundle::LedgerBundle;
use crate::record::event::RotationEvent;
use crate::record::parser::parse_payload;

pub fn validate_rotation_chain(bundle: &LedgerBundle) -> Result<()> {
    let mut last_seq = None;
    for record in &bundle.records {
        if record.kind != RecordKind::RotationEvent {
            continue;
        }
        let payload = parse_payload(record)?;
        if let crate::record::parser::ParsedPayload::Event(ev) = payload {
            if let Some(prev) = last_seq {
                if ev.seq.0 != prev + 1 {
                    return Err(Error::RotationGap { expected: prev + 1, found: ev.seq.0 });
                }
            }
            last_seq = Some(ev.seq.0);
        }
    }
    Ok(())
}
