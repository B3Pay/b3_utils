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

    hex_string_to_vec(str_ref.trim_start_matches("0x"))
}

pub fn hex_string_with_0x_to_u64<S: AsRef<str>>(stringlike: S) -> Result<u64, HelperError> {
    let str_ref = stringlike.as_ref();

    u64::from_str_radix(str_ref.trim_start_matches("0x"), 16)
        .map_err(|e| HelperError::HexStringToU64Error(e.to_string()))
}

pub fn hex_string_with_0x_to_u128<S: AsRef<str>>(stringlike: S) -> Result<u128, HelperError> {
    let str_ref = stringlike.as_ref();

    u128::from_str_radix(str_ref.trim_start_matches("0x"), 16)
        .map_err(|e| HelperError::HexStringToU128Error(e.to_string()))
}

#[cfg(test)]
mod test {
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
}
