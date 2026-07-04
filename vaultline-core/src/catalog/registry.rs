//! Credential and policy catalog for VRL bundles.

use crate::core::types::RecordKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogEntry {
    pub id: u32,
    pub kind: RecordKind,
    pub name: &'static str,
    pub ttl_default: u32,
    pub requires_seal: bool,
}

pub const CATALOG: &[CatalogEntry] = &[
    CatalogEntry {
        id: 1000,
        kind: RecordKind::CredentialEntry,
        name: "edge-rotate-daily",
        ttl_default: 3600,
        requires_seal: false,
    },
    CatalogEntry {
        id: 1001,
        kind: RecordKind::PolicyBinding,
        name: "edge-rotate-hourly",
        ttl_default: 7200,
        requires_seal: true,
    },
    CatalogEntry {
        id: 1002,
        kind: RecordKind::RotationEvent,
        name: "bootstrap-once",
        ttl_default: 10800,
        requires_seal: true,
    },
    CatalogEntry {
        id: 1003,
        kind: RecordKind::DeferredVerify,
        name: "mtls-refresh",
        ttl_default: 14400,
        requires_seal: false,
    },
    CatalogEntry {
        id: 1004,
        kind: RecordKind::CredentialEntry,
        name: "api-key-rotate",
        ttl_default: 18000,
        requires_seal: true,
    },
    CatalogEntry {
        id: 1005,
        kind: RecordKind::PolicyBinding,
        name: "ssh-host-rotate",
        ttl_default: 21600,
        requires_seal: true,
    },
    CatalogEntry {
        id: 1006,
        kind: RecordKind::RotationEvent,
        name: "oauth-client-rotate",
        ttl_default: 25200,
        requires_seal: false,
    },
    CatalogEntry {
        id: 1007,
        kind: RecordKind::DeferredVerify,
        name: "vault-token-rotate",
        ttl_default: 28800,
        requires_seal: true,
    },
    CatalogEntry {
        id: 1008,
        kind: RecordKind::CredentialEntry,
        name: "cert-auto-renew",
        ttl_default: 32400,
        requires_seal: true,
    },
    CatalogEntry {
        id: 1009,
        kind: RecordKind::PolicyBinding,
        name: "manual-breakglass",
        ttl_default: 36000,
        requires_seal: false,
    },
    CatalogEntry {
        id: 1010,
        kind: RecordKind::RotationEvent,
        name: "deferred-verify-ok",
        ttl_default: 39600,
        requires_seal: true,
    },
    CatalogEntry {
        id: 1011,
        kind: RecordKind::DeferredVerify,
        name: "strict-chain",
        ttl_default: 43200,
        requires_seal: true,
    },
    CatalogEntry {
        id: 1012,
        kind: RecordKind::CredentialEntry,
        name: "lenient-chain",
        ttl_default: 46800,
        requires_seal: false,
    },
    CatalogEntry {
        id: 1013,
        kind: RecordKind::PolicyBinding,
        name: "quorum-seal",
        ttl_default: 50400,
        requires_seal: true,
    },
    CatalogEntry {
        id: 1014,
        kind: RecordKind::RotationEvent,
        name: "single-seal",
        ttl_default: 54000,
        requires_seal: true,
    },
];

pub fn lookup(id: u32) -> Option<&'static CatalogEntry> {
    CATALOG.iter().find(|e| e.id == id)
}

pub fn by_name(name: &str) -> Option<&'static CatalogEntry> {
    CATALOG.iter().find(|e| e.name == name)
}

pub fn entries_for_kind(kind: RecordKind) -> impl Iterator<Item = &'static CatalogEntry> {
    CATALOG.iter().filter(move |e| e.kind == kind)
}
