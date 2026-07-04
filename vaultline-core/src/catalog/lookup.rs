//! Fast lookup helpers over catalog and taxonomy.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::catalog::registry::{lookup, CatalogEntry, CATALOG};
use crate::catalog::taxonomy::{find_taxon, TaxonId, TAXONOMY};
use crate::core::types::RecordKind;

#[derive(Debug)]
pub struct LookupIndex {
    by_id: BTreeMap<u32, usize>,
    by_name: BTreeMap<String, usize>,
}

impl LookupIndex {
    pub fn build() -> LookupIndex {
        let mut by_id = BTreeMap::new();
        let mut by_name = BTreeMap::new();
        for (i, e) in CATALOG.iter().enumerate() {
            by_id.insert(e.id, i);
            by_name.insert(e.name.to_string(), i);
        }
        LookupIndex { by_id, by_name }
    }
    pub fn get(&self, id: u32) -> Option<&'static CatalogEntry> {
        let i = *self.by_id.get(&id)?;
        CATALOG.get(i)
    }
    pub fn get_by_name(&self, name: &str) -> Option<&'static CatalogEntry> {
        let i = *self.by_name.get(name)?;
        CATALOG.get(i)
    }
}

pub fn suggest_ttl(kind: RecordKind, taxon: TaxonId) -> u32 {
    let base = find_taxon(taxon).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(86400);
    match kind {
        RecordKind::CredentialEntry => base,
        RecordKind::RotationEvent => base / 2,
        RecordKind::PolicyBinding => base * 2,
        RecordKind::DeferredVerify => base / 4,
        RecordKind::RevocationNotice => 0,
    }
}

pub fn taxon_tls_client_rotation_secs() -> u32 {
    find_taxon(TaxonId(0)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_tls_server_rotation_secs() -> u32 {
    find_taxon(TaxonId(1)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_mtls_rotation_secs() -> u32 {
    find_taxon(TaxonId(2)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_jwt_signing_rotation_secs() -> u32 {
    find_taxon(TaxonId(3)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_jwt_encryption_rotation_secs() -> u32 {
    find_taxon(TaxonId(4)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_api_key_rotation_secs() -> u32 {
    find_taxon(TaxonId(5)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_hmac_secret_rotation_secs() -> u32 {
    find_taxon(TaxonId(6)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_oauth_client_rotation_secs() -> u32 {
    find_taxon(TaxonId(7)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_ssh_host_rotation_secs() -> u32 {
    find_taxon(TaxonId(8)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_ssh_user_rotation_secs() -> u32 {
    find_taxon(TaxonId(9)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_vault_unseal_rotation_secs() -> u32 {
    find_taxon(TaxonId(10)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_vault_token_rotation_secs() -> u32 {
    find_taxon(TaxonId(11)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_k8s_sa_rotation_secs() -> u32 {
    find_taxon(TaxonId(12)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_aws_iam_rotation_secs() -> u32 {
    find_taxon(TaxonId(13)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_gcp_sa_rotation_secs() -> u32 {
    find_taxon(TaxonId(14)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_azure_sp_rotation_secs() -> u32 {
    find_taxon(TaxonId(15)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_webhook_signing_rotation_secs() -> u32 {
    find_taxon(TaxonId(16)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_device_identity_rotation_secs() -> u32 {
    find_taxon(TaxonId(17)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_firmware_key_rotation_secs() -> u32 {
    find_taxon(TaxonId(18)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_telemetry_key_rotation_secs() -> u32 {
    find_taxon(TaxonId(19)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_backup_encryption_rotation_secs() -> u32 {
    find_taxon(TaxonId(20)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_db_credential_rotation_secs() -> u32 {
    find_taxon(TaxonId(21)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_cache_auth_rotation_secs() -> u32 {
    find_taxon(TaxonId(22)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_mqtt_user_rotation_secs() -> u32 {
    find_taxon(TaxonId(23)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_coap_dtls_rotation_secs() -> u32 {
    find_taxon(TaxonId(24)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_grpc_mtls_rotation_secs() -> u32 {
    find_taxon(TaxonId(25)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_edge_bootstrap_rotation_secs() -> u32 {
    find_taxon(TaxonId(26)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}

pub fn taxon_pairing_code_rotation_secs() -> u32 {
    find_taxon(TaxonId(27)).map(|t| t.rotation_hours as u32 * 3600).unwrap_or(3600)
}
