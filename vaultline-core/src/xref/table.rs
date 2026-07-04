//! Cross-reference table linking records, seals, and policies.


use alloc::vec::Vec;

use crate::buf::Cursor;
use crate::core::constants::xref_kind;
use crate::core::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XrefEntry {
    pub kind: u8,
    pub from_id: u32,
    pub to_id: u32,
}

#[derive(Debug, Clone, Default)]
pub struct XrefTable {
    entries: Vec<XrefEntry>,
}

impl XrefTable {
    pub fn new() -> XrefTable {
        XrefTable::default()
    }
    pub fn parse(cur: &mut Cursor<'_>) -> Result<XrefTable> {
        let count = cur.read_u32_le()? as usize;
        if count > crate::core::limits::MAX_XREF {
            return Err(Error::LengthOverflow { field: "xref" });
        }
        let mut entries = Vec::with_capacity(count);
        for _ in 0..count {
            entries.push(XrefEntry {
                kind: cur.read_u8()?,
                from_id: cur.read_u32_le()?,
                to_id: cur.read_u32_le()?,
            });
        }
        Ok(XrefTable { entries })
    }
    pub fn entries(&self) -> &[XrefEntry] {
        &self.entries
    }
    pub fn links_to(&self, from: u32) -> Vec<u32> {
        self.entries
            .iter()
            .filter(|e| e.from_id == from)
            .map(|e| e.to_id)
            .collect()
    }
    pub fn record_seal_pairs(&self) -> Vec<(u32, u32)> {
        self.entries
            .iter()
            .filter(|e| e.kind == xref_kind::RECORD_TO_SEAL)
            .map(|e| (e.from_id, e.to_id))
            .collect()
    }
}
