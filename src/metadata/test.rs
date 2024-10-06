#[cfg(test)]
mod tests {
    use crate::metadata::Value;
    use candid::types::leb128;
    use num_traits::ToPrimitive;
    use serde_bytes::ByteBuf;
    use std::{collections::BTreeMap, io::Cursor};

    #[test]
    fn test_value_creation() {
        assert!(matches!(Value::from(42u64), Value::Nat(_)));
        assert!(matches!(Value::from(42i64), Value::Int(_)));
        assert!(matches!(Value::from(true), Value::Bool(true)));
        assert!(matches!(Value::from("hello"), Value::Text(_)));
        assert!(matches!(Value::from(vec![1u8, 2u8, 3u8]), Value::Blob(_)));
    }

    #[test]
    fn test_value_conversions() {
        let nat_value = Value::from(42u64);
        assert_eq!(
            nat_value.as_nat().map(|n| n.0.to_u64().unwrap()),
            Some(42u64)
        );

        let int_value = Value::from(-42i64);
        assert_eq!(
            int_value.as_int().map(|i| i.0.to_i64().unwrap()),
            Some(-42i64)
        );

        let bool_value = Value::from(true);
        assert_eq!(bool_value.as_bool(), Some(true));

        let text_value = Value::from("hello");
        assert_eq!(text_value.as_text(), Some(&"hello".to_string()));

        let blob_value = Value::from(vec![1u8, 2u8, 3u8]);
        assert_eq!(
            blob_value.as_blob(),
            Some(&ByteBuf::from(vec![1u8, 2u8, 3u8]))
        );
    }

    #[test]
    fn test_hash_consistency() {
        let value1 = Value::from(42u64);
        let value2 = Value::from(42u64);
        assert_eq!(value1.hash(), value2.hash());

        let value3 = Value::from(-42i64);
        let value4 = Value::from(-42i64);
        assert_eq!(value3.hash(), value4.hash());

        let value5 = Value::from("hello");
        let value6 = Value::from("hello");
        assert_eq!(value5.hash(), value6.hash());
    }

    #[test]
    fn test_hash_differences() {
        let value1 = Value::from(42u64);
        let value2 = Value::from(43u64);
        assert_ne!(value1.hash(), value2.hash());

        let value3 = Value::from(-42i64);
        let value4 = Value::from(-43i64);
        assert_ne!(value3.hash(), value4.hash());

        let value5 = Value::from("hello");
        let value6 = Value::from("world");
        assert_ne!(value5.hash(), value6.hash());
    }

    #[test]
    fn test_complex_structures() {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), Value::from(42u64));
        map.insert("key2".to_string(), Value::from("value"));

        let complex_value = Value::Array(vec![
            Value::from(1u64),
            Value::from("text"),
            Value::Map(map),
        ]);

        // Test that we can create and hash complex structures
        let _hash = complex_value.hash();

        // Test that identical complex structures have the same hash
        let mut map2 = BTreeMap::new();
        map2.insert("key1".to_string(), Value::from(42u64));
        map2.insert("key2".to_string(), Value::from("value"));

        let complex_value2 = Value::Array(vec![
            Value::from(1u64),
            Value::from("text"),
            Value::Map(map2),
        ]);

        assert_eq!(complex_value.hash(), complex_value2.hash());
    }

    #[test]
    fn test_display() {
        assert_eq!(Value::from(42u64).to_string(), "42");
        assert_eq!(Value::from(-42i64).to_string(), "-42");
        assert_eq!(Value::from(true).to_string(), "true");
        assert_eq!(Value::from("hello").to_string(), "hello");
        assert_eq!(Value::from(vec![1u8, 2u8, 3u8]).to_string(), "010203");
    }

    #[test]
    fn check_interface_spec_example() {
        let value = Value::Map({
            let mut m = BTreeMap::new();
            m.insert("request_type".to_string(), Value::text("call"));
            m.insert(
                "canister_id".to_string(),
                Value::blob(b"\x00\x00\x00\x00\x00\x00\x04\xD2".to_vec()),
            );
            m.insert("method_name".to_string(), Value::text("hello"));
            m.insert("arg".to_string(), Value::blob(b"DIDL\x00\xFD*".to_vec()));

            m
        });
        assert_eq!(
            hex::encode(value.hash()),
            "8781291c347db32a9d8c10eb62b710fce5a93be676474c42babc74c51858f94b"
        );
    }

    #[test]
    fn test_leb128() {
        for (n, expected) in [
            (0u128, vec![0]),
            (624485u128, vec![0xe5, 0x8e, 0x26]),
            (
                1677770607672807382u128,
                vec![0xd6, 0x9f, 0xb7, 0xe7, 0xa7, 0xef, 0xa8, 0xa4, 0x17],
            ),
        ] {
            let mut buf = vec![];
            leb128::encode_nat(&mut buf, n).unwrap();
            assert_eq!(buf, expected, "invalid encoding of integer {}", n);

            // Also test decoding
            let mut cursor = Cursor::new(buf);
            let decoded = leb128::decode_nat(&mut cursor).unwrap();
            assert_eq!(decoded, n, "invalid decoding of integer {}", n);
        }
    }

    #[test]
    fn test_sleb128() {
        for (n, expected) in [(0i128, vec![0]), (-123456i128, vec![0xc0, 0xbb, 0x78])] {
            let mut buf = vec![];
            leb128::encode_int(&mut buf, n).unwrap();
            assert_eq!(buf, expected, "invalid encoding of integer {}", n);

            // Also test decoding
            let mut cursor = Cursor::new(buf);
            let decoded = leb128::decode_int(&mut cursor).unwrap();
            assert_eq!(decoded, n, "invalid decoding of integer {}", n);
        }
    }

    #[test]
    fn test_test_vectors() {
        let test_vectors = vec![
            (
                Value::Nat(42u64.into()),
                "684888c0ebb17f374298b65ee2807526c066094c701bcc7ebbe1c1095f494fc1",
            ),
            (
                Value::Int((-42).into()),
                "de5a6f78116eca62d7fc5ce159d23ae6b889b365a1739ad2cf36f925a140d0cc",
            ),
            (
                Value::text("Hello, World!"),
                "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f",
            ),
            (
                Value::blob(hex::decode("01020304").unwrap()),
                "9f64a747e1b97f131fabb6b447296c9b6f0201e79fb3c5356e6c77e89b6a806a",
            ),
            (
                Value::Array(vec![
                    Value::Nat(3u8.into()),
                    Value::text("foo"),
                    Value::blob(hex::decode("0506").unwrap()),
                ]),
                "514a04011caa503990d446b7dec5d79e19c221ae607fb08b2848c67734d468d6",
            ),
            (
                Value::map(vec![
                    (
                        "from",
                        Value::blob(
                            hex::decode("00abcdef0012340056789a00bcdef000012345678900abcdef01")
                                .unwrap(),
                        ),
                    ),
                    (
                        "to",
                        Value::blob(
                            hex::decode("00ab0def0012340056789a00bcdef000012345678900abcdef01")
                                .unwrap(),
                        ),
                    ),
                    ("amount", Value::Nat(42u32.into())),
                    ("created_at", Value::Nat(1699218263u64.into())),
                    ("memo", Value::Nat(0u128.into())),
                ]),
                "c56ece650e1de4269c5bdeff7875949e3e2033f85b2d193c2ff4f7f78bdcfc75",
            ),
        ];

        for (input, expected) in test_vectors {
            assert_eq!(
                input.hash().to_vec(),
                hex::decode(expected).unwrap(),
                "input: {}",
                input
            );
        }
    }
}
