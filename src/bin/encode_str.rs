use rlp::Encodable;

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn main() {
    let input = std::env::args().nth(1).unwrap_or_else(|| "dog".to_string());
    let encoded = input.rlp_encode();

    println!("input: {:?}", input);
    println!("encoded: {:?}", encoded);
    println!("hex: 0x{}", to_hex(&encoded));
}
