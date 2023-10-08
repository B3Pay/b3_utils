use b3_utils::{
    hex_string_with_0x_to_u128, hex_string_with_0x_to_vec, log,
    logs::{export_log, LogEntry},
    memory::{
        init_stable_mem_refcell,
        timer::{DefaultTaskTimer, TaskTimerEntry},
        types::{DefaultVMCell, DefaultVMMap},
    },
    outcall::HttpOutcall,
    report_log, vec_to_hex_string_with_0x, NanoTimeStamp,
};
use ic_cdk::{api::management_canister::http_request::HttpResponse, init, query, update};
use serde_json::json;
use std::cell::RefCell;

mod receipt;
mod test;
mod transaction;
mod transfer;

type TransactionHash = String;
type ReceiptFrom = String;
type TranasactionValue = String;

thread_local! {
    static TASK_TIMER: RefCell<DefaultTaskTimer<[u8; 32]>> = init_stable_mem_refcell("timer", 1).unwrap();
    static TRANSACTIONS: RefCell<DefaultVMMap<TransactionHash, TranasactionValue>> = init_stable_mem_refcell("trasnactions", 2).unwrap();
    static RECEIPTS: RefCell<DefaultVMMap<TransactionHash, ReceiptFrom>> = init_stable_mem_refcell("receipts", 3).unwrap();
    static LATEST_BLOCK: RefCell<DefaultVMCell<String>> = init_stable_mem_refcell("latest_block", 4).unwrap();
}

const RECIPIENT: &str = "0xB51f94aEEebE55A3760E8169A22e536eBD3a6DCB";
const URL: &str = "https://eth-sepolia.g.alchemy.com/v2/ZpSPh3E7KZQg4mb3tN8WFXxG4Auntbxp";

#[init]
fn init() {
    log!("Init");
}

async fn get_asset_transfers(from_block: String) -> Result<transfer::Result, String> {
    let params = json!({
        "fromBlock": from_block,
        "toAddress": RECIPIENT,
        "category": ["external"],
    });

    let rpc = json!({
        "jsonrpc": "2.0",
        "id": 0,
        "method": "alchemy_getAssetTransfers",
        "params": [params]
    });

    log!("Request: {}", rpc.to_string());

    let request = HttpOutcall::new(&URL)
        .post(&rpc.to_string(), None)
        .send_with_closure(|response: HttpResponse| HttpResponse {
            status: response.status,
            body: response.body,
            ..Default::default()
        })
        .await;

    match request {
        Ok(response) => match serde_json::from_slice::<transfer::Root>(&response.body) {
            Ok(response_body) => {
                log!("{:?}", response_body);

                Ok(response_body.result)
            }
            Err(m) => {
                return report_log(m);
            }
        },
        Err(e) => Err(format!("Error: {}", e)),
    }
}

async fn get_transaction(hash: TransactionHash) -> Result<transaction::Result, String> {
    let rpc = json!({
        "jsonrpc": "2.0",
        "id": 0,
        "method": "eth_getTransactionByHash",
        "params": [hash]
    });

    let request = HttpOutcall::new(URL)
        .post(&rpc.to_string(), Some(1024))
        .send_with_closure(|response: HttpResponse| HttpResponse {
            status: response.status,
            body: response.body,
            ..Default::default()
        });

    match request.await {
        Ok(response) => match serde_json::from_slice::<transaction::Root>(&response.body) {
            Ok(response_body) => {
                log!("{:?}", response_body);

                Ok(response_body.result)
            }
            Err(m) => {
                return report_log(m);
            }
        },
        Err(m) => {
            return report_log(m);
        }
    }
}

async fn get_transaction_receipt(hash: TransactionHash) -> Result<receipt::Result, String> {
    let rpc = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "eth_getTransactionReceipt",
        "params": [hash]
    });

    let request = HttpOutcall::new(URL)
        .post(&rpc.to_string(), Some(2024))
        .send_with_closure(|response: HttpResponse| HttpResponse {
            status: response.status,
            body: response.body,
            ..Default::default()
        });

    match request.await {
        Ok(response) => match serde_json::from_slice::<receipt::Root>(&response.body) {
            Ok(response_body) => {
                log!("{:?}", response_body);

                Ok(response_body.result)
            }
            Err(m) => {
                return report_log(m);
            }
        },
        Err(m) => {
            return report_log(m);
        }
    }
}
#[update]
async fn get_latest_external_transfer(from_block: String) -> String {
    let transfers = get_asset_transfers(from_block).await.unwrap();

    transfers.transfers[0].hash.clone()
}

#[update]
async fn get_transaction_value(hash: TransactionHash) -> TranasactionValue {
    if let Some(value) = TRANSACTIONS.with(|t| t.borrow().get(&hash)) {
        return value;
    }

    match get_transaction(hash).await {
        Ok(transaction) => TRANSACTIONS.with(|t| {
            let value = if transaction.to.eq_ignore_ascii_case(RECIPIENT) {
                transaction.value.clone()
            } else {
                "wrong".to_string()
            };

            t.borrow_mut()
                .insert(transaction.hash.clone(), value.clone());

            value
        }),
        Err(m) => panic!("Error: {}", m),
    }
}

#[update]
async fn get_transaction_receipt_from(hash: TransactionHash) -> ReceiptFrom {
    if let Some(from) = RECEIPTS.with(|r| r.borrow().get(&hash)) {
        return from;
    }

    match get_transaction_receipt(hash).await {
        Ok(receipt) => RECEIPTS.with(|r| {
            let from = if receipt.status.eq_ignore_ascii_case("0x1") {
                receipt.from.clone()
            } else {
                "failed".to_string()
            };

            r.borrow_mut()
                .insert(receipt.transaction_hash, from.clone());

            from
        }),
        Err(m) => {
            panic!("Error: {}", m);
        }
    }
}

#[update]
async fn verify_transaction(hash: TransactionHash) -> (u128, ReceiptFrom) {
    let value_string = get_transaction_value(hash.clone()).await;
    let receipt = get_transaction_receipt_from(hash).await;

    let value = hex_string_with_0x_to_u128(value_string).unwrap();

    (value, receipt)
}

#[query]
async fn get_transaction_list() -> Vec<(TransactionHash, TranasactionValue)> {
    TRANSACTIONS.with(|t| t.borrow().iter().collect())
}

#[query]
async fn get_receipt_list() -> Vec<(TransactionHash, ReceiptFrom)> {
    RECEIPTS.with(|r| r.borrow().iter().collect())
}

#[query]
fn get_timers() -> Vec<TaskTimerEntry<[u8; 32]>> {
    TASK_TIMER.with(|s| {
        let state = s.borrow();

        state.get_timers()
    })
}

#[query]
fn print_log_entries() -> Vec<LogEntry> {
    export_log()
}

#[update]
fn schedule_task(after_sec: u64, hash: TransactionHash) {
    let time = NanoTimeStamp::now().add_secs(after_sec);

    let task: [u8; 32] = hex_string_with_0x_to_vec(hash).unwrap().try_into().unwrap();

    let timer = TaskTimerEntry { task, time };

    TASK_TIMER
        .with(|tt| {
            let mut tt = tt.borrow_mut();

            tt.push_timer(&timer)
        })
        .unwrap();

    log!("Task scheduled: {:?}", timer);

    reschedule();
}

#[export_name = "canister_global_timer"]
fn global_timer() {
    while let Some(task_time) = TASK_TIMER.with(|tt| {
        let tt = tt.borrow();

        tt.peek_timer()
    }) {
        if task_time.time.in_future() {
            reschedule();
            return;
        }
        TASK_TIMER.with(|tt| {
            let mut tt = tt.borrow_mut();

            tt.pop_timer()
        });

        ic_cdk::spawn(execute_task(task_time));
        reschedule();
    }
}

async fn execute_task(timer: TaskTimerEntry<[u8; 32]>) {
    let hash = vec_to_hex_string_with_0x(timer.task);

    let _ = get_transaction_receipt(hash).await;

    log!("Task executed: {:?}", timer);
}

fn reschedule() {
    if let Some(task_time) = TASK_TIMER.with(|tt| {
        let tt = tt.borrow();

        tt.peek_timer()
    }) {
        unsafe {
            ic0::global_timer_set(task_time.time.into());
        }
    }
}

ic_cdk::export_candid!();
