use core::fmt;

use candid::{CandidType, Principal};
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};

use ic_stable_structures::storable::{Blob, Bound};

#[derive(
    Debug, CandidType, Hash, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct StoredPrincipal(Principal);

impl StoredPrincipal {
    pub fn to_text(&self) -> String {
        self.0.to_text()
    }

    pub fn from_text(text: &str) -> Result<Self, String> {
        Principal::from_text(text)
            .map_err(|e| format!("Principal::from_text failed: {}", e))
            .map(Self)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, String> {
        Principal::try_from_slice(slice)
            .map_err(|e| format!("Principal::try_from_slice failed: {}", e))
            .map(Self)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Storable for StoredPrincipal {
    const BOUND: Bound = Blob::<29>::BOUND;

    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        std::borrow::Cow::Owned(
            Blob::<29>::try_from(self.0.as_slice())
                .expect("principal length should not exceed 29 bytes")
                .to_bytes()
                .into_owned(),
        )
    }

    fn from_bytes(bytes: std::borrow::Cow<'_, [u8]>) -> Self {
        Self(Principal::from_slice(
            Blob::<29>::from_bytes(bytes).as_slice(),
        ))
    }
}

impl fmt::Display for StoredPrincipal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_text())
    }
}

impl From<Principal> for StoredPrincipal {
    fn from(principal: Principal) -> Self {
        Self(principal)
    }
}

impl From<StoredPrincipal> for Principal {
    fn from(principal: StoredPrincipal) -> Self {
        principal.0
    }
}

impl From<&StoredPrincipal> for Principal {
    fn from(principal: &StoredPrincipal) -> Self {
        principal.0
    }
}

impl From<&Principal> for StoredPrincipal {
    fn from(principal: &Principal) -> Self {
        Self(*principal)
    }
}
