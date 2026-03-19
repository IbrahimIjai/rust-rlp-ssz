
pub mod types;
pub mod encode;
pub mod decode;

pub use types::{SszError, BYTES_PER_OFFSET};
pub use encode::{Encode, encode_bool, encode_u8, encode_u16, encode_u32, encode_u64, encode_u128};
pub use encode::{encode_bitvector, encode_bitlist, encode_container, encode_list, encode_vector_raw, ContainerField};
pub use decode::{Decode, decode_bool, decode_u8, decode_u16, decode_u32, decode_u64, decode_u128};
pub use decode::{decode_bitvector, decode_bitlist, decode_container, decode_vector_fixed, decode_list_fixed};
