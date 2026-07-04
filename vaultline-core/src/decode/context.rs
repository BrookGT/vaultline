//! Shared decode session state.


use crate::core::Limits;
use crate::decode::stage::DecodeContext;

#[derive(Debug)]
pub struct Session {
    pub limits: Limits,
    pub ctx: DecodeContext,
    pub records_seen: u32,
}

impl Session {
    pub fn new(limits: Limits) -> Session {
        Session { limits, ctx: DecodeContext::new(), records_seen: 0 }
    }
    pub fn bump_records(&mut self, n: u32) {
        self.records_seen = self.records_seen.saturating_add(n);
    }
}
