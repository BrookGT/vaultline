#![no_main]
use libfuzzer_sys::fuzz_target;
use vaultline_core::buf::Cursor;
use vaultline_core::seal::envelope::SealEnvelope;

fuzz_target!(|data: &[u8]| {
    let mut cur = Cursor::new(data);
    let _ = SealEnvelope::parse(&mut cur);
    if let Ok(seal) = SealEnvelope::parse(&mut Cursor::new(data)) {
        let _ = vaultline_core::seal::verify_deferred_stub(&seal);
    }
});
