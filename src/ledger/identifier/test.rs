#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use candid::Principal;

    use crate::{
        constants::DEFAULT_SUBACCOUNT,
        ledger::{AccountIdentifier, AccountIdentifierError, ChecksumError},
        Environment, Subaccount,
    };

    #[test]
    fn test_default_account_identifier() {
        let principal = Principal::from_slice(&[0; 29]);

        let account_id = AccountIdentifier::new(principal, None);

        assert_eq!(
            account_id.to_string(),
            "916f8a3e360d55c722bbf659090a408ad76da22f4e6351ec71cdfa3002b98d53"
        );

        let account_id = AccountIdentifier::new(Principal::management_canister(), None);

        assert_eq!(
            account_id.to_string(),
            "2d0e897f7e862d2b57d9bc9ea5c65f9a24ac6c074575f47898314b8d6cb0929d",
        );

        let subaccount = Subaccount::new(Environment::Production, 0);

        assert_eq!(
            subaccount.to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );

        let account_id = AccountIdentifier::new(Principal::from_slice(&[0, 32]), Some(subaccount));

        assert_eq!(
            account_id.to_string(),
            "ee918f38cb6becc036378e1cb83ad44938ddb5de6e61d243d3351889b5a9536f".to_string()
        );
    }

    #[test]
    fn test_principal_to_account_identifier() {
        let account_id = AccountIdentifier::from_str("aaaaa-aa");

        assert_eq!(
            account_id.unwrap(),
            AccountIdentifier::new(Principal::management_canister(), None)
        );

        let principal = Principal::from_slice(&[0, 32]);

        let account_id = AccountIdentifier::new(principal, None);

        assert_eq!(
            account_id.to_string(),
            "ee918f38cb6becc036378e1cb83ad44938ddb5de6e61d243d3351889b5a9536f"
        );
    }

    #[test]
    fn test_account_identifier_error() {
        let account_id = AccountIdentifier::from_str(
            "ee918f38cb6becc036378e1cb83ad44938ddb5de6e61d243d3351889b5a9536ff4",
        );

        assert_eq!(
            account_id.unwrap_err(),
            "ee918f38cb6becc036378e1cb83ad44938ddb5de6e61d243d3351889b5a9536ff4 has a length of 66 but we expected a length of 64 or 56"
        );
    }

    #[test]
    fn test_default_principal() {
        let principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();

        let account_id = AccountIdentifier::new(principal, Some(DEFAULT_SUBACCOUNT));

        assert_eq!(
            account_id.to_string(),
            "c8734e0cde2404bb36b86bff86ee6df4f69c16fbc9a37f3f1d4aad574fa8cb5c"
        );
    }

    #[test]
    fn test_account_identifier() {
        let principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();

        let account_id = AccountIdentifier::new(principal, Some(DEFAULT_SUBACCOUNT));
        assert_eq!(
            account_id.to_string(),
            "c8734e0cde2404bb36b86bff86ee6df4f69c16fbc9a37f3f1d4aad574fa8cb5c"
        );

        let principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();

        let account_id = AccountIdentifier::new(principal, None);
        assert_eq!(
            account_id.to_string(),
            "c8734e0cde2404bb36b86bff86ee6df4f69c16fbc9a37f3f1d4aad574fa8cb5c"
        );

        let principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();

        let account_id =
            AccountIdentifier::new(principal, Some(Subaccount::new(Environment::Production, 1)));

        assert_eq!(
            account_id.to_string(),
            "40900242935be3ae43f9f07262af078486d87f5eb8707da705d2605a6c2f1c9b"
        );
    }

    #[test]
    fn check_round_trip() {
        let ai = AccountIdentifier([7; 28]);
        let res = ai.to_hex();
        assert_eq!(
            res.parse(),
            Ok(ai),
            "The account identifier doesn't change after going back and forth between a string"
        )
    }

    #[test]
    fn check_encoding() {
        let ai = AccountIdentifier([7; 28]);

        let en1 = candid::encode_one(ai.clone()).unwrap();
        let en2 = candid::encode_one(ai.to_string()).unwrap();

        assert_eq!(
            &en1, &en2,
            "Candid encoding of an account identifier and a string should be identical"
        );

        let de1: String = candid::decode_one(&en1[..]).unwrap();
        let de2: AccountIdentifier = candid::decode_one(&en2[..]).unwrap();

        assert_eq!(
            de1.parse(),
            Ok(de2.clone()),
            "The types are the same after decoding, even through a different type"
        );

        assert_eq!(de2, ai, "And the value itself hasn't changed");
    }

    #[test]
    fn test_account_id_from_slice() {
        let length_27 = b"123456789_123456789_1234567".to_vec();
        assert_eq!(
            AccountIdentifier::from_slice(&length_27),
            Err(AccountIdentifierError::InvalidLength(length_27))
        );

        let length_28 = b"123456789_123456789_12345678".to_vec();
        assert_eq!(
            AccountIdentifier::from_slice(&length_28),
            Ok(AccountIdentifier(length_28.try_into().unwrap()))
        );

        let length_29 = b"123456789_123456789_123456789".to_vec();
        assert_eq!(
            AccountIdentifier::from_slice(&length_29),
            Err(AccountIdentifierError::InvalidLength(length_29))
        );

        let length_32 = [0; 32].to_vec();
        assert_eq!(
            AccountIdentifier::from_slice(&length_32),
            Err(AccountIdentifierError::InvalidChecksum(ChecksumError {
                input: length_32.try_into().unwrap(),
                expected_checksum: [128, 112, 119, 233],
                found_checksum: [0, 0, 0, 0],
            }))
        );

        // A 32-byte address with a valid checksum
        let length_32 = [
            128, 112, 119, 233, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ]
        .to_vec();
        assert_eq!(
            AccountIdentifier::from_slice(&length_32),
            Ok(AccountIdentifier([0; 28]))
        );
    }

    #[test]
    fn test_account_id_from_hex() {
        let length_56 = "00000000000000000000000000000000000000000000000000000000";
        assert_eq!(
            AccountIdentifier::from_hex(length_56),
            Ok(AccountIdentifier([0; 28]))
        );

        let length_57 = "000000000000000000000000000000000000000000000000000000000";
        assert!(AccountIdentifier::from_hex(length_57).is_err());

        let length_58 = "0000000000000000000000000000000000000000000000000000000000";
        assert_eq!(
        AccountIdentifier::from_hex(length_58),
        Err("0000000000000000000000000000000000000000000000000000000000 has a length of 58 but we expected a length of 64 or 56".to_string())
    );

        let length_64 = "0000000000000000000000000000000000000000000000000000000000000000";
        assert!(AccountIdentifier::from_hex(length_64)
            .unwrap_err()
            .contains("Checksum failed"));

        // Try again with correct checksum
        let length_64 = "807077e900000000000000000000000000000000000000000000000000000000";
        assert_eq!(
            AccountIdentifier::from_hex(length_64),
            Ok(AccountIdentifier([0; 28]))
        );
    }
}
