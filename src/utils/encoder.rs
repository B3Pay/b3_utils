use crate::error::HelperError;

pub fn vec_to_hex_string<V: AsRef<[u8]>>(data: V) -> String {
    hex::encode(data)
}

pub fn vec_to_hex_string_with_0x<V: AsRef<[u8]>>(data: V) -> String {
    format!("0x{}", vec_to_hex_string(data))
}

pub fn hex_string_to_vec<S: AsRef<str>>(stringlike: S) -> Result<Vec<u8>, HelperError> {
    let str_ref = stringlike.as_ref();

    hex::decode(str_ref).map_err(|e| HelperError::HexStringToVecError(e.to_string()))
}

pub fn hex_string_to_vec_without_0x<S: AsRef<str>>(stringlike: S) -> Result<Vec<u8>, HelperError> {
    let str_ref = stringlike.as_ref();

    hex_string_to_vec(str_ref.trim_start_matches("0x"))
}

pub fn hex_string_to_u64<S: AsRef<str>>(stringlike: S) -> Result<u64, HelperError> {
    let str_ref = stringlike.as_ref();

    u64::from_str_radix(str_ref.trim_start_matches("0x"), 16)
        .map_err(|e| HelperError::HexStringToU64Error(e.to_string()))
}

pub fn hex_string_to_u128<S: AsRef<str>>(stringlike: S) -> Result<u128, HelperError> {
    let str_ref = stringlike.as_ref();

    u128::from_str_radix(str_ref.trim_start_matches("0x"), 16)
        .map_err(|e| HelperError::HexStringToU128Error(e.to_string()))
}
