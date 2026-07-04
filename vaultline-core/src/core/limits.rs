//! Canonical size and depth limits for VRL bundles.


pub const MAX_DEPTH: usize = 64;
pub const MAX_BUNDLE: usize = 8 * 1024 * 1024;
pub const MAX_RECORD: usize = 256 * 1024;
pub const MAX_SEAL: usize = 4096;
pub const MAX_XREF: usize = 65536;
pub const MAX_DEFERRED: usize = 1024;
pub const MAX_NODE_ID: usize = 32;
pub const MAX_POLICY_NAME: usize = 128;
pub const MAX_KEY_MATERIAL: usize = 8192;

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub max_depth: usize,
    pub max_bundle: usize,
    pub max_record: usize,
    pub max_xref: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            max_depth: MAX_DEPTH,
            max_bundle: MAX_BUNDLE,
            max_record: MAX_RECORD,
            max_xref: MAX_XREF,
        }
    }
}

impl Limits {
    pub fn clamp_record(&self, len: usize) -> usize {
        len.min(self.max_record)
    }
    pub fn clamp_bundle(&self, len: usize) -> usize {
        len.min(self.max_bundle)
    }
    pub fn check_depth(&self, depth: usize) -> bool {
        depth <= self.max_depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults() {
        let l = Limits::default();
        assert!(l.max_depth >= 8);
    }
}
