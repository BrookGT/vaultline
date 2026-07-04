#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = vaultline_core::parse_bundle(data);
    let _ = vaultline_core::decode::scan_bundle_offsets(data);
    let _ = vaultline_core::pipeline::ingest(data);
});
