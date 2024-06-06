use anyhow::Error;

pub type LiquidSdkResult<T, E = LiquidSdkError> = Result<T, E>;

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
pub enum LiquidSdkError {
    #[error("Liquid SDK instance is already running")]
    AlreadyStarted,

    #[error("Error: {err}")]
    Generic { err: String },

    #[error("Service connectivity: {err}")]
    ServiceConnectivity { err: String },

    #[error("Liquid SDK instance is not running")]
    NotStarted,
}

impl From<anyhow::Error> for LiquidSdkError {
    fn from(e: Error) -> Self {
        LiquidSdkError::Generic { err: e.to_string() }
    }
}

impl From<serde_json::Error> for LiquidSdkError {
    fn from(err: serde_json::Error) -> Self {
        Self::Generic {
            err: err.to_string(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PaymentError {
    #[error("The specified funds have already been claimed")]
    AlreadyClaimed,

    #[error("Invoice amount is out of range")]
    AmountOutOfRange,

    #[error("Generic error: {err}")]
    Generic { err: String },

    #[error("The provided fees have expired")]
    InvalidOrExpiredFees,

    #[error("Cannot pay: not enough funds")]
    InsufficientFunds,

    #[error("The specified invoice is not valid")]
    InvalidInvoice,

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

    #[error("The payment has been refunded. Reason for failure: {err}")]
    Refunded { err: String, refund_tx_id: String },

    #[error("Could not sign/send the transaction: {err}")]
    SendError { err: String },

    #[error("Could not sign the transaction: {err}")]
    SignerError { err: String },
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

#[allow(clippy::match_single_binding)]
impl From<lwk_wollet::Error> for PaymentError {
    fn from(err: lwk_wollet::Error) -> Self {
        match err {
            _ => PaymentError::LwkError {
                err: format!("{err:?}"),
            },
        }
    }
}

#[allow(clippy::match_single_binding)]
impl From<lwk_signer::SignerError> for PaymentError {
    fn from(err: lwk_signer::SignerError) -> Self {
        match err {
            _ => PaymentError::SignerError {
                err: format!("{err:?}"),
            },
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

impl From<LiquidSdkError> for PaymentError {
    fn from(err: LiquidSdkError) -> Self {
        Self::Generic {
            err: err.to_string(),
        }
    }
}
