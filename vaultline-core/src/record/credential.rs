//! Credential material entry in rotation ledger.


use crate::buf::Cursor;
use crate::core::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialEntry {
    pub key_id: u32,
    pub material_hash: [u8; 32],
    pub ttl_secs: u32,
    pub policy_ref: u32,
    pub material_len: u32,
    pub material: Vec<u8>,
}

impl CredentialEntry {
    pub fn parse(cur: &mut Cursor<'_>) -> Result<CredentialEntry> {
        let key_id = cur.read_u32_le()?;
        let material_hash = cur.read_array::<32>()?;
        let ttl_secs = cur.read_u32_le()?;
        let policy_ref = cur.read_u32_le()?;
        let material_len = cur.read_u32_le()? as usize;
        if material_len > crate::core::limits::MAX_KEY_MATERIAL {
            return Err(Error::LengthOverflow { field: "key_material" });
        }
        let material = cur.read_slice(material_len)?.to_vec();
        Ok(CredentialEntry {
            key_id,
            material_hash,
            ttl_secs,
            policy_ref,
            material_len: material_len as u32,
            material,
        })
    }
}
