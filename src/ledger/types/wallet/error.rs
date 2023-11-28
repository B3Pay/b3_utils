use candid::{CandidType, Deserialize};

#[rustfmt::skip]
#[derive(CandidType, Deserialize)]
pub enum WalletError {
    InvalidSigner,
    ValidateSignerError(String),
    UpdateCanisterControllersError(String),
    VersionError(String),
    RateLimitExceeded,
    InvalidReleaseName(String),
    InvalidWalletCanister,
    InvalidAccountIdentifier,
    ReleaseNotFound,
    ReleaseNameNotFound,
    ReleaseAlreadyExists,
    WasmNotFound,
    WasmAlreadyLoaded,
    UserAlreadyExists,
    NoCanisterAvailable,
    UserNotFound,
    BugNotFound,
    BugsNotFound,
    OwnerMismatch { owner: String, user: String },
    UpdateControllersError(String),
    InstallArgError(String),
    EncodeError(String),
    WasmGetError(String),
    WasmHashError(String),
    InstallCodeError(String),
    WasmInstallError(String),
    WalletCanisterNotFound,
    WalletCanisterAlreadyInstalled,
    WalletCanisterRateError(String),
    WalletCanisterDoesNotExist(String),
    WalletCanisterAlreadyExists(String),
    CreateCanisterError(String),
    CanisterStatusError(String),
    CanisterIdNotFound,
}

use std::fmt;

#[rustfmt::skip]
impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WalletError::InvalidSigner => write!(f, "Invalid user!"),
            WalletError::ValidateSignerError(e) => write!(f, "Validate user error: {}", e),
            WalletError::UpdateCanisterControllersError(e) => write!(f, "Update canister controllers error: {}", e),
            WalletError::VersionError(e) => write!(f, "Version error: {}", e),
            WalletError::RateLimitExceeded => write!(f, "Rate limit exceeded!"),
            WalletError::InvalidWalletCanister => write!(f, "Invalid wallet canister!"),
            WalletError::OwnerMismatch { owner, user } => write!(f, "Owner mismatch: {} != {}", owner, user),
            WalletError::InstallArgError(e) => write!(f, "Install arg error: {}", e),
            WalletError::UpdateControllersError(e) => write!(f, "Update controllers error: {}", e),
            WalletError::WasmInstallError(e) => write!(f, "Wasm install error: {}", e),
            WalletError::InvalidReleaseName(e) => write!(f, "Invalid release name: {}", e),
            WalletError::InvalidAccountIdentifier => write!(f, "Invalid account identifier!"),
            WalletError::ReleaseNotFound => write!(f, "Release not found!"),
            WalletError::ReleaseNameNotFound => write!(f, "Release name not found!"),
            WalletError::UserAlreadyExists => write!(f, "User already exists!"),
            WalletError::BugsNotFound => write!(f, "Bugs not found!"),
            WalletError::BugNotFound => write!(f, "Bug not found!"),
            WalletError::UserNotFound => write!(f, "User not found!"),
            WalletError::NoCanisterAvailable => write!(f, "No canister available!"),
            WalletError::ReleaseAlreadyExists => write!(f, "Release already exists!"),
            WalletError::WasmNotFound => write!(f, "Wasm not found!"),
            WalletError::WasmAlreadyLoaded => write!(f, "Wasm already loaded!"),
            WalletError::WasmGetError(e) => write!(f, "Wasm get error: {}", e),
            WalletError::WasmHashError(e) => write!(f, "Wasm hash error: {}", e),
            WalletError::EncodeError(e) => write!(f, "Encode error: {}", e),
            WalletError::InstallCodeError(e) => write!(f, "Install code error: {}", e),
            WalletError::CreateCanisterError(e) => write!(f, "Create canister error: {}", e),
            WalletError::CanisterStatusError(e) => write!(f, "Wallet status error: {}", e),
            WalletError::CanisterIdNotFound => write!(f, "Canister id not found!"),
            WalletError::WalletCanisterRateError(e) => write!(f, "Wallet canister rate error: {}", e),
            WalletError::WalletCanisterNotFound => write!(f, "Wallet Canister id not found!"),
            WalletError::WalletCanisterDoesNotExist(e) => write!(f, "Wallet does not exist: {}", e),
            WalletError::WalletCanisterAlreadyExists(e) => write!(f, "Wallet already exists: {}", e),
            WalletError::WalletCanisterAlreadyInstalled => write!(f, "Wallet canister already installed!"),
        }
    }
}
