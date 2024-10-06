use std::collections::BTreeMap;

use candid::{types::leb128, CandidType, Deserialize, Int, Nat};
use num_traits::cast::ToPrimitive;
use serde::Serialize;
use serde_bytes::ByteBuf;
use sha2::{Digest, Sha256};

pub type Hash = [u8; 32];

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

// Helper methods for Value
impl Value {
    pub fn as_blob(&self) -> Option<&ByteBuf> {
        if let Value::Blob(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_text(&self) -> Option<&String> {
        if let Value::Text(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_nat(&self) -> Option<&Nat> {
        if let Value::Nat(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_nat64(&self) -> Option<u64> {
        if let Value::Nat64(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<&Int> {
        if let Value::Int(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        if let Value::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_map(&self) -> Option<&BTreeMap<String, Value>> {
        if let Value::Map(v) = self {
            Some(v)
        } else {
            None
        }
    }
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
                let mut buf = vec![];
                leb128::encode_nat(&mut buf, *n as u128).expect("bug: cannot encode a Nat64");
                Sha256::digest(&buf).into()
            }
            Value::Int(int) => {
                let v = int
                    .0
                    .to_i128()
                    .expect("BUG: blocks cannot contain integers that do not fit into the 128-bit representation");
                let mut buf = vec![];
                leb128::encode_int(&mut buf, v).expect("bug: cannot encode an Int");
                Sha256::digest(&buf).into()
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

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<Int> for Value {
    fn from(v: Int) -> Self {
        Value::Int(v)
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Value::Array(v)
    }
}

impl From<BTreeMap<String, Value>> for Value {
    fn from(v: BTreeMap<String, Value>) -> Self {
        Value::Map(v)
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
