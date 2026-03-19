# light-codecs

This repository is a Cargo workspace for lightweight Ethereum serialization libraries.

## Workspace Layout

- `crates/light-rlp`: RLP library
- `crates/light-ssz`: SSZ library

## Commands

Run all tests:

```powershell
cargo test
```

Run tests for one crate:

```powershell
cargo test -p light-rlp
cargo test -p light-ssz
```
