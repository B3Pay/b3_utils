use std::collections::{BTreeMap, HashMap};

use candid::{CandidType, Deserialize, Int, Nat};
use num_traits::cast::ToPrimitive;
use serde::Serialize;
use serde_bytes::ByteBuf;
use sha2::{Digest, Sha256};

const INT128_BUF_SIZE: usize = 19;
pub type Hash = [u8; 32];

pub type Metadata = HashMap<String, Value>;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Blob(ByteBuf),
    Text(String),
    Bool(bool),
    Nat(Nat),
    Nat64(u64),
    Int(Int),
    Array(Vec<Value>),
    Map(BTreeMap<String, Value>),
}

impl Value {
    pub fn text(t: impl ToString) -> Self {
        Self::Text(t.to_string())
    }

    pub fn blob(t: impl Into<Vec<u8>>) -> Self {
        Self::Blob(ByteBuf::from(t.into()))
    }

    pub fn map<S, V>(v: V) -> Self
    where
        S: ToString,
        V: IntoIterator<Item = (S, Value)>,
    {
        Self::Map(v.into_iter().map(|(s, v)| (s.to_string(), v)).collect())
    }

    /// Computes the representation-independent hash of a value.
    pub fn hash(&self) -> Hash {
        match self {
            Value::Nat(nat) => {
                let mut buf = vec![];
                nat.encode(&mut buf).expect("bug: cannot encode a Nat");
                Sha256::digest(&buf).into()
            }
            Value::Nat64(n) => {
                let mut buf = [0u8; INT128_BUF_SIZE];
                let offset = leb128(&mut buf, *n as u128);
                Sha256::digest(&buf[0..=offset]).into()
            }
            Value::Int(int) => {
                let v = int
                    .0
                    .to_i128()
                    .expect("BUG: blocks cannot contain integers that do not fit into the 128-bit representation");
                let mut buf = [0u8; INT128_BUF_SIZE];
                //TODO: Int should only use sleb128. Due to CiboriumValue only using Integer this is however not possible right now
                //      Unsigned Integers should be represented through Nat or Nat65: https://dfinity.atlassian.net/browse/FI-764
                let offset = match v >= 0 {
                    true => leb128(&mut buf, v as u128),
                    false => sleb128(&mut buf, v),
                };
                Sha256::digest(&buf[0..=offset]).into()
            }
            Value::Blob(bytes) => Sha256::digest(bytes).into(),
            Value::Text(text) => Sha256::digest(text.as_bytes()).into(),
            Value::Bool(b) => Sha256::digest(&[*b as u8]).into(),
            Value::Array(values) => {
                let mut hasher = Sha256::new();
                for v in values.iter() {
                    hasher.update(v.hash());
                }
                hasher.finalize().into()
            }
            Value::Map(map) => {
                let mut hpairs = Vec::with_capacity(map.len());
                for (k, v) in map.iter() {
                    let key_hash: Hash = Sha256::digest(k.as_bytes()).into();
                    hpairs.push((key_hash, v.hash()));
                }

                hpairs.sort_unstable();

                let mut hasher = Sha256::new();
                for (khash, vhash) in hpairs.iter() {
                    hasher.update(&khash[..]);
                    hasher.update(&vhash[..]);
                }
                hasher.finalize().into()
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Blob(bytes) => write!(f, "{}", hex::encode(bytes.as_ref())),
            Value::Text(text) => write!(f, "{}", text),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nat(nat) => write!(f, "{}", nat),
            Value::Nat64(nat64) => write!(f, "{}", nat64),
            Value::Int(int) => write!(f, "{}", int),
            Value::Array(array) => {
                write!(f, "Array(")?;
                let mut first = true;
                for e in array {
                    if first {
                        first = false
                    } else {
                        write!(f, ", ")?
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            Value::Map(map) => {
                write!(f, "Map(")?;
                let mut first = true;
                for (k, v) in map {
                    if first {
                        first = false
                    } else {
                        write!(f, ", ")?
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, ")")
            }
        }
    }
}

/// Encodes a 128-bit integer using unsigned LEB-128 encoding.
/// Returns the index of the last valid byte in the buffer.
fn leb128(buf: &mut [u8; INT128_BUF_SIZE], v: u128) -> usize {
    let mut n = v;
    let mut i = 0;

    loop {
        debug_assert!(i < INT128_BUF_SIZE);

        let byte = n as u8;
        n >>= 7;

        if n == 0 {
            buf[i] = byte & 0x7f;
            return i;
        } else {
            buf[i] = byte | 0x80;
            i += 1;
        }
    }
}

/// Encodes a 128-bit integer using signed LEB-128 encoding.
/// Returns the index of the last valid byte in the buffer.
///
fn sleb128(buf: &mut [u8; INT128_BUF_SIZE], v: i128) -> usize {
    let mut n = v;
    let mut i = 0;
    loop {
        debug_assert!(i < INT128_BUF_SIZE);

        let byte = n as u8;
        // Keep the sign bit for testing
        n >>= 6;
        if n == 0 || n == -1 {
            buf[i] = byte & 0x7f;
            return i;
        } else {
            // Remove the sign bit
            n >>= 1;
            buf[i] = byte | 0x80;
            i += 1;
        }
    }
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
    let mut buf = [0; INT128_BUF_SIZE];
    for (n, b) in [
        (0, &[0][..]),
        (624485, &[0xe5, 0x8e, 0x26][..]),
        (
            1677770607672807382,
            &[0xd6, 0x9f, 0xb7, 0xe7, 0xa7, 0xef, 0xa8, 0xa4, 0x17][..],
        ),
    ] {
        let i = leb128(&mut buf, n);
        assert_eq!(&buf[0..=i], b, "invalid encoding of integer {}", n);
    }
}

#[test]
fn test_sleb128() {
    let mut buf = [0; INT128_BUF_SIZE];
    for (n, b) in [(0, &[0][..]), (-123456, &[0xc0, 0xbb, 0x78][..])] {
        let i = sleb128(&mut buf, n);
        assert_eq!(&buf[0..=i], b, "invalid encoding of integer {}", n);
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

impl Value {
    pub fn entry(key: impl ToString, val: impl Into<Value>) -> (String, Self) {
        (key.to_string(), val.into())
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Int(Int::from(n))
    }
}

impl From<i128> for Value {
    fn from(n: i128) -> Self {
        Value::Int(Int::from(n))
    }
}

impl From<u64> for Value {
    fn from(n: u64) -> Self {
        Value::Nat(Nat::from(n))
    }
}

impl From<u128> for Value {
    fn from(n: u128) -> Self {
        Value::Nat(Nat::from(n))
    }
}

impl From<Nat> for Value {
    fn from(n: Nat) -> Self {
        Value::Nat(n)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Text(s)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        Value::Text(s.to_string())
    }
}

impl From<Vec<u8>> for Value {
    fn from(bytes: Vec<u8>) -> Value {
        Value::Blob(ByteBuf::from(bytes))
    }
}

impl<'a> From<&'a [u8]> for Value {
    fn from(bytes: &'a [u8]) -> Value {
        Value::Blob(ByteBuf::from(bytes.to_vec()))
    }
}
