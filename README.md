# vaultline

**vaultline** is a Rust library for **VRL (Vault Rotation Ledger)** binary bundles:
edge credential rotation snapshots with records, cryptographic seals, cross-references,
and deferred verification scheduling.

It targets constrained edge nodes that receive signed rotation ledger bundles over
intermittent links. The library is synchronous and allocation-aware: a small unsafe
memory substrate backs zero-copy wire views, record staging pools, and fixed-size
decode scratch for seal path resolution.

## Features

| module     | role |
|------------|------|
| `buf`      | arena, slab, pool, cursor, span handles (unsafe substrate) |
| `ledger`   | bundle header, snapshot metadata, epoch tracking |
| `record`   | credential entries, rotation events, policy bindings, deferred proofs |
| `seal`     | seal envelopes, algorithm registry, chain validation |
| `xref`     | cross-reference table, resolver, deferred link graph |
| `decode`   | TLV/LEB128 wire decode pipeline |
| `validate` | schema, rotation chain, deferred verify, quota rules |
| `runtime`  | edge session, snapshot cache, replay planner |
| `catalog`  | credential taxonomy and policy registry |
| `normalize`| canonical bundle ordering |
| `export`   | wire encoder and summary reports |

## Layout

```text
vaultline/
  vaultline-core/     # library
  fuzz/               # libFuzzer targets
  .clusterfuzzlite/   # hermetic OSS-Fuzz-style build
  vendor/             # vendored fuzz deps for offline builds
```

## Build

```bash
cargo check --workspace --offline
cargo test --workspace
```

Fuzzing (Linux + nightly + `cargo install cargo-fuzz`):

```bash
cargo fuzz build --offline -O
cargo fuzz run ledger_fuzzer
```

## License

MIT OR Apache-2.0
