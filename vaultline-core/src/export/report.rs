//! Human-readable validation reports.


use alloc::string::String;
use alloc::format;
use crate::validate::rules::ValidationReport;

pub fn format_report(r: &ValidationReport) -> String {
    format!(
        "schema={} xref={} chain={} deferred_due={}",
        r.schema_ok, r.xref_ok, r.chain_ok, r.deferred_due
    )
}
