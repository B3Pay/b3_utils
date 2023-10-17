use candid::{CandidType, Int, Nat};
use serde::{Deserialize, Serialize};

use serde_bytes::ByteBuf;

use crate::{ledger::icrc::ICRCAccount, Subaccount};

use super::{error::ICRC1TransferError, ICRC2ApproveError, ICRC2TransferFromError};

pub type TxIndex = Nat;

pub type ICRCTokens = Nat;

pub type ICRCMetadata = Vec<(String, ICRC1MetadataValue)>;

pub type ICRCMemo = Vec<u8>;

pub type ICRCTimestamp = u64;

pub type ICRC1TransferResult = Result<TxIndex, ICRC1TransferError>;

pub type ICRC2ApproveResult = Result<TxIndex, ICRC2ApproveError>;

pub type ICRC2TransferFromResult = Result<TxIndex, ICRC2TransferFromError>;

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub struct ICRC1TransferArgs {
    pub to: ICRCAccount,
    pub amount: Nat,
    pub fee: Option<Nat>,
    pub memo: Option<ICRCMemo>,
    pub created_at_time: Option<ICRCTimestamp>,
    pub from_subaccount: Option<Subaccount>,
}

/// Variant type for the `metadata` endpoint values.
#[derive(CandidType, Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub enum ICRC1MetadataValue {
    Nat(Nat),
    Int(Int),
    Text(String),
    Blob(ByteBuf),
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ICRC2TransferFromArgs {
    #[serde(default)]
    pub spender_subaccount: Option<Subaccount>,
    pub from: ICRCAccount,
    pub to: ICRCAccount,
    pub amount: Nat,
    #[serde(default)]
    pub fee: Option<Nat>,
    #[serde(default)]
    pub memo: Option<ICRCMemo>,
    #[serde(default)]
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ICRC2ApproveArgs {
    #[serde(default)]
    pub from_subaccount: Option<Subaccount>,
    pub spender: ICRCAccount,
    pub amount: Nat,
    #[serde(default)]
    pub expected_allowance: Option<Nat>,
    #[serde(default)]
    pub expires_at: Option<u64>,
    #[serde(default)]
    pub fee: Option<Nat>,
    #[serde(default)]
    pub memo: Option<ICRCMemo>,
    #[serde(default)]
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ICRC2AllowanceArgs {
    pub account: ICRCAccount,
    pub spender: ICRCAccount,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ICRC2Allowance {
    pub allowance: Nat,
    #[serde(default)]
    pub expires_at: Option<u64>,
}
