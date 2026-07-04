//! Record payload parser dispatching by kind.


use crate::buf::Cursor;
use crate::core::error::Result;
use crate::core::types::RecordKind;
use crate::record::credential::CredentialEntry;
use crate::record::deferred::DeferredVerify;
use crate::record::entry::RecordEntry;
use crate::record::event::RotationEvent;
use crate::record::policy::PolicyBinding;
use crate::record::revocation::RevocationNotice;

#[derive(Debug, Clone)]
pub enum ParsedPayload {
    Credential(CredentialEntry),
    Event(RotationEvent),
    Policy(PolicyBinding),
    Deferred(DeferredVerify),
    Revocation(RevocationNotice),
    Opaque(Vec<u8>),
}

pub fn parse_payload(entry: &RecordEntry) -> Result<ParsedPayload> {
    let mut cur = Cursor::new(&entry.payload);
    match entry.kind {
        RecordKind::CredentialEntry => {
            Ok(ParsedPayload::Credential(CredentialEntry::parse(&mut cur)?))
        }
        RecordKind::RotationEvent => Ok(ParsedPayload::Event(RotationEvent::parse(&mut cur)?)),
        RecordKind::PolicyBinding => Ok(ParsedPayload::Policy(PolicyBinding::parse(&mut cur)?)),
        RecordKind::DeferredVerify => {
            Ok(ParsedPayload::Deferred(DeferredVerify::parse(&mut cur)?))
        }
        RecordKind::RevocationNotice => {
            Ok(ParsedPayload::Revocation(RevocationNotice::parse(&mut cur)?))
        }
    }
}
