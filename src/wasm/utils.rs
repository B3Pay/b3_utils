use super::types::WasmHash;
use sha3::{Digest, Sha3_256};

pub fn sha256_wasm_hash(data: &[u8]) -> WasmHash {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let result = hasher.finalize();

    let mut wasm_hash: WasmHash = [0; 32];
    wasm_hash.copy_from_slice(&result);
    wasm_hash
}

pub fn sha256_wasm_hash_string(data: &[u8]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let result = hasher.finalize();

    hex::encode(result)
}
