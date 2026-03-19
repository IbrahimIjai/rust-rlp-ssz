#[derive(Debug, PartialEq, Eq)]
pub enum ByteRange {
    DirectByte,
    EmptyBytes,
    ShortString,
    LongString,
    ShortList,
    LongList,
}

pub fn classify(first_byte: u8) -> ByteRange {
    match first_byte {
        0x00..=0x7f => ByteRange::DirectByte,
        0x80 => ByteRange::EmptyBytes,
        0x81..=0xb7 => ByteRange::ShortString,
        0xb8..=0xbf => ByteRange::LongString,
        0xc0..=0xf7 => ByteRange::ShortList,
        0xf8..=0xff => ByteRange::LongList,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_boundaries() {
        assert_eq!(classify(0x00), ByteRange::DirectByte);
        assert_eq!(classify(0x7f), ByteRange::DirectByte);
        assert_eq!(classify(0x80), ByteRange::EmptyBytes);
        assert_eq!(classify(0x81), ByteRange::ShortString);
        assert_eq!(classify(0xb7), ByteRange::ShortString);
        assert_eq!(classify(0xb8), ByteRange::LongString);
        assert_eq!(classify(0xbf), ByteRange::LongString);
        assert_eq!(classify(0xc0), ByteRange::ShortList);
        assert_eq!(classify(0xf7), ByteRange::ShortList);
        assert_eq!(classify(0xf8), ByteRange::LongList);
        assert_eq!(classify(0xff), ByteRange::LongList);
    }
}
