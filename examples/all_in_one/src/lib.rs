use b3_utils::{
    hex_string_with_0x_to_u128,
    http::{HttpRequest, HttpResponse, HttpResponseBuilder},
    log_cycle,
    logs::{export_log, LogEntry},
    memory::{
        init_stable_mem_refcell,
        timer::{DefaultTaskTimer, TaskTimerEntry},
        types::{Bound, DefaultStableBTreeMap, PartitionDetail, Storable},
        with_stable_mem,
    },
    outcall::{HttpOutcall, HttpOutcallResponse},
    owner::{caller_is_owner, get_owner, set_owner},
    report_log, NanoTimeStamp,
};
use candid::{CandidType, Principal};
use ic_cdk::{init, post_upgrade, query, update};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::cell::RefCell;

mod receipt;
mod test;
mod transaction;
mod transfer;

type TransactionHash = String;
type ReceiptFrom = String;
type TranasactionValue = String;
type BlockNumber = String;

const RECIPIENT: &str = "0xB51f94aEEebE55A3760E8169A22e536eBD3a6DCB";
const URL: &str = "https://eth-sepolia.g.alchemy.com/v2/ZpSPh3E7KZQg4mb3tN8WFXxG4Auntbxp";

thread_local! {
    static TASK_TIMER: RefCell<DefaultTaskTimer<Task>> = init_stable_mem_refcell("timer", 1).unwrap();
    static TRANSACTIONS: RefCell<DefaultStableBTreeMap<TransactionHash, TranasactionValue>> = init_stable_mem_refcell("trasnactions", 2).unwrap();
    static RECEIPTS: RefCell<DefaultStableBTreeMap<TransactionHash, ReceiptFrom>> = init_stable_mem_refcell("receipts", 3).unwrap();
    static EXTERNAL_TRANSFERS: RefCell<DefaultStableBTreeMap<TransactionHash, String>> = init_stable_mem_refcell("external_transfers", 4).unwrap();
}

#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
enum Task {
    GetLatestExternalTransfer(BlockNumber),
    GetTransactionValue(TransactionHash),
    GetTransactionReceiptFrom(TransactionHash),
    VerifyTransaction(TransactionHash),
}

impl Storable for Task {
    const BOUND: Bound = Bound::Bounded {
        max_size: 32,
        is_fixed_size: false,
    };

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        candid::decode_one(bytes.as_ref()).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        candid::encode_one(self).unwrap().into()
    }
}

#[init]
fn init() {
    log_cycle!("Init");

    schedule_task(10, Task::GetLatestExternalTransfer("0x43d20e".to_string()));
}

#[post_upgrade]
fn post_upgrade() {
    log_cycle!("Post upgrade");
    reschedule();
}

#[query]
fn owner() -> Principal {
    get_owner()
}

#[update(guard = "caller_is_owner")]
fn change_owner(new_owner: Principal) {
    log_cycle!("Change owner: {}", new_owner);

    set_owner(new_owner.into()).unwrap();
}

#[update(guard = "caller_is_owner")]
fn stop_timer() {
    log_cycle!("Stop Timer");

    TASK_TIMER.with(|tt| {
        let mut tt = tt.borrow_mut();

        tt.clear_timer()
    });
}

#[update(guard = "caller_is_owner")]
fn start_timer() {
    log_cycle!("Start Timer");

    schedule_task(10, Task::GetLatestExternalTransfer("0x43d20e".to_string()));
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

    log_cycle!("Request: {}", rpc.to_string());

    let request = HttpOutcall::new(&URL)
        .post(&rpc.to_string(), None)
        .send_with_closure(|response: HttpOutcallResponse| HttpOutcallResponse {
            status: response.status,
            body: response.body,
            ..Default::default()
        })
        .await;

    match request {
        Ok(response) => match serde_json::from_slice::<transfer::Root>(&response.body) {
            Ok(response_body) => {
                log_cycle!("{:?}", response_body);

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
        .send_with_closure(|response: HttpOutcallResponse| HttpOutcallResponse {
            status: response.status,
            body: response.body,
            ..Default::default()
        });

    match request.await {
        Ok(response) => match serde_json::from_slice::<transaction::Root>(&response.body) {
            Ok(response_body) => {
                log_cycle!("{:?}", response_body);

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
        .send_with_closure(|response: HttpOutcallResponse| HttpOutcallResponse {
            status: response.status,
            body: response.body,
            ..Default::default()
        });

    match request.await {
        Ok(response) => match serde_json::from_slice::<receipt::Root>(&response.body) {
            Ok(response_body) => {
                log_cycle!("{:?}", response_body);

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

    EXTERNAL_TRANSFERS.with(|r| {
        let mut r = r.borrow_mut();

        for transfer in transfers.transfers.iter() {
            r.insert(
                transfer.hash.clone(),
                serde_json::to_string(&transfer).unwrap(),
            );
        }
    });

    if let Some(last_transfer) = transfers.transfers.last() {
        last_transfer.block_num.clone()
    } else {
        "latest".to_string()
    }
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
fn get_transaction_list() -> Vec<TranasactionValue> {
    TRANSACTIONS.with(|t| t.borrow().iter().map(|(_, v)| v.clone()).collect())
}

#[query]
fn get_receipt_list() -> Vec<ReceiptFrom> {
    RECEIPTS.with(|r| r.borrow().iter().map(|(_, v)| v.clone()).collect())
}

#[query]
fn get_external_transfers() -> Vec<String> {
    EXTERNAL_TRANSFERS.with(|r| r.borrow().iter().map(|(_, v)| v.clone()).collect())
}

#[query]
fn get_timers() -> Vec<TaskTimerEntry<Task>> {
    TASK_TIMER.with(|s| {
        let state = s.borrow();

        state.get_timers()
    })
}

#[query]
fn get_partition_details() -> Vec<PartitionDetail> {
    with_stable_mem(|pm| pm.partition_details())
}

#[query]
fn print_log_entries() -> Vec<LogEntry> {
    export_log()
}

#[update]
fn schedule_task(after_sec: u64, task: Task) {
    let time = NanoTimeStamp::now().add_secs(after_sec);

    let timer = TaskTimerEntry { task, time };

    TASK_TIMER
        .with(|tt| {
            let mut tt = tt.borrow_mut();

            tt.push_timer(&timer)
        })
        .unwrap();

    log_cycle!("Task scheduled: {:?}", timer);

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

        ic_cdk::spawn(execute_task(task_time.task));
        reschedule();
    }
}

async fn execute_task(task: Task) {
    match task {
        Task::GetLatestExternalTransfer(block_number) => {
            let next_block_number = get_latest_external_transfer(block_number).await;

            log_cycle!("Task executed: {}", next_block_number);

            schedule_task(60, Task::GetLatestExternalTransfer(next_block_number));
        }
        _ => panic!("Wrong task"),
    };
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

#[query]
fn http_request(req: HttpRequest) -> HttpResponse {
    match req.path() {
        "/metrics" => {
            let transactions = get_transaction_list();
            let receipts = get_receipt_list();
            let external_transfers = get_external_transfers();

            let list = json!({
                "transactions": transactions,
                "receipts": receipts,
                "external_transfers": external_transfers,
            });

            HttpResponseBuilder::ok()
                .header("Content-Type", "application/json; charset=utf-8")
                .with_body_and_content_length(serde_json::to_string(&list).unwrap_or_default())
                .build()
        }
        "/partition_details" => {
            let list = get_partition_details();

            HttpResponseBuilder::ok()
                .header("Content-Type", "application/json; charset=utf-8")
                .with_body_and_content_length(serde_json::to_string(&list).unwrap_or_default())
                .build()
        }
        "/timers" => {
            let list = get_timers();

            HttpResponseBuilder::ok()
                .header("Content-Type", "application/json; charset=utf-8")
                .with_body_and_content_length(serde_json::to_string(&list).unwrap_or_default())
                .build()
        }
        "/logs" => {
            let list = print_log_entries();

            HttpResponseBuilder::ok()
                .header("Content-Type", "application/json; charset=utf-8")
                .with_body_and_content_length(serde_json::to_string(&list).unwrap_or_default())
                .build()
        }
        _ => HttpResponseBuilder::not_found().build(),
    }
}

ic_cdk::export_candid!();
