use std::cell::RefCell;

use b3_utils::{
    hex_string_to_u64,
    http::HttpRequest,
    memory::{init_stable_mem_refcell, types::DefaultVMMap},
};
use candid::CandidType;
use ic_cdk::{api::management_canister::http_request::HttpResponse, query, update};
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Debug, Serialize, Deserialize)]
struct Main {
    jsonrpc: String,
    id: Number,
    result: Transaction,
}

#[derive(Debug, CandidType, Serialize, Deserialize)]
struct Transaction {
    #[serde(rename = "blockHash")]
    block_hash: String,
    #[serde(rename = "blockNumber")]
    block_number: String,
    hash: String,
    #[serde(rename = "accessList")]
    access_list: Vec<String>,
    #[serde(rename = "chainId")]
    chain_id: String,
    from: String,
    gas: String,
    #[serde(rename = "gasPrice")]
    gas_price: String,
    input: String,
    #[serde(rename = "maxFeePerGas")]
    max_fee_per_gas: String,
    #[serde(rename = "maxPriorityFeePerGas")]
    max_priority_fee_per_gas: String,
    nonce: String,
    r: String,
    s: String,
    to: String,
    #[serde(rename = "transactionIndex")]
    transaction_index: String,
    #[serde(rename = "type")]
    type_: String,
    #[serde(rename = "v")]
    v: String,
    value: String,
}
thread_local! {
    // static TASK_TIMER: RefCell<DefaultTaskTimer<Task>> = init_stable_mem_refcell("timer", 1).unwrap();

    static TRANSACTIONS: RefCell<DefaultVMMap<String, u64>> = init_stable_mem_refcell("map", 2).unwrap();
}

async fn get_transaction(hash: String) -> Result<Transaction, String> {
    let url = "https://eth-sepolia.g.alchemy.com/v2/ZpSPh3E7KZQg4mb3tN8WFXxG4Auntbxp".to_string();

    let json_string = format!(
        "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getTransactionByHash\",\"params\":[\"{}\"],\"id\":1}}",
        hash
    );

    let request = HttpRequest::new(url).post(&json_string, Some(1024));

    // parse the response and return the transaction list
    let result = request
        .send_with_closure(|response: HttpResponse| HttpResponse {
            status: response.status,
            body: response.body,
            ..Default::default()
        })
        .await;

    // return the transaction list
    match result {
        Ok(response) => {
            let main: Main = serde_json::from_slice(&response.body).unwrap();

            Ok(main.result)
        }
        Err(m) => {
            return Err(m);
        }
    }
}

#[update]
async fn get_transaction_value(hash: String) -> Result<u64, String> {
    let value = TRANSACTIONS.with(|transactions| transactions.borrow().get(&hash));

    match value {
        Some(value) => Ok(value),
        None => {
            let transaction = get_transaction(hash).await;

            match transaction {
                Ok(transaction) => {
                    let value = hex_string_to_u64(transaction.value).unwrap();

                    if transaction.to == "0xb51f94aeeebe55a3760e8169a22e536ebd3a6dcb" {
                        return Err("Invalid transaction".to_string());
                    }

                    TRANSACTIONS.with(|transactions| {
                        transactions.borrow_mut().insert(transaction.hash, value);
                    });

                    Ok(value)
                }
                Err(m) => Err(m),
            }
        }
    }
}

#[query]
async fn get_transaction_value_sum() -> u64 {
    TRANSACTIONS.with(|transactions| {
        transactions
            .borrow()
            .iter()
            .fold(0, |acc, (_, value)| acc + value)
    })
}

#[query]
async fn get_transaction_count() -> u64 {
    TRANSACTIONS.with(|transactions| transactions.borrow().len() as u64)
}

#[query]
async fn get_transaction_list() -> Vec<String> {
    TRANSACTIONS.with(|transactions| {
        transactions
            .borrow()
            .iter()
            .map(|(hash, _)| hash.clone())
            .collect()
    })
}

#[query]
async fn get_transaction_list_with_value() -> Vec<(String, u64)> {
    TRANSACTIONS.with(|transactions| {
        transactions
            .borrow()
            .iter()
            .map(|(hash, value)| (hash.clone(), value))
            .collect()
    })
}

ic_cdk::export_candid!();
