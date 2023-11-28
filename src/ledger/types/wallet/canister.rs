use crate::{
    api::{Cycles, InterCall, Management},
    ledger::types::{WalletCanisterInstallArg, WalletCanisterStatus},
    types::{CanisterId, ControllerIds, UserId},
    wasm::WasmHash,
};
use candid::CandidType;
use ic_cdk::api::management_canister::{
    main::{InstallCodeArgument, UpdateSettingsArgument},
    provisional::CanisterSettings,
};
use serde::{Deserialize, Serialize};

use super::error::WalletError;

#[derive(CandidType, Deserialize, Serialize, PartialEq, Clone)]
pub struct WalletCanister(pub CanisterId);

impl WalletCanister {
    pub fn canister_id(&self) -> CanisterId {
        self.0.clone()
    }

    /// Get the owner of the canister.
    pub async fn validate_signer(&self, signer_id: UserId) -> Result<bool, WalletError> {
        InterCall(self.0)
            .call("validate_signer", (signer_id,), Cycles::NoPay)
            .await
            .map_err(|err| WalletError::ValidateSignerError(err.to_string()))
    }

    /// Get the wasm hash of the canister.
    pub async fn wasm_hash(&self) -> Result<WasmHash, WalletError> {
        InterCall(self.0)
            .call("wasm_hash", (), Cycles::NoPay)
            .await
            .map_err(|err| WalletError::WasmHashError(err.to_string()))
    }

    /// Get the version of the canister.
    pub async fn version(&self) -> Result<String, WalletError> {
        InterCall(self.0)
            .call("version", (), Cycles::NoPay)
            .await
            .map_err(|err| WalletError::VersionError(err.to_string()))
    }

    /// Get the status of the canister.
    /// The caller must be a controller of the canister.
    pub async fn status(&self) -> Result<WalletCanisterStatus, WalletError> {
        InterCall(self.0)
            .call("status", (), Cycles::NoPay)
            .await
            .map_err(|err| WalletError::CanisterStatusError(err.to_string()))
    }

    /// Install the code for the canister.
    pub async fn install_code(&self, args: WalletCanisterInstallArg) -> Result<(), WalletError> {
        let canister_id = self.canister_id();

        let install_args = InstallCodeArgument {
            arg: args.arg,
            mode: args.mode,
            wasm_module: args.wasm_module,
            canister_id,
        };

        Management::install_code(install_args)
            .await
            .map_err(|err| WalletError::InstallCodeError(err.to_string()))
    }

    /// Update the controllers of the canister.
    /// The caller must be a controller of the canister.
    /// Default controllers are the owner and the user itself.
    pub async fn add_controllers(&self, mut controllers: ControllerIds) -> Result<(), WalletError> {
        let canister_id = self.0;

        let canister_status = Management::canister_status(canister_id.clone())
            .await
            .map_err(|err| WalletError::CanisterStatusError(err.to_string()))?;

        if !controllers.contains(&canister_id) {
            controllers.push(canister_id);
            canister_status
                .settings
                .controllers
                .iter()
                .for_each(|controller| {
                    if !controllers.contains(controller) {
                        controllers.push(controller.clone());
                    }
                });
        }

        let arg = UpdateSettingsArgument {
            canister_id,
            settings: CanisterSettings {
                controllers: Some(controllers),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
            },
        };

        Management::update_settings(arg)
            .await
            .map_err(|err| WalletError::UpdateCanisterControllersError(err.to_string()))
    }
}
