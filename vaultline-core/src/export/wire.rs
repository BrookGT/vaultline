//! Encode VRL bundles to wire format.


use alloc::vec::Vec;
use crate::core::error::Result;
use crate::ledger::bundle::LedgerBundle;
use crate::ledger::header::checksum_header;
use crate::record::writer::serialize_records;

pub fn encode_bundle(bundle: &LedgerBundle) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    let mut hdr = bundle.header.clone();
    hdr.header_crc = checksum_header(&hdr);
    hdr.write_to(&mut out);
    let records = serialize_records(&bundle.records);
    out.extend_from_slice(&records);
    out.extend_from_slice(&(bundle.seals.len() as u32).to_le_bytes());
    for s in &bundle.seals {
        s.write_to(&mut out);
    }
    let xref_count = bundle.xrefs.entries().len() as u32;
    out.extend_from_slice(&xref_count.to_le_bytes());
    for e in bundle.xrefs.entries() {
        out.push(e.kind);
        out.extend_from_slice(&e.from_id.to_le_bytes());
        out.extend_from_slice(&e.to_id.to_le_bytes());
    }
    Ok(out)
}
