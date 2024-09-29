use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;

pub enum CallCycles {
    NoPay,
    Pay(u64),
    Pay128(u128),
}

impl From<&EcdsaKeyId> for CallCycles {
    fn from(ecdsa_key_id: &EcdsaKeyId) -> Self {
        match ecdsa_key_id.name.as_str() {
            "key_1" => CallCycles::Pay(26_153_846_153),
            "test_key_1" => CallCycles::Pay(10_000_000_000),
            "dfx_test_key" => CallCycles::NoPay,
            _ => CallCycles::NoPay,
        }
    }
}

impl Default for CallCycles {
    fn default() -> Self {
        CallCycles::NoPay
    }
}

impl From<u64> for CallCycles {
    fn from(cycles: u64) -> Self {
        CallCycles::Pay(cycles)
    }
}

impl From<u128> for CallCycles {
    fn from(cycles: u128) -> Self {
        CallCycles::Pay128(cycles)
    }
}

impl From<CallCycles> for u64 {
    fn from(cycles: CallCycles) -> Self {
        match cycles {
            CallCycles::NoPay => 0,
            CallCycles::Pay(cycles) => cycles,
            CallCycles::Pay128(cycles) => cycles as u64,
        }
    }
}

impl From<CallCycles> for u128 {
    fn from(cycles: CallCycles) -> Self {
        match cycles {
            CallCycles::NoPay => 0,
            CallCycles::Pay(cycles) => cycles as u128,
            CallCycles::Pay128(cycles) => cycles,
        }
    }
}
