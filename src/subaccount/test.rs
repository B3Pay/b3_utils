#[cfg(test)]
mod test {
    use candid::Principal;

    use crate::{Environment, Subaccount};

    #[test]
    fn test_production_subaccount() {
        let subaccount = Subaccount::new(Environment::Production, 0);
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::new(Environment::Production, 0);
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::new(Environment::Production, 1);

        assert_eq!(subaccount.nonce(), 1);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
            ])
        );

        assert_eq!(subaccount.to_hex(), "1");

        let subaccount = "001".parse::<Subaccount>().unwrap();

        assert_eq!(
            subaccount,
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
            ])
        );

        let subaccount = Subaccount::from(512);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0
            ])
        )
    }

    #[test]
    fn test_development_subaccount() {
        let subaccount = Subaccount::new(Environment::Development, 0);
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::new(Environment::Development, 1);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 1
            ])
        );

        assert_eq!(subaccount.to_hex(), "ff0000000000000001");

        let subaccount = Subaccount::from_hex(
            &"0000000000000000000000000000000000000000000000ff0000000000000001",
        )
        .expect("Failed to parse subaccount");

        assert_eq!(subaccount, Subaccount::new(Environment::Development, 1));
    }

    #[test]
    fn test_staging_subaccount() {
        let subaccount = Subaccount::new(Environment::Staging, 0);
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 170, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::new(Environment::Staging, 1);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 170, 0, 0, 0, 0, 0, 0, 0, 1
            ])
        );

        assert_eq!(subaccount.to_hex(), "aa0000000000000001");
    }

    #[test]
    fn test_subaccount_from_principal() {
        let principal = "rwlgt-iiaaa-aaaaa-aaaaa-cai".parse::<Principal>().unwrap();

        let subaccount = Subaccount::from(principal);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                10, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ])
        );
    }

    #[test]
    fn test_subaccount_from_bytes() {
        let bytes = [
            10, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];

        let subaccount = Subaccount::from(bytes);

        assert_eq!(
            subaccount.to_hex(),
            "a00000000000000000101000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn test_subaccount_from_hex() {
        let subaccount =
            Subaccount::from_hex("a000000000000000001010").expect("Failed to parse subaccount");

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 0, 0, 0, 0, 0,
                0, 0, 0, 16, 16
            ])
        );
    }

    #[test]
    fn test_subaccount_from_u64() {
        let subaccount = Subaccount::from(1);

        println!("{}", subaccount.name());

        assert_eq!("1", subaccount.to_hex());

        let subaccount = Subaccount::from(u64::MAX);

        assert_eq!("ffffffffffffffff", subaccount.to_hex());
        assert_eq!("18446744073709551615", subaccount.nonce_id());
    }

    #[test]
    fn test_subaccount_from_hex_string() {
        // Test with a short string
        let subaccount = Subaccount::from_hex("").unwrap();
        assert_eq!(
            subaccount,
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::from_hex("ff0000000000000000").unwrap();
        assert_eq!(
            subaccount,
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0,
                0, 0, 0, 0, 0
            ])
        );

        // Test with a string of exactly 32 characters
        let subaccount = Subaccount::from_hex("12345678901234567890123456789012").unwrap();
        assert_eq!(
            subaccount,
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 52, 86, 120, 144, 18, 52, 86,
                120, 144, 18, 52, 86, 120, 144, 18
            ])
        );

        // Test with a string longer than 32 characters (should truncate)
        let subaccount = Subaccount::from_hex("123456789012345678901234567890123456").unwrap();
        assert_eq!(
            subaccount,
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 52, 86, 120, 144, 18, 52, 86, 120,
                144, 18, 52, 86, 120, 144, 18, 52, 86
            ])
        );
    }
}
