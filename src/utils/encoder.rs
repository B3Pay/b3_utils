use candid::Nat;

use crate::error::HelperError;

pub fn vec_to_hex_string<V: AsRef<[u8]>>(data: V) -> String {
    hex::encode(data)
}

pub fn vec_to_hex_string_with_0x<V: AsRef<[u8]>>(data: V) -> String {
    format!("0x{}", vec_to_hex_string(data))
}

pub fn u64_to_hex_string_with_0x(value: u64) -> String {
    format!("0x{:x}", value)
}

pub fn u128_to_hex_string_with_0x(value: u128) -> String {
    format!("0x{:x}", value)
}

pub fn hex_string_to_vec<S: AsRef<str>>(stringlike: S) -> Result<Vec<u8>, HelperError> {
    let str_ref = stringlike.as_ref();

    hex::decode(str_ref).map_err(|e| HelperError::HexStringToVecError(e.to_string()))
}

pub fn hex_string_with_0x_to_vec<S: AsRef<str>>(stringlike: S) -> Result<Vec<u8>, HelperError> {
    let str_ref = stringlike.as_ref();

    if !str_ref.starts_with("0x") {
        return Err(HelperError::InvalidHexString);
    }

    hex_string_to_vec(&str_ref[2..])
}

pub fn hex_string_with_0x_to_u64<S: AsRef<str>>(stringlike: S) -> Result<u64, HelperError> {
    let str_ref = stringlike.as_ref();

    if !str_ref.starts_with("0x") {
        return Err(HelperError::InvalidHexString);
    }

    u64::from_str_radix(&str_ref[2..], 16)
        .map_err(|e| HelperError::HexStringToU64Error(e.to_string()))
}

pub fn hex_string_with_0x_to_u128<S: AsRef<str>>(stringlike: S) -> Result<u128, HelperError> {
    let str_ref = stringlike.as_ref();

    if !str_ref.starts_with("0x") {
        return Err(HelperError::InvalidHexString);
    }

    u128::from_str_radix(&str_ref[2..], 16)
        .map_err(|e| HelperError::HexStringToU128Error(e.to_string()))
}

pub fn hex_string_with_0x_to_nat<S: AsRef<str>>(stringlike: S) -> Result<Nat, HelperError> {
    let str_ref = stringlike.as_ref();

    if !str_ref.starts_with("0x") {
        return Err(HelperError::InvalidHexString);
    }

    let decoded =
        hex::decode(&str_ref[2..]).map_err(|e| HelperError::HexStringToNatError(e.to_string()))?;

    let mut result = Nat::from(0u8);
    let base = Nat::from(256u16);

    for byte in decoded {
        result = result * base.clone() + Nat::from(byte);
    }

    Ok(result)
}

/// Converts a string to a slug.
pub fn name_to_slug(name: &str) -> String {
    name.to_lowercase()
        .trim()
        .replace(|c: char| !c.is_alphanumeric() && c != '-' && c != '_', "-")
        .replace(|c: char| c.is_whitespace(), "-")
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}

#[cfg(test)]
mod test {
    use candid::Nat;

    use super::*;

    #[test]
    fn test_vec_to_hex_string() {
        let data = vec![1, 2, 3, 4, 5];
        let hex_string = vec_to_hex_string(&data);
        assert_eq!(hex_string, "0102030405");
    }

    #[test]
    fn test_vec_to_hex_string_with_0x() {
        let data = vec![1, 2, 3, 4, 5];
        let hex_string = vec_to_hex_string_with_0x(&data);
        assert_eq!(hex_string, "0x0102030405");
    }

    #[test]
    fn test_u64_to_hex_string_with_0x() {
        let value = 1234567890;
        let hex_string = u64_to_hex_string_with_0x(value);
        assert_eq!(hex_string, "0x499602d2");
    }

    #[test]
    fn test_u128_to_hex_string_with_0x() {
        let value = 12345678901234567890;
        let hex_string = u128_to_hex_string_with_0x(value);
        assert_eq!(hex_string, "0xab54a98ceb1f0ad2");
    }

    #[test]
    fn test_hex_string_to_vec() {
        let hex_string = "0102030405";
        let data = hex_string_to_vec(hex_string).unwrap();
        assert_eq!(data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_eth_address_to_vec() {
        let hex_string = "0x22C64EA6fA6c67B9331cc45967A257456B03D518";
        let data = hex_string_with_0x_to_vec(hex_string).unwrap();
        assert_eq!(
            data,
            vec![
                34, 198, 78, 166, 250, 108, 103, 185, 51, 28, 196, 89, 103, 162, 87, 69, 107, 3,
                213, 24
            ]
        );
    }

    #[test]
    fn test_hex_string_with_0x_to_vec() {
        let hex_string = "0x0102030405";
        let data = hex_string_with_0x_to_vec(hex_string).unwrap();
        assert_eq!(data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_hex_string_with_0x_to_u64() {
        let hex_string = "0x499602d2";
        let value = hex_string_with_0x_to_u64(hex_string).unwrap();
        assert_eq!(value, 1234567890);
    }

    #[test]
    fn test_hex_string_with_0x_to_u128() {
        let hex_string = "0xab54a98ceb1f0ad2";
        let value = hex_string_with_0x_to_u128(hex_string).unwrap();
        assert_eq!(value, 12345678901234567890);
    }

    #[test]
    fn test_hex_string_with_0x_to_nat() {
        let hex_string = "0xab54a98ceb1f0ad2";
        let value = hex_string_with_0x_to_nat(hex_string).unwrap();
        assert_eq!(value, Nat::from(12345678901234567890u128));
    }
    #[test]
    fn test_hex_string_with_0x_to_nat2() {
        let hex_string = "0x000000000000000000000000000000000000000000000000002386f26fc10000";
        let value = hex_string_with_0x_to_nat(hex_string).unwrap();
        assert_eq!(value, Nat::from(10000000000000000u64));
    }

    #[test]
    fn test_name_to_slug() {
        assert_eq!(name_to_slug("Example App Name"), "example-app-name");
        assert_eq!(name_to_slug("  Another Test!   "), "another-test");
        assert_eq!(name_to_slug("Special&^Chars"), "special-chars");
    }
}
