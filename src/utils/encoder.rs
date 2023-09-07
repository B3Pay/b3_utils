use crate::error::HelperError;

pub fn vec_to_hex_string(data: &[u8]) -> String {
    hex::encode(data)
}

pub fn vec_to_hex_string_with_prefix(data: &[u8]) -> String {
    format!("0x{}", vec_to_hex_string(data))
}

pub fn hex_string_to_vec(data: &str) -> Result<Vec<u8>, HelperError> {
    hex::decode(data).map_err(|e| HelperError::HexStringToVecError(e.to_string()))
}

pub fn hex_string_to_vec_without_prefix(data: &str) -> Result<Vec<u8>, HelperError> {
    hex_string_to_vec(data.trim_start_matches("0x"))
}
