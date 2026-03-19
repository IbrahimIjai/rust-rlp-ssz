# light-rlp

`light-rlp` is a small Rust implementation of Ethereum's RLP format.

The library crate name is `light_rlp`, so code examples import it as:

```rust
use light_rlp::{decode, encode, Encodable, RlpItem};
```

## What RLP Is

RLP means Recursive Length Prefix.

It is Ethereum's serialization format for nested byte strings and lists. Ethereum uses it to encode core protocol data such as:

- transactions
- block headers
- receipts
- trie-related structures

RLP is intentionally simple. At the protocol level, it only has two shapes:

- a byte string
- a list of RLP items

Everything else is reduced to one of those shapes before encoding.

## Core Rules

For byte strings:

- a single byte in `0x00..=0x7f` encodes to itself
- an empty byte string encodes to `0x80`
- a single byte in `0x80..=0xff` encodes as `0x81` followed by the byte
- a byte string of length `0..=55` uses prefix `0x80 + len`
- a byte string longer than `55` uses the long form with `0xb7 + len(len)`

For lists:

- an empty list encodes to `0xc0`
- a list payload of length `0..=55` uses prefix `0xc0 + payload_len`
- a list payload longer than `55` uses the long form with `0xf7 + len(len)`

This library also enforces canonical decoding rules for malformed encodings.

## What This Library Supports

The core value model is:

```rust
pub enum RlpItem {
    Bytes(Vec<u8>),
    List(Vec<RlpItem>),
}
```

The `Encodable` trait is implemented for:

- `Vec<u8>`
- `&[u8]`
- `str`
- `&str`
- `String`
- `u8`
- `u64`
- `u128`
- `usize`
- `bool`
- `Vec<&str>`
- `Vec<String>`
- `Vec<Vec<u8>>`
- `&[&str]`
- `&[String]`
- `&[Vec<u8>]`

Boolean encoding follows the same convention used by Go Ethereum:

- `false -> 0x80`
- `true -> 0x01`

## Usage

### Encode and Decode a Byte String

```rust
use light_rlp::{decode, encode, RlpItem};

let item = RlpItem::Bytes(b"dog".to_vec());
let encoded = encode(&item);

assert_eq!(encoded, vec![0x83, 0x64, 0x6f, 0x67]);
assert_eq!(decode(&encoded).unwrap(), item);
```

### Encode With `Encodable`

```rust
use light_rlp::Encodable;

assert_eq!("dog".rlp_encode(), vec![0x83, 0x64, 0x6f, 0x67]);
assert_eq!(1024u64.rlp_encode(), vec![0x82, 0x04, 0x00]);
assert_eq!(false.rlp_encode(), vec![0x80]);
assert_eq!(true.rlp_encode(), vec![0x01]);
```

### Encode a List

```rust
use light_rlp::{encode, RlpItem};

let item = RlpItem::List(vec![
    RlpItem::Bytes(b"cat".to_vec()),
    RlpItem::Bytes(b"dog".to_vec()),
]);

let encoded = encode(&item);

assert_eq!(
    encoded,
    vec![0xc8, 0x83, 0x63, 0x61, 0x74, 0x83, 0x64, 0x6f, 0x67]
);
```

### Decode Output Shape

`decode` returns `RlpItem`, not typed Rust values:

```rust
use light_rlp::{decode, RlpItem};

assert_eq!(decode(&[0x80]).unwrap(), RlpItem::Bytes(vec![]));
assert_eq!(decode(&[0xc0]).unwrap(), RlpItem::List(vec![]));
assert_eq!(decode(&[0x01]).unwrap(), RlpItem::Bytes(vec![0x01]));
```

That means:

- `false` encodes to `0x80`, then decodes to `RlpItem::Bytes(vec![])`
- `true` encodes to `0x01`, then decodes to `RlpItem::Bytes(vec![0x01])`

The library does not yet provide typed decoders such as `decode_bool()` or `decode_u64()`.

## Running Tests

```powershell
cargo test -p light-rlp
```

## Current Scope

This project currently matches the RLP output rules for the supported byte-string, list, integer, and boolean cases.

It is not yet a full feature-equivalent replacement for Go Ethereum's `rlp` package. In particular, it does not yet provide:

- typed decode helpers
- generic `Vec<T: Encodable>` support
- struct encoding rules like Go Ethereum
- reflection-based encoding behavior
