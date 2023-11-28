use crate::{
    api::{Cycles, InterCall, InterCallError},
    types::CanisterId,
};

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

use super::{
    ICRC2Allowance, ICRC2ApproveArgs, ICRC2ApproveResult, ICRC2TransferFromArgs,
    ICRC2TransferFromResult,
};

#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ICRC2(pub CanisterId);

impl ICRC2 {
    pub async fn allowance(&self) -> Result<ICRC2Allowance, InterCallError> {
        InterCall(self.0)
            .call("icrc2_allowance", (), Cycles::NoPay)
            .await
    }

    pub async fn approve(
        &self,
        args: ICRC2ApproveArgs,
    ) -> Result<ICRC2ApproveResult, InterCallError> {
        InterCall(self.0)
            .call("icrc2_approve", args, Cycles::NoPay)
            .await
    }

    pub async fn transfer_from(
        &self,
        args: ICRC2TransferFromArgs,
    ) -> Result<ICRC2TransferFromResult, InterCallError> {
        InterCall(self.0)
            .call("icrc2_transfer_from", args, Cycles::NoPay)
            .await
    }
}

impl From<Principal> for ICRC2 {
    fn from(principal: Principal) -> Self {
        Self(principal)
    }
}

impl From<&Principal> for ICRC2 {
    fn from(principal: &Principal) -> Self {
        Self(principal.clone())
    }
}

impl From<&str> for ICRC2 {
    fn from(principal: &str) -> Self {
        let principal = Principal::from_text(principal)
            .map_err(|_| "ICRC2: Invalid principal".to_string())
            .unwrap();

        Self(principal)
    }
}
