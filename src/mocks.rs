use candid::Principal;

pub fn time_mock() -> u64 {
    use std::time::SystemTime;

    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => d.as_nanos() as u64,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn canister_balance_mock() -> u128 {
    1000
}

pub fn performance_counter_mock(_n: u32) -> u64 {
    1000
}

//only use for test cases
pub fn id_mock() -> Principal {
    Principal::management_canister()
}
