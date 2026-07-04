#![no_main]
use libfuzzer_sys::fuzz_target;
use vaultline_core::buf::Cursor;
use vaultline_core::record::entry::RecordEntry;

fuzz_target!(|data: &[u8]| {
    let mut cur = Cursor::new(data);
    while cur.remaining() > 0 {
        if RecordEntry::parse(&mut cur).is_err() {
            break;
        }
    }
    let _ = vaultline_core::record::writer::serialize_records(&[]);
});
