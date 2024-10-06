use std::collections::{BTreeMap, HashMap};

mod test;
mod value;
pub use value::*;

use candid::{Int, Nat};
use serde_bytes::ByteBuf;
use sha2::{Digest, Sha256};

pub type Metadata = HashMap<String, Value>;

pub struct MetadataHelper;

impl MetadataHelper {
    // Create a new empty Metadata
    pub fn new() -> Metadata {
        Metadata::new()
    }

    // Insert a key-value pair into Metadata
    pub fn insert(metadata: &mut Metadata, key: &str, value: Value) {
        metadata.insert(key.to_string(), value);
    }

    pub fn insert_auto<K, V>(metadata: &mut Metadata, key: K, value: V)
    where
        K: Into<String>,
        V: Into<AutoValue>,
    {
        metadata.insert(key.into(), value.into().into());
    }

    // Get a value from Metadata
    pub fn get<'a>(metadata: &'a Metadata, key: &'a str) -> Option<&'a Value> {
        metadata.get(key)
    }

    // Remove a key-value pair from Metadata
    pub fn remove(metadata: &mut Metadata, key: &str) -> Option<Value> {
        metadata.remove(key)
    }

    // Check if Metadata contains a key
    pub fn contains_key(metadata: &Metadata, key: &str) -> bool {
        metadata.contains_key(key)
    }

    // Get the number of key-value pairs in Metadata
    pub fn len(metadata: &Metadata) -> usize {
        metadata.len()
    }

    // Check if Metadata is empty
    pub fn is_empty(metadata: &Metadata) -> bool {
        metadata.is_empty()
    }

    // Get all keys in Metadata
    pub fn keys(metadata: &Metadata) -> Vec<&String> {
        metadata.keys().collect()
    }

    // Get all values in Metadata
    pub fn values(metadata: &Metadata) -> Vec<&Value> {
        metadata.values().collect()
    }

    // Merge two Metadata instances
    pub fn merge(metadata1: &mut Metadata, metadata2: &Metadata) {
        for (key, value) in metadata2 {
            metadata1.insert(key.clone(), value.clone());
        }
    }

    // Create a deep clone of Metadata
    pub fn deep_clone(metadata: &Metadata) -> Metadata {
        metadata.clone()
    }

    // Compute the hash of entire Metadata
    pub fn hash(metadata: &Metadata) -> Vec<u8> {
        let mut hasher = Sha256::new();
        for (key, value) in metadata {
            hasher.update(key.as_bytes());
            hasher.update(value.hash().as_ref());
        }
        hasher.finalize().to_vec()
    }
}

pub enum AutoValue {
    Blob(Vec<u8>),
    Text(String),
    Bool(bool),
    Nat(Nat),
    Nat64(u64),
    Int(Int),
    Array(Vec<AutoValue>),
    Map(BTreeMap<String, AutoValue>),
}

impl From<AutoValue> for Value {
    fn from(auto_value: AutoValue) -> Self {
        match auto_value {
            AutoValue::Blob(v) => Value::Blob(ByteBuf::from(v)),
            AutoValue::Text(v) => Value::Text(v),
            AutoValue::Bool(v) => Value::Bool(v),
            AutoValue::Nat(v) => Value::Nat(v),
            AutoValue::Nat64(v) => Value::Nat64(v),
            AutoValue::Int(v) => Value::Int(v),
            AutoValue::Array(v) => Value::Array(v.into_iter().map(Value::from).collect()),
            AutoValue::Map(v) => {
                Value::Map(v.into_iter().map(|(k, v)| (k, Value::from(v))).collect())
            }
        }
    }
}

impl From<Vec<u8>> for AutoValue {
    fn from(v: Vec<u8>) -> Self {
        AutoValue::Blob(v)
    }
}

impl From<String> for AutoValue {
    fn from(v: String) -> Self {
        AutoValue::Text(v)
    }
}

impl From<&str> for AutoValue {
    fn from(v: &str) -> Self {
        AutoValue::Text(v.to_string())
    }
}

impl From<bool> for AutoValue {
    fn from(v: bool) -> Self {
        AutoValue::Bool(v)
    }
}

impl From<Nat> for AutoValue {
    fn from(v: Nat) -> Self {
        AutoValue::Nat(v)
    }
}

impl From<u64> for AutoValue {
    fn from(v: u64) -> Self {
        AutoValue::Nat64(v)
    }
}

impl From<Int> for AutoValue {
    fn from(v: Int) -> Self {
        AutoValue::Int(v)
    }
}

impl<T: Into<AutoValue>> From<Vec<T>> for AutoValue {
    fn from(v: Vec<T>) -> Self {
        AutoValue::Array(v.into_iter().map(Into::into).collect())
    }
}

impl<K: Into<String>, V: Into<AutoValue>> From<BTreeMap<K, V>> for AutoValue {
    fn from(v: BTreeMap<K, V>) -> Self {
        AutoValue::Map(v.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
    }
}
