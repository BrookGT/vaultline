//! Normalize individual records.


use crate::core::flags::RecordFlags;
use crate::record::entry::RecordEntry;

pub fn strip_redacted(mut entry: RecordEntry) -> RecordEntry {
    if entry.flags.contains(RecordFlags::REDACTED) {
        entry.payload.clear();
    }
    entry
}
