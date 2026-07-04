//! VRL bundle header layout and validation.


use crate::buf::Cursor;
use crate::core::constants::{HEADER_SIZE, WIRE_VERSION};
use crate::core::error::{Error, Result};
use crate::core::flags::BundleFlags;
use crate::core::types::{Epoch, NodeId, MAGIC};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerHeader {
    pub version: u16,
    pub epoch: Epoch,
    pub node: NodeId,
    pub record_count: u32,
    pub seal_offset: u32,
    pub xref_offset: u32,
    pub flags: BundleFlags,
    pub header_crc: u32,
}

impl LedgerHeader {
    pub fn parse(cur: &mut Cursor<'_>) -> Result<LedgerHeader> {
        if cur.remaining() < HEADER_SIZE {
            return Err(Error::UnexpectedEof);
        }
        let magic = cur.read_array::<4>()?;
        if magic != MAGIC {
            return Err(Error::protocol("bad magic"));
        }
        let version = cur.read_u16_le()?;
        if version > WIRE_VERSION {
            return Err(Error::protocol("unsupported version"));
        }
        let epoch = Epoch(cur.read_u64_le()?);
        let node = NodeId(cur.read_array::<16>()?);
        let record_count = cur.read_u32_le()?;
        let seal_offset = cur.read_u32_le()?;
        let xref_offset = cur.read_u32_le()?;
        let flags = BundleFlags::from_le(cur.read_u32_le()?);
        let header_crc = cur.read_u32_le()?;
        Ok(LedgerHeader {
            version,
            epoch,
            node,
            record_count,
            seal_offset,
            xref_offset,
            flags,
            header_crc,
        })
    }
    pub fn write_to(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&MAGIC);
        out.extend_from_slice(&self.version.to_le_bytes());
        out.extend_from_slice(&self.epoch.0.to_le_bytes());
        out.extend_from_slice(self.node.as_bytes());
        out.extend_from_slice(&self.record_count.to_le_bytes());
        out.extend_from_slice(&self.seal_offset.to_le_bytes());
        out.extend_from_slice(&self.xref_offset.to_le_bytes());
        out.extend_from_slice(&self.flags.to_le().to_le_bytes());
        out.extend_from_slice(&self.header_crc.to_le_bytes());
    }
}

pub fn checksum_header(h: &LedgerHeader) -> u32 {
    let mut crc = crate::core::constants::CRC_INIT;
    let mut buf = Vec::with_capacity(HEADER_SIZE);
    h.write_to(&mut buf);
    for b in &buf[4..] {
        crc = crc32_step(crc, *b);
    }
    crc
}

fn crc32_step(mut crc: u32, b: u8) -> u32 {
    crc ^= b as u32;
    for _ in 0..8 {
        crc = if crc & 1 != 0 {
            (crc >> 1) ^ 0xEDB8_8320
        } else {
            crc >> 1
        };
    }
    crc
}
