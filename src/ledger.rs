mod base32;
pub mod currency;

mod icrc;
pub use icrc::*;

mod identifier;
pub use identifier::*;

mod types;
pub use types::*;

pub mod constants;

use sha3::{Digest, Keccak256, Sha3_256};

pub fn raw_sha3_256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn raw_keccak256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
