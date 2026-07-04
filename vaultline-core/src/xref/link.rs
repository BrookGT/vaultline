//! Single xref link helpers.


use crate::core::constants::xref_kind;
use crate::xref::table::XrefEntry;

pub fn record_to_seal(record_id: u32, seal_id: u32) -> XrefEntry {
    XrefEntry {
        kind: xref_kind::RECORD_TO_SEAL,
        from_id: record_id,
        to_id: seal_id,
    }
}

pub fn deferred_link(record_id: u32, verify_record: u32) -> XrefEntry {
    XrefEntry {
        kind: xref_kind::DEFERRED_LINK,
        from_id: record_id,
        to_id: verify_record,
    }
}
