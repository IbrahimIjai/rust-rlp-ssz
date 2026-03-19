use rlp::Encodable;

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn main() {
    let input = std::env::args()
        .nth(1)
        .map(|s| matches!(s.as_str(), "true" | "1"))
        .unwrap_or(false);
    let encoded = input.rlp_encode();

    println!("input: {}", input);
    println!("encoded: {:?}", encoded);
    println!("hex: 0x{}", to_hex(&encoded));
}
