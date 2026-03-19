# rust-light-rlp

Lightweight RLP for Ethereum.

## What It Does

- encodes and decodes RLP byte strings
- encodes and decodes RLP lists
- enforces canonical checks on decode

## Crate Name

Package:

```toml
rust-light-rlp
```

Import:

```rust
use light_rlp::{decode, encode, Encodable, RlpItem};
```

## Example

```rust
use light_rlp::{decode, encode, Encodable, RlpItem};

let item = RlpItem::Bytes(b"dog".to_vec());
let encoded = encode(&item);

assert_eq!(encoded, vec![0x83, 0x64, 0x6f, 0x67]);
assert_eq!(decode(&encoded).unwrap(), item);
assert_eq!(true.rlp_encode(), vec![0x01]);
```

## Run

```powershell
cargo build -p rust-light-rlp
cargo test -p rust-light-rlp
```
