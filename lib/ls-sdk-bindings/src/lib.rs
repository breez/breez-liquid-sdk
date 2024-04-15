use std::sync::Arc;

use anyhow::{Error, Result};
use ls_sdk::{
    model::PaymentError, Network, PreparePaymentResponse, ReceivePaymentRequest,
    ReceivePaymentResponse, SendPaymentResponse, Wallet, WalletInfo,
};

// TODO Unify error enum
#[derive(Debug, thiserror::Error)]
pub enum LsSdkError {
    #[error("Error: {err}")]
    Generic { err: String },
}

impl From<anyhow::Error> for LsSdkError {
    fn from(e: Error) -> Self {
        LsSdkError::Generic { err: e.to_string() }
    }
}

pub fn init(
    mnemonic: String,
    data_dir: Option<String>,
    network: Network,
) -> Result<Arc<BindingWallet>, LsSdkError> {
    let ln_sdk = Wallet::init(&mnemonic, data_dir, network)?;
    Ok(Arc::from(BindingWallet { ln_sdk }))
}

pub struct BindingWallet {
    ln_sdk: Arc<Wallet>,
}

impl BindingWallet {
    pub fn get_info(&self, with_scan: bool) -> Result<WalletInfo, LsSdkError> {
        self.ln_sdk.get_info(with_scan).map_err(Into::into)
    }

    pub fn send_payment(
        &self,
        req: PreparePaymentResponse,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.ln_sdk.send_payment(&req)
    }

    pub fn receive_payment(
        &self,
        req: ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.ln_sdk.receive_payment(req)
    }
}

uniffi::include_scaffolding!("ls_sdk");
