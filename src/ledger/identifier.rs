use candid::{CandidType, Principal};
use serde::de::{self, Error};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha224};

use std::convert::TryInto;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

mod error;
mod test;

pub use error::*;

use crate::Subaccount;

const ACCOUNT_DOMAIN_SEPERATOR: &[u8] = b"\x0Aaccount-id";

#[derive(Clone, Debug, PartialEq)]
pub struct AccountIdentifier(pub [u8; 28]);

impl AccountIdentifier {
    pub fn new(account: Principal, sub_account: Option<Subaccount>) -> AccountIdentifier {
        let mut hash = Sha224::new();
        hash.update(ACCOUNT_DOMAIN_SEPERATOR);
        hash.update(account.as_slice());

        let sub_account = sub_account.unwrap_or_default();
        hash.update(&sub_account.as_slice());

        AccountIdentifier(hash.finalize().into())
    }

    pub fn from_hex(hex_str: &str) -> Result<AccountIdentifier, String> {
        let hex: Vec<u8> = hex::decode(hex_str).map_err(|e| e.to_string())?;
        Self::from_slice(&hex[..]).map_err(|err| match err {
            // Since the input was provided in hex, return an error that is hex-friendly.
            AccountIdentifierError::InvalidLength(_) => format!(
                "{} has a length of {} but we expected a length of 64 or 56",
                hex_str,
                hex_str.len()
            ),
            AccountIdentifierError::InvalidChecksum(err) => err.to_string(),
            _ => err.to_string(),
        })
    }

    /// Converts a blob into an `AccountIdentifier`.
    ///
    /// The blob can be either:
    ///
    /// 1. The 32-byte canonical format (4 byte checksum + 28 byte hash).
    /// 2. The 28-byte hash.
    ///
    /// If the 32-byte canonical format is provided, the checksum is verified.
    pub fn from_slice(v: &[u8]) -> Result<AccountIdentifier, AccountIdentifierError> {
        // Try parsing it as a 32-byte blob.
        match v.try_into() {
            Ok(h) => {
                // It's a 32-byte blob. Validate the checksum.
                check_sum(h).map_err(AccountIdentifierError::InvalidChecksum)
            }
            Err(_) => {
                // Try parsing it as a 28-byte hash.
                match v.try_into() {
                    Ok(hash) => Ok(AccountIdentifier(hash)),
                    Err(_) => Err(AccountIdentifierError::InvalidLength(v.to_vec())),
                }
            }
        }
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.to_vec())
    }

    /// Converts this account identifier into a binary "address".
    /// The address is CRC32(identifier) . identifier.
    pub fn to_address(&self) -> [u8; 32] {
        let mut result = [0u8; 32];
        result[0..4].copy_from_slice(&self.generate_checksum());
        result[4..32].copy_from_slice(&self.0);
        result
    }

    /// Tries to parse an account identifier from a binary address.
    pub fn from_address(blob: [u8; 32]) -> Result<Self, ChecksumError> {
        check_sum(blob)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        [&self.generate_checksum()[..], &self.0[..]].concat()
    }

    pub fn generate_checksum(&self) -> [u8; 4] {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&self.0);
        hasher.finalize().to_be_bytes()
    }
}

fn check_sum(hex: [u8; 32]) -> Result<AccountIdentifier, ChecksumError> {
    // Get the checksum provided
    let found_checksum = &hex[0..4];

    // Copy the hash into a new array
    let mut hash = [0; 28];
    hash.copy_from_slice(&hex[4..32]);

    let account_id = AccountIdentifier(hash);
    let expected_checksum = account_id.generate_checksum();

    // Check the generated checksum matches
    if expected_checksum == found_checksum {
        Ok(account_id)
    } else {
        Err(ChecksumError {
            input: hex,
            expected_checksum,
            found_checksum: found_checksum.try_into().unwrap(),
        })
    }
}

impl Display for AccountIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.to_hex().fmt(f)
    }
}

impl FromStr for AccountIdentifier {
    type Err = String;

    fn from_str(s: &str) -> Result<AccountIdentifier, String> {
        match Principal::from_str(s) {
            Ok(principal) => Ok(AccountIdentifier::from(principal)),
            Err(_) => AccountIdentifier::from_hex(s),
        }
    }
}

impl Serialize for AccountIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_hex().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AccountIdentifier {
    // This is the canonical way to read a this from string
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
        D::Error: de::Error,
    {
        let hex: [u8; 32] = hex::serde::deserialize(deserializer)?;
        check_sum(hex).map_err(D::Error::custom)
    }
}

impl From<Principal> for AccountIdentifier {
    fn from(principal: Principal) -> Self {
        AccountIdentifier::new(principal, None)
    }
}

impl From<AccountIdentifier> for serde_bytes::ByteBuf {
    fn from(account: AccountIdentifier) -> Self {
        serde_bytes::ByteBuf::from(account.to_vec())
    }
}

impl CandidType for AccountIdentifier {
    // The type expected for account identifier is
    fn _ty() -> candid::types::Type {
        String::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        self.to_hex().idl_serialize(serializer)
    }
}
