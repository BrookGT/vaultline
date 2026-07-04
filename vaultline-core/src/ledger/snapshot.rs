//! Edge credential rotation snapshot metadata.


use crate::core::types::{Epoch, NodeId, RotationSeq};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RotationSnapshot {
    pub node: NodeId,
    pub epoch: Epoch,
    pub seq: RotationSeq,
    pub credential_count: u32,
    pub pending_verify: u32,
    pub last_seal_id: u32,
}

impl RotationSnapshot {
    pub fn new(node: NodeId, epoch: Epoch) -> RotationSnapshot {
        RotationSnapshot {
            node,
            epoch,
            seq: RotationSeq(0),
            credential_count: 0,
            pending_verify: 0,
            last_seal_id: 0,
        }
    }
    pub fn advance(&mut self, seq: RotationSeq) {
        self.seq = seq;
        self.epoch = Epoch(self.epoch.0.saturating_add(1));
    }
    pub fn register_credential(&mut self) {
        self.credential_count = self.credential_count.saturating_add(1);
    }
    pub fn register_deferred(&mut self) {
        self.pending_verify = self.pending_verify.saturating_add(1);
    }
}
