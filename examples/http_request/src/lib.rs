use b3_utils::{
    log, log_cycle,
    logs::{export_log, LogEntry},
};
use ic_cdk::{
    api::management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
        TransformArgs, TransformContext,
    },
    query, update,
};

mod cost;
use cost::HttpCost;

#[update]
async fn http_post(url: String, json_string: String, max_response_bytes: u64) -> String {
    // Log cycle balance before the HTTP request
    let initial_balance = ic_cdk::api::canister_balance();
    log_cycle!("Initial cycle balance: {}", initial_balance);

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

    let cycle_cost = HttpCost::total(&request);

    log!("Estimated cycle cost: {} cycles", cycle_cost);

    // Log cycle balance after the HTTP request

    let response = http_request(request, cycle_cost).await;

    let final_balance = ic_cdk::api::canister_balance();
    log!("Final cycle balance: {}", final_balance);

    // Log the cycle difference
    log!(
        "Cycles used for the HTTP request: {}",
        initial_balance - final_balance
    );

    match response {
        Ok((response,)) => {
            log_cycle!("reponse size: {}", response.body.len());
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

#[query]
fn print_log_entries() -> Vec<LogEntry> {
    export_log()
}

ic_cdk::export_candid!();
