#![allow(rustdoc::private_intra_doc_links)]
//! # Breez SDK - Liquid
//!
//! This SDK provides developers with an end-to-end solution for integrating self-custodial Lightning
//! payments into their apps and services. It eliminates the need for third-parties, simplifies the
//! complexities of Bitcoin and Lightning, and enables seamless onboarding for billions of users to
//! the future of peer-to-peer payments.
//!
//! The Liquid implementation is a nodeless Lightning integration. It offers a self-custodial,
//! end-to-end solution for integrating Lightning payments, utilizing the Liquid Network with
//! on-chain interoperability and third-party fiat on-ramps.
//!
//! * Sending payments (via protocols such as: bolt11, lnurl-pay, lightning address, btc address)
//! * Receiving payments (via protocols such as: bolt11, lnurl-withdraw, btc address)
//! * Interacting with a wallet (e.g. balance, max allow to pay, max allow to receive, on-chain balance)
//!
//! ## Getting Started
//!
//! The following code initialize the SDK and make it ready to be used:
//!
//! ```ignore
//! let mnemonic = Mnemonic::generate_in(Language::English, 12)?;
//!
//! // Create the default config
//! let mut config = sdk::LiquidSdk::default_config(LiquidNetwork::Mainnet);
//!
//! // Customize the config object according to your needs
//! config.working_dir = "path to an existing directory".into();
//!
//! let connect_request = ConnectRequest {
//!     mnemonic: mnemonic.to_string(),
//!     config,
//! };
//! let sdk = sdk::LiquidSdk::connect(connect_request).await?;
//!
//! ```
//!
//! We can now receive payments
//!
//! ```ignore
//! // Fetch the Receive limits
//! let current_limits = sdk.fetch_lightning_limits().await?;
//! info!("Minimum amount: {} sats", current_limits.receive.min_sat);
//! info!("Maximum amount: {} sats", current_limits.receive.max_sat);
//!
//! // Set the amount you wish the payer to send, which should be within the above limits
//! let prepare_receive_response = sdk
//!     .prepare_receive_payment(&PrepareReceivePaymentRequest {
//!         receive_method: None, // TODO Add documentation
//!         payer_amount_sat: 5_000,
//!     })
//!     .await?;
//!
//! // If the fees are acceptable, continue to create the Receive Payment
//! let receive_fees_sat = prepare_receive_response.fees_sat;
//!
//! let receive_payment_response = sdk.receive_payment(&prepare_receive_response).await?;
//!
//! let invoice = receive_payment_response.invoice;
//! ```
//!
//! or make payments
//! ```ignore
//! // Set the BOLT11 invoice you wish to pay
//! let prepare_send_response = sdk
//!     .prepare_send_payment(&PrepareSendRequest {
//!         invoice: "...".to_string(),
//!     })
//!     .await?;
//!
//! // If the fees are acceptable, continue to create the Send Payment
//! let send_fees_sat = prepare_send_response.fees_sat;
//!
//! let send_response = sdk.send_payment(&prepare_send_response).await?;
//! let payment = send_response.payment;
//! ```
//!
//! At any point we can fetch the wallet state
//! ```ignore
//! let wallet_info = sdk.get_info().await?;
//! let balance_sat = wallet_info.balance_sat;
//! let pending_send_sat = wallet_info.pending_send_sat;
//! let pending_receive_sat = wallet_info.pending_receive_sat;
//! ```
//!
//! or fetch other useful infos, like the current mempool [model::RecommendedFees]
//! ```ignore
//! let fees = sdk.recommended_fees().await?;
//! ```
//!
//! These different types of operations are described below in more detail.
//!
//! ### Initializing the SDK
//!
//! There are two simple steps necessary to initialize the SDK:
//!
//! 1. [sdk::LiquidSdk::default_config] to construct the SDK [model::Config]
//! 2. [sdk::LiquidSdk::connect] to initialize the [sdk::LiquidSdk] instance
//!
//! Although you can create your own config from scratch it is recommended to use the
//! [sdk::LiquidSdk::default_config] method and customize it according to your needs.
//! Once the [model::Config] is created it is passed to the [sdk::LiquidSdk::connect] method
//! along with the mnemonic.
//!
//! Now your SDK is ready to be used.
//!
//! ### Sending a Lightning payment
//!
//! * [sdk::LiquidSdk::prepare_send_payment] to check fees
//! * [sdk::LiquidSdk::send_payment] to pay an invoice
//!
//! ### Receiving a Lightning payment
//!
//! * [sdk::LiquidSdk::prepare_receive_payment] to check fees
//! * [sdk::LiquidSdk::receive_payment] to generate an invoice
//!
//! ### Sending an onchain payment
//!
//! * [sdk::LiquidSdk::prepare_pay_onchain] to check fees
//! * [sdk::LiquidSdk::pay_onchain] to pay to a Bitcoin address
//!
//! ### Receiving an onchain payment
//!
//! * [sdk::LiquidSdk::prepare_receive_onchain] to check fees
//! * [sdk::LiquidSdk::receive_onchain] to generate a Bitcoin address
//! * [sdk::LiquidSdk::list_refundables] to get a list of refundable swaps
//! * [sdk::LiquidSdk::prepare_refund] to check the refund fees
//! * [sdk::LiquidSdk::refund] to broadcast a refund transaction
//!
//! ### Using LNURL
//!
//! * [parse] the LNURL endpoint URL to get the workflow parameters
//! * [sdk::LiquidSdk::lnurl_pay] to pay to the parsed LNURL
//! * [sdk::LiquidSdk::lnurl_withdraw] to withdraw from the parsed LNURL
//!
//! ### Supporting fiat currencies
//!
//! * [sdk::LiquidSdk::list_fiat_currencies] to get the supported fiat currencies
//! * [sdk::LiquidSdk::fetch_fiat_rates] to get the current exchange rates

//! ### Utilities
//!
//! * [sdk::LiquidSdk::recommended_fees] for the recommended mempool fees
//! * [parse] to parse a string into an [InputType]
//! * [parse_invoice] to parse a string into an [LNInvoice]
//!
//!
//! ## Bindings
//!
//! * Dart
//! * Flutter
//! * Kotlin
//! * Python
//! * React-Native
//! * Swift
//!
//! ## Support
//!
//! Join this [telegram group](https://t.me/breezsdk).

#[cfg(feature = "frb")]
pub(crate) mod bindings;
pub(crate) mod buy;
pub(crate) mod chain;
pub(crate) mod chain_swap;
pub mod error;
pub(crate) mod event;
#[cfg(feature = "frb")]
pub(crate) mod frb_generated;
pub mod logger;
pub mod model;
pub mod persist;
pub(crate) mod receive_swap;
pub mod sdk;
pub(crate) mod send_swap;
pub(crate) mod swapper;
pub(crate) mod test_utils;
pub(crate) mod utils;
pub(crate) mod wallet;

pub use sdk_common::prelude::*;

#[allow(ambiguous_glob_reexports)]
#[rustfmt::skip]
pub mod prelude {
    pub use crate::*;
    pub use crate::model::*;
    pub use crate::sdk::*;
}
