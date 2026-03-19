# rust-light-ssz

Lightweight SSZ for Ethereum.

## What It Does

- encodes and decodes SSZ primitives
- supports vectors, lists, bitvectors, bitlists, and containers
- exposes low-level helpers

## Crate Name

Package:

```toml
rust-light-ssz
```

Import:

```rust
use light_ssz::{decode_u64, encode_u64};
```

## Example

```rust
use light_ssz::{decode_u64, encode_u64};

let encoded = encode_u64(1025);

assert_eq!(encoded, vec![0x01, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
assert_eq!(decode_u64(&encoded).unwrap(), 1025);
```

## Run

```powershell
cargo build -p rust-light-ssz
cargo test -p rust-light-ssz
```
