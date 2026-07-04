#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = vaultline_core::decode::decode_bundle(data);
    let _ = vaultline_core::decode::tlv::parse_tlv(data);
    let mut ctx = vaultline_core::decode::stage::DecodeContext::new();
    let mut cur = vaultline_core::buf::Cursor::new(data);
    let _ = vaultline_core::decode::stage::decode_header(&mut cur);
    let _ = ctx.touch_arena_cache(0);
});
