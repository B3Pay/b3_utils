use candid::{CandidType, Deserialize};

#[rustfmt::skip]
#[derive(CandidType, Deserialize)]
pub enum AppCallError {
    CreateCanisterError(String),
    ValidateUserError(String),
    UpdateCanisterControllersError(String),
    CanisterStatusError(String),
    VersionError(String),
    CanisterInfoError(String),
    InstallCodeError(String),
    UninstallCodeError(String),
    WasmHashError(String),
}

use std::fmt;

#[rustfmt::skip]
impl fmt::Display for AppCallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppCallError::ValidateUserError(e) => write!(f, "Validate user error: {}", e),
            AppCallError::CreateCanisterError(e) => write!(f, "Create canister error: {}", e),
            AppCallError::UpdateCanisterControllersError(e) => write!(f, "Update canister controllers error: {}", e),
            AppCallError::VersionError(e) => write!(f, "Version error: {}", e),
            AppCallError::CanisterStatusError(e) => write!(f, "Canister status error: {}", e),
            AppCallError::CanisterInfoError(e) => write!(f, "Wallet info error: {}", e),
            AppCallError::InstallCodeError(e) => write!(f, "Install code error: {}", e),
            AppCallError::UninstallCodeError(e) => write!(f, "Uninstall code error: {}", e),
            AppCallError::WasmHashError(e) => write!(f, "Wasm hash error: {}", e),

        }
    }
}
