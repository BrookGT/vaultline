//! Rotation event record payload.


use crate::buf::Cursor;
use crate::core::error::Result;
use crate::core::types::RotationSeq;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RotationEvent {
    pub prev_hash: [u8; 32],
    pub new_hash: [u8; 32],
    pub trigger: u8,
    pub timestamp: u64,
    pub seq: RotationSeq,
}

impl RotationEvent {
    pub fn parse(cur: &mut Cursor<'_>) -> Result<RotationEvent> {
        let prev_hash = cur.read_array::<32>()?;
        let new_hash = cur.read_array::<32>()?;
        let trigger = cur.read_u8()?;
        let timestamp = cur.read_u64_le()?;
        let seq = RotationSeq(cur.read_u64_le()?);
        Ok(RotationEvent { prev_hash, new_hash, trigger, timestamp, seq })
    }
    pub fn write_to(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.prev_hash);
        out.extend_from_slice(&self.new_hash);
        out.push(self.trigger);
        out.extend_from_slice(&self.timestamp.to_le_bytes());
        out.extend_from_slice(&self.seq.0.to_le_bytes());
    }
}
