//! Policy binding attached to credential records.


use crate::buf::Cursor;
use crate::core::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyBinding {
    pub policy_id: u32,
    pub name_len: u16,
    pub name: Vec<u8>,
    pub constraint_mask: u32,
}

impl PolicyBinding {
    pub fn parse(cur: &mut Cursor<'_>) -> Result<PolicyBinding> {
        let policy_id = cur.read_u32_le()?;
        let name_len = cur.read_u16_le()? as usize;
        if name_len > crate::core::limits::MAX_POLICY_NAME {
            return Err(Error::LengthOverflow { field: "policy_name" });
        }
        let name = cur.read_slice(name_len)?.to_vec();
        let constraint_mask = cur.read_u32_le()?;
        Ok(PolicyBinding { policy_id, name_len: name_len as u16, name, constraint_mask })
    }
}
