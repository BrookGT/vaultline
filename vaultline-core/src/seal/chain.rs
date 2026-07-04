//! Chained seal list validation.


use alloc::vec::Vec;

use crate::core::error::{Error, Result};
use crate::seal::envelope::SealEnvelope;

pub fn validate_chain(seals: &[SealEnvelope]) -> Result<()> {
    for w in seals.windows(2) {
        if w[1].key_id < w[0].key_id {
            return Err(Error::seal("key id regression"));
        }
    }
    Ok(())
}

pub fn find_seal<'a>(seals: &'a [SealEnvelope], id: u32) -> Option<&'a SealEnvelope> {
    seals.iter().find(|s| s.id.0 == id)
}
