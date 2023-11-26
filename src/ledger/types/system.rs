use crate::{types::CanisterId, NanoTimeStamp};
use candid::{CandidType, Decode, Encode};
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(CandidType, Deserialize, Serialize)]
pub struct SystemCanisterStatus {
    pub status_at: NanoTimeStamp,
    pub version: String,
    pub user_status: u64,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Bug {
    pub name: String,
    pub version: String,
    pub logs: Vec<String>,
    pub description: String,
    pub canister_id: CanisterId,
}

impl Default for Bug {
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

#[cfg(feature = "stable_memory")]
use crate::memory::types::{Bound, Storable};

#[cfg(feature = "stable_memory")]
impl Storable for Bug {
    const BOUND: Bound = Bound::Unbounded;

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
}
