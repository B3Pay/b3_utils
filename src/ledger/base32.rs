const BASE32_ALPHABET: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";

/// simple base32 encoding for Account
/// should not be used for anything else
pub(crate) fn base32_encode_account(input: &[u8]) -> String {
    let mut output = String::with_capacity((input.len() * 8 + 4) / 5);

    for chunk in input.chunks(5) {
        let mut buffer = 0u64;
        for (i, &byte) in chunk.iter().enumerate() {
            buffer |= (byte as u64) << (8 * (4 - i));
        }

        for i in 0..((chunk.len() * 8 + 4) / 5) {
            let index = ((buffer >> (35 - 5 * i)) & 0x1f) as usize;
            output.push(BASE32_ALPHABET[index] as char);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base32_encode() {
        assert_eq!(base32_encode_account(b""), "");
        assert_eq!(base32_encode_account(b"f"), "my");
        assert_eq!(base32_encode_account(b"fo"), "mzxq");
        assert_eq!(base32_encode_account(b"foo"), "mzxw6");
        assert_eq!(base32_encode_account(b"foob"), "mzxw6yq");
        assert_eq!(base32_encode_account(b"fooba"), "mzxw6ytb");
        assert_eq!(base32_encode_account(b"foobar"), "mzxw6ytboi");
    }

    pub fn base32_decode_account(input: &str) -> Result<Vec<u8>, &'static str> {
        let mut output = Vec::with_capacity(input.len() * 5 / 8);
        let mut buffer = 0u64;
        let mut buffer_length = 0;

        for c in input.chars() {
            buffer <<= 5;

            if let Some(pos) = BASE32_ALPHABET.iter().position(|&x| x == (c as u8)) {
                buffer |= pos as u64;
                buffer_length += 5;

                if buffer_length >= 8 {
                    output.push((buffer >> (buffer_length - 8)) as u8);
                    buffer_length -= 8;
                }
            } else {
                return Err("Invalid character in input");
            }
        }

        if buffer_length >= 8 {
            output.push((buffer >> (buffer_length - 8)) as u8);
        }

        Ok(output)
    }

    #[test]
    fn test_base32_decode() {
        assert_eq!(base32_decode_account(""), Ok(vec![]));
        assert_eq!(base32_decode_account("my"), Ok(b"f".to_vec()));
        assert_eq!(base32_decode_account("mzxq"), Ok(b"fo".to_vec()));
        assert_eq!(base32_decode_account("mzxw6"), Ok(b"foo".to_vec()));
        assert_eq!(base32_decode_account("mzxw6yq"), Ok(b"foob".to_vec()));
        assert_eq!(base32_decode_account("mzxw6ytb"), Ok(b"fooba".to_vec()));
        assert_eq!(base32_decode_account("mzxw6ytboi"), Ok(b"foobar".to_vec()));
    }
}
