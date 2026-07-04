//! Edge cache of recently seen rotation snapshots.


use alloc::collections::BTreeMap;
use crate::core::types::NodeId;
use crate::ledger::snapshot::RotationSnapshot;

#[derive(Debug, Default)]
pub struct SnapshotCache {
    entries: BTreeMap<[u8; 16], RotationSnapshot>,
    capacity: usize,
}

impl SnapshotCache {
    pub fn with_capacity(cap: usize) -> SnapshotCache {
        SnapshotCache { entries: BTreeMap::new(), capacity: cap }
    }
    pub fn insert(&mut self, snap: RotationSnapshot) {
        if self.entries.len() >= self.capacity {
            if let Some(k) = self.entries.keys().next().copied() {
                self.entries.remove(&k);
            }
        }
        self.entries.insert(*snap.node.as_bytes(), snap);
    }
    pub fn get(&self, node: &NodeId) -> Option<&RotationSnapshot> {
        self.entries.get(node.as_bytes())
    }
}
