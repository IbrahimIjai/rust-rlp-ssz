
pub fn encode_bool(value: bool) -> Vec<u8> {
    vec![if value { 0x01 } else { 0x00 }]
}


pub fn encode_u8(value: u8) -> Vec<u8> {
    vec![value]
}


pub fn encode_u16(value: u16) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

pub fn encode_u32(value: u32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}


pub fn encode_u64(value: u64) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Encode a u128 as SSZ little-endian — 16 bytes.
pub fn encode_u128(value: u128) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

pub fn encode_u256(lo: u128, hi: u128) -> Vec<u8> {
    let mut out = Vec::with_capacity(32);
    out.extend_from_slice(&lo.to_le_bytes());
    out.extend_from_slice(&hi.to_le_bytes());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_true_is_0x01() {
        assert_eq!(encode_bool(true), vec![0x01]);
    }

    #[test]
    fn bool_false_is_0x00() {
        assert_eq!(encode_bool(false), vec![0x00]);
    }

    #[test]
    fn u16_1025_is_little_endian() {
        // 1025 = 0x0401 big-endian → [0x01, 0x04] little-endian
        assert_eq!(encode_u16(1025), vec![0x01, 0x04]);
    }

    #[test]
    fn u64_37_is_padded_to_8_bytes() {
        assert_eq!(encode_u64(37), vec![37, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn u64_1025() {
        // spec example: '0104000000000000'
        let encoded = encode_u64(1025);
        assert_eq!(hex::encode(&encoded), "0104000000000000");
    }

    #[test]
    fn u64_zero_is_eight_zero_bytes() {
        assert_eq!(encode_u64(0), vec![0u8; 8]);
    }

    #[test]
    fn u32_offset_little_endian() {
        // offset value 16 used in container example → [16, 0, 0, 0]
        assert_eq!(encode_u32(16), vec![16, 0, 0, 0]);
    }
}
