use b3_utils::{
    api::{CallCycles, InterCall},
    ledger::ICRCAccount,
    owner::caller_is_owner,
    rpc::{
        EthSepoliaService, GetTransactionReceiptResult, MultiGetTransactionReceiptResult,
        RpcServices, EVM_RPC,
    },
    vec_to_hex_string_with_0x, Subaccount,
};
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::{query, update};

const LEDGER: &str = "apia6-jaaaa-aaaar-qabma-cai";
const MINTER: &str = "jzenf-aiaaa-aaaar-qaa7q-cai";

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
        .call("icrc1_balance_of", account, CallCycles::NoPay)
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
        .call("icrc2_approve", approve, CallCycles::NoPay)
        .await
        .unwrap()
}

#[update]
async fn eth_get_transaction_receipt(hash: String) -> Result<GetTransactionReceiptResult, String> {
    // Make the call to the EVM_RPC canister
    let result: Result<(MultiGetTransactionReceiptResult,), String> = EVM_RPC
        .eth_get_transaction_receipt(
            RpcServices::EthSepolia(Some(vec![
                EthSepoliaService::PublicNode,
                EthSepoliaService::BlockPi,
                EthSepoliaService::Ankr,
            ])),
            None,
            hash,
            10_000_000_000,
        )
        .await
        .map_err(|e| format!("Failed to call eth_getTransactionReceipt: {:?}", e));

    match result {
        Ok((MultiGetTransactionReceiptResult::Consistent(receipt),)) => Ok(receipt),
        Ok((MultiGetTransactionReceiptResult::Inconsistent(error),)) => Err(format!(
            "EVM_RPC returned inconsistent results: {:?}",
            error
        )),
        Err(e) => Err(format!("Error calling EVM_RPC: {}", e)),
    }
}

type WithdrawalResult = Result<RetrieveEthRequest, WithdrawalError>;

#[update(guard = "caller_is_owner")]
async fn withdraw_eth(amount: Nat, recipient: String) -> WithdrawalResult {
    let withraw = WithdrawalArg { amount, recipient };

    InterCall::from(MINTER)
        .call("withdraw_eth", withraw, CallCycles::NoPay)
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
