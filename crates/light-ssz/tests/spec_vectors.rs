use light_ssz::*;

#[test]
fn encode_bool_true() {
    assert_eq!(encode_bool(true), vec![0x01]);
}

#[test]
fn encode_bool_false() {
    assert_eq!(encode_bool(false), vec![0x00]);
}

#[test]
fn encode_u64_1025_matches_spec() {
    assert_eq!(hex::encode(encode_u64(1025)), "0104000000000000");
}

#[test]
fn encode_u64_37_little_endian() {
    assert_eq!(encode_u64(37), vec![37, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn encode_u64_55() {
    assert_eq!(encode_u64(55), vec![55, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn encode_u16_1025() {
    assert_eq!(encode_u16(1025), vec![0x01, 0x04]);
}

#[test]
fn decode_bool_true() {
    assert_eq!(decode_bool(&[0x01]), Ok(true));
}

#[test]
fn decode_bool_false() {
    assert_eq!(decode_bool(&[0x00]), Ok(false));
}

#[test]
fn decode_bool_invalid() {
    assert_eq!(decode_bool(&[0x02]), Err(SszError::InvalidBoolean(0x02)));
}

#[test]
fn decode_u64_1025() {
    let bytes = hex::decode("0104000000000000").unwrap();
    assert_eq!(decode_u64(&bytes), Ok(1025));
}

#[test]
fn decode_u64_too_short() {
    assert_eq!(decode_u64(&[1, 2, 3]), Err(SszError::InputTooShort));
}

#[test]
fn encode_vector_u64_256_512_768() {
    let elements = vec![encode_u64(256), encode_u64(512), encode_u64(768)];
    assert_eq!(
        hex::encode(encode_vector_raw(&elements)),
        "000100000000000000020000000000000003000000000000"
    );
}

#[test]
fn decode_vector_u64_256_512_768() {
    let bytes = hex::decode("000100000000000000020000000000000003000000000000").unwrap();
    let result = decode_vector_fixed(&bytes, 8, 3, |b| decode_u64(b)).unwrap();
    assert_eq!(result, vec![256u64, 512, 768]);
}

#[test]
fn encode_list_u64_1024_2048_3072() {
    let elements = vec![encode_u64(1024), encode_u64(2048), encode_u64(3072)];
    assert_eq!(
        hex::encode(encode_list(&elements, 5).unwrap()),
        "00040000000000000008000000000000000c000000000000"
    );
}

#[test]
fn encode_list_same_as_vector_for_fixed_elements() {
    let elements = vec![encode_u64(1024), encode_u64(2048), encode_u64(3072)];
    assert_eq!(encode_list(&elements, 5).unwrap(), encode_vector_raw(&elements));
}

#[test]
fn decode_list_u64_1024_2048_3072() {
    let bytes = hex::decode("00040000000000000008000000000000000c000000000000").unwrap();
    let result = decode_list_fixed(&bytes, 8, 5, |b| decode_u64(b)).unwrap();
    assert_eq!(result, vec![1024u64, 2048, 3072]);
}

#[test]
fn encode_bitvector_8_pattern() {
    let bits = vec![false, false, true, false, true, true, false, true];
    assert_eq!(hex::encode(encode_bitvector(&bits)), "b4");
}

#[test]
fn encode_bitvector_single_high_bit() {
    let mut bits = vec![false; 8];
    bits[7] = true;
    assert_eq!(hex::encode(encode_bitvector(&bits)), "80");
}

#[test]
fn encode_bitvector_5_compact() {
    let bits = vec![true, false, true, false, true];
    assert_eq!(hex::encode(encode_bitvector(&bits)), "15");
}

#[test]
fn encode_bitlist_three_zeros() {
    let bits = vec![false, false, false];
    assert_eq!(hex::encode(encode_bitlist(&bits, 100).unwrap()), "08");
}

#[test]
fn encode_bitlist_eight_zeros_needs_extra_byte() {
    let bits = vec![false; 8];
    assert_eq!(hex::encode(encode_bitlist(&bits, 8).unwrap()), "0001");
}

#[test]
fn decode_bitvector_b4() {
    let bytes = hex::decode("b4").unwrap();
    let bits = decode_bitvector(&bytes, 8).unwrap();
    assert_eq!(bits, vec![false, false, true, false, true, true, false, true]);
}

#[test]
fn decode_bitlist_08_gives_three_zeros() {
    let bytes = hex::decode("08").unwrap();
    let bits = decode_bitlist(&bytes, 100).unwrap();
    assert_eq!(bits, vec![false, false, false]);
}

#[test]
fn decode_bitlist_roundtrip() {
    let original = vec![true, false, true, true, false];
    let encoded = encode_bitlist(&original, 100).unwrap();
    let decoded = decode_bitlist(&encoded, 100).unwrap();
    assert_eq!(decoded, original);
}

#[test]
fn encode_container_dummy_example() {
    let encoded = encode_container(&[
        ContainerField::Fixed(encode_u32(37)),
        ContainerField::Fixed(encode_u32(55)),
        ContainerField::Variable(vec![1, 2, 3, 4]),
        ContainerField::Fixed(encode_u32(22)),
    ]);
    assert_eq!(encoded, vec![37u8, 0, 0, 0, 55, 0, 0, 0, 16, 0, 0, 0, 22, 0, 0, 0, 1, 2, 3, 4]);
}

#[test]
fn encode_container_alice_variable() {
    let encoded = encode_container(&[ContainerField::Variable(vec![1, 2, 3])]);
    assert_eq!(hex::encode(&encoded), "04000000010203");
}

#[test]
fn encode_container_bob_fixed() {
    let encoded = encode_container(&[ContainerField::Fixed(vec![1, 2, 3])]);
    assert_eq!(hex::encode(&encoded), "010203");
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

#[test]
fn roundtrip_u64() {
    let values = [0u64, 1, 37, 255, 1024, u64::MAX];
    for v in values {
        let encoded = encode_u64(v);
        assert_eq!(decode_u64(&encoded).unwrap(), v, "roundtrip failed for {v}");
    }
}

#[test]
fn roundtrip_container_with_two_variable_fields() {
    let encoded = encode_container(&[
        ContainerField::Variable(vec![1, 2, 3]),
        ContainerField::Variable(vec![4, 5, 6, 7, 8]),
    ]);
    let schema = [None, None];
    let fields = decode_container(&encoded, &schema).unwrap();
    assert_eq!(fields[0], &[1u8, 2, 3]);
    assert_eq!(fields[1], &[4u8, 5, 6, 7, 8]);
}
