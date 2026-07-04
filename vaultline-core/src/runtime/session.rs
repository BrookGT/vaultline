//! Runtime session for edge rotation ledger replay.


use crate::core::Limits;
use crate::decode::context::Session as DecodeSession;
use crate::ledger::bundle::LedgerBundle;

#[derive(Debug)]
pub struct RuntimeSession {
    pub decode: DecodeSession,
    pub bundles_loaded: u32,
}

impl RuntimeSession {
    pub fn new(limits: Limits) -> RuntimeSession {
        RuntimeSession { decode: DecodeSession::new(limits), bundles_loaded: 0 }
    }
    pub fn ingest(&mut self, bundle: &LedgerBundle) {
        self.bundles_loaded = self.bundles_loaded.saturating_add(1);
        self.decode.bump_records(bundle.record_count() as u32);
    }
}
