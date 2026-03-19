pub fn encode_bytes(bytes: &[u8]) -> Vec<u8> {
    if bytes.is_empty() {
        return vec![0x80];
    }

    if bytes.len() == 1 && bytes[0] < 0x80 {
        return vec![bytes[0]];
    }

    if bytes.len() <= 55 {
        let mut out = Vec::with_capacity(1 + bytes.len());
        out.push(0x80 + bytes.len() as u8);
        out.extend_from_slice(bytes);
        return out;
    }

    let len_bytes = encode_length_be(bytes.len());
    let mut out = Vec::with_capacity(1 + len_bytes.len() + bytes.len());
    out.push(0xb7 + len_bytes.len() as u8);
    out.extend_from_slice(&len_bytes);
    out.extend_from_slice(bytes);
    out
}

pub(crate) fn encode_length_be(mut n: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    while n > 0 {
        bytes.push(n as u8);
        n >>= 8;
    }
    bytes.reverse();
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_bytes_encodes_to_0x80() {
        assert_eq!(encode_bytes(&[]), vec![0x80]);
    }

    #[test]
    fn single_byte_below_0x80_is_self_describing() {
        assert_eq!(encode_bytes(&[0x00]), vec![0x00]);
        assert_eq!(encode_bytes(&[0x7f]), vec![0x7f]);
    }

    #[test]
    fn single_byte_at_or_above_0x80_gets_prefix() {
        assert_eq!(encode_bytes(&[0x80]), vec![0x81, 0x80]);
    }

    #[test]
    fn short_string_dog() {
        assert_eq!(encode_bytes(b"dog"), vec![0x83, 0x64, 0x6f, 0x67]);
    }

    #[test]
    fn encode_length_be_values() {
        assert_eq!(encode_length_be(0x100), vec![0x01, 0x00]);
        assert_eq!(encode_length_be(0xff), vec![0xff]);
        assert_eq!(encode_length_be(1024), vec![0x04, 0x00]);
    }
}
