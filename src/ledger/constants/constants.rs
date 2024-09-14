use crate::{ledger::currency::ICPToken, NanoTimeStamp};

pub const SYSTEM_RATE_LIMIT: u64 = NanoTimeStamp::NS_PER_SECOND;

pub const IC_TRANSACTION_FEE_ICP: ICPToken = ICPToken::from_e8s(10_000);
