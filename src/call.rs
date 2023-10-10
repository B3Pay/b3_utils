mod error;
pub use error::*;

mod traits;

use candid::CandidType;
use ic_cdk::api::call::{call, call_with_payment};
use serde::de::DeserializeOwned;

use crate::{constants::MANAGMENT_CANISTER_ID, types::CanisterId};

pub struct InterCall(pub CanisterId);

impl InterCall {
    pub fn new(canister_id: CanisterId) -> Self {
        Self(canister_id)
    }

    pub async fn call<A, R>(&self, method: &str, args: A) -> Result<R, InterCallError>
    where
        A: CandidType,
        R: CandidType + DeserializeOwned,
    {
        let res: Result<(R,), _> = call(self.0, method, (args,)).await;

        match res {
            Ok((res,)) => Ok(res),
            Err(e) => Err(InterCallError::CallError(method.to_string(), e.1)),
        }
    }

    pub async fn call_with_payment<A, R>(
        &self,
        method: &str,
        args: A,
        cycles: u64,
    ) -> Result<R, InterCallError>
    where
        A: CandidType,
        R: CandidType + DeserializeOwned,
    {
        let res: Result<(R,), _> = call_with_payment(self.0, method, (args,), cycles).await;

        match res {
            Ok((res,)) => Ok(res),
            Err(e) => Err(InterCallError::CallError(method.to_string(), e.1)),
        }
    }
}

pub struct ManagmentCall;

impl ManagmentCall {
    pub async fn call<A, R>(method: &str, args: A, cycles: Option<u64>) -> Result<R, InterCallError>
    where
        A: CandidType,
        R: CandidType + DeserializeOwned,
    {
        let res: Result<(R,), _> = match cycles {
            Some(cycles) => call_with_payment(MANAGMENT_CANISTER_ID, method, (args,), cycles).await,
            None => call(MANAGMENT_CANISTER_ID, method, (args,)).await,
        };

        match res {
            Ok((res,)) => Ok(res),
            Err(e) => Err(InterCallError::CallError(method.to_string(), e.1)),
        }
    }
}
