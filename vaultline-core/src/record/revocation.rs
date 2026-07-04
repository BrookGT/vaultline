//! Credential revocation notice.


use crate::buf::Cursor;
use crate::core::error::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevocationNotice {
    pub key_id: u32,
    pub reason: u8,
    pub effective: u64,
}

impl RevocationNotice {
    pub fn parse(cur: &mut Cursor<'_>) -> Result<RevocationNotice> {
        Ok(RevocationNotice {
            key_id: cur.read_u32_le()?,
            reason: cur.read_u8()?,
            effective: cur.read_u64_le()?,
        })
    }
}
