use b3_utils::http::HttpRequest;
use ic_cdk::{api::management_canister::http_request::HttpResponse, update};

#[update]
async fn http_post(url: String, json_string: String, max_response_bytes: Option<u64>) -> String {
    let request = HttpRequest::new(url).post(&json_string, max_response_bytes);

    let result = request
        .send_with_closure(|response| HttpResponse {
            status: response.status,
            body: response.body,
            ..Default::default()
        })
        .await;

    match result {
        Ok(response) => {
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err(m) => {
            format!("The http_request resulted in an error. Error: {:?}", m)
        }
    }
}

ic_cdk::export_candid!();
