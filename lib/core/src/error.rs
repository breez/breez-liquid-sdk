use anyhow::Error;
use lwk_wollet::secp256k1;
use sdk_common::prelude::LnUrlAuthError;

pub type SdkResult<T, E = SdkError> = Result<T, E>;

#[macro_export]
macro_rules! ensure_sdk {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

// TODO Unify error enum
#[derive(Debug, thiserror::Error)]
pub enum SdkError {
    #[error("Liquid SDK instance is already running")]
    AlreadyStarted,

    #[error("Error: {err}")]
    Generic { err: String },

    #[error("Liquid SDK instance is not running")]
    NotStarted,

    #[error("Service connectivity: {err}")]
    ServiceConnectivity { err: String },
}

impl From<anyhow::Error> for SdkError {
    fn from(e: Error) -> Self {
        SdkError::Generic { err: e.to_string() }
    }
}

impl From<boltz_client::error::Error> for SdkError {
    fn from(err: boltz_client::error::Error) -> Self {
        match err {
            boltz_client::error::Error::HTTP(e) => SdkError::Generic {
                err: format!("Could not contact servers: {e:?}"),
            },
            _ => SdkError::Generic {
                err: format!("{err:?}"),
            },
        }
    }
}

impl From<secp256k1::Error> for SdkError {
    fn from(err: secp256k1::Error) -> Self {
        SdkError::Generic {
            err: format!("{err:?}"),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PaymentError {
    #[error("The specified funds have already been claimed")]
    AlreadyClaimed,

    #[error("The specified funds have already been sent")]
    AlreadyPaid,

    #[error("The payment is already in progress")]
    PaymentInProgress,

    #[error("Amount is out of range")]
    AmountOutOfRange,

    #[error("Amount is missing: {err}")]
    AmountMissing { err: String },

    #[error("Network mismatch: {err}")]
    NetworkMismatch { err: String },

    #[error("Generic error: {err}")]
    Generic { err: String },

    #[error("The provided fees have expired")]
    InvalidOrExpiredFees,

    #[error("Cannot pay: not enough funds")]
    InsufficientFunds,

    #[error("The specified invoice is not valid: {err}")]
    InvalidInvoice { err: String },

    #[error("The generated preimage is not valid")]
    InvalidPreimage,

    #[error("Lwk error: {err}")]
    LwkError { err: String },

    #[error("Boltz did not return any pairs from the request")]
    PairsNotFound,

    #[error("The payment timed out")]
    PaymentTimeout,

    #[error("Could not store the swap details locally")]
    PersistError,

    #[error("Could not process the Receive Payment: {err}")]
    ReceiveError { err: String },

    #[error("The payment has been refunded. Reason for failure: {err}")]
    Refunded { err: String, refund_tx_id: String },

    #[error("The payment is a self-transfer, which is not supported")]
    SelfTransferNotSupported,

    #[error("Could not process the Send Payment: {err}")]
    SendError { err: String },

    #[error("Could not sign the transaction: {err}")]
    SignerError { err: String },
}
impl PaymentError {
    pub(crate) fn receive_error(err: &str) -> Self {
        Self::ReceiveError {
            err: err.to_string(),
        }
    }
}

impl From<boltz_client::error::Error> for PaymentError {
    fn from(err: boltz_client::error::Error) -> Self {
        match err {
            boltz_client::error::Error::HTTP(e) => PaymentError::Generic {
                err: format!("Could not contact servers: {e:?}"),
            },
            _ => PaymentError::Generic {
                err: format!("{err:?}"),
            },
        }
    }
}

impl From<boltz_client::bitcoin::hex::HexToArrayError> for PaymentError {
    fn from(err: boltz_client::bitcoin::hex::HexToArrayError) -> Self {
        PaymentError::Generic {
            err: format!("{err:?}"),
        }
    }
}

impl From<lwk_wollet::Error> for PaymentError {
    fn from(err: lwk_wollet::Error) -> Self {
        match err {
            lwk_wollet::Error::InsufficientFunds => PaymentError::InsufficientFunds,
            _ => PaymentError::LwkError {
                err: format!("{err:?}"),
            },
        }
    }
}

impl From<lwk_signer::SignerError> for PaymentError {
    fn from(err: lwk_signer::SignerError) -> Self {
        PaymentError::SignerError {
            err: format!("{err:?}"),
        }
    }
}

impl From<anyhow::Error> for PaymentError {
    fn from(err: anyhow::Error) -> Self {
        Self::Generic {
            err: err.to_string(),
        }
    }
}

impl From<SdkError> for PaymentError {
    fn from(err: SdkError) -> Self {
        Self::Generic {
            err: err.to_string(),
        }
    }
}

impl From<crate::bitcoin::util::bip32::Error> for PaymentError {
    fn from(err: crate::bitcoin::util::bip32::Error) -> Self {
        Self::SignerError {
            err: err.to_string(),
        }
    }
}

impl From<PaymentError> for LnUrlAuthError {
    fn from(value: PaymentError) -> Self {
        Self::Generic {
            err: format!("Failed to perform LNURL-auth: {value:?}"),
        }
    }
}
