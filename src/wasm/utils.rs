use super::types::WasmHash;
use sha2::{Digest, Sha256};

pub fn vec_to_wasm_hash(data: Vec<u8>) -> WasmHash {
    let mut wasm_hash: WasmHash = [0; 32];
    wasm_hash.copy_from_slice(&data);
    wasm_hash
}

pub fn sha256_wasm_hash(data: &[u8]) -> WasmHash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    let mut wasm_hash: WasmHash = [0; 32];
    wasm_hash.copy_from_slice(&result);
    wasm_hash
}

pub fn sha256_wasm_hash_string(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    hex::encode(result)
}
