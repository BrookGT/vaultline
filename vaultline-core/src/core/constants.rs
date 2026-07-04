//! Named constants for VRL wire format.


pub const WIRE_VERSION: u16 = 1;
pub const HEADER_SIZE: usize = 64;
pub const RECORD_HDR: usize = 12;
pub const SEAL_HDR: usize = 16;
pub const XREF_ENTRY: usize = 8;
pub const TLV_TAG_SIZE: usize = 2;
pub const CRC_INIT: u32 = 0xEDB8_8320;

pub const TAG_BUNDLE_ID: u16 = 0x0001;
pub const TAG_NODE_ID: u16 = 0x0002;
pub const TAG_EPOCH: u16 = 0x0003;
pub const TAG_RECORD: u16 = 0x0010;
pub const TAG_SEAL: u16 = 0x0020;
pub const TAG_XREF: u16 = 0x0030;
pub const TAG_POLICY: u16 = 0x0040;
pub const TAG_PROOF: u16 = 0x0050;

pub mod seal_algo {
    pub const HMAC_SHA256: u8 = 1;
    pub const HMAC_SHA512: u8 = 2;
    pub const ED25519: u8 = 3;
    pub const DEFERRED_STUB: u8 = 0xFF;
}

pub mod xref_kind {
    pub const RECORD_TO_SEAL: u8 = 1;
    pub const SEAL_TO_RECORD: u8 = 2;
    pub const POLICY_BIND: u8 = 3;
    pub const DEFERRED_LINK: u8 = 4;
}
