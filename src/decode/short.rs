use super::decode_one;
use crate::types::{RlpError, RlpItem};

pub fn decode_short_string(input: &[u8]) -> Result<(RlpItem, &[u8]), RlpError> {
    let first = input[0];
    let length = (first - 0x80) as usize;

    if length == 1 {
        let next = *input.get(1).ok_or(RlpError::InputTooShort)?;
        if next < 0x80 {
            return Err(RlpError::NonCanonicalSingleByte);
        }
    }

    let start = 1;
    let end = start + length;
    if end > input.len() {
        return Err(RlpError::LengthOutOfBounds);
    }

    Ok((RlpItem::Bytes(input[start..end].to_vec()), &input[end..]))
}

pub fn decode_short_list(input: &[u8]) -> Result<(RlpItem, &[u8]), RlpError> {
    let first = input[0];
    let payload_len = (first - 0xc0) as usize;

    let payload_start = 1;
    let payload_end = payload_start + payload_len;
    if payload_end > input.len() {
        return Err(RlpError::LengthOutOfBounds);
    }

    let payload = &input[payload_start..payload_end];
    let items = decode_list_payload(payload)?;

    Ok((RlpItem::List(items), &input[payload_end..]))
}

pub fn decode_list_payload(mut payload: &[u8]) -> Result<Vec<RlpItem>, RlpError> {
    let mut items = Vec::new();
    while !payload.is_empty() {
        let (item, rest) = decode_one(payload)?;
        items.push(item);
        payload = rest;
    }
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_string_dog() {
        let (item, rest) = decode_short_string(&[0x83, 0x64, 0x6f, 0x67]).unwrap();
        assert_eq!(item, RlpItem::Bytes(b"dog".to_vec()));
        assert!(rest.is_empty());
    }

    #[test]
    fn empty_list_0xc0() {
        let (item, rest) = decode_short_list(&[0xc0]).unwrap();
        assert_eq!(item, RlpItem::List(vec![]));
        assert!(rest.is_empty());
    }

    #[test]
    fn non_canonical_single_byte_is_rejected() {
        assert_eq!(
            decode_short_string(&[0x81, 0x7f]),
            Err(RlpError::NonCanonicalSingleByte)
        );
    }
}
