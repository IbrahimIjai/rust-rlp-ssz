mod dispatch;
mod short;
mod long;

use crate::types::{RlpError, RlpItem};
use dispatch::{classify, ByteRange};
use long::{decode_long_list, decode_long_string};
use short::{decode_short_list, decode_short_string};

pub fn decode_one(input: &[u8]) -> Result<(RlpItem, &[u8]), RlpError> {
    let first = *input.first().ok_or(RlpError::InputTooShort)?;

    match classify(first) {
        ByteRange::DirectByte => Ok((RlpItem::Bytes(vec![first]), &input[1..])),
        ByteRange::EmptyBytes => Ok((RlpItem::Bytes(vec![]), &input[1..])),
        ByteRange::ShortString => decode_short_string(input),
        ByteRange::LongString => decode_long_string(input),
        ByteRange::ShortList => decode_short_list(input),
        ByteRange::LongList => decode_long_list(input),
    }
}

pub fn decode(input: &[u8]) -> Result<RlpItem, RlpError> {
    let (item, rest) = decode_one(input)?;
    if !rest.is_empty() {
        return Err(RlpError::TrailingBytes);
    }
    Ok(item)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_direct_byte() {
        assert_eq!(decode(&[0x00]), Ok(RlpItem::Bytes(vec![0x00])));
        assert_eq!(decode(&[0x7f]), Ok(RlpItem::Bytes(vec![0x7f])));
    }

    #[test]
    fn decode_empty() {
        assert_eq!(decode(&[0x80]), Ok(RlpItem::Bytes(vec![])));
        assert_eq!(decode(&[0xc0]), Ok(RlpItem::List(vec![])));
    }

    #[test]
    fn trailing_bytes_is_error() {
        assert_eq!(decode(&[0x80, 0x00]), Err(RlpError::TrailingBytes));
    }
}
