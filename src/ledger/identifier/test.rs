#[cfg(test)]
mod tests {
    use candid::Principal;

    use crate::{
        ledger::{constants::DEFAULT_SUBACCOUNT, AccountIdentifier},
        Environment, Subaccount,
    };

    #[test]
    fn test_default_account_identifier() {
        let account_id =
            AccountIdentifier::new(Principal::from_slice(&[0; 32]), DEFAULT_SUBACCOUNT);
        assert_eq!(
            account_id.to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000",
        );

        let subaccount = Subaccount::new(Environment::Production, 0);

        assert_eq!(
            subaccount.to_string(),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );

        let account_id = AccountIdentifier::new(Principal::from_slice(&[0, 32]), subaccount);

        assert_eq!(
            account_id.to_string(),
            "ee918f38cb6becc036378e1cb83ad44938ddb5de6e61d243d3351889b5a9536f".to_string()
        );
    }

    #[test]
    fn test_default_principal() {
        let principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();

        let account_id = AccountIdentifier::new(principal, DEFAULT_SUBACCOUNT);

        assert_eq!(
            account_id.to_string(),
            "c8734e0cde2404bb36b86bff86ee6df4f69c16fbc9a37f3f1d4aad574fa8cb5c"
        );
    }

    #[test]
    fn test_account_identifier() {
        let principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();

        let account_id = AccountIdentifier::new(principal, DEFAULT_SUBACCOUNT);
        assert_eq!(
            account_id.to_string(),
            "c8734e0cde2404bb36b86bff86ee6df4f69c16fbc9a37f3f1d4aad574fa8cb5c"
        );

        let principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();

        let account_id =
            AccountIdentifier::new(principal, Subaccount::new(Environment::Production, 0));
        assert_eq!(
            account_id.to_string(),
            "c8734e0cde2404bb36b86bff86ee6df4f69c16fbc9a37f3f1d4aad574fa8cb5c"
        );

        let principal = Principal::from_text("rdmx6-jaaaa-aaaaa-aaadq-cai").unwrap();

        let account_id =
            AccountIdentifier::new(principal, Subaccount::new(Environment::Production, 1));

        assert_eq!(
            account_id.to_string(),
            "40900242935be3ae43f9f07262af078486d87f5eb8707da705d2605a6c2f1c9b"
        );
    }
}
