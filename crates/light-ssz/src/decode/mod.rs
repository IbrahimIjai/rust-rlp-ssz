pub mod basic;
pub mod composite;

pub use basic::*;
pub use composite::*;

use crate::types::SszError;

pub trait Decode: Sized {
    fn ssz_decode(bytes: &[u8]) -> Result<Self, SszError>;
}

impl Decode for bool {
    fn ssz_decode(bytes: &[u8]) -> Result<Self, SszError> {
        basic::decode_bool(bytes)
    }
}

impl Decode for u8 {
    fn ssz_decode(bytes: &[u8]) -> Result<Self, SszError> {
        basic::decode_u8(bytes)
    }
}

impl Decode for u16 {
    fn ssz_decode(bytes: &[u8]) -> Result<Self, SszError> {
        basic::decode_u16(bytes)
    }
}

impl Decode for u32 {
    fn ssz_decode(bytes: &[u8]) -> Result<Self, SszError> {
        basic::decode_u32(bytes)
    }
}

impl Decode for u64 {
    fn ssz_decode(bytes: &[u8]) -> Result<Self, SszError> {
        basic::decode_u64(bytes)
    }
}

impl Decode for u128 {
    fn ssz_decode(bytes: &[u8]) -> Result<Self, SszError> {
        basic::decode_u128(bytes)
    }
}
