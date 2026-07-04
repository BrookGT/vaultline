//! Seal envelope wrapping record digests.


use alloc::vec::Vec;

use crate::buf::Cursor;
use crate::core::error::{Error, Result};
use crate::core::types::SealId;
use crate::seal::algo::SealAlgorithm;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SealEnvelope {
    pub id: SealId,
    pub algo: SealAlgorithm,
    pub key_id: u32,
    pub nonce: [u8; 12],
    pub digest: Vec<u8>,
}

impl SealEnvelope {
    pub fn parse(cur: &mut Cursor<'_>) -> Result<SealEnvelope> {
        let id = SealId(cur.read_u32_le()?);
        let algo = SealAlgorithm::from_tag(cur.read_u8()?)
            .ok_or(Error::protocol("seal algo"))?;
        let key_id = cur.read_u32_le()?;
        let nonce = cur.read_array::<12>()?;
        let dlen = algo.digest_len();
        let digest = cur.read_slice(dlen)?.to_vec();
        Ok(SealEnvelope { id, algo, key_id, nonce, digest })
    }
    pub fn write_to(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.id.0.to_le_bytes());
        out.push(self.algo.tag());
        out.extend_from_slice(&self.key_id.to_le_bytes());
        out.extend_from_slice(&self.nonce);
        out.extend_from_slice(&self.digest);
    }
}
