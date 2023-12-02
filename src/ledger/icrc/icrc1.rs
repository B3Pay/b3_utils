use crate::{
    api::{CallCycles, InterCall, InterCallError},
    ledger::icrc::ICRCAccount,
    types::CanisterId,
};

use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

use super::{ICRC1TransferArgs, ICRC1TransferResult, ICRCMetadata};

#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ICRC1(pub CanisterId);

impl ICRC1 {
    pub async fn name(&self) -> Result<String, InterCallError> {
        InterCall(self.0)
            .call("icrc1_name", (), CallCycles::NoPay)
            .await
    }

    pub async fn fee(&self) -> Result<Nat, InterCallError> {
        InterCall(self.0)
            .call("icrc1_fee", (), CallCycles::NoPay)
            .await
    }

    pub async fn symbol(&self) -> Result<String, InterCallError> {
        InterCall(self.0)
            .call("icrc1_symbol", (), CallCycles::NoPay)
            .await
    }

    pub async fn decimals(&self) -> Result<u8, InterCallError> {
        InterCall(self.0)
            .call("icrc1_decimals", (), CallCycles::NoPay)
            .await
    }

    pub async fn metadata(&self) -> Result<ICRCMetadata, InterCallError> {
        InterCall(self.0)
            .call("icrc1_metadata", (), CallCycles::NoPay)
            .await
    }

    pub async fn total_supply(&self) -> Result<Nat, InterCallError> {
        InterCall(self.0)
            .call("icrc1_total_supply", (), CallCycles::NoPay)
            .await
    }

    pub async fn balance_of(&self, account: ICRCAccount) -> Result<Nat, InterCallError> {
        InterCall(self.0)
            .call("icrc1_balance_of", account, CallCycles::NoPay)
            .await
    }

    pub async fn transfer(
        &self,
        args: ICRC1TransferArgs,
    ) -> Result<ICRC1TransferResult, InterCallError> {
        InterCall(self.0)
            .call("icrc1_transfer", args, CallCycles::NoPay)
            .await
    }
}

impl From<Principal> for ICRC1 {
    fn from(principal: Principal) -> Self {
        Self(principal)
    }
}

impl From<&Principal> for ICRC1 {
    fn from(principal: &Principal) -> Self {
        Self(principal.clone())
    }
}

impl From<&str> for ICRC1 {
    fn from(principal: &str) -> Self {
        let principal = Principal::from_text(principal)
            .map_err(|_| "ICRC1: Invalid principal".to_string())
            .unwrap();

        Self(principal)
    }
}
