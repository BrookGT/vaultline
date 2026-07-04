//! Normalize seal ordering.


use alloc::vec::Vec;
use crate::seal::envelope::SealEnvelope;

pub fn sort_seals(mut seals: Vec<SealEnvelope>) -> Vec<SealEnvelope> {
    seals.sort_by_key(|s| s.id.0);
    seals
}
