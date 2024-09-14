use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub jsonrpc: String,
    pub id: i64,
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub block_hash: String,
    pub block_number: String,
    pub contract_address: Value,
    pub cumulative_gas_used: String,
    pub effective_gas_price: String,
    pub from: String,
    pub gas_used: String,
    pub logs: Vec<Value>,
    pub logs_bloom: String,
    pub status: String,
    pub to: String,
    pub transaction_hash: String,
    pub transaction_index: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
