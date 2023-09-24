use crate::{types::CanisterId, NanoTimeStamp};
use candid::{CandidType, Decode, Encode};
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(CandidType, Deserialize, Serialize)]
pub struct SystemCanisterStatus {
    pub status_at: NanoTimeStamp,
    pub version: String,
    pub user_status: usize,
    pub canister_id: CanisterId,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Bug {
    pub name: String,
    pub description: String,
    pub logs: Vec<String>,
    pub version: String,
    pub canister_id: CanisterId,
}

#[cfg(feature = "stable_memory")]
use crate::memory::types::{Bound, Storable};

#[cfg(feature = "stable_memory")]
impl<'de> Storable for Bug {
    const BOUND: Bound = Bound::Unbounded;

    fn from_bytes<'a>(bytes: Cow<'a, [u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
}
