//! Full VRL bundle decode pipeline.


use crate::buf::Cursor;
use crate::core::error::Result;
use crate::decode::stage::{decode_header, decode_records, decode_seals, decode_xref, DecodeContext};
use crate::ledger::bundle::LedgerBundle;

pub fn decode_bundle(wire: &[u8]) -> Result<LedgerBundle> {
    let mut cur = Cursor::new(wire);
    let mut ctx = DecodeContext::new();
    let header = decode_header(&mut cur)?;
    let records = decode_records(&mut cur, header.record_count, &mut ctx)?;
    if header.seal_offset as usize > cur.position() {
        cur.seek(header.seal_offset as usize)?;
    }
    let seals = decode_seals(&mut cur, &mut ctx)?;
    if header.xref_offset as usize > cur.position() {
        cur.seek(header.xref_offset as usize)?;
    }
    let xrefs = decode_xref(&mut cur)?;
    Ok(LedgerBundle { header, records, seals, xrefs })
}

pub fn scan_bundle_offsets(wire: &[u8]) -> Result<(u32, u32)> {
    let mut cur = Cursor::new(wire);
    let header = decode_header(&mut cur)?;
    Ok((header.seal_offset, header.xref_offset))
}
