#![cfg(test)]

use anyhow::Result;
use async_trait::async_trait;
use boltz_client::elements::Script;
use lwk_wollet::{bitcoin::consensus::deserialize, elements::hex::FromHex, History};

use crate::{
    chain::{bitcoin::BitcoinChainService, liquid::LiquidChainService},
    prelude::RecommendedFees,
    utils,
};

#[derive(Default)]
pub(crate) struct MockLiquidChainService {}

impl MockLiquidChainService {
    pub(crate) fn new() -> Self {
        MockLiquidChainService::default()
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
        unimplemented!()
    }

    async fn get_scripts_history(&self, _scripts: &[&Script]) -> Result<Vec<Vec<History>>> {
        unimplemented!()
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

pub(crate) struct MockBitcoinChainService {}

impl MockBitcoinChainService {
    pub(crate) fn new() -> Self {
        MockBitcoinChainService {}
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
        unimplemented!()
    }

    fn get_scripts_history(
        &self,
        _scripts: &[&boltz_client::bitcoin::Script],
    ) -> Result<Vec<Vec<History>>> {
        unimplemented!()
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
