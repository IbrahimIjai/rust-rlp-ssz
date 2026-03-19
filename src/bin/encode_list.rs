use rlp::{encode, RlpItem};

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn main() {
    let item = RlpItem::List(vec![
        RlpItem::Bytes(b"cat".to_vec()),
        RlpItem::Bytes(b"dog".to_vec()),
    ]);
    let encoded = encode(&item);

    println!("input: {:?}", item);
    println!("encoded: {:?}", encoded);
    println!("hex: 0x{}", to_hex(&encoded));
}
