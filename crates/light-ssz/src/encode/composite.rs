use super::basic::encode_u32;
use crate::types::{SszError, BYTES_PER_OFFSET};

pub fn encode_vector<F>(elements: &[impl AsRef<[u8]>], _encode_element: F) -> Vec<u8>
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    elements.iter().flat_map(|e| e.as_ref().to_vec()).collect()
}

pub fn encode_vector_raw(encoded_elements: &[Vec<u8>]) -> Vec<u8> {
    encoded_elements.iter().flatten().cloned().collect()
}

pub fn encode_list(encoded_elements: &[Vec<u8>], max_len: usize) -> Result<Vec<u8>, SszError> {
    if encoded_elements.len() > max_len {
        return Err(SszError::ListTooLong {
            len: encoded_elements.len(),
            max: max_len,
        });
    }
    Ok(encoded_elements.iter().flatten().cloned().collect())
}

pub fn encode_bitvector(bits: &[bool]) -> Vec<u8> {
    let num_bytes = bits.len().div_ceil(8);
    let mut bytes = vec![0u8; num_bytes];
    for (i, &bit) in bits.iter().enumerate() {
        if bit {
            bytes[i / 8] |= 1 << (i % 8);
        }
    }
    bytes
}

pub fn encode_bitlist(bits: &[bool], max_len: usize) -> Result<Vec<u8>, SszError> {
    if bits.len() > max_len {
        return Err(SszError::ListTooLong {
            len: bits.len(),
            max: max_len,
        });
    }

    let total_bits = bits.len() + 1;
    let num_bytes = total_bits.div_ceil(8);
    let mut bytes = vec![0u8; num_bytes];

    for (i, &bit) in bits.iter().enumerate() {
        if bit {
            bytes[i / 8] |= 1 << (i % 8);
        }
    }

    let sentinel_pos = bits.len();
    bytes[sentinel_pos / 8] |= 1 << (sentinel_pos % 8);

    Ok(bytes)
}

pub enum ContainerField {
    Fixed(Vec<u8>),
    Variable(Vec<u8>),
}

pub fn encode_container(fields: &[ContainerField]) -> Vec<u8> {
    let fixed_part_size: usize = fields
        .iter()
        .map(|f| match f {
            ContainerField::Fixed(b) => b.len(),
            ContainerField::Variable(_) => BYTES_PER_OFFSET,
        })
        .sum();

    let mut fixed_part = Vec::new();
    let mut heap = Vec::new();

    for field in fields {
        match field {
            ContainerField::Fixed(bytes) => fixed_part.extend_from_slice(bytes),
            ContainerField::Variable(bytes) => {
                let offset = fixed_part_size + heap.len();
                fixed_part.extend_from_slice(&encode_u32(offset as u32));
                heap.extend_from_slice(bytes);
            }
        }
    }

    fixed_part.extend_from_slice(&heap);
    fixed_part
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode::basic::{encode_u32, encode_u64};

    #[test]
    fn vector_u64_concatenates() {
        let elements = vec![encode_u64(256), encode_u64(512), encode_u64(768)];
        let encoded = encode_vector_raw(&elements);
        assert_eq!(hex::encode(&encoded), "000100000000000000020000000000000003000000000000");
    }

    #[test]
    fn list_u64_same_as_vector_when_toplevel() {
        let elements = vec![encode_u64(1024), encode_u64(2048), encode_u64(3072)];
        let encoded = encode_list(&elements, 5).unwrap();
        assert_eq!(hex::encode(&encoded), "00040000000000000008000000000000000c000000000000");
    }

    #[test]
    fn list_exceeding_max_is_error() {
        let elements = vec![encode_u64(1), encode_u64(2), encode_u64(3)];
        assert!(encode_list(&elements, 2).is_err());
    }

    #[test]
    fn bitvector_10_matches_spec() {
        let bits = vec![false, false, true, false, true, true, false, true];
        let encoded = encode_bitvector(&bits);
        assert_eq!(hex::encode(&encoded), "b4");
    }

    #[test]
    fn bitvector_single_bit() {
        let mut bits = vec![false; 8];
        bits[7] = true;
        let encoded = encode_bitvector(&bits);
        assert_eq!(hex::encode(&encoded), "80");
    }

    #[test]
    fn bitlist_three_zeros() {
        let bits = vec![false, false, false];
        let encoded = encode_bitlist(&bits, 100).unwrap();
        assert_eq!(hex::encode(&encoded), "08");
    }

    #[test]
    fn bitlist_eight_zeros_needs_extra_byte() {
        let bits = vec![false; 8];
        let encoded = encode_bitlist(&bits, 8).unwrap();
        assert_eq!(hex::encode(&encoded), "0001");
    }

    #[test]
    fn container_with_fixed_and_variable_fields() {
        let fields = vec![
            ContainerField::Fixed(encode_u32(37)),
            ContainerField::Fixed(encode_u32(55)),
            ContainerField::Variable(vec![1, 2, 3, 4]),
            ContainerField::Fixed(encode_u32(22)),
        ];
        let encoded = encode_container(&fields);
        let expected = vec![37u8, 0, 0, 0, 55, 0, 0, 0, 16, 0, 0, 0, 22, 0, 0, 0, 1, 2, 3, 4];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn container_alice_vs_bob() {
        let alice = encode_container(&[ContainerField::Variable(vec![1, 2, 3])]);
        assert_eq!(hex::encode(&alice), "04000000010203");

        let bob = encode_container(&[ContainerField::Fixed(vec![1, 2, 3])]);
        assert_eq!(hex::encode(&bob), "010203");
    }
}
