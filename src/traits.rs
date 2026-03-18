use crate::encode::encode;
use crate::types::RlpItem;

pub trait Encodable {
    fn to_rlp_item(&self) -> RlpItem;

    fn rlp_encode(&self) -> Vec<u8> {
        encode(&self.to_rlp_item())
    }
} 

impl Encodable for Vec<u8> {
    fn to_rlp_item(&self) -> RlpItem {
        RlpItem::Bytes(self.clone())
    }
}

impl Encodable for &[u8] {
    fn to_rlp_item(&self) -> RlpItem {
        RlpItem::Bytes(self.to_vec())
    }
}

impl Encodable for str {
    fn to_rlp_item(&self) -> RlpItem {
        RlpItem::Bytes(self.as_bytes().to_vec())
    }
}

impl Encodable for String {
    fn to_rlp_item(&self) -> RlpItem {
        RlpItem::Bytes(self.as_bytes().to_vec())
    }
}

impl Encodable for u8 {
    fn to_rlp_item(&self) -> RlpItem {
        if *self == 0 {
            RlpItem::Bytes(vec![])
        } else {
            RlpItem::Bytes(vec![*self])
        }
    }
}

impl Encodable for u64 {
    fn to_rlp_item(&self) -> RlpItem {
        if *self == 0 {
            return RlpItem::Bytes(vec![]);
        }
        let bytes = self.to_be_bytes();
        let stripped = strip_leading_zeros(&bytes);
        RlpItem::Bytes(stripped.to_vec())
    }
}

impl Encodable for u128 {
    fn to_rlp_item(&self) -> RlpItem {
        if *self == 0 {
            return RlpItem::Bytes(vec![]);
        }
        let bytes = self.to_be_bytes();
        let stripped = strip_leading_zeros(&bytes);
        RlpItem::Bytes(stripped.to_vec())
    }
}

impl Encodable for usize {
    fn to_rlp_item(&self) -> RlpItem {
        (*self as u64).to_rlp_item()
    }
}

impl Encodable for Vec<&str> {
    fn to_rlp_item(&self) -> RlpItem {
        RlpItem::List(self.iter().map(|x| x.to_rlp_item()).collect())
    }
}

impl Encodable for Vec<String> {
    fn to_rlp_item(&self) -> RlpItem {
        RlpItem::List(self.iter().map(|x| x.to_rlp_item()).collect())
    }
}

impl Encodable for Vec<Vec<u8>> {
    fn to_rlp_item(&self) -> RlpItem {
        RlpItem::List(self.iter().map(|x| x.to_rlp_item()).collect())
    }
}

impl Encodable for &[&str] {
    fn to_rlp_item(&self) -> RlpItem {
        RlpItem::List(self.iter().map(|x| x.to_rlp_item()).collect())
    }
}

impl Encodable for &[String] {
    fn to_rlp_item(&self) -> RlpItem {
        RlpItem::List(self.iter().map(|x| x.to_rlp_item()).collect())
    }
}

impl Encodable for &[Vec<u8>] {
    fn to_rlp_item(&self) -> RlpItem {
        RlpItem::List(self.iter().map(|x| x.to_rlp_item()).collect())
    }
}

impl Encodable for &str {
    fn to_rlp_item(&self) -> RlpItem {
        RlpItem::Bytes(self.as_bytes().to_vec())
    }
}

fn strip_leading_zeros(bytes: &[u8]) -> &[u8] {
    let first_nonzero = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    &bytes[first_nonzero..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_zero_is_empty() {
        assert_eq!(0u8.rlp_encode(), vec![0x80]);
    }

    #[test]
    fn u8_self_describing() {
        assert_eq!(1u8.rlp_encode(), vec![0x01]);
        assert_eq!(0x7fu8.rlp_encode(), vec![0x7f]);
    }

    #[test]
    fn u64_1024() {
        assert_eq!(1024u64.rlp_encode(), vec![0x82, 0x04, 0x00]);
    }

    #[test]
    fn string_dog() {
        assert_eq!("dog".rlp_encode(), vec![0x83, 0x64, 0x6f, 0x67]);
    }

    #[test]
    fn vec_of_strings() {
        let items: Vec<&str> = vec!["cat", "dog"];
        let encoded = items.as_slice().rlp_encode();
        assert_eq!(encoded[0], 0xc8);
    }
}
