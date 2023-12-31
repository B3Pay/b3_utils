use candid::{CandidType, Principal};
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, CandidType, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct StoredPrincipal(Principal);

use ic_stable_structures::storable::{Blob, Bound};

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
