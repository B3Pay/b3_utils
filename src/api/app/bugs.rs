use candid::{CandidType, Decode, Encode};
use ic_cdk::api::management_canister::provisional::CanisterId;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct AppBug {
    pub name: String,
    pub version: String,
    pub logs: Vec<String>,
    pub description: String,
    pub canister_id: CanisterId,
}

impl Default for AppBug {
    fn default() -> Self {
        Self {
            logs: vec![],
            name: "".to_string(),
            version: "".to_string(),
            canister_id: ic_cdk::id(),
            description: "".to_string(),
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct AppBugs(Vec<AppBug>);

impl AppBugs {
    pub fn new(bug: AppBug) -> Self {
        Self(vec![bug])
    }

    pub fn push(&mut self, bug: AppBug) {
        self.0.push(bug);
    }

    pub fn drain(&mut self) -> Vec<AppBug> {
        self.0.drain(..).collect()
    }
}

#[cfg(feature = "stable_memory")]
use crate::memory::types::{Bound, Storable};

#[cfg(feature = "stable_memory")]
impl Storable for AppBug {
    const BOUND: Bound = Bound::Unbounded;

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
}

#[cfg(feature = "stable_memory")]
impl Storable for AppBugs {
    const BOUND: Bound = Bound::Unbounded;

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
}
