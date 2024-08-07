//! Dart / flutter bindings

use std::sync::Arc;

use anyhow::Result;
use flutter_rust_bridge::frb;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
pub use sdk_common::prelude::{
    AesSuccessActionDataDecrypted, AesSuccessActionDataResult, BitcoinAddressData, CurrencyInfo,
    FiatCurrency, InputType, LNInvoice, LnUrlAuthRequestData, LnUrlErrorData, LnUrlPayErrorData,
    LnUrlPayRequest, LnUrlPayRequestData, LnUrlWithdrawRequest, LnUrlWithdrawRequestData,
    LocaleOverrides, LocalizedName, MessageSuccessActionData, Network, Rate, RouteHint,
    RouteHintHop, SuccessActionProcessed, Symbol, UrlSuccessActionData,
};

use crate::{error::*, frb_generated::StreamSink, model::*, sdk::LiquidSdk};

pub struct BindingEventListener {
    pub stream: StreamSink<SdkEvent>,
}

impl EventListener for BindingEventListener {
    fn on_event(&self, e: SdkEvent) {
        let _ = self.stream.add(e);
    }
}

struct DartBindingLogger {
    log_stream: StreamSink<LogEntry>,
}

impl DartBindingLogger {
    fn init(log_stream: StreamSink<LogEntry>) -> Result<(), SetLoggerError> {
        let binding_logger: DartBindingLogger = DartBindingLogger { log_stream };
        log::set_boxed_logger(Box::new(binding_logger))
            .map(|_| log::set_max_level(LevelFilter::Trace))
    }
}

impl log::Log for DartBindingLogger {
    fn enabled(&self, m: &Metadata) -> bool {
        m.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let _ = self.log_stream.add(LogEntry {
                line: record.args().to_string(),
                level: record.level().as_str().to_string(),
            });
        }
    }
    fn flush(&self) {}
}

pub async fn connect(req: ConnectRequest) -> Result<BindingLiquidSdk, SdkError> {
    let ln_sdk = LiquidSdk::connect(req).await?;
    Ok(BindingLiquidSdk { sdk: ln_sdk })
}

/// If used, this must be called before `connect`. It can only be called once.
pub fn breez_log_stream(s: StreamSink<LogEntry>) -> Result<()> {
    DartBindingLogger::init(s).map_err(|_| SdkError::Generic {
        err: "Log stream already created".into(),
    })?;
    Ok(())
}

#[frb(sync)]
pub fn default_config(network: LiquidNetwork) -> Config {
    LiquidSdk::default_config(network)
}

pub async fn parse(input: String) -> Result<InputType, PaymentError> {
    LiquidSdk::parse(&input).await
}

#[frb(sync)]
pub fn parse_invoice(input: String) -> Result<LNInvoice, PaymentError> {
    LiquidSdk::parse_invoice(&input)
}

pub struct BindingLiquidSdk {
    sdk: Arc<LiquidSdk>,
}

impl BindingLiquidSdk {
    pub async fn get_info(&self) -> Result<GetInfoResponse, SdkError> {
        self.sdk.get_info().await.map_err(Into::into)
    }

    pub async fn add_event_listener(
        &self,
        listener: StreamSink<SdkEvent>,
    ) -> Result<String, SdkError> {
        self.sdk
            .add_event_listener(Box::new(BindingEventListener { stream: listener }))
            .await
    }

    pub async fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        self.sdk.prepare_send_payment(&req).await
    }

    pub async fn send_payment(
        &self,
        req: PrepareSendResponse,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.sdk.send_payment(&req).await
    }

    pub async fn prepare_receive_payment(
        &self,
        req: PrepareReceivePaymentRequest,
    ) -> Result<PrepareReceivePaymentResponse, PaymentError> {
        self.sdk.prepare_receive_payment(&req).await
    }

    pub async fn receive_payment(
        &self,
        req: ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.sdk.receive_payment(&req).await
    }

    pub async fn fetch_lightning_limits(
        &self,
    ) -> Result<LightningPaymentLimitsResponse, PaymentError> {
        self.sdk.fetch_lightning_limits().await
    }

    pub async fn fetch_onchain_limits(&self) -> Result<OnchainPaymentLimitsResponse, PaymentError> {
        self.sdk.fetch_onchain_limits().await
    }

    pub async fn prepare_pay_onchain(
        &self,
        req: PreparePayOnchainRequest,
    ) -> Result<PreparePayOnchainResponse, PaymentError> {
        self.sdk.prepare_pay_onchain(&req).await
    }

    pub async fn pay_onchain(
        &self,
        req: PayOnchainRequest,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.sdk.pay_onchain(&req).await
    }

    pub async fn prepare_receive_onchain(
        &self,
        req: PrepareReceiveOnchainRequest,
    ) -> Result<PrepareReceiveOnchainResponse, PaymentError> {
        self.sdk.prepare_receive_onchain(&req).await
    }

    pub async fn receive_onchain(
        &self,
        req: PrepareReceiveOnchainResponse,
    ) -> Result<ReceiveOnchainResponse, PaymentError> {
        self.sdk.receive_onchain(&req).await
    }

    pub async fn prepare_buy_bitcoin(
        &self,
        req: PrepareBuyBitcoinRequest,
    ) -> Result<PrepareBuyBitcoinResponse, PaymentError> {
        self.sdk.prepare_buy_bitcoin(&req).await
    }

    pub async fn buy_bitcoin(&self, req: BuyBitcoinRequest) -> Result<String, PaymentError> {
        self.sdk.buy_bitcoin(&req).await
    }

    pub async fn list_payments(
        &self,
        req: ListPaymentsRequest,
    ) -> Result<Vec<Payment>, PaymentError> {
        self.sdk.list_payments(&req).await
    }

    pub async fn lnurl_pay(
        &self,
        req: LnUrlPayRequest,
    ) -> Result<LnUrlPayResult, duplicates::LnUrlPayError> {
        self.sdk.lnurl_pay(req).await.map_err(Into::into)
    }

    pub async fn lnurl_withdraw(
        &self,
        req: LnUrlWithdrawRequest,
    ) -> Result<duplicates::LnUrlWithdrawResult, duplicates::LnUrlWithdrawError> {
        self.sdk
            .lnurl_withdraw(req)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    pub async fn lnurl_auth(
        &self,
        req_data: LnUrlAuthRequestData,
    ) -> Result<duplicates::LnUrlCallbackStatus, duplicates::LnUrlAuthError> {
        self.sdk
            .lnurl_auth(req_data)
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    pub async fn fetch_fiat_rates(&self) -> Result<Vec<Rate>, SdkError> {
        self.sdk.fetch_fiat_rates().await
    }

    pub async fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>, SdkError> {
        self.sdk.list_fiat_currencies().await
    }

    pub async fn list_refundables(&self) -> Result<Vec<RefundableSwap>, SdkError> {
        self.sdk.list_refundables().await
    }

    pub async fn prepare_refund(
        &self,
        req: PrepareRefundRequest,
    ) -> Result<PrepareRefundResponse, SdkError> {
        self.sdk.prepare_refund(&req).await
    }

    pub async fn refund(&self, req: RefundRequest) -> Result<RefundResponse, PaymentError> {
        self.sdk.refund(&req).await
    }

    pub async fn rescan_onchain_swaps(&self) -> Result<(), SdkError> {
        self.sdk.rescan_onchain_swaps().await
    }

    pub async fn sync(&self) -> Result<(), SdkError> {
        self.sdk.sync().await.map_err(Into::into)
    }

    pub async fn recommended_fees(&self) -> Result<RecommendedFees, SdkError> {
        self.sdk.recommended_fees().await.map_err(Into::into)
    }

    #[frb(sync)]
    pub fn empty_wallet_cache(&self) -> Result<(), SdkError> {
        self.sdk.empty_wallet_cache().map_err(Into::into)
    }

    #[frb(sync)]
    pub fn backup(&self, req: BackupRequest) -> Result<(), SdkError> {
        self.sdk.backup(req).map_err(Into::into)
    }

    #[frb(sync)]
    pub fn restore(&self, req: RestoreRequest) -> Result<(), SdkError> {
        self.sdk.restore(req).map_err(Into::into)
    }

    pub async fn disconnect(&self) -> Result<(), SdkError> {
        self.sdk.disconnect().await
    }
}

// === FRB mirroring
//
// This section contains frb "mirroring" structs and enums.
// These are needed by the flutter bridge in order to use structs defined in an external crate.
// See <https://cjycode.com/flutter_rust_bridge/v1/feature/lang_external.html#types-in-other-crates>

#[frb(mirror(Network))]
pub enum _Network {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}

#[frb(mirror(LNInvoice))]
pub struct _LNInvoice {
    pub bolt11: String,
    pub network: Network,
    pub payee_pubkey: String,
    pub payment_hash: String,
    pub description: Option<String>,
    pub description_hash: Option<String>,
    pub amount_msat: Option<u64>,
    pub timestamp: u64,
    pub expiry: u64,
    pub routing_hints: Vec<RouteHint>,
    pub payment_secret: Vec<u8>,
    pub min_final_cltv_expiry_delta: u64,
}

#[frb(mirror(RouteHint))]
pub struct _RouteHint {
    pub hops: Vec<RouteHintHop>,
}

#[frb(mirror(RouteHintHop))]
pub struct _RouteHintHop {
    pub src_node_id: String,
    pub short_channel_id: u64,
    pub fees_base_msat: u32,
    pub fees_proportional_millionths: u32,
    pub cltv_expiry_delta: u64,
    pub htlc_minimum_msat: Option<u64>,
    pub htlc_maximum_msat: Option<u64>,
}

#[frb(mirror(InputType))]
pub enum _InputType {
    BitcoinAddress { address: BitcoinAddressData },
    Bolt11 { invoice: LNInvoice },
    NodeId { node_id: String },
    Url { url: String },
    LnUrlPay { data: LnUrlPayRequestData },
    LnUrlWithdraw { data: LnUrlWithdrawRequestData },
    LnUrlAuth { data: LnUrlAuthRequestData },
    LnUrlError { data: LnUrlErrorData },
}

#[frb(mirror(BitcoinAddressData))]
pub struct _BitcoinAddressData {
    pub address: String,
    pub network: sdk_common::prelude::Network,
    pub amount_sat: Option<u64>,
    pub label: Option<String>,
    pub message: Option<String>,
}

#[frb(mirror(LnUrlPayRequestData))]
pub struct _LnUrlPayRequestData {
    pub callback: String,
    pub min_sendable: u64,
    pub max_sendable: u64,
    pub metadata_str: String,
    pub comment_allowed: u16,
    pub domain: String,
    pub allows_nostr: bool,
    pub nostr_pubkey: Option<String>,
    pub ln_address: Option<String>,
}

#[frb(mirror(LnUrlPayRequest))]
pub struct _LnUrlPayRequest {
    pub data: LnUrlPayRequestData,
    pub amount_msat: u64,
    pub comment: Option<String>,
    pub payment_label: Option<String>,
    pub validate_success_action_url: Option<bool>,
}

#[frb(mirror(SuccessActionProcessed))]
pub enum _SuccessActionProcessed {
    Aes { result: AesSuccessActionDataResult },
    Message { data: MessageSuccessActionData },
    Url { data: UrlSuccessActionData },
}

#[frb(mirror(AesSuccessActionDataResult))]
pub enum _AesSuccessActionDataResult {
    Decrypted { data: AesSuccessActionDataDecrypted },
    ErrorStatus { reason: String },
}

#[frb(mirror(AesSuccessActionDataDecrypted))]
pub struct _AesSuccessActionDataDecrypted {
    pub description: String,
    pub plaintext: String,
}

#[frb(mirror(MessageSuccessActionData))]
pub struct _MessageSuccessActionData {
    pub message: String,
}

#[frb(mirror(UrlSuccessActionData))]
pub struct _UrlSuccessActionData {
    pub description: String,
    pub url: String,
    pub matches_callback_domain: bool,
}

#[frb(mirror(LnUrlPayErrorData))]
pub struct _LnUrlPayErrorData {
    pub payment_hash: String,
    pub reason: String,
}

#[frb(mirror(LnUrlWithdrawRequestData))]
pub struct _LnUrlWithdrawRequestData {
    pub callback: String,
    pub k1: String,
    pub default_description: String,
    pub min_withdrawable: u64,
    pub max_withdrawable: u64,
}

#[frb(mirror(LnUrlAuthRequestData))]
pub struct _LnUrlAuthRequestData {
    pub k1: String,
    pub action: Option<String>,
    pub domain: String,
    pub url: String,
}

#[frb(mirror(LnUrlErrorData))]
pub struct _LnUrlErrorData {
    pub reason: String,
}

#[frb(mirror(LnUrlWithdrawRequest))]
pub struct _LnUrlWithdrawRequest {
    pub data: LnUrlWithdrawRequestData,
    pub amount_msat: u64,
    pub description: Option<String>,
}

#[frb(mirror(Rate))]
pub struct _Rate {
    pub coin: String,
    pub value: f64,
}

#[frb(mirror(FiatCurrency))]
pub struct _FiatCurrency {
    pub id: String,
    pub info: CurrencyInfo,
}

#[frb(mirror(CurrencyInfo))]
pub struct _CurrencyInfo {
    pub name: String,
    pub fraction_size: u32,
    pub spacing: Option<u32>,
    pub symbol: Option<Symbol>,
    pub uniq_symbol: Option<Symbol>,
    pub localized_name: Vec<LocalizedName>,
    pub locale_overrides: Vec<LocaleOverrides>,
}

#[frb(mirror(LocaleOverrides))]
pub struct _LocaleOverrides {
    pub locale: String,
    pub spacing: Option<u32>,
    pub symbol: Symbol,
}

#[frb(mirror(LocalizedName))]
pub struct _LocalizedName {
    pub locale: String,
    pub name: String,
}

#[frb(mirror(Symbol))]
pub struct _Symbol {
    pub grapheme: Option<String>,
    pub template: Option<String>,
    pub rtl: Option<bool>,
    pub position: Option<u32>,
}

/// External structs that cannot be mirrored for FRB, so are therefore duplicated instead
pub mod duplicates {
    use sdk_common::prelude::*;
    use serde::{Deserialize, Serialize};
    use thiserror::Error;

    use crate::error::PaymentError;

    #[derive(Clone, Debug, Error)]
    pub enum LnUrlPayError {
        /// This error is raised when attempting to pay an invoice that has already being paid.
        #[error("Invoice already paid")]
        AlreadyPaid,

        /// This error is raised when a general error occurs not specific to other error variants
        /// in this enum.
        #[error("Generic: {err}")]
        Generic { err: String },

        /// This error is raised when the amount from the parsed invoice is not set.
        #[error("Invalid amount: {err}")]
        InvalidAmount { err: String },

        /// This error is raised when the lightning invoice cannot be parsed.
        #[error("Invalid invoice: {err}")]
        InvalidInvoice { err: String },

        /// This error is raised when the lightning invoice is for a different Bitcoin network.
        #[error("Invalid network: {err}")]
        InvalidNetwork { err: String },

        /// This error is raised when the decoded LNURL URI is not compliant to the specification.
        #[error("Invalid uri: {err}")]
        InvalidUri { err: String },

        /// This error is raised when the lightning invoice has passed it's expiry time.
        #[error("Invoice expired: {err}")]
        InvoiceExpired { err: String },

        /// This error is raised when attempting to make a payment by the node fails.
        #[error("Payment failed: {err}")]
        PaymentFailed { err: String },

        /// This error is raised when attempting to make a payment takes too long.
        #[error("Payment timeout: {err}")]
        PaymentTimeout { err: String },

        /// This error is raised when no route can be found when attempting to make a
        /// payment by the node.
        #[error("Route not found: {err}")]
        RouteNotFound { err: String },

        /// This error is raised when the route is considered too expensive when
        /// attempting to make a payment by the node.
        #[error("Route too expensive: {err}")]
        RouteTooExpensive { err: String },

        /// This error is raised when a connection to an external service fails.
        #[error("Service connectivity: {err}")]
        ServiceConnectivity { err: String },
    }
    impl From<sdk_common::prelude::LnUrlPayError> for LnUrlPayError {
        fn from(value: sdk_common::prelude::LnUrlPayError) -> Self {
            match value {
                sdk_common::prelude::LnUrlPayError::AlreadyPaid => Self::AlreadyPaid,
                sdk_common::prelude::LnUrlPayError::Generic { err } => Self::Generic { err },
                sdk_common::prelude::LnUrlPayError::InvalidAmount { err } => {
                    Self::InvalidAmount { err }
                }
                sdk_common::prelude::LnUrlPayError::InvalidInvoice { err } => {
                    Self::InvalidInvoice { err }
                }
                sdk_common::prelude::LnUrlPayError::InvalidNetwork { err } => {
                    Self::InvalidNetwork { err }
                }
                sdk_common::prelude::LnUrlPayError::InvalidUri { err } => Self::InvalidUri { err },
                sdk_common::prelude::LnUrlPayError::InvoiceExpired { err } => {
                    Self::InvoiceExpired { err }
                }
                sdk_common::prelude::LnUrlPayError::PaymentFailed { err } => {
                    Self::PaymentFailed { err }
                }
                sdk_common::prelude::LnUrlPayError::PaymentTimeout { err } => {
                    Self::PaymentTimeout { err }
                }
                sdk_common::prelude::LnUrlPayError::RouteNotFound { err } => {
                    Self::RouteNotFound { err }
                }
                sdk_common::prelude::LnUrlPayError::RouteTooExpensive { err } => {
                    Self::RouteTooExpensive { err }
                }
                sdk_common::prelude::LnUrlPayError::ServiceConnectivity { err } => {
                    Self::ServiceConnectivity { err }
                }
            }
        }
    }

    impl From<PaymentError> for sdk_common::prelude::LnUrlPayError {
        fn from(value: PaymentError) -> Self {
            Self::Generic {
                err: format!("{value}"),
            }
        }
    }

    #[derive(Debug, Error)]
    pub enum LnUrlWithdrawError {
        /// This error is raised when a general error occurs not specific to other error variants
        /// in this enum.
        #[error("Generic: {err}")]
        Generic { err: String },

        /// This error is raised when the amount is zero or the amount does not cover
        /// the cost to open a new channel.
        #[error("Invalid amount: {err}")]
        InvalidAmount { err: String },

        /// This error is raised when the lightning invoice cannot be parsed.
        #[error("Invalid invoice: {err}")]
        InvalidInvoice { err: String },

        /// This error is raised when the decoded LNURL URI is not compliant to the specification.
        #[error("Invalid uri: {err}")]
        InvalidUri { err: String },

        /// This error is raised when no routing hints were able to be added to the invoice
        /// while trying to receive a payment.
        #[error("No routing hints: {err}")]
        InvoiceNoRoutingHints { err: String },

        /// This error is raised when a connection to an external service fails.
        #[error("Service connectivity: {err}")]
        ServiceConnectivity { err: String },
    }

    impl From<sdk_common::prelude::LnUrlWithdrawError> for LnUrlWithdrawError {
        fn from(value: sdk_common::prelude::LnUrlWithdrawError) -> Self {
            match value {
                sdk_common::prelude::LnUrlWithdrawError::Generic { err } => Self::Generic { err },
                sdk_common::prelude::LnUrlWithdrawError::InvalidAmount { err } => {
                    Self::InvalidAmount { err }
                }
                sdk_common::prelude::LnUrlWithdrawError::InvalidInvoice { err } => {
                    Self::InvalidInvoice { err }
                }
                sdk_common::prelude::LnUrlWithdrawError::InvalidUri { err } => {
                    Self::InvalidUri { err }
                }
                sdk_common::prelude::LnUrlWithdrawError::InvoiceNoRoutingHints { err } => {
                    Self::InvoiceNoRoutingHints { err }
                }
                sdk_common::prelude::LnUrlWithdrawError::ServiceConnectivity { err } => {
                    Self::ServiceConnectivity { err }
                }
            }
        }
    }

    impl From<PaymentError> for sdk_common::prelude::LnUrlWithdrawError {
        fn from(value: PaymentError) -> Self {
            Self::Generic {
                err: format!("{value}"),
            }
        }
    }

    #[derive(Clone, Serialize)]
    pub enum LnUrlWithdrawResult {
        Ok { data: LnUrlWithdrawSuccessData },
        Timeout { data: LnUrlWithdrawSuccessData },
        ErrorStatus { data: LnUrlErrorData },
    }
    impl From<sdk_common::prelude::LnUrlWithdrawResult> for LnUrlWithdrawResult {
        fn from(value: sdk_common::prelude::LnUrlWithdrawResult) -> Self {
            match value {
                sdk_common::prelude::LnUrlWithdrawResult::Ok { data } => {
                    Self::Ok { data: data.into() }
                }
                sdk_common::prelude::LnUrlWithdrawResult::Timeout { data } => {
                    Self::Timeout { data: data.into() }
                }
                sdk_common::prelude::LnUrlWithdrawResult::ErrorStatus { data } => {
                    Self::ErrorStatus { data }
                }
            }
        }
    }

    #[derive(Clone, Deserialize, Debug, Serialize)]
    pub struct LnUrlWithdrawSuccessData {
        pub invoice: LNInvoice,
    }
    impl From<sdk_common::prelude::LnUrlWithdrawSuccessData> for LnUrlWithdrawSuccessData {
        fn from(value: sdk_common::prelude::LnUrlWithdrawSuccessData) -> Self {
            Self {
                invoice: value.invoice,
            }
        }
    }

    #[derive(Debug, Error)]
    pub enum LnUrlAuthError {
        /// This error is raised when a general error occurs not specific to other error variants
        /// in this enum.
        #[error("Generic: {err}")]
        Generic { err: String },

        /// This error is raised when the decoded LNURL URI is not compliant to the specification.
        #[error("Invalid uri: {err}")]
        InvalidUri { err: String },

        /// This error is raised when a connection to an external service fails.
        #[error("Service connectivity: {err}")]
        ServiceConnectivity { err: String },
    }
    impl From<sdk_common::prelude::LnUrlAuthError> for LnUrlAuthError {
        fn from(value: prelude::LnUrlAuthError) -> Self {
            match value {
                sdk_common::prelude::LnUrlAuthError::Generic { err } => Self::Generic { err },
                sdk_common::prelude::LnUrlAuthError::InvalidUri { err } => Self::InvalidUri { err },
                sdk_common::prelude::LnUrlAuthError::ServiceConnectivity { err } => {
                    Self::ServiceConnectivity { err }
                }
            }
        }
    }

    /// Contains the result of the entire LNURL interaction, as reported by the LNURL endpoint.
    ///
    /// * `Ok` indicates the interaction with the endpoint was valid, and the endpoint
    ///  - started to pay the invoice asynchronously in the case of LNURL-withdraw,
    ///  - verified the client signature in the case of LNURL-auth,////// * `Error` indicates a generic issue the LNURL endpoint encountered, including a freetext
    ///    description of the reason.
    ///
    /// Both cases are described in LUD-03 <https://github.com/lnurl/luds/blob/luds/03.md> & LUD-04: <https://github.com/lnurl/luds/blob/luds/04.md>
    #[derive(Clone, Deserialize, Debug, Serialize)]
    #[serde(rename_all = "UPPERCASE")]
    #[serde(tag = "status")]
    pub enum LnUrlCallbackStatus {
        /// On-wire format is: `{"status": "OK"}`
        Ok,
        /// On-wire format is: `{"status": "ERROR", "reason": "error details..."}`
        #[serde(rename = "ERROR")]
        ErrorStatus {
            #[serde(flatten)]
            data: LnUrlErrorData,
        },
    }
    impl From<sdk_common::prelude::LnUrlCallbackStatus> for LnUrlCallbackStatus {
        fn from(value: prelude::LnUrlCallbackStatus) -> Self {
            match value {
                sdk_common::prelude::LnUrlCallbackStatus::Ok => Self::Ok,
                sdk_common::prelude::LnUrlCallbackStatus::ErrorStatus { data } => {
                    Self::ErrorStatus { data }
                }
            }
        }
    }
}
