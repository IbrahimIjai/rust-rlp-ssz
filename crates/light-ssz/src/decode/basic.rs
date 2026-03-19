use crate::types::SszError;

pub fn decode_bool(bytes: &[u8]) -> Result<bool, SszError> {
    match bytes.first() {
        Some(&0x00) => Ok(false),
        Some(&0x01) => Ok(true),
        Some(&b) => Err(SszError::InvalidBoolean(b)),
        None => Err(SszError::InputTooShort),
    }
}

pub fn decode_u8(bytes: &[u8]) -> Result<u8, SszError> {
    bytes.first().copied().ok_or(SszError::InputTooShort)
}

pub fn decode_u16(bytes: &[u8]) -> Result<u16, SszError> {
    if bytes.len() < 2 {
        return Err(SszError::InputTooShort);
    }
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

pub fn decode_u32(bytes: &[u8]) -> Result<u32, SszError> {
    if bytes.len() < 4 {
        return Err(SszError::InputTooShort);
    }
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub fn decode_u64(bytes: &[u8]) -> Result<u64, SszError> {
    if bytes.len() < 8 {
        return Err(SszError::InputTooShort);
    }
    Ok(u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]))
}

pub fn decode_u128(bytes: &[u8]) -> Result<u128, SszError> {
    if bytes.len() < 16 {
        return Err(SszError::InputTooShort);
    }
    let arr: [u8; 16] = bytes[..16].try_into().unwrap();
    Ok(u128::from_le_bytes(arr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_bool_roundtrip() {
        assert_eq!(decode_bool(&[0x01]), Ok(true));
        assert_eq!(decode_bool(&[0x00]), Ok(false));
    }

    #[test]
    fn decode_bool_invalid_byte() {
        assert_eq!(decode_bool(&[0x02]), Err(SszError::InvalidBoolean(0x02)));
    }

    #[test]
    fn decode_u16_1025() {
        assert_eq!(decode_u16(&[0x01, 0x04]), Ok(1025));
    }

    #[test]
    fn decode_u64_37() {
        assert_eq!(decode_u64(&[37, 0, 0, 0, 0, 0, 0, 0]), Ok(37));
    }

    #[test]
    fn decode_u64_spec_example() {
        let bytes = hex::decode("0104000000000000").unwrap();
        assert_eq!(decode_u64(&bytes), Ok(1025));
    }

    #[test]
    fn decode_u64_too_short() {
        assert_eq!(decode_u64(&[1, 2, 3]), Err(SszError::InputTooShort));
    }
}
