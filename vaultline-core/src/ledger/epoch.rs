//! Epoch ordering and gap detection for rotation ledgers.


use crate::core::error::{Error, Result};
use crate::core::types::Epoch;

#[derive(Debug, Clone, Default)]
pub struct EpochTracker {
    last: Option<Epoch>,
    gaps: u32,
}

impl EpochTracker {
    pub fn new() -> EpochTracker {
        EpochTracker::default()
    }
    pub fn observe(&mut self, epoch: Epoch) -> Result<()> {
        if let Some(prev) = self.last {
            if epoch.0 <= prev.0 {
                return Err(Error::protocol("epoch not monotonic"));
            }
            if epoch.0 > prev.0 + 1 {
                self.gaps = self.gaps.saturating_add(1);
            }
        }
        self.last = Some(epoch);
        Ok(())
    }
    pub fn gap_count(&self) -> u32 {
        self.gaps
    }
    pub fn last(&self) -> Option<Epoch> {
        self.last
    }
}
