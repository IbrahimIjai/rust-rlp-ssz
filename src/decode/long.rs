use super::short::decode_list_payload;
use crate::types::{RlpError, RlpItem};

pub fn decode_long_string(input: &[u8]) -> Result<(RlpItem, &[u8]), RlpError> {
    let first = input[0];
    let lenlen = (first - 0xb7) as usize;

    let (length, content_start) = read_long_length(input, 1, lenlen)?;

    let content_end = content_start + length;
    if content_end > input.len() {
        return Err(RlpError::LengthOutOfBounds);
    }

    Ok((RlpItem::Bytes(input[content_start..content_end].to_vec()), &input[content_end..]))
}

pub fn decode_long_list(input: &[u8]) -> Result<(RlpItem, &[u8]), RlpError> {
    let first = input[0];
    let lenlen = (first - 0xf7) as usize;

    let (payload_len, payload_start) = read_long_length(input, 1, lenlen)?;

    let payload_end = payload_start + payload_len;
    if payload_end > input.len() {
        return Err(RlpError::LengthOutOfBounds);
    }

    let payload = &input[payload_start..payload_end];
    let items = decode_list_payload(payload)?;

    Ok((RlpItem::List(items), &input[payload_end..]))
}

fn read_long_length(input: &[u8], offset: usize, lenlen: usize) -> Result<(usize, usize), RlpError> {
    if lenlen == 0 {
        return Err(RlpError::ZeroLenLen);
    }

    let len_end = offset + lenlen;
    if len_end > input.len() {
        return Err(RlpError::InputTooShort);
    }

    let len_bytes = &input[offset..len_end];
    if len_bytes[0] == 0x00 {
        return Err(RlpError::NonCanonicalLength);
    }

    let mut length: usize = 0;
    for &b in len_bytes {
        length = length
            .checked_shl(8)
            .ok_or(RlpError::LengthOutOfBounds)?
            .checked_add(b as usize)
            .ok_or(RlpError::LengthOutOfBounds)?;
    }

    if length <= 55 {
        return Err(RlpError::NonCanonicalLength);
    }

    Ok((length, len_end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_zero_in_length_is_rejected() {
        let input = vec![0xb9, 0x00, 0x38];
        assert_eq!(decode_long_string(&input), Err(RlpError::NonCanonicalLength));
    }

    #[test]
    fn long_string_roundtrip() {
        let data: Vec<u8> = (0u8..56).collect();
        let mut encoded = vec![0xb8, 0x38];
        encoded.extend_from_slice(&data);

        let (item, rest) = decode_long_string(&encoded).unwrap();
        assert_eq!(item, RlpItem::Bytes(data));
        assert!(rest.is_empty());
    }
}
