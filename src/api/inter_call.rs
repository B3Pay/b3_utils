mod error;
pub use error::*;

mod traits;

use candid::CandidType;
use ic_cdk::api::call::{call, call_with_payment, call_with_payment128};
use serde::de::DeserializeOwned;

use crate::{api::types::Cycles, types::CanisterId};

pub struct InterCall(pub CanisterId);

impl InterCall {
    pub async fn call<A, R>(
        &self,
        method: &str,
        args: A,
        cycles: Cycles,
    ) -> Result<R, InterCallError>
    where
        A: CandidType,
        R: CandidType + DeserializeOwned,
    {
        let res: Result<(R,), _> = match cycles {
            Cycles::Pay128(cycles) => call_with_payment128(self.0, method, (args,), cycles).await,
            Cycles::Pay(cycles) => call_with_payment(self.0, method, (args,), cycles).await,
            Cycles::NoPay => call(self.0, method, (args,)).await,
        };

        match res {
            Ok((res,)) => Ok(res),
            Err(e) => Err(InterCallError::CallError(method.to_string(), e.1)),
        }
    }
}
