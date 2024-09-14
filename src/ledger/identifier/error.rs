use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(CandidType, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum AccountIdentifierError {
    InvalidAccountIdentifier,
    InvalidLength(Vec<u8>),
    InvalidChecksum(ChecksumError),
}

#[rustfmt::skip]
impl fmt::Display for AccountIdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountIdentifierError::InvalidChecksum(err) => write!(f, "{}", err),
            AccountIdentifierError::InvalidLength(input) => write!(
                f,
                "Received an invalid AccountIdentifier with length {} bytes instead of the expected 28 or 32.",
                input.len()
            ),
            AccountIdentifierError::InvalidAccountIdentifier => write!(f, "Invalid account identifier")
        }
    }
}

/// An error for reporting invalid checksums.
#[derive(CandidType, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ChecksumError {
    pub input: [u8; 32],
    pub expected_checksum: [u8; 4],
    pub found_checksum: [u8; 4],
}

impl fmt::Display for ChecksumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Checksum failed for {}, expected check bytes {} but found {}",
            hex::encode(&self.input[..]),
            hex::encode(self.expected_checksum),
            hex::encode(self.found_checksum),
        )
    }
}
