use super::byte_string::encode_length_be;
use super::encode;
use crate::types::RlpItem;

pub fn encode_list(items: &[RlpItem]) -> Vec<u8> {
    let payload: Vec<u8> = items.iter().flat_map(|item| encode(item)).collect();

    if payload.len() <= 55 {
        let mut out = Vec::with_capacity(1 + payload.len());
        out.push(0xc0 + payload.len() as u8);
        out.extend_from_slice(&payload);
        out
    } else {
        let len_bytes = encode_length_be(payload.len());
        let mut out = Vec::with_capacity(1 + len_bytes.len() + payload.len());
        out.push(0xf7 + len_bytes.len() as u8);
        out.extend_from_slice(&len_bytes);
        out.extend_from_slice(&payload);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RlpItem;

    #[test]
    fn empty_list_encodes_to_0xc0() {
        assert_eq!(encode_list(&[]), vec![0xc0]);
    }

    #[test]
    fn list_of_empty_strings() {
        let items = vec![RlpItem::Bytes(vec![]), RlpItem::Bytes(vec![])];
        assert_eq!(encode_list(&items), vec![0xc2, 0x80, 0x80]);
    }

    #[test]
    fn nested_list() {
        let items = vec![RlpItem::List(vec![]), RlpItem::List(vec![])];
        assert_eq!(encode_list(&items), vec![0xc2, 0xc0, 0xc0]);
    }

    #[test]
    fn list_with_dog_and_cat() {
        let items = vec![
            RlpItem::Bytes(b"dog".to_vec()),
            RlpItem::Bytes(b"cat".to_vec()),
        ];
        let encoded = encode_list(&items);
        assert_eq!(encoded[0], 0xc8);
        assert_eq!(encoded.len(), 9);
    }
}
