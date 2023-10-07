use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonRpcRequest<'a> {
    jsonrpc: &'a str,
    method: &'a str,
    params: Vec<&'a str>,
    id: u64,
}

impl JsonRpcRequest<'_> {
    pub fn new<'a>(method: &'a str, params: Vec<&'a str>, id: u64) -> JsonRpcRequest<'a> {
        JsonRpcRequest {
            jsonrpc: "2.0",
            method,
            params,
            id,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
