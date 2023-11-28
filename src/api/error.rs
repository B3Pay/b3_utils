use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum ManagementError {
    CallError(String, String),
}

#[rustfmt::skip]
impl std::fmt::Display for ManagementError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ManagementError::CallError(method, msg) => write!(f, "Error calling method {}: {}", method, msg),
        }
    }
}
