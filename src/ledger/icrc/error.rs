use candid::{CandidType, Deserialize, Nat};

use std::fmt;

use super::ICRCTimestamp;

#[rustfmt::skip]
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum ICRC1TransferError {
    BadFee { expected_fee: Nat },
    BadBurn { min_burn_amount: Nat },
    InsufficientFunds { balance: Nat },
    TooOld,
    CreatedInFuture { ledger_time: ICRCTimestamp },
    Duplicate { duplicate_of: Nat },
    TemporarilyUnavailable,
    GenericError { error_code: Nat, message: String },
}

#[rustfmt::skip]
impl fmt::Display for ICRC1TransferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ICRC1TransferError::BadFee { expected_fee } => write!(f, "Bad fee: expected {}", expected_fee),
            ICRC1TransferError::BadBurn { min_burn_amount } => write!(f, "Bad burn: minimum burn amount is {}", min_burn_amount),
            ICRC1TransferError::InsufficientFunds { balance } => write!(f, "Insufficient funds: balance is {}", balance),
            ICRC1TransferError::TooOld => write!(f, "Transaction is too old"),
            ICRC1TransferError::CreatedInFuture { ledger_time } => write!(f, "Transaction created in the future: {}", ledger_time),
            ICRC1TransferError::Duplicate { duplicate_of } => write!(f, "Duplicate transaction: duplicate of {}", duplicate_of),
            ICRC1TransferError::TemporarilyUnavailable => write!(f, "Temporarily unavailable"),
            ICRC1TransferError::GenericError { error_code, message } => write!(f, "Generic error: {} - {}", error_code, message),
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ICRC2ApproveError {
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

#[rustfmt::skip]
impl fmt::Display for ICRC2ApproveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ICRC2ApproveError::BadFee { expected_fee } => write!(f, "Bad fee: expected {}", expected_fee),
            ICRC2ApproveError::InsufficientFunds { balance } => write!(f, "Insufficient funds: balance is {}", balance),
            ICRC2ApproveError::AllowanceChanged { current_allowance } => write!(f, "Allowance changed: current allowance is {}", current_allowance),
            ICRC2ApproveError::Expired { ledger_time } => write!(f, "Transaction expired: {}", ledger_time),
            ICRC2ApproveError::TooOld => write!(f, "Transaction is too old"),
            ICRC2ApproveError::CreatedInFuture { ledger_time } => write!(f, "Transaction created in the future: {}", ledger_time),
            ICRC2ApproveError::Duplicate { duplicate_of } => write!(f, "Duplicate transaction: duplicate of {}", duplicate_of),
            ICRC2ApproveError::TemporarilyUnavailable => write!(f, "Temporarily unavailable"),
            ICRC2ApproveError::GenericError { error_code, message } => write!(f, "Generic error: {} - {}", error_code, message),
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ICRC2TransferFromError {
    BadFee { expected_fee: Nat },
    BadBurn { min_burn_amount: Nat },
    // The [from] account does not hold enough funds for the transfer.
    InsufficientFunds { balance: Nat },
    // The caller exceeded its allowance.
    InsufficientAllowance { allowance: Nat },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: Nat },
    TemporarilyUnavailable,
    GenericError { error_code: Nat, message: String },
}

#[rustfmt::skip]
impl fmt::Display for ICRC2TransferFromError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ICRC2TransferFromError::BadFee { expected_fee } => write!(f, "Bad fee: expected {}", expected_fee),
            ICRC2TransferFromError::BadBurn { min_burn_amount } => write!(f, "Bad burn: minimum burn amount is {}", min_burn_amount),
            ICRC2TransferFromError::InsufficientFunds { balance } => write!(f, "Insufficient funds: balance is {}", balance),
            ICRC2TransferFromError::InsufficientAllowance { allowance } => write!(f, "Insufficient allowance: allowance is {}", allowance),
            ICRC2TransferFromError::TooOld => write!(f, "Transaction is too old"),
            ICRC2TransferFromError::CreatedInFuture { ledger_time } => write!(f, "Transaction created in the future: {}", ledger_time),
            ICRC2TransferFromError::Duplicate { duplicate_of } => write!(f, "Duplicate transaction: duplicate of {}", duplicate_of),
            ICRC2TransferFromError::TemporarilyUnavailable => write!(f, "Temporarily unavailable"),
            ICRC2TransferFromError::GenericError { error_code, message } => write!(f, "Generic error: {} - {}", error_code, message),
        }
    }
}
