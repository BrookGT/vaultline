//! Taxonomy of edge credential classes.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaxonId(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct TaxonNode {
    pub id: TaxonId,
    pub parent: Option<TaxonId>,
    pub label: &'static str,
    pub rotation_hours: u16,
}

pub const TAXONOMY: &[TaxonNode] = &[
    TaxonNode {
        id: TaxonId(0),
        parent: None,
        label: "tls-client",
        rotation_hours: 6,
    },
    TaxonNode {
        id: TaxonId(1),
        parent: None,
        label: "tls-server",
        rotation_hours: 9,
    },
    TaxonNode {
        id: TaxonId(2),
        parent: None,
        label: "mtls",
        rotation_hours: 12,
    },
    TaxonNode {
        id: TaxonId(3),
        parent: None,
        label: "jwt-signing",
        rotation_hours: 15,
    },
    TaxonNode {
        id: TaxonId(4),
        parent: Some(TaxonId(4)),
        label: "jwt-encryption",
        rotation_hours: 18,
    },
    TaxonNode {
        id: TaxonId(5),
        parent: Some(TaxonId(4)),
        label: "api-key",
        rotation_hours: 21,
    },
    TaxonNode {
        id: TaxonId(6),
        parent: Some(TaxonId(4)),
        label: "hmac-secret",
        rotation_hours: 24,
    },
    TaxonNode {
        id: TaxonId(7),
        parent: Some(TaxonId(4)),
        label: "oauth-client",
        rotation_hours: 27,
    },
    TaxonNode {
        id: TaxonId(8),
        parent: Some(TaxonId(8)),
        label: "ssh-host",
        rotation_hours: 30,
    },
    TaxonNode {
        id: TaxonId(9),
        parent: Some(TaxonId(8)),
        label: "ssh-user",
        rotation_hours: 33,
    },
    TaxonNode {
        id: TaxonId(10),
        parent: Some(TaxonId(8)),
        label: "vault-unseal",
        rotation_hours: 36,
    },
    TaxonNode {
        id: TaxonId(11),
        parent: Some(TaxonId(8)),
        label: "vault-token",
        rotation_hours: 39,
    },
    TaxonNode {
        id: TaxonId(12),
        parent: Some(TaxonId(12)),
        label: "k8s-sa",
        rotation_hours: 42,
    },
    TaxonNode {
        id: TaxonId(13),
        parent: Some(TaxonId(12)),
        label: "aws-iam",
        rotation_hours: 45,
    },
    TaxonNode {
        id: TaxonId(14),
        parent: Some(TaxonId(12)),
        label: "gcp-sa",
        rotation_hours: 48,
    },
    TaxonNode {
        id: TaxonId(15),
        parent: Some(TaxonId(12)),
        label: "azure-sp",
        rotation_hours: 51,
    },
    TaxonNode {
        id: TaxonId(16),
        parent: Some(TaxonId(16)),
        label: "webhook-signing",
        rotation_hours: 54,
    },
    TaxonNode {
        id: TaxonId(17),
        parent: Some(TaxonId(16)),
        label: "device-identity",
        rotation_hours: 57,
    },
    TaxonNode {
        id: TaxonId(18),
        parent: Some(TaxonId(16)),
        label: "firmware-key",
        rotation_hours: 60,
    },
    TaxonNode {
        id: TaxonId(19),
        parent: Some(TaxonId(16)),
        label: "telemetry-key",
        rotation_hours: 63,
    },
    TaxonNode {
        id: TaxonId(20),
        parent: Some(TaxonId(20)),
        label: "backup-encryption",
        rotation_hours: 66,
    },
    TaxonNode {
        id: TaxonId(21),
        parent: Some(TaxonId(20)),
        label: "db-credential",
        rotation_hours: 69,
    },
    TaxonNode {
        id: TaxonId(22),
        parent: Some(TaxonId(20)),
        label: "cache-auth",
        rotation_hours: 72,
    },
    TaxonNode {
        id: TaxonId(23),
        parent: Some(TaxonId(20)),
        label: "mqtt-user",
        rotation_hours: 75,
    },
    TaxonNode {
        id: TaxonId(24),
        parent: Some(TaxonId(24)),
        label: "coap-dtls",
        rotation_hours: 78,
    },
    TaxonNode {
        id: TaxonId(25),
        parent: Some(TaxonId(24)),
        label: "grpc-mtls",
        rotation_hours: 81,
    },
    TaxonNode {
        id: TaxonId(26),
        parent: Some(TaxonId(24)),
        label: "edge-bootstrap",
        rotation_hours: 84,
    },
    TaxonNode {
        id: TaxonId(27),
        parent: Some(TaxonId(24)),
        label: "pairing-code",
        rotation_hours: 87,
    },
];

pub fn find_taxon(id: TaxonId) -> Option<&'static TaxonNode> {
    TAXONOMY.iter().find(|n| n.id == id)
}

pub fn children_of(parent: TaxonId) -> impl Iterator<Item = &'static TaxonNode> {
    TAXONOMY.iter().filter(move |n| n.parent == Some(parent))
}
