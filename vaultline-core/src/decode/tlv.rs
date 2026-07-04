//! TLV records in VRL wire bodies.


use alloc::vec::Vec;

use crate::buf::Cursor;
use crate::core::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlvRecord {
    pub tag: u16,
    pub value: Vec<u8>,
}

pub fn parse_tlv(data: &[u8]) -> Result<(Vec<TlvRecord>, usize)> {
    let mut cur = Cursor::new(data);
    let count = cur.read_u32_le()? as usize;
    let mut records = Vec::with_capacity(count.min(1024));
    for _ in 0..count {
        let tag = cur.read_u16_le()?;
        let len = cur.read_u32_le()? as usize;
        if len > crate::core::limits::MAX_RECORD {
            return Err(Error::LengthOverflow { field: "tlv" });
        }
        let value = cur.read_slice(len)?.to_vec();
        records.push(TlvRecord { tag, value });
    }
    Ok((records, cur.position()))
}
