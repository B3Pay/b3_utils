use candid::CandidType;
use ic_cdk::api::{
    call::{call, call_with_payment, call_with_payment128},
    management_canister::{
        main::{
            CanisterInfoRequest, CanisterInfoResponse, CanisterStatusResponse,
            CreateCanisterArgument, InstallCodeArgument, UpdateSettingsArgument,
        },
        provisional::CanisterIdRecord,
    },
};
use serde::de::DeserializeOwned;

use crate::{constants::MANAGMENT_CANISTER_ID, types::CanisterId};

use self::error::ManagementError;

pub mod error;

mod inter;
pub use inter::*;

mod types;
pub use types::*;

mod cycles;
pub use cycles::*;

mod app;
pub use app::*;

pub struct Management;

impl Management {
    pub async fn call<A, R>(method: &str, args: A, cycles: CallCycles) -> Result<R, ManagementError>
    where
        A: CandidType,
        R: CandidType + DeserializeOwned,
    {
        let res: Result<(R,), _> = match cycles {
            CallCycles::Pay128(cycles) => {
                call_with_payment128(MANAGMENT_CANISTER_ID, method, (args,), cycles).await
            }
            CallCycles::Pay(cycles) => {
                call_with_payment(MANAGMENT_CANISTER_ID, method, (args,), cycles).await
            }
            CallCycles::NoPay => call(MANAGMENT_CANISTER_ID, method, (args,)).await,
        };

        match res {
            Ok((res,)) => Ok(res),
            Err(e) => Err(ManagementError::CallError(method.to_string(), e.1)),
        }
    }

    pub async fn create_canister(
        arg: CreateCanisterArgument,
        cycles: u128,
    ) -> Result<CanisterIdRecord, ManagementError> {
        Management::call("create_canister", arg, CallCycles::Pay128(cycles)).await
    }

    pub async fn install_code(arg: InstallCodeArgument) -> Result<(), ManagementError> {
        Management::call("install_code", arg, CallCycles::NoPay).await
    }

    pub async fn update_settings(arg: UpdateSettingsArgument) -> Result<(), ManagementError> {
        Management::call("update_settings", arg, CallCycles::NoPay).await
    }

    pub async fn canister_status(
        canister_id: CanisterId,
    ) -> Result<CanisterStatusResponse, ManagementError> {
        let arg = CanisterIdRecord { canister_id };

        Management::call("canister_status", arg, CallCycles::NoPay).await
    }

    pub async fn start_canister(canister_id: CanisterId) -> Result<(), ManagementError> {
        let arg = CanisterIdRecord { canister_id };

        Management::call("start_canister", arg, CallCycles::NoPay).await
    }

    pub async fn stop_canister(canister_id: CanisterId) -> Result<(), ManagementError> {
        let arg = CanisterIdRecord { canister_id };

        Management::call("stop_canister", arg, CallCycles::NoPay).await
    }

    pub async fn delete_canister(canister_id: CanisterId) -> Result<(), ManagementError> {
        let arg = CanisterIdRecord { canister_id };

        Management::call("delete_canister", arg, CallCycles::NoPay).await
    }

    pub async fn uninstall_code(canister_id: CanisterId) -> Result<(), ManagementError> {
        let arg = CanisterIdRecord { canister_id };

        Management::call("uninstall_code", arg, CallCycles::NoPay).await
    }

    pub async fn deposit_cycles(
        canister_id: CanisterId,
        cycles: u128,
    ) -> Result<(), ManagementError> {
        let arg = CanisterIdRecord { canister_id };

        Management::call("deposit_cycles", arg, CallCycles::Pay128(cycles)).await
    }

    pub async fn raw_rand() -> Result<Vec<u8>, ManagementError> {
        Management::call("raw_rand", (), CallCycles::NoPay).await
    }

    pub async fn canister_info(
        canister_id: CanisterId,
        num_requested_changes: Option<u64>,
    ) -> Result<CanisterInfoResponse, ManagementError> {
        let arg = CanisterInfoRequest {
            canister_id,
            num_requested_changes,
        };

        Management::call("canister_info", arg, CallCycles::NoPay).await
    }
}
