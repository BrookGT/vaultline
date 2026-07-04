//! Wire-level identifiers and epoch types for VRL.


use core::fmt;

pub const MAGIC: [u8; 4] = *b"VRL\x01";

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub [u8; 16]);

impl NodeId {
    pub fn from_bytes(b: [u8; 16]) -> NodeId {
        NodeId(b)
    }
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&x| x == 0)
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeId(")?;
        for (i, b) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ":")?;
            }
            write!(f, "{b:02x}")?;
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Epoch(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecordId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SealId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordKind {
    CredentialEntry = 1,
    RotationEvent = 2,
    PolicyBinding = 3,
    DeferredVerify = 4,
    RevocationNotice = 5,
}

impl RecordKind {
    pub fn from_tag(tag: u8) -> Option<RecordKind> {
        match tag {
            1 => Some(RecordKind::CredentialEntry),
            2 => Some(RecordKind::RotationEvent),
            3 => Some(RecordKind::PolicyBinding),
            4 => Some(RecordKind::DeferredVerify),
            5 => Some(RecordKind::RevocationNotice),
            _ => None,
        }
    }
    pub fn tag(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RotationSeq(pub u64);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn magic() {
        assert_eq!(&MAGIC[..3], b"VRL");
    }
}
