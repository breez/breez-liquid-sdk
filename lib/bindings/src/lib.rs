//! Uniffi bindings

use std::sync::Arc;

use anyhow::Result;
use breez_sdk_liquid::{
    error::*, liquid::LiquidAddressData, logger::Logger, model::*, sdk::LiquidSdk,
    AesSuccessActionDataDecrypted, AesSuccessActionDataResult, BitcoinAddressData, CurrencyInfo,
    FiatCurrency, InputType, LNInvoice, LnUrlAuthError, LnUrlAuthRequestData, LnUrlCallbackStatus,
    LnUrlErrorData, LnUrlPayError, LnUrlPayErrorData, LnUrlPayRequest, LnUrlPayRequestData,
    LnUrlWithdrawError, LnUrlWithdrawRequest, LnUrlWithdrawRequestData, LnUrlWithdrawResult,
    LnUrlWithdrawSuccessData, LocaleOverrides, LocalizedName, MessageSuccessActionData, Network,
    Rate, RouteHint, RouteHintHop, SuccessActionProcessed, Symbol, UrlSuccessActionData,
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
        return m.level() <= Level::Trace && *m.target() != *"breez_sdk_liquid_bindings";
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
pub fn set_logger(logger: Box<dyn Logger>) -> Result<(), SdkError> {
    UniffiBindingLogger::init(logger).map_err(|_| SdkError::Generic {
        err: "Logger already created".into(),
    })?;
    Ok(())
}

pub fn connect(req: ConnectRequest) -> Result<Arc<BindingLiquidSdk>, SdkError> {
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
    pub fn add_event_listener(&self, listener: Box<dyn EventListener>) -> SdkResult<String> {
        rt().block_on(self.sdk.add_event_listener(listener))
    }

    pub fn remove_event_listener(&self, id: String) -> SdkResult<()> {
        rt().block_on(self.sdk.remove_event_listener(id))
    }

    pub fn get_info(&self) -> Result<GetInfoResponse, SdkError> {
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
        req: SendPaymentRequest,
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
        req: ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        rt().block_on(self.sdk.receive_payment(&req))
    }

    pub fn fetch_lightning_limits(&self) -> Result<LightningPaymentLimitsResponse, PaymentError> {
        rt().block_on(self.sdk.fetch_lightning_limits())
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

    pub fn prepare_buy_bitcoin(
        &self,
        req: PrepareBuyBitcoinRequest,
    ) -> Result<PrepareBuyBitcoinResponse, PaymentError> {
        rt().block_on(self.sdk.prepare_buy_bitcoin(&req))
    }

    pub fn buy_bitcoin(&self, req: BuyBitcoinRequest) -> Result<String, PaymentError> {
        rt().block_on(self.sdk.buy_bitcoin(&req))
    }

    pub fn list_payments(&self, req: ListPaymentsRequest) -> Result<Vec<Payment>, PaymentError> {
        rt().block_on(self.sdk.list_payments(&req))
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

    pub fn fetch_fiat_rates(&self) -> Result<Vec<Rate>, SdkError> {
        rt().block_on(self.sdk.fetch_fiat_rates())
    }

    pub fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>, SdkError> {
        rt().block_on(self.sdk.list_fiat_currencies())
    }

    pub fn list_refundables(&self) -> SdkResult<Vec<RefundableSwap>> {
        rt().block_on(self.sdk.list_refundables())
    }

    pub fn prepare_refund(&self, req: PrepareRefundRequest) -> SdkResult<PrepareRefundResponse> {
        rt().block_on(self.sdk.prepare_refund(&req))
    }

    pub fn refund(&self, req: RefundRequest) -> Result<RefundResponse, PaymentError> {
        rt().block_on(self.sdk.refund(&req))
    }

    pub fn rescan_onchain_swaps(&self) -> SdkResult<()> {
        rt().block_on(self.sdk.rescan_onchain_swaps())
    }

    pub fn sync(&self) -> SdkResult<()> {
        rt().block_on(self.sdk.sync()).map_err(Into::into)
    }

    pub fn recommended_fees(&self) -> SdkResult<RecommendedFees> {
        rt().block_on(self.sdk.recommended_fees())
    }

    pub fn empty_wallet_cache(&self) -> SdkResult<()> {
        self.sdk.empty_wallet_cache().map_err(Into::into)
    }

    pub fn backup(&self, req: BackupRequest) -> SdkResult<()> {
        self.sdk.backup(req).map_err(Into::into)
    }

    pub fn restore(&self, req: RestoreRequest) -> SdkResult<()> {
        self.sdk.restore(req).map_err(Into::into)
    }

    pub fn disconnect(&self) -> SdkResult<()> {
        rt().block_on(self.sdk.disconnect())
    }
}

uniffi::include_scaffolding!("breez_sdk_liquid");
