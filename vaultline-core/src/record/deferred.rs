//! Deferred verification proof blob.


use crate::buf::Cursor;
use crate::core::error::{Error, Result};
use crate::core::types::Epoch;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeferredVerify {
    pub verify_at: Epoch,
    pub proof_len: u32,
    pub proof: Vec<u8>,
    pub target_record: u32,
}

impl DeferredVerify {
    pub fn parse(cur: &mut Cursor<'_>) -> Result<DeferredVerify> {
        let verify_at = Epoch(cur.read_u64_le()?);
        let proof_len = cur.read_u32_le()? as usize;
        if proof_len > crate::core::limits::MAX_DEFERRED {
            return Err(Error::LengthOverflow { field: "deferred_proof" });
        }
        let proof = cur.read_slice(proof_len)?.to_vec();
        let target_record = cur.read_u32_le()?;
        Ok(DeferredVerify { verify_at, proof_len: proof_len as u32, proof, target_record })
    }
    pub fn is_due(&self, now: Epoch) -> bool {
        now.0 >= self.verify_at.0
    }
}
