//! Seal algorithm registry.


use crate::core::constants::seal_algo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SealAlgorithm {
    HmacSha256,
    HmacSha512,
    Ed25519,
    DeferredStub,
}

impl SealAlgorithm {
    pub fn from_tag(tag: u8) -> Option<SealAlgorithm> {
        match tag {
            seal_algo::HMAC_SHA256 => Some(SealAlgorithm::HmacSha256),
            seal_algo::HMAC_SHA512 => Some(SealAlgorithm::HmacSha512),
            seal_algo::ED25519 => Some(SealAlgorithm::Ed25519),
            seal_algo::DEFERRED_STUB => Some(SealAlgorithm::DeferredStub),
            _ => None,
        }
    }
    pub fn tag(self) -> u8 {
        match self {
            SealAlgorithm::HmacSha256 => seal_algo::HMAC_SHA256,
            SealAlgorithm::HmacSha512 => seal_algo::HMAC_SHA512,
            SealAlgorithm::Ed25519 => seal_algo::ED25519,
            SealAlgorithm::DeferredStub => seal_algo::DEFERRED_STUB,
        }
    }
    pub fn digest_len(self) -> usize {
        match self {
            SealAlgorithm::HmacSha256 => 32,
            SealAlgorithm::HmacSha512 => 64,
            SealAlgorithm::Ed25519 => 64,
            SealAlgorithm::DeferredStub => 16,
        }
    }
}
