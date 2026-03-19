mod byte_string;
mod list;

pub use byte_string::encode_bytes;
pub use list::encode_list;

use crate::types::RlpItem;

pub fn encode(item: &RlpItem) -> Vec<u8> {
    match item {
        RlpItem::Bytes(bytes) => encode_bytes(bytes),
        RlpItem::List(items) => encode_list(items),
    }
}
