use super::basic::decode_u32;
use crate::types::{SszError, BYTES_PER_OFFSET};

pub fn decode_vector_fixed<T, F>(
    bytes: &[u8],
    element_size: usize,
    expected_count: usize,
    decode_element: F,
) -> Result<Vec<T>, SszError>
where
    F: Fn(&[u8]) -> Result<T, SszError>,
{
    let expected_len = element_size * expected_count;
    if bytes.len() != expected_len {
        return Err(SszError::InvalidLength {
            got: bytes.len(),
            element_size,
        });
    }
    bytes.chunks(element_size).map(decode_element).collect()
}

pub fn decode_list_fixed<T, F>(
    bytes: &[u8],
    element_size: usize,
    max_len: usize,
    decode_element: F,
) -> Result<Vec<T>, SszError>
where
    F: Fn(&[u8]) -> Result<T, SszError>,
{
    if bytes.len() % element_size != 0 {
        return Err(SszError::InvalidLength {
            got: bytes.len(),
            element_size,
        });
    }
    let count = bytes.len() / element_size;
    if count > max_len {
        return Err(SszError::ListTooLong { len: count, max: max_len });
    }
    bytes.chunks(element_size).map(decode_element).collect()
}

pub fn decode_list_variable<T, F>(
    bytes: &[u8],
    max_len: usize,
    decode_element: F,
) -> Result<Vec<T>, SszError>
where
    F: Fn(&[u8]) -> Result<T, SszError>,
{
    if bytes.is_empty() {
        return Ok(vec![]);
    }
    if bytes.len() < BYTES_PER_OFFSET {
        return Err(SszError::InputTooShort);
    }

    let first_offset = decode_u32(&bytes[..BYTES_PER_OFFSET])? as usize;
    if first_offset % BYTES_PER_OFFSET != 0 {
        return Err(SszError::InvalidFirstOffset {
            got: first_offset,
            expected: first_offset,
        });
    }

    let num_elements = first_offset / BYTES_PER_OFFSET;
    if num_elements > max_len {
        return Err(SszError::ListTooLong { len: num_elements, max: max_len });
    }

    let mut offsets = Vec::with_capacity(num_elements);
    for i in 0..num_elements {
        let start = i * BYTES_PER_OFFSET;
        let offset = decode_u32(&bytes[start..start + BYTES_PER_OFFSET])? as usize;
        offsets.push(offset);
    }

    for w in offsets.windows(2) {
        if w[0] >= w[1] {
            return Err(SszError::OffsetsNotAscending);
        }
    }

    let mut result = Vec::with_capacity(num_elements);
    for (i, &offset) in offsets.iter().enumerate() {
        let end = if i + 1 < offsets.len() { offsets[i + 1] } else { bytes.len() };
        if offset > bytes.len() || end > bytes.len() {
            return Err(SszError::OffsetOutOfBounds { offset, len: bytes.len() });
        }
        result.push(decode_element(&bytes[offset..end])?);
    }

    Ok(result)
}

pub fn decode_bitvector(bytes: &[u8], n: usize) -> Result<Vec<bool>, SszError> {
    let expected_bytes = n.div_ceil(8);
    if bytes.len() != expected_bytes {
        return Err(SszError::InvalidLength {
            got: bytes.len(),
            element_size: 1,
        });
    }

    if n % 8 != 0 {
        let last = bytes[bytes.len() - 1];
        let valid_bits = n % 8;
        let mask = !((1u8 << valid_bits) - 1);
        if last & mask != 0 {
            return Err(SszError::ExtraBitsSet);
        }
    }

    let mut bits = Vec::with_capacity(n);
    for i in 0..n {
        let byte = bytes[i / 8];
        bits.push((byte >> (i % 8)) & 1 == 1);
    }
    Ok(bits)
}

pub fn decode_bitlist(bytes: &[u8], max_len: usize) -> Result<Vec<bool>, SszError> {
    if bytes.is_empty() {
        return Err(SszError::MissingSentinelBit);
    }

    let last_byte = bytes[bytes.len() - 1];
    if last_byte == 0 {
        return Err(SszError::MissingSentinelBit);
    }

    let highest_bit_in_last = 7 - last_byte.leading_zeros() as usize;
    let sentinel_pos = (bytes.len() - 1) * 8 + highest_bit_in_last;

    if sentinel_pos > max_len {
        return Err(SszError::ListTooLong { len: sentinel_pos, max: max_len });
    }

    let mut bits = Vec::with_capacity(sentinel_pos);
    for i in 0..sentinel_pos {
        let byte = bytes[i / 8];
        bits.push((byte >> (i % 8)) & 1 == 1);
    }
    Ok(bits)
}

pub fn decode_container<'a>(
    bytes: &'a [u8],
    field_sizes: &[Option<usize>],
) -> Result<Vec<&'a [u8]>, SszError> {
    let fixed_part_size: usize = field_sizes
        .iter()
        .map(|s| match s {
            Some(n) => *n,
            None => BYTES_PER_OFFSET,
        })
        .sum();

    if bytes.len() < fixed_part_size {
        return Err(SszError::InputTooShort);
    }

    let has_variable = field_sizes.iter().any(|s| s.is_none());
    if has_variable {
        let mut cursor = 0;
        let mut first_offset_pos = None;
        for size in field_sizes {
            match size {
                Some(n) => cursor += n,
                None => {
                    first_offset_pos = Some(cursor);
                    break;
                }
            }
        }
        if let Some(pos) = first_offset_pos {
            let first_offset = decode_u32(&bytes[pos..pos + BYTES_PER_OFFSET])? as usize;
            if first_offset != fixed_part_size {
                return Err(SszError::InvalidFirstOffset {
                    got: first_offset,
                    expected: fixed_part_size,
                });
            }
        }
    }

    let mut variable_offsets = Vec::new();
    let mut fixed_cursor = 0;
    for size in field_sizes {
        match size {
            Some(n) => fixed_cursor += n,
            None => {
                let offset = decode_u32(&bytes[fixed_cursor..fixed_cursor + BYTES_PER_OFFSET])? as usize;
                variable_offsets.push(offset);
                fixed_cursor += BYTES_PER_OFFSET;
            }
        }
    }
    variable_offsets.push(bytes.len());

    let mut result = Vec::with_capacity(field_sizes.len());
    let mut fixed_cursor = 0;
    let mut var_idx = 0;

    for size in field_sizes {
        match size {
            Some(n) => {
                result.push(&bytes[fixed_cursor..fixed_cursor + n]);
                fixed_cursor += n;
            }
            None => {
                let start = variable_offsets[var_idx];
                let end = variable_offsets[var_idx + 1];
                if start > bytes.len() || end > bytes.len() {
                    return Err(SszError::OffsetOutOfBounds { offset: start, len: bytes.len() });
                }
                result.push(&bytes[start..end]);
                fixed_cursor += BYTES_PER_OFFSET;
                var_idx += 1;
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::basic::{decode_u32, decode_u64};

    #[test]
    fn decode_vector_u64() {
        let bytes = hex::decode("000100000000000000020000000000000003000000000000").unwrap();
        let result = decode_vector_fixed(&bytes, 8, 3, |b| decode_u64(b)).unwrap();
        assert_eq!(result, vec![256u64, 512, 768]);
    }

    #[test]
    fn decode_list_u64() {
        let bytes = hex::decode("00040000000000000008000000000000000c000000000000").unwrap();
        let result = decode_list_fixed(&bytes, 8, 5, |b| decode_u64(b)).unwrap();
        assert_eq!(result, vec![1024u64, 2048, 3072]);
    }

    #[test]
    fn decode_bitvector_8_b4() {
        let bytes = hex::decode("b4").unwrap();
        let bits = decode_bitvector(&bytes, 8).unwrap();
        assert_eq!(bits, vec![false, false, true, false, true, true, false, true]);
    }

    #[test]
    fn decode_bitlist_three_zeros() {
        let bytes = hex::decode("08").unwrap();
        let bits = decode_bitlist(&bytes, 100).unwrap();
        assert_eq!(bits, vec![false, false, false]);
    }

    #[test]
    fn decode_container_dummy_example() {
        let bytes = vec![37u8, 0, 0, 0, 55, 0, 0, 0, 16, 0, 0, 0, 22, 0, 0, 0, 1, 2, 3, 4];
        let schema = [Some(4), Some(4), None, Some(4)];
        let fields = decode_container(&bytes, &schema).unwrap();

        assert_eq!(decode_u32(fields[0]).unwrap(), 37);
        assert_eq!(decode_u32(fields[1]).unwrap(), 55);
        assert_eq!(fields[2], &[1u8, 2, 3, 4]);
        assert_eq!(decode_u32(fields[3]).unwrap(), 22);
    }
}
