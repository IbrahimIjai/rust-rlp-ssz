use rlp::Encodable;

fn from_hex(s: &str) -> Result<Vec<u8>, String> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    if s.len() % 2 != 0 {
        return Err("hex string must have even length".to_string());
    }

    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    for i in (0..bytes.len()).step_by(2) {
        let pair = std::str::from_utf8(&bytes[i..i + 2]).map_err(|_| "invalid utf-8".to_string())?;
        let byte = u8::from_str_radix(pair, 16).map_err(|_| format!("invalid hex byte: {pair}"))?;
        out.push(byte);
    }
    Ok(out)
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn main() {
    let raw = std::env::args().nth(1).unwrap_or_else(|| "80".to_string());
    let input = from_hex(&raw).expect("failed to parse hex input");
    let encoded = input.rlp_encode();

    println!("input bytes: {:?}", input);
    println!("input hex: 0x{}", raw.trim_start_matches("0x"));
    println!("encoded: {:?}", encoded);
    println!("encoded hex: 0x{}", to_hex(&encoded));
}
