use crate::{error::HelperError, nonce::Nonce, types::CanisterId, NanoTimeStamp};
use candid::{ser::IDLBuilder, utils::ArgumentEncoder, CandidType, Encode, Principal};
use ic_cdk::api::management_canister::main::{
    CanisterInstallMode, CanisterStatusResponse, WasmModule,
};
use serde::{Deserialize, Serialize};

pub type AppVersion = String;

pub struct AppInstallArg {
    pub arg: Vec<u8>,
    pub wasm_module: WasmModule,
    pub mode: CanisterInstallMode,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub struct AppInitArgs {
    pub owner_id: Principal,
    pub forge_id: CanisterId,
}

impl AppInitArgs {
    pub fn encode(&self) -> Result<Vec<u8>, HelperError> {
        Encode!(&self).map_err(|e| HelperError::EncodeError(e.to_string()))
    }
}

#[derive(Debug, PartialEq, Eq, CandidType, Clone, Deserialize, Serialize)]
pub struct RequestJoinArgs {
    pub name: String,
    pub signer_id: Principal,
}

impl ArgumentEncoder for RequestJoinArgs {
    fn encode(self, ser: &mut IDLBuilder) -> candid::Result<()> {
        ser.arg(&self.name)?;
        ser.arg(&self.signer_id)?;
        ser.serialize_to_vec()?;

        Ok(())
    }
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppStatus {
    pub name: String,
    pub version: String,
    pub status_at: NanoTimeStamp,
    pub canister_id: CanisterId,
    pub account_status: AppAccountsNonce,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType, Default, Clone, Deserialize, Serialize)]
pub struct AppAccountsNonce {
    pub development: Nonce,
    pub production: Nonce,
    pub staging: Nonce,
}

pub enum CallCycles {
    NoPay,
    Pay(u64),
    Pay128(u128),
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::{decode_one, encode_one, Principal};

    #[test]
    fn test_request_join_args_decoding() {
        // Create an instance of RequestJoinArgs
        let original_args = RequestJoinArgs {
            name: "Alice".to_string(),
            signer_id: Principal::anonymous(),
        };

        // Serialize the instance
        let encoded_args = encode_one(&original_args).expect("Failed to encode");

        // Deserialize the serialized data
        let decoded_args: RequestJoinArgs = decode_one(&encoded_args).expect("Failed to decode");

        // Assert that the original and deserialized instances are equal
        assert_eq!(original_args, decoded_args);
    }
}
