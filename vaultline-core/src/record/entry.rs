//! Unified record entry wrapper for all VRL record kinds.


use alloc::vec::Vec;

use crate::buf::Cursor;
use crate::core::error::{Error, Result};
use crate::core::flags::RecordFlags;
use crate::core::types::{RecordId, RecordKind};

#[derive(Debug, Clone)]
pub struct RecordEntry {
    pub id: RecordId,
    pub kind: RecordKind,
    pub flags: RecordFlags,
    pub payload: Vec<u8>,
}

impl RecordEntry {
    pub fn parse(cur: &mut Cursor<'_>) -> Result<RecordEntry> {
        let id = RecordId(cur.read_u32_le()?);
        let tag = cur.read_u8()?;
        let kind = RecordKind::from_tag(tag).ok_or(Error::protocol("record kind"))?;
        let flags = RecordFlags::from_le(cur.read_u16_le()?);
        let len = cur.read_u32_le()? as usize;
        if len > crate::core::limits::MAX_RECORD {
            return Err(Error::LengthOverflow { field: "record" });
        }
        let payload = cur.read_slice(len)?.to_vec();
        Ok(RecordEntry { id, kind, flags, payload })
    }
    pub fn is_deferred(&self) -> bool {
        self.flags.contains(RecordFlags::DEFERRED)
            || matches!(self.kind, RecordKind::DeferredVerify)
    }
    pub fn needs_seal(&self) -> bool {
        self.flags.contains(RecordFlags::NEEDS_SEAL)
    }
    pub fn write_to(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.id.0.to_le_bytes());
        out.push(self.kind.tag());
        out.extend_from_slice(&self.flags.to_le().to_le_bytes());
        out.extend_from_slice(&(self.payload.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.payload);
    }
}
