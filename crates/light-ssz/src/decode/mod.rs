pub mod basic;
pub mod composite;

pub use basic::*;
pub use composite::*;

use crate::types::SszError;

/// The core SSZ decoding trait.
///
/// Implement this for your own types to decode from raw SSZ bytes.
/// The decoder needs to know the schema — SSZ is NOT self-describing.
pub trait Decode: Sized {
    /// Decode from a complete byte slice.
    fn ssz_decode(bytes: &[u8]) -> Result<Self, SszError>;
}

// --- Blanket impls for primitive types ---

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
