use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: i64,
    pub jsonrpc: String,
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub transfers: Vec<Transfer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub asset: String,
    pub block_num: String,
    pub category: String,
    #[serde(rename = "erc1155Metadata")]
    pub erc1155metadata: Value,
    #[serde(rename = "erc721TokenId")]
    pub erc721token_id: Value,
    pub from: String,
    pub hash: String,
    pub raw_contract: RawContract,
    pub to: String,
    pub token_id: Value,
    pub unique_id: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawContract {
    pub address: Value,
    pub decimal: String,
    pub value: String,
}
