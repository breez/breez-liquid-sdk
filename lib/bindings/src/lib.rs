//! Uniffi bindings

use std::sync::Arc;

use anyhow::Result;
use breez_liquid_sdk::logger::Logger;
use breez_liquid_sdk::{
    error::*, model::*, sdk::LiquidSdk, AesSuccessActionDataDecrypted, AesSuccessActionDataResult,
    BitcoinAddressData, CurrencyInfo, FiatCurrency, InputType, LNInvoice, LnUrlAuthError,
    LnUrlAuthRequestData, LnUrlCallbackStatus, LnUrlErrorData, LnUrlPayError, LnUrlPayErrorData,
    LnUrlPayRequest, LnUrlPayRequestData, LnUrlWithdrawError, LnUrlWithdrawRequest,
    LnUrlWithdrawRequestData, LnUrlWithdrawResult, LnUrlWithdrawSuccessData, LocaleOverrides,
    LocalizedName, MessageSuccessActionData, Network, Rate, RouteHint, RouteHintHop,
    SuccessActionProcessed, Symbol, UrlSuccessActionData,
};
use log::{Metadata, Record, SetLoggerError};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use uniffi::deps::log::{Level, LevelFilter};

static RT: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

fn rt() -> &'static Runtime {
    &RT
}

struct UniffiBindingLogger {
    logger: Box<dyn Logger>,
}

impl UniffiBindingLogger {
    fn init(logger: Box<dyn Logger>) -> Result<(), SetLoggerError> {
        let binding_logger: UniffiBindingLogger = UniffiBindingLogger { logger };
        log::set_boxed_logger(Box::new(binding_logger))
            .map(|_| log::set_max_level(LevelFilter::Trace))
    }
}

impl log::Log for UniffiBindingLogger {
    fn enabled(&self, m: &Metadata) -> bool {
        // ignore the internal uniffi log to prevent infinite loop.
        return m.level() <= Level::Trace && *m.target() != *"breez_liquid_sdk_bindings";
    }

    fn log(&self, record: &Record) {
        self.logger.log(LogEntry {
            line: record.args().to_string(),
            level: record.level().as_str().to_string(),
        });
    }
    fn flush(&self) {}
}

/// If used, this must be called before `connect`
pub fn set_logger(logger: Box<dyn Logger>) -> Result<(), LiquidSdkError> {
    UniffiBindingLogger::init(logger).map_err(|_| LiquidSdkError::Generic {
        err: "Logger already created".into(),
    })?;
    Ok(())
}

pub fn connect(req: ConnectRequest) -> Result<Arc<BindingLiquidSdk>, LiquidSdkError> {
    rt().block_on(async {
        let sdk = LiquidSdk::connect(req).await?;
        Ok(Arc::from(BindingLiquidSdk { sdk }))
    })
}

pub fn default_config(network: LiquidNetwork) -> Config {
    LiquidSdk::default_config(network)
}

pub fn parse(input: String) -> Result<InputType, PaymentError> {
    rt().block_on(async { LiquidSdk::parse(&input).await })
}
pub fn parse_invoice(input: String) -> Result<LNInvoice, PaymentError> {
    LiquidSdk::parse_invoice(&input)
}

pub struct BindingLiquidSdk {
    sdk: Arc<LiquidSdk>,
}

impl BindingLiquidSdk {
    pub fn add_event_listener(&self, listener: Box<dyn EventListener>) -> LiquidSdkResult<String> {
        rt().block_on(self.sdk.add_event_listener(listener))
    }

    pub fn remove_event_listener(&self, id: String) -> LiquidSdkResult<()> {
        rt().block_on(self.sdk.remove_event_listener(id))
    }

    pub fn get_info(&self) -> Result<GetInfoResponse, LiquidSdkError> {
        rt().block_on(self.sdk.get_info()).map_err(Into::into)
    }

    pub fn prepare_send_payment(
        &self,
        req: PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        rt().block_on(self.sdk.prepare_send_payment(&req))
    }

    pub fn send_payment(
        &self,
        req: PrepareSendResponse,
    ) -> Result<SendPaymentResponse, PaymentError> {
        rt().block_on(self.sdk.send_payment(&req))
    }

    pub fn prepare_receive_payment(
        &self,
        req: PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        rt().block_on(self.sdk.prepare_receive_payment(&req))
    }

    pub fn receive_payment(
        &self,
        req: PrepareReceiveResponse,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        rt().block_on(self.sdk.receive_payment(&req))
    }

    pub fn fetch_onchain_limits(&self) -> Result<OnchainPaymentLimitsResponse, PaymentError> {
        rt().block_on(self.sdk.fetch_onchain_limits())
    }

    pub fn prepare_pay_onchain(
        &self,
        req: PreparePayOnchainRequest,
    ) -> Result<PreparePayOnchainResponse, PaymentError> {
        rt().block_on(self.sdk.prepare_pay_onchain(&req))
    }

    pub fn pay_onchain(&self, req: PayOnchainRequest) -> Result<SendPaymentResponse, PaymentError> {
        rt().block_on(self.sdk.pay_onchain(&req))
    }

    pub fn prepare_receive_onchain(
        &self,
        req: PrepareReceiveOnchainRequest,
    ) -> Result<PrepareReceiveOnchainResponse, PaymentError> {
        rt().block_on(self.sdk.prepare_receive_onchain(&req))
    }

    pub fn receive_onchain(
        &self,
        req: PrepareReceiveOnchainResponse,
    ) -> Result<ReceiveOnchainResponse, PaymentError> {
        rt().block_on(self.sdk.receive_onchain(&req))
    }

    pub fn list_payments(&self) -> Result<Vec<Payment>, PaymentError> {
        rt().block_on(self.sdk.list_payments())
    }

    pub fn lnurl_pay(&self, req: LnUrlPayRequest) -> Result<LnUrlPayResult, LnUrlPayError> {
        rt().block_on(self.sdk.lnurl_pay(req)).map_err(Into::into)
    }

    pub fn lnurl_withdraw(
        &self,
        req: LnUrlWithdrawRequest,
    ) -> Result<LnUrlWithdrawResult, LnUrlWithdrawError> {
        rt().block_on(self.sdk.lnurl_withdraw(req))
            .map_err(Into::into)
    }

    pub fn lnurl_auth(
        &self,
        req_data: LnUrlAuthRequestData,
    ) -> Result<LnUrlCallbackStatus, LnUrlAuthError> {
        rt().block_on(self.sdk.lnurl_auth(req_data))
    }

    pub fn fetch_fiat_rates(&self) -> Result<Vec<Rate>, LiquidSdkError> {
        rt().block_on(self.sdk.fetch_fiat_rates())
    }

    pub fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>, LiquidSdkError> {
        rt().block_on(self.sdk.list_fiat_currencies())
    }

    pub fn list_refundables(&self) -> LiquidSdkResult<Vec<RefundableSwap>> {
        rt().block_on(self.sdk.list_refundables())
    }

    pub fn prepare_refund(
        &self,
        req: PrepareRefundRequest,
    ) -> LiquidSdkResult<PrepareRefundResponse> {
        rt().block_on(self.sdk.prepare_refund(&req))
    }

    pub fn refund(&self, req: RefundRequest) -> Result<RefundResponse, PaymentError> {
        rt().block_on(self.sdk.refund(&req))
    }

    pub fn rescan_onchain_swaps(&self) -> LiquidSdkResult<()> {
        rt().block_on(self.sdk.rescan_onchain_swaps())
    }

    pub fn sync(&self) -> LiquidSdkResult<()> {
        rt().block_on(self.sdk.sync()).map_err(Into::into)
    }

    pub fn empty_wallet_cache(&self) -> LiquidSdkResult<()> {
        self.sdk.empty_wallet_cache().map_err(Into::into)
    }

    pub fn backup(&self, req: BackupRequest) -> LiquidSdkResult<()> {
        self.sdk.backup(req).map_err(Into::into)
    }

    pub fn restore(&self, req: RestoreRequest) -> LiquidSdkResult<()> {
        self.sdk.restore(req).map_err(Into::into)
    }

    pub fn disconnect(&self) -> LiquidSdkResult<()> {
        rt().block_on(self.sdk.disconnect())
    }
}

uniffi::include_scaffolding!("breez_liquid_sdk");
