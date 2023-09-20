use ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument;

const BASE: u128 = 49_140_000;
const PER_BYTE: u128 = 5200;
const PER_KIB: u128 = 104_000;

pub struct HttpCost;

impl HttpCost {
    // Calculate the total cost for an HTTP request
    pub fn total(arg: &CanisterHttpRequestArgument) -> u128 {
        let max_resp_bytes = Self::max_resp_bytes(arg);
        let enc_arg_size = Self::enc_arg_size(arg);

        BASE + enc_arg_size * PER_BYTE + max_resp_bytes * PER_KIB
    }

    // Get the maximum response bytes, defaulting to 2 MiB if not provided
    fn max_resp_bytes(arg: &CanisterHttpRequestArgument) -> u128 {
        arg.max_response_bytes
            .map_or(2 * 1024 * 1024, |n| n as u128)
    }

    // Get the size of the encoded arguments
    fn enc_arg_size(arg: &CanisterHttpRequestArgument) -> u128 {
        let arg_raw = candid::utils::encode_args((arg,)).expect("Failed to encode arguments.");
        arg_raw.len() as u128 + "http_request".len() as u128
    }
}
