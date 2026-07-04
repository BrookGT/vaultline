//! Seal verification and deferred stub checks.


use crate::core::error::{Error, Result};
use crate::record::entry::RecordEntry;
use crate::seal::envelope::SealEnvelope;

pub fn verify_seal(seal: &SealEnvelope, record: &RecordEntry) -> Result<()> {
    let expected = compute_stub_digest(record);
    if seal.digest.len() < expected.len() {
        return Err(Error::seal("digest short"));
    }
    if seal.digest[..expected.len()] != expected {
        return Err(Error::seal("digest mismatch"));
    }
    Ok(())
}

pub fn verify_deferred_stub(seal: &SealEnvelope) -> Result<()> {
    if !matches!(seal.algo, crate::seal::algo::SealAlgorithm::DeferredStub) {
        return Err(Error::VerifyDeferred { detail: "not deferred seal" });
    }
    Ok(())
}

fn compute_stub_digest(record: &RecordEntry) -> [u8; 16] {
    let mut out = [0u8; 16];
    let mut acc: u64 = 0xcbf29ce484222325;
    for b in &record.payload {
        acc ^= *b as u64;
        acc = acc.wrapping_mul(0x100000001b3);
    }
    acc ^= record.id.0 as u64;
    out[..8].copy_from_slice(&acc.to_le_bytes());
    out[8..].copy_from_slice(&(record.payload.len() as u64).to_le_bytes());
    out
}
