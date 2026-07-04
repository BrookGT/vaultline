#![no_main]
use libfuzzer_sys::fuzz_target;
use vaultline_core::core::{Epoch, Limits};

fuzz_target!(|data: &[u8]| {
    if let Ok(bundle) = vaultline_core::parse_bundle(data) {
        let limits = Limits::default();
        let _ = vaultline_core::validate::validate_bundle(&bundle, &limits, Epoch(0));
        let _ = vaultline_core::xref::XrefResolver::new(&bundle).validate_all();
        let _ = vaultline_core::runtime::replay_snapshot(&bundle);
    }
});
