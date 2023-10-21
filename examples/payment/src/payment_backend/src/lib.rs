use b3_utils::{
    call::InterCall,
    hex_string_with_0x_to_u64,
    ledger::ICRCAccount,
    outcall::{HttpOutcall, HttpOutcallResponse},
    owner::caller_is_owner,
    vec_to_hex_string_with_0x, Subaccount,
};
use candid::{CandidType, Nat, Principal};
use ic_cdk::{query, update};
use serde_derive::Deserialize;
use serde_json::json;

mod transaction;

const LEDGER: &str = "apia6-jaaaa-aaaar-qabma-cai";
const MINTER: &str = "jzenf-aiaaa-aaaar-qaa7q-cai";
const URL: &str = "https://eth-sepolia.g.alchemy.com/v2/ZpSPh3E7KZQg4mb3tN8WFXxG4Auntbxp";

#[derive(CandidType, Deserialize)]
pub struct WithdrawalArg {
    pub amount: Nat,
    pub recipient: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RetrieveEthRequest {
    pub block_index: Nat,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum WithdrawalError {
    AmountTooLow { min_withdrawal_amount: Nat },
    InsufficientFunds { balance: Nat },
    InsufficientAllowance { allowance: Nat },
    TemporarilyUnavailable(String),
}

type WithdrawalResult = Result<RetrieveEthRequest, WithdrawalError>;

#[derive(CandidType, Clone, Debug, PartialEq, Eq)]
pub struct ApproveArgs {
    pub from_subaccount: Option<Subaccount>,
    pub spender: ICRCAccount,
    pub amount: Nat,
    pub expected_allowance: Option<Nat>,
    pub expires_at: Option<u64>,
    pub memo: Option<Vec<u8>>,
    pub fee: Option<Nat>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ApproveError {
    BadFee { expected_fee: Nat },
    InsufficientFunds { balance: Nat },
    AllowanceChanged { current_allowance: Nat },
    Expired { ledger_time: u64 },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: Nat },
    TemporarilyUnavailable,
    GenericError { error_code: Nat, message: String },
}

type ApproveResult = Result<Nat, ApproveError>;

#[update(guard = "caller_is_owner")]
async fn balance() -> Nat {
    let account = ICRCAccount::new(ic_cdk::id(), None);

    InterCall::from(LEDGER)
        .call("icrc1_balance_of", account)
        .await
        .unwrap()
}

#[update(guard = "caller_is_owner")]
async fn approve(amount: Nat) -> ApproveResult {
    let approve = ApproveArgs {
        from_subaccount: None,
        spender: ICRCAccount::new(Principal::from_text(&MINTER).unwrap(), None),
        amount,
        expected_allowance: None,
        expires_at: None,
        memo: None,
        fee: None,
        created_at_time: None,
    };

    InterCall::from(LEDGER)
        .call("icrc2_approve", approve)
        .await
        .unwrap()
}

async fn get_transaction(hash: String) -> Result<transaction::Result, String> {
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
            Ok(response_body) => Ok(response_body.result),
            Err(m) => Err(format!("Error: {}", m)),
        },
        Err(m) => Err(format!("Error: {}", m)),
    }
}

#[update]
async fn verify_transaction(hash: String) -> bool {
    let tx = get_transaction(hash).await.unwrap();

    ic_cdk::println!("Transaction: {:?}", tx);

    let amount: Nat = match hex_string_with_0x_to_u64(tx.value) {
        Ok(amount) => amount.into(),
        Err(m) => panic!("{}", m.to_string()),
    };

    let recipient = tx.to;

    todo!("Verify transaction");
}

#[update(guard = "caller_is_owner")]
async fn withdraw_eth(amount: Nat, recipient: String) -> WithdrawalResult {
    let withraw = WithdrawalArg { amount, recipient };

    InterCall::from(MINTER)
        .call("withdraw_eth", withraw)
        .await
        .unwrap()
}

#[query]
async fn deposit_principal() -> String {
    let subaccount = Subaccount::from(ic_cdk::id());

    let bytes32 = subaccount.to_bytes32().unwrap();

    vec_to_hex_string_with_0x(bytes32)
}

ic_cdk::export_candid!();
