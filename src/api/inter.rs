mod error;
mod traits;

use crate::{api::CallCycles, types::CanisterId};
use candid::CandidType;
pub use error::*;
use ic_cdk::api::call::{call, call_with_payment, call_with_payment128, CallResult};
use serde::de::DeserializeOwned;

pub struct InterCall(pub CanisterId);

impl InterCall {
    pub async fn call<A, R>(
        &self,
        method: &str,
        args: A,
        cycles: CallCycles,
    ) -> Result<R, InterCallError>
    where
        A: CandidType,
        R: CandidType + DeserializeOwned,
    {
        let res: CallResult<(R,)> = match cycles {
            CallCycles::Pay128(cycles) => {
                call_with_payment128(self.0, method, (args,), cycles).await
            }
            CallCycles::Pay(cycles) => call_with_payment(self.0, method, (args,), cycles).await,
            CallCycles::NoPay => call(self.0, method, (args,)).await,
        };

        match res {
            Ok((res,)) => Ok(res),
            Err(e) => Err(InterCallError::CallError(method.to_string(), e.1)),
        }
    }
}
