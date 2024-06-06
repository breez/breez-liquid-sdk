use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::{error::LiquidSdkError, model::Network, utils};

pub const MAINNET_MEMPOOL_SPACE_URL: &str = "https://blockstream.info/liquid";
pub const TESTNET_MEMPOOL_SPACE_URL: &str = "https://liquid.network/liquidtestnet";

#[async_trait]
pub trait ChainService: Send + Sync {
    async fn get_transaction_hex(self: Arc<Self>, tx_id: &str) -> Result<String, LiquidSdkError>;
}

#[derive(Clone)]
pub(crate) struct MempoolSpace {
    pub(crate) network: Network,
}

impl MempoolSpace {
    pub(crate) fn new(network: Network) -> Self {
        MempoolSpace { network }
    }

    pub(crate) fn base_url(&self) -> &'static str {
        match self.network {
            Network::Mainnet => MAINNET_MEMPOOL_SPACE_URL,
            Network::Testnet => TESTNET_MEMPOOL_SPACE_URL,
        }
    }
}

impl Default for MempoolSpace {
    fn default() -> Self {
        MempoolSpace {
            network: Network::Mainnet,
        }
    }
}

#[async_trait]
impl ChainService for MempoolSpace {
    async fn get_transaction_hex(self: Arc<Self>, tx_id: &str) -> Result<String, LiquidSdkError> {
        let url = format!("{}/api/tx/{}/hex", self.base_url(), tx_id);
        let (hex, status_code) = utils::get_and_log_response(&url).await?;
        if !status_code.is_success() {
            return Err(LiquidSdkError::ServiceConnectivity {
                err: format!("Could not retrieve transaction: {status_code}"),
            });
        }
        Ok(hex)
    }
}
