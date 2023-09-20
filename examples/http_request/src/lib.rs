use b3_utils::{
    http::HttpRequest,
    log_cycle,
    logs::{export_log, LogEntry},
};
use ic_cdk::{
    api::management_canister::http_request::{HttpHeader, HttpResponse, TransformArgs},
    query, update,
};

#[update]
async fn http_post(url: String, json_string: String, max_response_bytes: u64) -> String {
    log_cycle!("Calling http_post");

    let request = HttpRequest::new(url).post(&json_string, Some(max_response_bytes));

    let cycle_cost = request.calculate_cycle_cost();

    log_cycle!("calculated cycle cost: {}", cycle_cost);

    // Using the send method
    let response_result = request.send().await;

    log_cycle!("After http_request");

    match response_result {
        Ok(response) => {
            log_cycle!("response size: {}", response.body.len());
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err(m) => {
            format!("The http_request resulted in an error. Error: {:?}", m)
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
