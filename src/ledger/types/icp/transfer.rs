mod error;
pub use error::*;

use crate::ledger::currency::ICPToken;
use crate::ledger::types::{TransferBlockIndex, TransferMemo};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct ICPTransferTimestamp {
    /// Number of nanoseconds from the UNIX epoch in UTC timezone.
    pub timestamp_nanos: u64,
}

#[derive(Debug, CandidType, Deserialize, Serialize, Clone)]
pub struct ICPTransferArgs {
    pub memo: TransferMemo,
    pub fee: ICPToken,
    pub amount: ICPToken,
    pub to: serde_bytes::ByteBuf,
    pub from_subaccount: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<ICPTransferTimestamp>,
}

pub type ICPTransferResult = Result<TransferBlockIndex, ICPTransferError>;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct ICPAccountBalanceArgs {
    pub account: serde_bytes::ByteBuf,
}
