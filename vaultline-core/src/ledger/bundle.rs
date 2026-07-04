//! Complete VRL rotation ledger bundle.


use alloc::vec::Vec;

use crate::core::error::Result;
use crate::ledger::header::LedgerHeader;
use crate::record::entry::RecordEntry;
use crate::seal::envelope::SealEnvelope;
use crate::xref::table::XrefTable;

#[derive(Debug, Clone)]
pub struct LedgerBundle {
    pub header: LedgerHeader,
    pub records: Vec<RecordEntry>,
    pub seals: Vec<SealEnvelope>,
    pub xrefs: XrefTable,
}

impl LedgerBundle {
    pub fn record_count(&self) -> usize {
        self.records.len()
    }
    pub fn has_deferred(&self) -> bool {
        self.records.iter().any(|r| r.is_deferred())
    }
    pub fn epoch(&self) -> u64 {
        self.header.epoch.0
    }
}

pub fn parse_bundle(wire: &[u8]) -> Result<LedgerBundle> {
    crate::decode::pipeline::decode_bundle(wire)
}

pub fn build_bundle(bundle: &LedgerBundle) -> Result<Vec<u8>> {
    crate::export::wire::encode_bundle(bundle)
}
