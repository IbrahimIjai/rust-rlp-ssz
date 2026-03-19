pub mod basic;
pub mod composite;

pub use basic::*;
pub use composite::*;

/// The core SSZ encoding trait.
///
/// Implement this for your own types to plug into the SSZ ecosystem.
/// Two methods are required:
///   - `is_fixed_size()` — tells containers whether to inline or offset
///   - `ssz_encode()` — produces the encoded bytes
///
/// One method has a default:
///   - `fixed_size()` — returns Some(N) for fixed-size types, None for variable
pub trait Encode {
    /// Whether this type always encodes to the same number of bytes.
    /// Fixed-size types are inlined in containers.
    /// Variable-size types get a 4-byte offset placeholder in the container's
    /// fixed part, and their data goes to the heap at the end.
    fn is_fixed_size() -> bool;

    /// For fixed-size types, returns the byte length. None for variable types.
    fn fixed_size() -> Option<usize> {
        None
    }

    /// Encode this value to SSZ bytes.
    fn ssz_encode(&self) -> Vec<u8>;
}

// --- Blanket impls for primitive types ---

impl Encode for bool {
    fn is_fixed_size() -> bool { true }
    fn fixed_size() -> Option<usize> { Some(1) }
    fn ssz_encode(&self) -> Vec<u8> { basic::encode_bool(*self) }
}

impl Encode for u8 {
    fn is_fixed_size() -> bool { true }
    fn fixed_size() -> Option<usize> { Some(1) }
    fn ssz_encode(&self) -> Vec<u8> { basic::encode_u8(*self) }
}

impl Encode for u16 {
    fn is_fixed_size() -> bool { true }
    fn fixed_size() -> Option<usize> { Some(2) }
    fn ssz_encode(&self) -> Vec<u8> { basic::encode_u16(*self) }
}

impl Encode for u32 {
    fn is_fixed_size() -> bool { true }
    fn fixed_size() -> Option<usize> { Some(4) }
    fn ssz_encode(&self) -> Vec<u8> { basic::encode_u32(*self) }
}

impl Encode for u64 {
    fn is_fixed_size() -> bool { true }
    fn fixed_size() -> Option<usize> { Some(8) }
    fn ssz_encode(&self) -> Vec<u8> { basic::encode_u64(*self) }
}

impl Encode for u128 {
    fn is_fixed_size() -> bool { true }
    fn fixed_size() -> Option<usize> { Some(16) }
    fn ssz_encode(&self) -> Vec<u8> { basic::encode_u128(*self) }
}

/// Encode a fixed-size array of encodable elements (acts like Vector[T, N]).
impl<T: Encode, const N: usize> Encode for [T; N] {
    fn is_fixed_size() -> bool { T::is_fixed_size() }
    fn fixed_size() -> Option<usize> {
        T::fixed_size().map(|s| s * N)
    }
    fn ssz_encode(&self) -> Vec<u8> {
        self.iter().flat_map(|e| e.ssz_encode()).collect()
    }
}

/// Encode a Vec<T> (acts like List[T, ∞] — caller validates max_len separately).
impl<T: Encode> Encode for Vec<T> {
    fn is_fixed_size() -> bool { false }
    fn fixed_size() -> Option<usize> { None }
    fn ssz_encode(&self) -> Vec<u8> {
        if T::is_fixed_size() {
            // Fixed-element list: just concatenate
            self.iter().flat_map(|e| e.ssz_encode()).collect()
        } else {
            // Variable-element list: needs offset table
            encode_variable_list(self)
        }
    }
}

/// Encode a Vec<T> where T is variable-size.
/// The encoding is an offset table followed by the element data.
fn encode_variable_list<T: Encode>(items: &[T]) -> Vec<u8> {
    let offset_table_size = items.len() * crate::types::BYTES_PER_OFFSET;
    let mut offsets: Vec<u8> = Vec::new();
    let mut data: Vec<u8> = Vec::new();

    for item in items {
        let offset = offset_table_size + data.len();
        offsets.extend_from_slice(&basic::encode_u32(offset as u32));
        data.extend_from_slice(&item.ssz_encode());
    }

    offsets.extend_from_slice(&data);
    offsets
}
