//! Resolve xref targets across bundle components.


use crate::core::error::{Error, Result};
use crate::ledger::bundle::LedgerBundle;
use crate::xref::table::XrefTable;

#[derive(Debug)]
pub struct XrefResolver<'a> {
    bundle: &'a LedgerBundle,
}

impl<'a> XrefResolver<'a> {
    pub fn new(bundle: &'a LedgerBundle) -> XrefResolver<'a> {
        XrefResolver { bundle }
    }
    pub fn resolve_record_to_seal(&self, record_id: u32) -> Result<u32> {
        let targets = self.bundle.xrefs.links_to(record_id);
        targets.first().copied().ok_or(Error::XrefBroken { from: record_id, to: 0 })
    }
    pub fn validate_all(&self) -> Result<()> {
        for e in self.bundle.xrefs.entries() {
            self.check_entry(e.from_id, e.to_id)?;
        }
        Ok(())
    }
    fn check_entry(&self, from: u32, to: u32) -> Result<()> {
        let has_from = self.bundle.records.iter().any(|r| r.id.0 == from)
            || self.bundle.seals.iter().any(|s| s.id.0 == from);
        let has_to = self.bundle.records.iter().any(|r| r.id.0 == to)
            || self.bundle.seals.iter().any(|s| s.id.0 == to);
        if !has_from || !has_to {
            return Err(Error::XrefBroken { from, to });
        }
        Ok(())
    }
}
