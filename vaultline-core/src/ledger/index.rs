//! Record index for fast lookup inside a bundle.


use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::core::types::RecordId;
use crate::record::entry::RecordEntry;

#[derive(Debug, Default)]
pub struct RecordIndex {
    by_id: BTreeMap<u32, usize>,
    order: Vec<u32>,
}

impl RecordIndex {
    pub fn new() -> RecordIndex {
        RecordIndex::default()
    }
    pub fn rebuild(records: &[RecordEntry]) -> RecordIndex {
        let mut idx = RecordIndex::new();
        for (i, r) in records.iter().enumerate() {
            idx.by_id.insert(r.id.0, i);
            idx.order.push(r.id.0);
        }
        idx
    }
    pub fn get<'a>(&self, records: &'a [RecordEntry], id: RecordId) -> Option<&'a RecordEntry> {
        let pos = *self.by_id.get(&id.0)?;
        records.get(pos)
    }
    pub fn ids(&self) -> &[u32] {
        &self.order
    }
}
