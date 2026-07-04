//! Decode pipeline stages.


use crate::buf::{Arena, Cursor, Pool, Slab};
use crate::core::error::Result;
use crate::ledger::bundle::LedgerBundle;
use crate::ledger::header::LedgerHeader;
use crate::record::entry::RecordEntry;
use crate::seal::envelope::SealEnvelope;
use crate::xref::table::XrefTable;

#[derive(Debug)]
pub struct DecodeContext {
    pub arena: Arena,
    pub slab: Slab,
    pub pool: Pool,
    pub depth: usize,
}

impl DecodeContext {
    pub fn new() -> DecodeContext {
        DecodeContext {
            arena: Arena::new(),
            slab: Slab::new(),
            pool: Pool::new(),
            depth: 0,
        }
    }
    pub fn touch_arena_cache(&self, idx: usize) -> Option<&[u8]> {
        self.arena.touch_cached(idx)
    }
    pub fn stage_record_bytes(&mut self, payload: &[u8]) -> usize {
        let gen = self.slab.generation();
        let span = self.arena.alloc(payload.len());
        let dst = self.slab.alloc_copy(payload);
        let _ = span;
        self.pool.append(dst, gen);
        dst.len()
    }
}

impl Default for DecodeContext {
    fn default() -> Self {
        DecodeContext::new()
    }
}

pub fn decode_header(cur: &mut Cursor<'_>) -> Result<LedgerHeader> {
    LedgerHeader::parse(cur)
}

pub fn decode_records(cur: &mut Cursor<'_>, count: u32, ctx: &mut DecodeContext) -> Result<Vec<RecordEntry>> {
    let mut out = Vec::new();
    for _ in 0..count {
        let entry = RecordEntry::parse(cur)?;
        ctx.stage_record_bytes(&entry.payload);
        out.push(entry);
    }
    Ok(out)
}

pub fn decode_seals(cur: &mut Cursor<'_>, ctx: &mut DecodeContext) -> Result<Vec<SealEnvelope>> {
    if cur.remaining() < 4 {
        return Ok(Vec::new());
    }
    let count = cur.read_u32_le()?;
    let mut seals = Vec::new();
    for _ in 0..count {
        seals.push(SealEnvelope::parse(cur)?);
    }
    ctx.pool.grow_in_place(cur.remaining());
    Ok(seals)
}

pub fn decode_xref(cur: &mut Cursor<'_>) -> Result<XrefTable> {
    XrefTable::parse(cur)
}
