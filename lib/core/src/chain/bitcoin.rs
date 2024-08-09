use std::{thread, time::Duration};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use boltz_client::{Address, ToHex};
use electrum_client::{Client, ElectrumApi, GetBalanceRes, HeaderNotification};
use log::info;
use lwk_wollet::{
    bitcoin::{
        consensus::{deserialize, serialize},
        Script, Transaction, Txid,
    },
    hashes::{sha256, Hash},
    ElectrumOptions, ElectrumUrl, Error, History,
};
use sdk_common::prelude::get_parse_and_log_response;

use crate::model::{Config, RecommendedFees};

/// Trait implemented by types that can fetch data from a blockchain data source.
#[allow(dead_code)]
#[async_trait]
pub trait BitcoinChainService: Send + Sync {
    /// Get the blockchain latest block
    fn tip(&mut self) -> Result<HeaderNotification>;

    /// Broadcast a transaction
    fn broadcast(&self, tx: &Transaction) -> Result<Txid>;

    /// Get a list of transactions
    fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>>;

    /// Get the transactions involved for a script
    fn get_script_history(&self, script: &Script) -> Result<Vec<History>>;

    /// Get the transactions involved for a script
    async fn get_script_history_with_retry(
        &self,
        script: &Script,
        retries: u64,
    ) -> Result<Vec<History>>;

    /// Return the confirmed and unconfirmed balances of a script hash
    fn script_get_balance(&self, script: &Script) -> Result<GetBalanceRes>;

    /// Verify that a transaction appears in the address script history
    async fn verify_tx(
        &self,
        address: &Address,
        tx_id: &str,
        tx_hex: &str,
        verify_confirmation: bool,
    ) -> Result<Transaction>;

    /// Get the recommended fees, in sat/vbyte
    async fn recommended_fees(&self) -> Result<RecommendedFees>;
}

pub(crate) struct HybridBitcoinChainService {
    client: Client,
    tip: HeaderNotification,
    config: Config,
}
impl HybridBitcoinChainService {
    pub fn new(config: Config) -> Result<Self, Error> {
        Self::with_options(config, ElectrumOptions::default())
    }

    /// Creates an Electrum client specifying non default options like timeout
    pub fn with_options(config: Config, options: ElectrumOptions) -> Result<Self, Error> {
        let url = ElectrumUrl::new(&config.bitcoin_electrum_url, true, true);
        let client = url.build_client(&options)?;
        let header = client.block_headers_subscribe_raw()?;
        let tip: HeaderNotification = header.try_into()?;

        Ok(Self {
            client,
            tip,
            config,
        })
    }
}

#[async_trait]
impl BitcoinChainService for HybridBitcoinChainService {
    fn tip(&mut self) -> Result<HeaderNotification> {
        let mut maybe_popped_header = None;
        while let Some(header) = self.client.block_headers_pop_raw()? {
            maybe_popped_header = Some(header)
        }

        match maybe_popped_header {
            Some(popped_header) => {
                let tip: HeaderNotification = popped_header.try_into()?;
                self.tip = tip;
            }
            None => {
                // https://github.com/bitcoindevkit/rust-electrum-client/issues/124
                // It might be that the client has reconnected and subscriptions don't persist
                // across connections. Calling `client.ping()` won't help here because the
                // successful retry will prevent us knowing about the reconnect.
                if let Ok(header) = self.client.block_headers_subscribe_raw() {
                    let tip: HeaderNotification = header.try_into()?;
                    self.tip = tip;
                }
            }
        }

        Ok(self.tip.clone())
    }

    fn broadcast(&self, tx: &Transaction) -> Result<Txid> {
        let txid = self.client.transaction_broadcast_raw(&serialize(tx))?;
        Ok(Txid::from_raw_hash(txid.to_raw_hash()))
    }

    fn get_transactions(&self, txids: &[Txid]) -> Result<Vec<Transaction>> {
        let mut result = vec![];
        for tx in self.client.batch_transaction_get_raw(txids)? {
            let tx: Transaction = deserialize(&tx)?;
            result.push(tx);
        }
        Ok(result)
    }

    fn get_script_history(&self, script: &Script) -> Result<Vec<History>> {
        Ok(self
            .client
            .script_get_history(script)?
            .into_iter()
            .map(Into::into)
            .collect())
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
            script_history = self.get_script_history(script)?;
            match script_history.is_empty() {
                true => {
                    retry += 1;
                    info!(
                        "Script history for {} got zero transactions, retrying in {} seconds...",
                        script_hash, retry
                    );
                    thread::sleep(Duration::from_secs(retry));
                }
                false => break,
            }
        }
        Ok(script_history)
    }

    fn script_get_balance(&self, script: &Script) -> Result<GetBalanceRes> {
        Ok(self.client.script_get_balance(script)?)
    }

    async fn verify_tx(
        &self,
        address: &Address,
        tx_id: &str,
        tx_hex: &str,
        verify_confirmation: bool,
    ) -> Result<Transaction> {
        let script_pubkey = address.script_pubkey();
        let script = script_pubkey.as_script();

        let script_history = self.get_script_history_with_retry(script, 5).await?;
        let lockup_tx_history = script_history.iter().find(|h| h.txid.to_hex().eq(tx_id));

        match lockup_tx_history {
            Some(history) => {
                info!("Bitcoin transaction found, verifying transaction content...");
                let tx: Transaction = deserialize(&hex::decode(tx_hex)?)?;
                if !tx.txid().to_hex().eq(&history.txid.to_hex()) {
                    return Err(anyhow!(
                        "Bitcoin transaction id and hex do not match: {} vs {}",
                        tx_id,
                        tx.txid().to_hex()
                    ));
                }

                if verify_confirmation && history.height <= 0 {
                    return Err(anyhow!(
                        "Bitcoin transaction was not confirmed, txid={} waiting for confirmation",
                        tx_id,
                    ));
                }
                Ok(tx)
            }
            None => Err(anyhow!(
                "Bitcoin transaction was not found, txid={} waiting for broadcast",
                tx_id,
            )),
        }
    }

    async fn recommended_fees(&self) -> Result<RecommendedFees> {
        get_parse_and_log_response(
            &format!("{}/v1/fees/recommended", self.config.mempoolspace_url),
            true,
        )
        .await
        .map_err(Into::into)
    }
}
