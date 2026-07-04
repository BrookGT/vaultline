# VRL Wire Format

**VRL** (Vault Rotation Ledger) is a binary format for edge credential rotation
snapshots. Bundles contain a fixed header, a sequence of typed records, seal
envelopes, and a cross-reference table linking records to seals and deferred
verification targets.

## Header (64 bytes)

| offset | size | field |
|--------|------|-------|
| 0 | 4 | magic `VRL\x01` |
| 4 | 2 | wire version (currently 1) |
| 6 | 8 | epoch (u64 LE) |
| 14 | 16 | node id |
| 30 | 4 | record count |
| 34 | 4 | seal section offset |
| 38 | 4 | xref table offset |
| 42 | 4 | bundle flags |
| 46 | 4 | header CRC32 |
| 50 | 14 | reserved (zero) |

## Record entry

Each record is length-prefixed:

| field | size |
|-------|------|
| id | u32 LE |
| kind tag | u8 |
| flags | u16 LE |
| payload length | u32 LE |
| payload | variable |

### Record kinds

| tag | kind |
|-----|------|
| 1 | CredentialEntry — key material hash, TTL, policy ref |
| 2 | RotationEvent — prev/new hash chain, trigger, timestamp |
| 3 | PolicyBinding — named policy constraints |
| 4 | DeferredVerify — proof blob scheduled for later verification |
| 5 | RevocationNotice — credential revocation marker |

## Seal envelope

| field | size |
|-------|------|
| seal id | u32 LE |
| algorithm | u8 |
| key id | u32 LE |
| nonce | 12 bytes |
| digest | algorithm-dependent |

Seal algorithms: HMAC-SHA256 (32), HMAC-SHA512 (64), Ed25519 (64), deferred stub (16).

## Cross-reference table

| field | size |
|-------|------|
| entry count | u32 LE |
| entries | count × 9 bytes |

Each xref entry:

| field | size |
|-------|------|
| kind | u8 |
| from id | u32 LE |
| to id | u32 LE |

Xref kinds: record→seal (1), seal→record (2), policy bind (3), deferred link (4).

## Deferred verification

Records tagged `DeferredVerify` carry a target epoch and proof blob. The runtime
planner collects due items when `now >= verify_at`. Seals marked `DEFERRED_STUB`
accept deferred proofs until the edge node performs full verification.

## Limits

- Max bundle size: 8 MiB
- Max single record: 256 KiB
- Max xref entries: 65536
- Max parse depth: 64
