#![cfg(test)]

use anyhow::Result;
use async_trait::async_trait;
use lwk_wollet::elements::{BlockHash, Txid};
use lwk_wollet::{bitcoin::consensus::deserialize, elements::hex::FromHex};

use crate::{
    chain::{bitcoin::BitcoinChainService, liquid::LiquidChainService},
    prelude::RecommendedFees,
    utils,
};

#[derive(Clone)]
pub(crate) struct MockHistory {
    pub txid: Txid,
    pub height: i32,
    pub block_hash: Option<BlockHash>,
    pub block_timestamp: Option<u32>,
}

impl From<MockHistory> for lwk_wollet::History {
    fn from(h: MockHistory) -> Self {
        lwk_wollet::History {
            txid: h.txid,
            height: h.height,
            block_hash: h.block_hash,
            block_timestamp: h.block_timestamp,
        }
    }
}

#[derive(Default)]
pub(crate) struct MockLiquidChainService {
    history: Vec<MockHistory>,
}

impl MockLiquidChainService {
    pub(crate) fn new() -> Self {
        MockLiquidChainService::default()
    }

    pub(crate) fn set_history(&mut self, history: Vec<MockHistory>) -> &mut Self {
        self.history = history;
        self
    }
}

#[async_trait]
impl LiquidChainService for MockLiquidChainService {
    async fn tip(&mut self) -> Result<u32> {
        unimplemented!()
    }

    async fn broadcast(
        &self,
        tx: &lwk_wollet::elements::Transaction,
        _swap_id: Option<&str>,
    ) -> Result<lwk_wollet::elements::Txid> {
        Ok(tx.txid())
    }

    async fn get_transactions(
        &self,
        _txids: &[lwk_wollet::elements::Txid],
    ) -> Result<Vec<lwk_wollet::elements::Transaction>> {
        unimplemented!()
    }

    async fn get_script_history(
        &self,
        _scripts: &lwk_wollet::elements::Script,
    ) -> Result<Vec<lwk_wollet::History>> {
        Ok(self.history.clone().into_iter().map(Into::into).collect())
    }

    async fn get_script_history_with_retry(
        &self,
        _script: &lwk_wollet::elements::Script,
        _retries: u64,
    ) -> Result<Vec<lwk_wollet::History>> {
        Ok(self.history.clone().into_iter().map(Into::into).collect())
    }

    async fn verify_tx(
        &self,
        _address: &boltz_client::ElementsAddress,
        _tx_id: &str,
        tx_hex: &str,
        _verify_confirmation: bool,
    ) -> Result<lwk_wollet::elements::Transaction> {
        utils::deserialize_tx_hex(tx_hex)
    }
}

pub(crate) struct MockBitcoinChainService {
    history: Vec<MockHistory>,
}

impl MockBitcoinChainService {
    pub(crate) fn new() -> Self {
        MockBitcoinChainService { history: vec![] }
    }

    pub(crate) fn set_history(&mut self, history: Vec<MockHistory>) -> &mut Self {
        self.history = history;
        self
    }
}

#[async_trait]
impl BitcoinChainService for MockBitcoinChainService {
    fn tip(&mut self) -> Result<electrum_client::HeaderNotification> {
        unimplemented!()
    }

    fn broadcast(
        &self,
        _tx: &boltz_client::bitcoin::Transaction,
    ) -> Result<boltz_client::bitcoin::Txid, anyhow::Error> {
        unimplemented!()
    }

    fn get_transactions(
        &self,
        _txids: &[boltz_client::bitcoin::Txid],
    ) -> Result<Vec<boltz_client::bitcoin::Transaction>> {
        unimplemented!()
    }

    fn get_script_history(
        &self,
        _script: &boltz_client::bitcoin::Script,
    ) -> Result<Vec<lwk_wollet::History>> {
        Ok(self.history.clone().into_iter().map(Into::into).collect())
    }

    async fn get_script_history_with_retry(
        &self,
        _script: &boltz_client::bitcoin::Script,
        _retries: u64,
    ) -> Result<Vec<lwk_wollet::History>> {
        Ok(self.history.clone().into_iter().map(Into::into).collect())
    }

    fn script_get_balance(
        &self,
        _script: &boltz_client::bitcoin::Script,
    ) -> Result<electrum_client::GetBalanceRes> {
        unimplemented!()
    }

    async fn verify_tx(
        &self,
        _address: &boltz_client::Address,
        _tx_id: &str,
        tx_hex: &str,
        _verify_confirmation: bool,
    ) -> Result<boltz_client::bitcoin::Transaction> {
        Ok(deserialize(&Vec::<u8>::from_hex(tx_hex).map_err(
            |err| anyhow::anyhow!("Could not deserialize transaction: {err:?}"),
        )?)?)
    }

    async fn recommended_fees(&self) -> Result<RecommendedFees> {
        unimplemented!()
    }
}
