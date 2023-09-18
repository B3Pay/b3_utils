use std::ops::Mul;

use ic_cdk::{
    api::management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
        TransformArgs, TransformContext,
    },
    query, update,
};

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
}

fn calculate_cycle_cost(arg: &CanisterHttpRequestArgument) -> u128 {
    // Calculate max_response_bytes, defaulting to 2 MiB if not provided
    let max_response_bytes = match arg.max_response_bytes {
        Some(ref n) => *n as u128,
        None => 2 * 1024 * 1024, // default 2MiB
    };

    // Encode the arguments to get their size
    let arg_raw = candid::utils::encode_args((arg,)).expect("Failed to encode arguments.");

    // Scale the cost based on the subnet size
    (3_000_000u128
        + 60_000u128 * 13
        + (arg_raw.len() as u128 + "http_request".len() as u128) * 400
        + max_response_bytes * 800)
        * 13
}

#[update]
async fn http_post(url: String, json_string: String, max_response_bytes: u64) -> String {
    // Log cycle balance before the HTTP request
    let initial_balance = ic_cdk::api::canister_balance();
    ic_cdk::println!("Initial cycle balance: {}", initial_balance);

    let request_headers = vec![HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }];

    let json_utf8 = json_string.into_bytes();
    let request_body = Some(json_utf8.clone());

    let request = CanisterHttpRequestArgument {
        url: url.clone(),
        max_response_bytes: Some(max_response_bytes),
        method: HttpMethod::POST,
        headers: request_headers.clone(),
        body: request_body,
        transform: Some(TransformContext::from_name("transform".to_owned(), vec![])),
    };

    let cycle_cost = calculate_cycle_cost(&request);

    ic_cdk::println!("Estimated cycle cost: {} cycles", cycle_cost);

    // Log cycle balance after the HTTP request
    let final_balance = ic_cdk::api::canister_balance();
    ic_cdk::println!("Final cycle balance: {}", final_balance);

    let response = http_request(request, cycle_cost).await;

    // Log the cycle difference
    ic_cdk::println!(
        "Cycles used for the HTTP request: {}",
        initial_balance - final_balance
    );

    match response {
        Ok((response,)) => {
            ic_cdk::println!("reponse size: {}", response.body.len());
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err((r, m)) => {
            format!(
                "The http_request resulted in an error. RejectionCode: {:?}, Error: {:?}",
                r, m
            )
        }
    }
}

#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];

    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
        ..Default::default()
    };

    if res.status == 200 {
        res.body = raw.response.body;
    }

    res
}

ic_cdk::export_candid!();
