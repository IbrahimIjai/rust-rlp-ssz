use rlp::{decode, RlpItem};

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

fn main() {
    let hex = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "83646f67".to_string());
    let encoded = from_hex(&hex).expect("failed to parse hex input");
    let decoded = decode(&encoded).expect("failed to decode input");

    println!("hex: 0x{}", hex.trim_start_matches("0x"));
    println!("encoded: {:?}", encoded);
    println!("decoded: {:?}", decoded);

    if let RlpItem::Bytes(bytes) = decoded {
        if let Ok(text) = String::from_utf8(bytes.clone()) {
            println!("as utf8: {:?}", text);
        }
    }
}
