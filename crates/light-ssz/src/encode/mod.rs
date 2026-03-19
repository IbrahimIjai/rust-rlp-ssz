pub mod basic;
pub mod composite;

pub use basic::*;
pub use composite::*;

pub trait Encode {
    fn is_fixed_size() -> bool;

    fn fixed_size() -> Option<usize> {
        None
    }

    fn ssz_encode(&self) -> Vec<u8>;
}

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

impl<T: Encode, const N: usize> Encode for [T; N] {
    fn is_fixed_size() -> bool { T::is_fixed_size() }

    fn fixed_size() -> Option<usize> {
        T::fixed_size().map(|s| s * N)
    }

    fn ssz_encode(&self) -> Vec<u8> {
        self.iter().flat_map(|e| e.ssz_encode()).collect()
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn is_fixed_size() -> bool { false }
    fn fixed_size() -> Option<usize> { None }

    fn ssz_encode(&self) -> Vec<u8> {
        if T::is_fixed_size() {
            self.iter().flat_map(|e| e.ssz_encode()).collect()
        } else {
            encode_variable_list(self)
        }
    }
}

fn encode_variable_list<T: Encode>(items: &[T]) -> Vec<u8> {
    let offset_table_size = items.len() * crate::types::BYTES_PER_OFFSET;
    let mut offsets = Vec::new();
    let mut data = Vec::new();

    for item in items {
        let offset = offset_table_size + data.len();
        offsets.extend_from_slice(&basic::encode_u32(offset as u32));
        data.extend_from_slice(&item.ssz_encode());
    }

    offsets.extend_from_slice(&data);
    offsets
}
