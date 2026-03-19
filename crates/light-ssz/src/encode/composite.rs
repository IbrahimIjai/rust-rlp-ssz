use crate::types::{SszError, BYTES_PER_OFFSET};
use super::basic::encode_u32;


pub fn encode_vector<F>(elements: &[impl AsRef<[u8]>], _encode_element: F) -> Vec<u8>
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    elements.iter().flat_map(|e| e.as_ref().to_vec()).collect()
}


pub fn encode_vector_raw(encoded_elements: &[Vec<u8>]) -> Vec<u8> {
    encoded_elements.iter().flatten().cloned().collect()
}

pub fn encode_list(
    encoded_elements: &[Vec<u8>],
    max_len: usize,
) -> Result<Vec<u8>, SszError> {
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

    // Pack the actual bits
    for (i, &bit) in bits.iter().enumerate() {
        if bit {
            bytes[i / 8] |= 1 << (i % 8);
        }
    }

    // Set the sentinel bit at position bits.len()
    let sentinel_pos = bits.len();
    bytes[sentinel_pos / 8] |= 1 << (sentinel_pos % 8);

    Ok(bytes)
}

pub enum ContainerField {
    /// Fixed-size field — bytes go directly inline in order.
    Fixed(Vec<u8>),
    /// Variable-size field — a 4-byte offset goes inline, bytes go to heap.
    Variable(Vec<u8>),
}

pub fn encode_container(fields: &[ContainerField]) -> Vec<u8> {
    // Step 1: compute the total fixed-part size
    let fixed_part_size: usize = fields.iter().map(|f| match f {
        ContainerField::Fixed(b) => b.len(),
        ContainerField::Variable(_) => BYTES_PER_OFFSET,
    }).sum();

    let mut fixed_part: Vec<u8> = Vec::new();
    let mut heap: Vec<u8> = Vec::new();

    // Step 2: build fixed part and heap
    for field in fields {
        match field {
            ContainerField::Fixed(bytes) => {
                fixed_part.extend_from_slice(bytes);
            }
            ContainerField::Variable(bytes) => {
                // offset = where this field's data starts in the full output
                // = fixed_part_size + current heap length
                let offset = fixed_part_size + heap.len();
                fixed_part.extend_from_slice(&encode_u32(offset as u32));
                heap.extend_from_slice(bytes);
            }
        }
    }

    // Step 3: concatenate
    fixed_part.extend_from_slice(&heap);
    fixed_part
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode::basic::{encode_u64, encode_u32};

    #[test]
    fn vector_u64_concatenates() {
        // Vector[uint64, 3]([256, 512, 768])
        // spec hex: 000100000000000000020000000000000003000000000000
        let elements: Vec<Vec<u8>> = vec![
            encode_u64(256),
            encode_u64(512),
            encode_u64(768),
        ];
        let encoded = encode_vector_raw(&elements);
        assert_eq!(hex::encode(&encoded), "000100000000000000020000000000000003000000000000");
    }

    #[test]
    fn list_u64_same_as_vector_when_toplevel() {
        // List[uint64, 5]([1024, 2048, 3072])
        // spec hex: 00040000000000000008000000000000000c000000000000
        let elements: Vec<Vec<u8>> = vec![
            encode_u64(1024),
            encode_u64(2048),
            encode_u64(3072),
        ];
        let encoded = encode_list(&elements, 5).unwrap();
        assert_eq!(hex::encode(&encoded), "00040000000000000008000000000000000c000000000000");
    }

    #[test]
    fn list_exceeding_max_is_error() {
        let elements: Vec<Vec<u8>> = vec![encode_u64(1), encode_u64(2), encode_u64(3)];
        assert!(encode_list(&elements, 2).is_err());
    }

    #[test]
    fn bitvector_10_matches_spec() {
        // Bitvector[10](1,0,1,1,0,1,0,0,1,0) → spec says "b480"
        // but python spec says Bitvector[8](0,0,1,0,1,1,0,1) = 'b4'
        // Let's verify our packing: bits[0]=0,bits[1]=0,bits[2]=1,bits[3]=0,bits[4]=1,bits[5]=1,bits[6]=0,bits[7]=1
        // byte = 0+(0<<1)+(1<<2)+(0<<3)+(1<<4)+(1<<5)+(0<<6)+(1<<7) = 4+16+32+128 = 180 = 0xb4 ✓
        let bits = vec![false, false, true, false, true, true, false, true];
        let encoded = encode_bitvector(&bits);
        assert_eq!(hex::encode(&encoded), "b4");
    }

    #[test]
    fn bitvector_single_bit() {
        // Bitvector[8](0,0,0,0,0,0,0,1) = '80'
        // bit[7]=1 → 1<<7 = 128 = 0x80
        let mut bits = vec![false; 8];
        bits[7] = true;
        let encoded = encode_bitvector(&bits);
        assert_eq!(hex::encode(&encoded), "80");
    }

    #[test]
    fn bitlist_three_zeros() {
        // Bitlist[100]([0,0,0]) → spec says '08'
        // bits=[F,F,F], sentinel at index 3 → byte = 0|(0<<1)|(0<<2)|(1<<3) = 8 = 0x08
        let bits = vec![false, false, false];
        let encoded = encode_bitlist(&bits, 100).unwrap();
        assert_eq!(hex::encode(&encoded), "08");
    }

    #[test]
    fn bitlist_eight_zeros_needs_extra_byte() {
        // Bitlist[8]([0,0,0,0,0,0,0,0]) → spec says '0001'
        // 8 real bits fill byte0=0x00, sentinel goes to byte1 bit0 = 0x01
        let bits = vec![false; 8];
        let encoded = encode_bitlist(&bits, 8).unwrap();
        assert_eq!(hex::encode(&encoded), "0001");
    }

    #[test]
    fn container_with_fixed_and_variable_fields() {
        // ethereum.org example:
        // Dummy { number1:37(u32), number2:55(u32), vector:vec![1,2,3,4], number3:22(u32) }
        // expected: [37,0,0,0, 55,0,0,0, 16,0,0,0, 22,0,0,0, 1,2,3,4]
        // fixed_part_size = 4+4+4+4 = 16, offset for vector = 16+0 = 16
        let fields = vec![
            ContainerField::Fixed(encode_u32(37)),
            ContainerField::Fixed(encode_u32(55)),
            ContainerField::Variable(vec![1, 2, 3, 4]),
            ContainerField::Fixed(encode_u32(22)),
        ];
        let encoded = encode_container(&fields);
        let expected = vec![
            37u8,0,0,0,   // number1
            55,0,0,0,     // number2
            16,0,0,0,     // offset for vector (points to byte 16)
            22,0,0,0,     // number3
            1,2,3,4,      // vector data on heap
        ];
        assert_eq!(encoded, expected);
    }

    #[test]
    fn container_alice_vs_bob() {
        // From spec: Alice has variable x=[1,2,3], Bob has fixed x=[1,2,3]
        // Alice: x is List[uint8,3] → variable → offset(4 bytes) + data
        //   fixed_part_size = 4 (just the offset)
        //   offset = 4, data = [1,2,3]
        //   → [04,00,00,00, 01,02,03] = '04000000010203'
        let alice = encode_container(&[
            ContainerField::Variable(vec![1, 2, 3]),
        ]);
        assert_eq!(hex::encode(&alice), "04000000010203");

        // Bob: x is Vector[uint8,3] → fixed → inline
        //   → [01,02,03] = '010203'
        let bob = encode_container(&[
            ContainerField::Fixed(vec![1, 2, 3]),
        ]);
        assert_eq!(hex::encode(&bob), "010203");
    }
}
