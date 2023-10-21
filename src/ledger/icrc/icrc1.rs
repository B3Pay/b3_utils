use crate::{
    call::{InterCall, InterCallError},
    ledger::icrc::ICRCAccount,
    types::CanisterId,
};

use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

use super::{ICRC1TransferArgs, ICRC1TransferResult, ICRCMetadata};

#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ICRC1(pub CanisterId);

impl ICRC1 {
    pub fn new(canister_id: CanisterId) -> Self {
        ICRC1(canister_id)
    }

    pub async fn name(&self) -> Result<String, InterCallError> {
        InterCall::from(self.0).call("icrc1_name", ()).await
    }

    pub async fn fee(&self) -> Result<Nat, InterCallError> {
        InterCall::from(self.0).call("icrc1_fee", ()).await
    }

    pub async fn symbol(&self) -> Result<String, InterCallError> {
        InterCall::from(self.0).call("icrc1_symbol", ()).await
    }

    pub async fn decimals(&self) -> Result<u8, InterCallError> {
        InterCall::from(self.0).call("icrc1_decimals", ()).await
    }

    pub async fn metadata(&self) -> Result<ICRCMetadata, InterCallError> {
        InterCall::from(self.0).call("icrc1_metadata", ()).await
    }

    pub async fn total_supply(&self) -> Result<Nat, InterCallError> {
        InterCall::from(self.0).call("icrc1_total_supply", ()).await
    }

    pub async fn balance_of(&self, account: ICRCAccount) -> Result<Nat, InterCallError> {
        InterCall::from(self.0)
            .call("icrc1_balance_of", account)
            .await
    }

    pub async fn transfer(
        &self,
        args: ICRC1TransferArgs,
    ) -> Result<ICRC1TransferResult, InterCallError> {
        InterCall::from(self.0).call("icrc1_transfer", args).await
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
