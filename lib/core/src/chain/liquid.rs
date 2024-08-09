use std::{str::FromStr, thread, time::Duration};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use boltz_client::ToHex;
use log::info;
use lwk_wollet::elements::hex::FromHex;
use lwk_wollet::{
    elements::{pset::serialize::Serialize, Address, BlockHash, Script, Transaction, Txid},
    hashes::{sha256, Hash},
    BlockchainBackend, ElectrumClient, ElectrumUrl, History,
};
use reqwest::Response;
use serde::Deserialize;

use crate::{
    model::{Config, LiquidNetwork},
    utils,
};

const LIQUID_ESPLORA_URL: &str = "https://lq1.breez.technology/liquid/api";

#[async_trait]
pub trait LiquidChainService: Send + Sync {
    /// Get the blockchain latest block
    async fn tip(&mut self) -> Result<u32>;

    /// Broadcast a transaction
    async fn broadcast(&self, tx: &Transaction, swap_id: Option<&str>) -> Result<Txid>;

    /// Get a list of transactions
    async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>>;

    /// Get the transactions involved in a list of scripts including lowball
    async fn get_script_history(&self, scripts: &Script) -> Result<Vec<History>>;

    /// Get the transactions involved in a list of scripts including lowball
    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<History>>;

    /// Verify that a transaction appears in the address script history
    async fn verify_tx(
        &self,
        address: &Address,
        tx_id: &str,
        tx_hex: &str,
        verify_confirmation: bool,
    ) -> Result<Transaction>;
}

#[derive(Deserialize)]
struct EsploraTx {
    txid: Txid,
    status: Status,
}

#[derive(Deserialize)]
struct Status {
    block_height: Option<i32>,
    block_hash: Option<BlockHash>,
}

pub(crate) struct HybridLiquidChainService {
    electrum_client: ElectrumClient,
    network: LiquidNetwork,
}

impl HybridLiquidChainService {
    pub(crate) fn new(config: Config) -> Result<Self> {
        let electrum_client =
            ElectrumClient::new(&ElectrumUrl::new(&config.liquid_electrum_url, true, true))?;
        Ok(Self {
            electrum_client,
            network: config.network,
        })
    }
}

#[async_trait]
impl LiquidChainService for HybridLiquidChainService {
    async fn tip(&mut self) -> Result<u32> {
        Ok(self.electrum_client.tip()?.height)
    }

    async fn broadcast(&self, tx: &Transaction, swap_id: Option<&str>) -> Result<Txid> {
        match self.network {
            LiquidNetwork::Mainnet => {
                let tx_bytes = tx.serialize();
                info!("Broadcasting Liquid tx: {}", tx_bytes.to_hex());
                let client = reqwest::Client::new();
                let response = client
                    .post(format!("{LIQUID_ESPLORA_URL}/tx"))
                    .header("Swap-ID", swap_id.unwrap_or_default())
                    .body(tx_bytes.to_hex())
                    .send()
                    .await?;
                let txid = Txid::from_str(&response.text().await?)?;
                Ok(txid)
            }
            LiquidNetwork::Testnet => Ok(self.electrum_client.broadcast(tx)?),
        }
    }

    async fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        Ok(self.electrum_client.get_transactions(txids)?)
    }

    async fn get_script_history(&self, script: &Script) -> Result<Vec<History>> {
        match self.network {
            LiquidNetwork::Mainnet => {
                let script = lwk_wollet::elements::bitcoin::Script::from_bytes(script.as_bytes());
                let script_hash = sha256::Hash::hash(script.as_bytes())
                    .to_byte_array()
                    .to_hex();
                let url = format!("{}/scripthash/{}/txs", LIQUID_ESPLORA_URL, script_hash);
                // TODO must handle paging -> https://github.com/blockstream/esplora/blob/master/API.md#addresses
                let response = get_with_retry(&url, 3).await?;
                let json: Vec<EsploraTx> = response.json().await?;

                let history: Vec<History> = json.into_iter().map(Into::into).collect();
                Ok(history)
            }
            LiquidNetwork::Testnet => {
                let mut history_vec = self.electrum_client.get_scripts_history(&[script])?;
                let h = history_vec.pop();
                Ok(h.unwrap_or(vec![]))
            }
        }
    }

    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<History>> {
        let script_hash = sha256::Hash::hash(script.as_bytes())
            .to_byte_array()
            .to_hex();
        info!("Fetching script history for {}", script_hash);
        let mut script_history = vec![];

        let mut retry = 0;
        while retry <= retries {
            script_history = self.get_script_history(script).await?;
            match script_history.is_empty() {
                true => {
                    retry += 1;
                    info!("Script history for {script_hash} is empty, retrying in 1 second... ({retry} of {retries})");
                    // Waiting 1s between retries, so we detect the new tx as soon as possible
                    thread::sleep(Duration::from_secs(1));
                }
                false => break,
            }
        }
        Ok(script_history)
    }

    async fn verify_tx(
        &self,
        address: &Address,
        tx_id: &str,
        tx_hex: &str,
        verify_confirmation: bool,
    ) -> Result<Transaction> {
        let script = Script::from_hex(
            hex::encode(address.to_unconfidential().script_pubkey().as_bytes()).as_str(),
        )
        .map_err(|e| anyhow!("Failed to get script from address {e:?}"))?;

        let script_history = self.get_script_history_with_retry(&script, 30).await?;
        let lockup_tx_history = script_history.iter().find(|h| h.txid.to_hex().eq(tx_id));

        match lockup_tx_history {
            Some(history) => {
                info!("Liquid transaction found, verifying transaction content...");
                let tx: Transaction = utils::deserialize_tx_hex(tx_hex)?;
                if !tx.txid().to_hex().eq(&history.txid.to_hex()) {
                    return Err(anyhow!(
                        "Liquid transaction id and hex do not match: {} vs {}",
                        tx_id,
                        tx.txid().to_hex()
                    ));
                }

                if verify_confirmation && history.height <= 0 {
                    return Err(anyhow!(
                        "Liquid transaction was not confirmed, txid={} waiting for confirmation",
                        tx_id,
                    ));
                }
                Ok(tx)
            }
            None => Err(anyhow!(
                "Liquid transaction was not found, txid={} waiting for broadcast",
                tx_id,
            )),
        }
    }
}

async fn get_with_retry(url: &str, retries: usize) -> Result<Response> {
    let mut attempt = 0;
    loop {
        info!("liquid chain service get_with_retry for url {url}");
        let response = reqwest::get(url).await?;
        attempt += 1;
        // 429 Too many requests
        // 503 Service Temporarily Unavailable
        if response.status() == 429 || response.status() == 503 {
            if attempt >= retries {
                return Err(anyhow!("Too many retry".to_string()));
            }
            let secs = 1 << attempt;

            thread::sleep(Duration::from_secs(secs));
        } else {
            return Ok(response);
        }
    }
}

impl From<EsploraTx> for History {
    fn from(value: EsploraTx) -> Self {
        let status = value.status;
        History {
            txid: value.txid,
            height: status.block_height.unwrap_or_default(),
            block_hash: status.block_hash,
            block_timestamp: None,
        }
    }
}
