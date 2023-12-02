use candid::Principal;

use super::InterCall;

impl From<Principal> for InterCall {
    fn from(principal: Principal) -> Self {
        Self(principal)
    }
}

impl From<&Principal> for InterCall {
    fn from(principal: &Principal) -> Self {
        Self(principal.clone())
    }
}

impl From<&str> for InterCall {
    fn from(principal: &str) -> Self {
        let principal = Principal::from_text(principal)
            .map_err(|_| "InterCall: Invalid principal".to_string())
            .unwrap();

        Self(principal)
    }
}
