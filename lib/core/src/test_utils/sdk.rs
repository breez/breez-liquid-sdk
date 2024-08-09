#![cfg(test)]

use anyhow::{anyhow, Result};
use sdk_common::prelude::{BreezServer, STAGING_BREEZSERVER_URL};
use std::sync::Arc;

use tokio::sync::{watch, Mutex, RwLock};

use crate::{
    buy::BuyBitcoinService, chain_swap::ChainSwapStateHandler, event::EventManager, model::Config,
    persist::Persister, receive_swap::ReceiveSwapStateHandler, sdk::LiquidSdk,
    send_swap::SendSwapStateHandler,
};

use super::{
    chain::{MockBitcoinChainService, MockLiquidChainService},
    status_stream::MockStatusStream,
    swapper::MockSwapper,
    wallet::MockWallet,
};

pub(crate) fn new_liquid_sdk(
    persister: Arc<Persister>,
    swapper: Arc<MockSwapper>,
    status_stream: Arc<MockStatusStream>,
) -> Result<LiquidSdk> {
    let liquid_chain_service = Arc::new(Mutex::new(MockLiquidChainService::new()));
    let bitcoin_chain_service = Arc::new(Mutex::new(MockBitcoinChainService::new()));

    new_liquid_sdk_with_chain_services(
        persister,
        swapper,
        status_stream,
        liquid_chain_service,
        bitcoin_chain_service,
    )
}

pub(crate) fn new_liquid_sdk_with_chain_services(
    persister: Arc<Persister>,
    swapper: Arc<MockSwapper>,
    status_stream: Arc<MockStatusStream>,
    liquid_chain_service: Arc<Mutex<MockLiquidChainService>>,
    bitcoin_chain_service: Arc<Mutex<MockBitcoinChainService>>,
) -> Result<LiquidSdk> {
    let mut config = Config::testnet();
    config.working_dir = persister
        .get_database_dir()
        .to_str()
        .ok_or(anyhow!("An invalid SDK directory was specified"))?
        .to_string();

    let onchain_wallet = Arc::new(MockWallet::new());

    let send_swap_state_handler = SendSwapStateHandler::new(
        config.clone(),
        onchain_wallet.clone(),
        persister.clone(),
        swapper.clone(),
        liquid_chain_service.clone(),
    );

    let receive_swap_state_handler = ReceiveSwapStateHandler::new(
        config.clone(),
        onchain_wallet.clone(),
        persister.clone(),
        swapper.clone(),
        liquid_chain_service.clone(),
    );

    let chain_swap_state_handler = Arc::new(ChainSwapStateHandler::new(
        config.clone(),
        onchain_wallet.clone(),
        persister.clone(),
        swapper.clone(),
        liquid_chain_service.clone(),
        bitcoin_chain_service.clone(),
    )?);

    let event_manager = Arc::new(EventManager::new());
    let (shutdown_sender, shutdown_receiver) = watch::channel::<()>(());

    let breez_server = Arc::new(BreezServer::new(STAGING_BREEZSERVER_URL.into(), None)?);

    let buy_bitcoin_service =
        Arc::new(BuyBitcoinService::new(config.clone(), breez_server.clone()));

    Ok(LiquidSdk {
        config,
        onchain_wallet,
        persister,
        event_manager,
        status_stream,
        swapper,
        liquid_chain_service,
        bitcoin_chain_service,
        fiat_api: breez_server,
        is_started: RwLock::new(true),
        shutdown_sender,
        shutdown_receiver,
        send_swap_state_handler,
        receive_swap_state_handler,
        chain_swap_state_handler,
        buy_bitcoin_service,
    })
}
