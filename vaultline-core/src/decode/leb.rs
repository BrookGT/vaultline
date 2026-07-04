//! LEB128 decode for compact record indices.


use crate::buf::Cursor;
use crate::core::error::{Error, Result};

pub fn read_u32_leb(cur: &mut Cursor<'_>) -> Result<u32> {
    let mut result: u32 = 0;
    let mut shift = 0;
    for _ in 0..5 {
        let byte = cur.read_u8()?;
        result |= ((byte & 0x7f) as u32) << shift;
        if byte & 0x80 == 0 {
            return Ok(result);
        }
        shift += 7;
    }
    Err(Error::InvalidEncoding { scheme: "leb128" })
}

pub fn write_u32_leb(mut value: u32, out: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}
