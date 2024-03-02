pub mod bugs;
pub mod error;

use crate::{
    api::{AppInstallArg, AppStatus, AppVersion, CallCycles, InterCall, Management},
    types::{CanisterId, ControllerId, ControllerIds, UserId},
};
use candid::CandidType;
use ic_cdk::api::management_canister::main::{
    CanisterInfoResponse, InstallCodeArgument, UpdateSettingsArgument,
};
use ic_cdk::api::management_canister::{
    main::CreateCanisterArgument, provisional::CanisterSettings,
};

use serde::{Deserialize, Serialize};

use self::error::AppCallError;

#[derive(CandidType, Deserialize, Serialize, PartialEq, Clone)]
pub struct AppCall(pub CanisterId);

impl AppCall {
    pub fn canister_id(&self) -> CanisterId {
        self.0.clone()
    }

    /// create a new canister and save the canister id.
    pub async fn create_with_cycles(
        controllers: Vec<ControllerId>,
        cycles: u128,
    ) -> Result<Self, AppCallError> {
        let args = CreateCanisterArgument {
            settings: Some(CanisterSettings {
                controllers: Some(controllers.clone()),
                compute_allocation: None,
                memory_allocation: None,
                freezing_threshold: None,
                reserved_cycles_limit: None,
            }),
        };

        let result = Management::create_canister(args, cycles).await;

        match result {
            Ok(result) => Ok(Self(result.canister_id)),
            Err(err) => Err(AppCallError::CreateCanisterError(err.to_string())),
        }
    }

    /// Get the owner of the canister.
    pub async fn validate_user(&self, signer_id: UserId) -> Result<bool, AppCallError> {
        InterCall(self.0)
            .call("validate_user", (signer_id,), CallCycles::NoPay)
            .await
            .map_err(|err| AppCallError::ValidateUserError(err.to_string()))
    }

    /// Get the version of the canister.
    pub async fn version(&self) -> Result<AppVersion, AppCallError> {
        InterCall(self.0)
            .call("version", (), CallCycles::NoPay)
            .await
            .map_err(|err| AppCallError::VersionError(err.to_string()))
    }

    /// Get the status of the canister.
    /// The caller must be a controller of the canister.
    pub async fn status(&self) -> Result<AppStatus, AppCallError> {
        InterCall(self.0)
            .call("status", (), CallCycles::NoPay)
            .await
            .map_err(|err| AppCallError::CanisterStatusError(err.to_string()))
    }

    /// Get the status of the canister.
    /// The caller must be a controller of the canister.
    pub async fn info(
        &self,
        num_requested_changes: Option<u64>,
    ) -> Result<CanisterInfoResponse, AppCallError> {
        Management::canister_info(self.0, num_requested_changes)
            .await
            .map_err(|err| AppCallError::CanisterInfoError(err.to_string()))
    }

    /// Get the module hash of the canister.
    pub async fn module_hash(&self) -> Result<Option<Vec<u8>>, AppCallError> {
        self.info(None)
            .await
            .map(|info| info.module_hash)
            .map_err(|err| AppCallError::CanisterInfoError(err.to_string()))
    }

    /// Install the code for the canister.
    pub async fn install_code(&self, args: AppInstallArg) -> Result<(), AppCallError> {
        let canister_id = self.canister_id();

        let install_args = InstallCodeArgument {
            arg: args.arg,
            mode: args.mode,
            wasm_module: args.wasm_module,
            canister_id,
        };

        Management::install_code(install_args)
            .await
            .map_err(|err| AppCallError::InstallCodeError(err.to_string()))
    }

    /// Uninstall the code for the canister.
    /// The caller must be a controller of the canister.
    pub async fn uninstall_code(&self) -> Result<(), AppCallError> {
        Management::uninstall_code(self.0)
            .await
            .map_err(|err| AppCallError::UninstallCodeError(err.to_string()))
    }

    /// Update the controllers of the canister.
    /// The caller must be a controller of the canister.
    /// Default controllers are the owner and the user itself.
    pub async fn add_controllers(
        &self,
        mut controllers: ControllerIds,
    ) -> Result<(), AppCallError> {
        let canister_id = self.0;

        let canister_status = Management::canister_status(canister_id.clone())
            .await
            .map_err(|err| AppCallError::CanisterStatusError(err.to_string()))?;

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
                reserved_cycles_limit: None,
            },
        };

        Management::update_settings(arg)
            .await
            .map_err(|err| AppCallError::UpdateCanisterControllersError(err.to_string()))
    }
}
