//! Record serialization helpers.


use alloc::vec::Vec;

use crate::core::flags::RecordFlags;
use crate::core::types::{RecordId, RecordKind};
use crate::record::entry::RecordEntry;

pub fn build_record(id: u32, kind: RecordKind, flags: RecordFlags, payload: &[u8]) -> RecordEntry {
    RecordEntry {
        id: RecordId(id),
        kind,
        flags,
        payload: payload.to_vec(),
    }
}

pub fn serialize_records(records: &[RecordEntry]) -> Vec<u8> {
    let mut out = Vec::new();
    for r in records {
        r.write_to(&mut out);
    }
    out
}
