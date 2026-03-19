use light_rlp::{decode, encode, Encodable, RlpItem};

#[test]
fn encode_empty_string() {
    assert_eq!(encode(&RlpItem::Bytes(vec![])), vec![0x80]);
}

#[test]
fn encode_single_zero_byte() {
    assert_eq!(0u8.rlp_encode(), vec![0x80]);
}

#[test]
fn encode_bool_false() {
    assert_eq!(false.rlp_encode(), vec![0x80]);
}

#[test]
fn encode_bool_true() {
    assert_eq!(true.rlp_encode(), vec![0x01]);
}

#[test]
fn encode_single_byte_below_0x80() {
    assert_eq!(encode(&RlpItem::Bytes(vec![0x7f])), vec![0x7f]);
}

#[test]
fn encode_single_byte_0x00() {
    assert_eq!(encode(&RlpItem::Bytes(vec![0x00])), vec![0x00]);
}

#[test]
fn encode_dog() {
    assert_eq!("dog".rlp_encode(), vec![0x83, 0x64, 0x6f, 0x67]);
}

#[test]
fn encode_cat() {
    assert_eq!("cat".rlp_encode(), vec![0x83, 0x63, 0x61, 0x74]);
}

#[test]
fn encode_list_cat_dog() {
    let item = RlpItem::List(vec![
        RlpItem::Bytes(b"cat".to_vec()),
        RlpItem::Bytes(b"dog".to_vec()),
    ]);
    assert_eq!(
        encode(&item),
        vec![0xc8, 0x83, 0x63, 0x61, 0x74, 0x83, 0x64, 0x6f, 0x67]
    );
}

#[test]
fn encode_empty_list() {
    assert_eq!(encode(&RlpItem::List(vec![])), vec![0xc0]);
}

#[test]
fn encode_nested_empty_lists() {
    let item = RlpItem::List(vec![RlpItem::List(vec![]), RlpItem::List(vec![])]);
    assert_eq!(encode(&item), vec![0xc2, 0xc0, 0xc0]);
}

#[test]
fn encode_deeply_nested_list() {
    let item = RlpItem::List(vec![
        RlpItem::List(vec![]),
        RlpItem::List(vec![RlpItem::List(vec![])]),
        RlpItem::List(vec![
            RlpItem::List(vec![]),
            RlpItem::List(vec![RlpItem::List(vec![])]),
        ]),
    ]);
    assert_eq!(
        encode(&item),
        vec![0xc7, 0xc0, 0xc1, 0xc0, 0xc3, 0xc0, 0xc1, 0xc0]
    );
}

#[test]
fn encode_integer_1024() {
    assert_eq!(1024u64.rlp_encode(), vec![0x82, 0x04, 0x00]);
}

#[test]
fn encode_lorem_ipsum() {
    let s = "Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    let encoded = s.rlp_encode();
    assert_eq!(encoded[0], 0xb8);
    assert_eq!(encoded[1], 56u8);
    assert_eq!(&encoded[2..], s.as_bytes());
}

#[test]
fn decode_dog() {
    let encoded = vec![0x83, 0x64, 0x6f, 0x67];
    assert_eq!(decode(&encoded).unwrap(), RlpItem::Bytes(b"dog".to_vec()));
}

#[test]
fn decode_empty_string() {
    assert_eq!(decode(&[0x80]).unwrap(), RlpItem::Bytes(vec![]));
}

#[test]
fn decode_empty_list() {
    assert_eq!(decode(&[0xc0]).unwrap(), RlpItem::List(vec![]));
}

#[test]
fn decode_cat_dog_list() {
    let encoded = vec![0xc8, 0x83, 0x63, 0x61, 0x74, 0x83, 0x64, 0x6f, 0x67];
    let expected = RlpItem::List(vec![
        RlpItem::Bytes(b"cat".to_vec()),
        RlpItem::Bytes(b"dog".to_vec()),
    ]);
    assert_eq!(decode(&encoded).unwrap(), expected);
}

#[test]
fn decode_direct_byte_range() {
    for b in 0x00u8..=0x7f {
        let decoded = decode(&[b]).unwrap();
        assert_eq!(decoded, RlpItem::Bytes(vec![b]));
    }
}

#[test]
fn decode_lorem_ipsum() {
    let s = "Lorem ipsum dolor sit amet, consectetur adipisicing elit";
    let encoded = s.rlp_encode();
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, RlpItem::Bytes(s.as_bytes().to_vec()));
}

#[test]
fn roundtrip_nested_list() {
    let item = RlpItem::List(vec![
        RlpItem::Bytes(b"hello".to_vec()),
        RlpItem::List(vec![
            RlpItem::Bytes(b"world".to_vec()),
            RlpItem::Bytes(vec![0x01]),
        ]),
    ]);
    let encoded = encode(&item);
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, item);
}

#[test]
fn roundtrip_long_list() {
    let items: Vec<RlpItem> = (0..60).map(|i| RlpItem::Bytes(vec![i as u8])).collect();
    let item = RlpItem::List(items);
    let encoded = encode(&item);
    assert!(
        encoded[0] >= 0xf8,
        "expected long list prefix, got 0x{:02x}",
        encoded[0]
    );
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, item);
}
