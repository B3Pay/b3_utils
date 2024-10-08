//! # B3Utils Library
//!
//! `b3_utils` is a Rust library designed to simplify the development of applications and wallets on the Internet Computer. It provides utility functions and types for handling various operations such as transfers, tokens, timestamps, and more.
//!
//! ## Features
//!
//! - `vetkd`: Enables functionality related to vetkd. Includes dependencies `ic_bls12_381`, `sha2`, and `subtle`.
//! - `logging`: Enables logging functionality.
//! - `ledger`: Enables ledger-related functionalities.
//! - `owner`: Enables owner-related functionalities.
//! - `sha2`: Enables SHA-2 hashing functionality.
//! - `wasm`: Enables WebAssembly-related functionalities.
//! - `rpc`: Enables EVM-RPC-canister functionalities.
//!
//! To enable a feature, add it to your `Cargo.toml` like so:
//!
//! ```toml
//! [dependencies]
//! b3_utils = { version = "0.12", features = ["exprimental_vetkd", "logging"] }
//! ```
//!
//! ## Examples
//!
//! Here's a simple example of how to create a new [icrc1 account](https://internetcomputer.org/docs/current/developer-docs/integrations/icrc-1/) :
//!
//! ```rust
//! use b3_utils::ledger::ICRCAccount;
//! use b3_utils::Subaccount;
//! use b3_utils::Environment;
//! use candid::Principal;
//!
//! let owner = Principal::from_text("b7pqa-qqaaa-aaaap-abdva-cai").unwrap();
//! let subaccount = Subaccount::new(Environment::Production, 1);
//!
//! let account = ICRCAccount::new(owner, Some(subaccount));
//!
//! assert_eq!(account.to_text(), "b7pqa-qqaaa-aaaap-abdva-cai-vpwy45i.1");
//! ```
//!
//! For more detailed examples, see the documentation for each module.
//!
//! ## More Information
//!
//! For more information, see the [API documentation](https://docs.rs/b3_utils).
//!
//! ## License
//!
//! This project is licensed under the MIT License.

pub mod constants;
pub mod error;
pub mod http;
pub mod metrics;
pub mod mocks;
pub mod nonce;
pub mod status;
pub mod types;

pub mod outcall;
pub use outcall::*;

pub mod api;

mod timestamp;
pub use timestamp::*;

mod subaccount;
pub use subaccount::*;

mod environment;
pub use environment::*;

mod utils;
pub use utils::*;

pub mod owner;

#[cfg(feature = "metadata")]
pub mod metadata;

#[cfg(feature = "sha2")]
pub mod sha2;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "ledger")]
pub mod ledger;

#[cfg(feature = "logging")]
pub mod logs;

#[cfg(feature = "notifier")]
pub mod notifier;

pub mod memory;
pub mod principal;

#[cfg(feature = "rpc")]
pub mod rpc;

// VETKD API not yet ready for use
#[cfg(feature = "exprimental_vetkd")]
pub mod vetkd;
