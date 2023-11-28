use crate::{
    error::HelperError,
    nonce::Nonce,
    types::{CanisterId, CanisterIds, ControllerId, Metadata},
    wasm::WasmModule,
    NanoTimeStamp,
};
use candid::{CandidType, Encode, Principal};
use ic_cdk::api::management_canister::main::{CanisterInstallMode, CanisterStatusResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod canister;
pub mod error;

pub use canister::WalletCanister;

pub type WalletCanisters = Vec<WalletCanister>;

pub type WalletVersion = String;

pub struct WalletCanisterInstallArg {
    pub arg: Vec<u8>,
    pub wasm_module: WasmModule,
    pub mode: CanisterInstallMode,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct WalletController {
    pub name: String,
    pub metadata: Metadata,
}

impl WalletController {
    pub fn new(name: String, metadata: Option<Metadata>) -> Self {
        Self {
            name,
            metadata: metadata.unwrap_or_default(),
        }
    }
}

pub type WalletControllerMap = HashMap<ControllerId, WalletController>;

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct WalletInititializeArgs {
    pub controllers: WalletControllerMap,
    pub metadata: Option<Metadata>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletCanisterInitArgs {
    pub owner_id: Principal,
    pub system_id: CanisterId,
}

impl WalletCanisterInitArgs {
    pub fn encode(&self) -> Result<Vec<u8>, HelperError> {
        Encode!(&self).map_err(|e| HelperError::EncodeError(e.to_string()))
    }
}

#[derive(CandidType, Deserialize, Serialize)]
pub enum UserStatus {
    Registered,
    Unregistered,
    SingleCanister(CanisterId),
    MultipleCanister(CanisterIds),
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct UserCanisterStatus {
    pub version: String,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletCanisterStatus {
    pub name: String,
    pub version: String,
    pub status_at: NanoTimeStamp,
    pub canister_id: CanisterId,
    pub account_status: WalletAccountsNonce,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Default, Clone, Deserialize, Serialize)]
pub struct WalletAccountsNonce {
    pub development: Nonce,
    pub production: Nonce,
    pub staging: Nonce,
}
