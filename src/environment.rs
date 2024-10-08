use candid::CandidType;
use ic_cdk::api::management_canister::ecdsa::{EcdsaCurve, EcdsaKeyId};
use serde::{Deserialize, Serialize};
use std::fmt;

mod test;

use super::constants::{DEVELOPMENT_PREFIX_NUMBER, STAGING_PREFIX_NUMBER};

#[derive(CandidType, Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub enum Environment {
    Development,
    Staging,
    #[default]
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Staging => write!(f, "staging"),
            Environment::Production => write!(f, "production"),
        }
    }
}

impl Environment {
    pub fn from_identifier(identifier: u8) -> Self {
        identifier.into()
    }

    pub fn identifier(&self) -> u8 {
        match self {
            Environment::Production => 0,
            Environment::Staging => STAGING_PREFIX_NUMBER,
            Environment::Development => DEVELOPMENT_PREFIX_NUMBER,
        }
    }

    pub fn to_name(&self, counter: u64) -> String {
        let counter = counter.to_string();

        match self {
            Environment::Development => ["Development", "Account", &counter].join(" "),
            Environment::Production => ["Account", &counter].join(" "),
            Environment::Staging => ["Staging", "Account", &counter].join(" "),
        }
    }
}

impl From<u8> for Environment {
    fn from(value: u8) -> Self {
        match value {
            0 => Environment::Production,
            STAGING_PREFIX_NUMBER => Environment::Staging,
            DEVELOPMENT_PREFIX_NUMBER => Environment::Development,
            _ => Environment::Production,
        }
    }
}

impl From<String> for Environment {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "development" => Environment::Development,
            "staging" => Environment::Staging,
            "production" => Environment::Production,
            _ => Environment::Production,
        }
    }
}

impl From<Environment> for EcdsaKeyId {
    fn from(env: Environment) -> Self {
        if env == Environment::Production {
            EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: "key_1".to_string(),
            }
        } else if env == Environment::Staging {
            EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: "test_key_1".to_string(),
            }
        } else {
            EcdsaKeyId {
                curve: EcdsaCurve::Secp256k1,
                name: "dfx_test_key".to_string(),
            }
        }
    }
}
