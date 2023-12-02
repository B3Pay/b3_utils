use std::collections::HashMap;

use crate::{nonce::Nonce, Subaccount};
use candid::Principal;

/// Represents the ID of a controller in the system.
pub type ControllerId = Principal;

/// Represents a list of controller IDs.
pub type ControllerIds = Vec<ControllerId>;

/// Represents a list of controller IDs.
pub type AppControllerMap = HashMap<ControllerId, String>;

/// Represents the ID of a canister in the system.
pub type CanisterId = Principal;

/// Represents a list of canister IDs.
pub type CanisterIds = Vec<CanisterId>;

/// Represents the ID of a user in the system.
pub type UserId = Subaccount;

/// Represents the ID of a role in the system.
pub type RoleId = Nonce;

/// Represents the ID of an operation in the system.
pub type OperationId = u64;

/// Represents a deadline timestamp.
pub type Deadline = u64;
