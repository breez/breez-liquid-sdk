//! Dart / flutter bindings

use std::sync::Arc;

use anyhow::Result;
use flutter_rust_bridge::frb;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

use crate::model::lnurl::WrappedLnUrlPayResult;
use crate::{error::*, frb_generated::StreamSink, model::*, sdk::LiquidSdk, *};
use sdk_common::prelude::LnUrlPayRequest;

pub struct BindingEventListener {
    pub stream: StreamSink<LiquidSdkEvent>,
}

impl EventListener for BindingEventListener {
    fn on_event(&self, e: LiquidSdkEvent) {
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

pub async fn connect(req: ConnectRequest) -> Result<BindingLiquidSdk, LiquidSdkError> {
    let ln_sdk = LiquidSdk::connect(req).await?;
    Ok(BindingLiquidSdk { sdk: ln_sdk })
}

/// If used, this must be called before `connect`. It can only be called once.
pub fn breez_log_stream(s: StreamSink<LogEntry>) -> Result<()> {
    DartBindingLogger::init(s).map_err(|_| LiquidSdkError::Generic {
        err: "Log stream already created".into(),
    })?;
    Ok(())
}

#[frb(sync)]
pub fn default_config(network: LiquidSdkNetwork) -> Config {
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
    pub async fn get_info(&self) -> Result<GetInfoResponse, LiquidSdkError> {
        self.sdk.get_info().await.map_err(Into::into)
    }

    pub async fn add_event_listener(
        &self,
        listener: StreamSink<LiquidSdkEvent>,
    ) -> Result<String, LiquidSdkError> {
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
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        self.sdk.prepare_receive_payment(&req).await
    }

    pub async fn receive_payment(
        &self,
        req: PrepareReceiveResponse,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.sdk.receive_payment(&req).await
    }

    pub async fn list_payments(&self) -> Result<Vec<Payment>, PaymentError> {
        self.sdk.list_payments().await
    }

    pub async fn lnurl_pay(
        &self,
        req: LnUrlPayRequest,
    ) -> Result<WrappedLnUrlPayResult, duplicates::LnUrlPayError> {
        self.sdk.lnurl_pay(req).await.map_err(Into::into)
    }

    pub async fn sync(&self) -> Result<(), LiquidSdkError> {
        self.sdk.sync().await.map_err(Into::into)
    }

    #[frb(sync)]
    pub fn empty_wallet_cache(&self) -> Result<(), LiquidSdkError> {
        self.sdk.empty_wallet_cache().map_err(Into::into)
    }

    #[frb(sync)]
    pub fn backup(&self, req: BackupRequest) -> Result<(), LiquidSdkError> {
        self.sdk.backup(req).map_err(Into::into)
    }

    #[frb(sync)]
    pub fn restore(&self, req: RestoreRequest) -> Result<(), LiquidSdkError> {
        self.sdk.restore(req).map_err(Into::into)
    }

    pub async fn disconnect(&self) -> Result<(), LiquidSdkError> {
        self.sdk.disconnect().await
    }
}

/// External structs that cannot be mirrored for FRB, so are therefore duplicated instead
pub mod duplicates {
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
                sdk_common::prelude::LnUrlPayError::InvalidAmount { err } => Self::InvalidAmount { err },
                sdk_common::prelude::LnUrlPayError::InvalidInvoice { err } => Self::InvalidInvoice { err },
                sdk_common::prelude::LnUrlPayError::InvalidNetwork { err } => Self::InvalidNetwork { err },
                sdk_common::prelude::LnUrlPayError::InvalidUri { err } => Self::InvalidUri { err },
                sdk_common::prelude::LnUrlPayError::InvoiceExpired { err } => Self::InvoiceExpired { err },
                sdk_common::prelude::LnUrlPayError::PaymentFailed { err } => Self::PaymentFailed { err },
                sdk_common::prelude::LnUrlPayError::PaymentTimeout { err } => Self::PaymentTimeout { err },
                sdk_common::prelude::LnUrlPayError::RouteNotFound { err } => Self::RouteNotFound { err },
                sdk_common::prelude::LnUrlPayError::RouteTooExpensive { err } => Self::RouteTooExpensive { err },
                sdk_common::prelude::LnUrlPayError::ServiceConnectivity { err } => Self::ServiceConnectivity { err },
            }
        }
    }

    impl From<PaymentError> for sdk_common::prelude::LnUrlPayError {
        fn from(value: PaymentError) -> Self {
            Self::Generic {
                err: format!("{value}")
            }
        }
    }
}