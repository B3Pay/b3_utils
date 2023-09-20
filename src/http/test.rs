#[cfg(test)]
mod tests {
    use crate::http::{
        HttpRequest, HttpsOutcallCost, HTTPS_OUTCALL_BASE_COST, HTTPS_OUTCALL_REQ_COST_PER_BYTE,
        HTTPS_OUTCALL_RESP_COST_PER_BYTE,
    };
    use ic_cdk::api::management_canister::http_request::{CanisterHttpRequestArgument, HttpMethod};

    #[test]
    fn test_new_http_request() {
        let request = HttpRequest::new("https://example.com".to_string());
        assert_eq!(request.0.url, "https://example.com");
        assert_eq!(request.0.method, HttpMethod::GET);
        assert!(request.0.headers.is_empty());
        assert!(request.0.body.is_none());
        assert!(request.0.max_response_bytes.is_none());
        assert!(request.0.transform.is_none());
    }

    #[test]
    fn test_http_request_get() {
        let request = HttpRequest::new("https://example.com".to_string()).get();
        assert_eq!(request.0.method, HttpMethod::GET);
    }

    #[test]
    fn test_http_request_post() {
        let request =
            HttpRequest::new("https://example.com".to_string()).post("{}", Some(1024 * 1024));
        assert_eq!(request.0.method, HttpMethod::POST);
        assert_eq!(request.0.body.unwrap(), b"{}".to_vec());
        assert_eq!(request.0.max_response_bytes.unwrap(), 1024 * 1024);
    }

    #[test]
    fn test_calculate_cycle_cost() {
        let request =
            HttpRequest::new("https://example.com".to_string()).post("{}", Some(1024 * 1024));
        let cycle_cost = request.calculate_cycle_cost();

        // Replace with the expected cycle cost based on your calculation
        let expected_cycle_cost = HttpsOutcallCost::total(&request.0);
        assert_eq!(cycle_cost, expected_cycle_cost);
    }

    #[test]
    fn test_https_outcall_cost_total() {
        let request = CanisterHttpRequestArgument {
            url: "https://example.com".to_string(),
            headers: vec![],
            method: HttpMethod::GET,
            max_response_bytes: Some(1024 * 1024),
            transform: None,
            body: None,
        };

        let expected_cost = HTTPS_OUTCALL_BASE_COST
            + HttpsOutcallCost::enc_arg_size(&request) * HTTPS_OUTCALL_REQ_COST_PER_BYTE
            + HttpsOutcallCost::max_resp_bytes(&request) * HTTPS_OUTCALL_RESP_COST_PER_BYTE;

        assert_eq!(HttpsOutcallCost::total(&request), expected_cost);
    }
}
