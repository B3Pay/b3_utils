use crate::{error::HelperError, nonce::Nonce, types::CanisterId, NanoTimeStamp};
use candid::{CandidType, Encode, Principal};
use ic_cdk::api::management_canister::main::{
    CanisterInstallMode, CanisterStatusResponse, WasmModule,
};
use serde::{Deserialize, Serialize};

pub type AppVersion = String;

pub struct AppInstallArg {
    pub arg: Vec<u8>,
    pub wasm_module: WasmModule,
    pub mode: CanisterInstallMode,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct AppInitArgs {
    pub owner_id: Principal,
    pub forge_id: CanisterId,
}

impl AppInitArgs {
    pub fn encode(&self) -> Result<Vec<u8>, HelperError> {
        Encode!(&self).map_err(|e| HelperError::EncodeError(e.to_string()))
    }
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppStatus {
    pub name: String,
    pub version: String,
    pub status_at: NanoTimeStamp,
    pub canister_id: CanisterId,
    pub account_status: AppAccountsNonce,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Default, Clone, Deserialize, Serialize)]
pub struct AppAccountsNonce {
    pub development: Nonce,
    pub production: Nonce,
    pub staging: Nonce,
}

pub enum CallCycles {
    NoPay,
    Pay(u64),
    Pay128(u128),
}
