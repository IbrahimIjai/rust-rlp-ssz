# rust-light-codecs

Small Rust workspace for Ethereum serialization libraries.

## What This Project Is

This repo contains two focused libraries:

- `rust-light-rlp`
- `rust-light-ssz`

## RLP

RLP means Recursive Length Prefix.

It is used for Ethereum execution-layer data such as transactions, block headers, and receipts.

RLP works with:

- byte strings
- lists

## SSZ

SSZ means Simple Serialize.

It is used for Ethereum consensus-side data such as fixed-size integers, vectors, lists, bitfields, and containers.

## Quick Usage

### RLP

```rust
use light_rlp::{decode, encode, RlpItem};

let item = RlpItem::Bytes(b"dog".to_vec());
let encoded = encode(&item);

assert_eq!(encoded, vec![0x83, 0x64, 0x6f, 0x67]);
assert_eq!(decode(&encoded).unwrap(), item);
```

### SSZ

```rust
use light_ssz::{decode_u64, encode_u64};

let encoded = encode_u64(1025);

assert_eq!(encoded, vec![0x01, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
assert_eq!(decode_u64(&encoded).unwrap(), 1025);
```

## Folder Structure

```text
.
|-- Cargo.toml
|-- README.md
`-- crates
    |-- light-rlp
    |   |-- Cargo.toml
    |   |-- README.md
    |   |-- src
    |   `-- tests
    `-- light-ssz
        |-- Cargo.toml
        |-- README.md
        |-- src
        `-- tests
```

## Commands

```powershell
cargo build
cargo test
cargo build -p rust-light-rlp
cargo build -p rust-light-ssz
cargo test -p rust-light-rlp
cargo test -p rust-light-ssz
```
