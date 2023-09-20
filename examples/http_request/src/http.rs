use ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument;
use std::ops::Mul;

pub enum HttpCycleCost {
    Base = 49_140_000,
    PerByte = 5200,
    PerKib = 104_000,
}

impl HttpCycleCost {
    pub fn calculate(&self, bytes: u64) -> u64 {
        match self {
            HttpCycleCost::Base => Self::Base as u64,
            HttpCycleCost::PerByte => bytes.mul(Self::PerByte as u64),
            HttpCycleCost::PerKib => bytes.mul(Self::PerKib as u64),
        }
    }

    pub fn calculate_cycle_cost(arg: &CanisterHttpRequestArgument) -> u128 {
        // Calculate max_response_bytes, defaulting to 2 MiB if not provided
        let max_response_bytes = match arg.max_response_bytes {
            Some(ref n) => *n as u128,
            None => 2 * 1024 * 1024, // default 2MiB
        };

        // Encode the arguments to get their size
        let arg_raw = candid::utils::encode_args((arg,)).expect("Failed to encode arguments.");

        // Scale the cost based on the subnet size
        Self::Base as u128
            + (arg_raw.len() as u128 + "http_request".len() as u128) * Self::PerByte as u128
            + max_response_bytes * Self::PerKib as u128
    }
}
