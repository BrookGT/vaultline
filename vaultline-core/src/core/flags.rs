//! Bundle and record flag words.


use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BundleFlags(pub u32);

impl BundleFlags {
    pub const COMPRESSED: u32 = 0x0000_0001;
    pub const PARTIAL: u32 = 0x0000_0002;
    pub const DEFERRED_OK: u32 = 0x0000_0004;
    pub const EDGE_CACHED: u32 = 0x0000_0008;
    pub const HAS_XREF: u32 = 0x0000_0010;
    pub const SEAL_CHAINED: u32 = 0x0000_0020;

    pub fn empty() -> BundleFlags {
        BundleFlags(0)
    }
    pub fn from_le(v: u32) -> BundleFlags {
        BundleFlags(v)
    }
    pub fn to_le(self) -> u32 {
        self.0
    }
    pub fn contains(self, bit: u32) -> bool {
        self.0 & bit != 0
    }
    pub fn insert(&mut self, bit: u32) {
        self.0 |= bit;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecordFlags(pub u16);

impl RecordFlags {
    pub const PINNED: u16 = 0x0001;
    pub const REDACTED: u16 = 0x0002;
    pub const TOMBSTONE: u16 = 0x0004;
    pub const NEEDS_SEAL: u16 = 0x0008;
    pub const DEFERRED: u16 = 0x0010;

    pub fn empty() -> RecordFlags {
        RecordFlags(0)
    }
    pub fn from_le(v: u16) -> RecordFlags {
        RecordFlags(v)
    }
    pub fn to_le(self) -> u16 {
        self.0
    }
    pub fn contains(self, bit: u16) -> bool {
        self.0 & bit != 0
    }
}

impl fmt::Display for BundleFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BundleFlags(0x{:08x})", self.0)
    }
}
