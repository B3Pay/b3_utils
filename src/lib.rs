//! # B3Utils Library
//!
//! `b3_utils` is a Rust library designed to simplify the development of applications and wallets on the Internet Computer. It provides utility functions and types for handling various operations such as transfers, tokens, timestamps, and more.
//!
//! ## Features
//!
//! - `vetkd`: Enables functionality related to vetkd. Includes dependencies `ic_bls12_381`, `sha2`, and `subtle`.
//! - `stable_memory`: Enables stable memory features. Includes the `b3-stable-structures` dependency.
//! - `logging`: Enables logging functionality.
//! - `ledger`: Enables ledger-related functionalities.
//!
//! To enable a feature, add it to your `Cargo.toml` like so:
//!
//! ```toml
//! [dependencies]
//! my_library = { version = "0.1", features = ["vetkd", "logging"] }
//! ```
//! ## Modules
//!  
//! - **Account** - Create and manage accounts.
//! - **Constants** - Constants used by the library.
//! - **Currency** - Currency types and conversion functions.
//! - **Environment** - Environment types and conversion functions.
//! - **Error** - Error types and conversion functions.
//! - **Owner** - Owner types and conversion functions.
//! - **Release** - Release types and conversion functions.
//! - **Subaccount** - Subaccount types and conversion functions.
//! - **System** - System types and conversion functions.
//! - **Timestamp** - Timestamp types and conversion functions.
//! - **Types** - Types used by the library.
//! - **Utils** - Utility functions.
//! - **Wallet** - Wallet types and conversion functions.
//! - **Wasm** - Wasm types and conversion functions.
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
pub mod mocks;
pub mod nonce;
pub mod owner;
pub mod release;
pub mod types;
pub mod wasm;

mod timestamp;
pub use timestamp::*;

mod subaccount;
pub use subaccount::*;

mod environment;
pub use environment::*;

mod utils;
pub use utils::*;

#[cfg(feature = "ledger")]
pub mod ledger;

#[cfg(feature = "logging")]
pub mod logs;

#[cfg(feature = "stable_memory")]
pub mod memory;

#[cfg(feature = "vetkd")]
pub mod vetkd;
