use std::collections::HashMap;
use std::time::Instant;
use std::{fs, path::PathBuf, str::FromStr, sync::Arc, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use boltz_client::{swaps::boltzv2::*, util::secrets::Preimage, Bolt11Invoice};
use boltz_client::{LockTime, ToHex};
use chain::liquid::{HybridLiquidChainService, LiquidChainService};
use futures_util::stream::select_all;
use futures_util::StreamExt;
use log::{debug, error, info, warn};
use lwk_wollet::bitcoin::hex::DisplayHex;
use lwk_wollet::hashes::{sha256, Hash};
use lwk_wollet::secp256k1::ThirtyTwoByteHash;
use lwk_wollet::{elements, ElectrumUrl, ElementsNetwork};
use sdk_common::bitcoin::secp256k1::Secp256k1;
use sdk_common::bitcoin::util::bip32::ChildNumber;
use sdk_common::prelude::{FiatAPI, FiatCurrency, LnUrlPayError, LnUrlWithdrawError, Rate};
use tokio::sync::{watch, Mutex, RwLock};
use tokio::time::MissedTickBehavior;
use tokio_stream::wrappers::BroadcastStream;
use url::Url;

use crate::chain::bitcoin::{self, BitcoinChainService};
use crate::chain_swap::ChainSwapStateHandler;
use crate::error::SdkError;
use crate::model::PaymentState::*;
use crate::receive_swap::ReceiveSwapStateHandler;
use crate::send_swap::SendSwapStateHandler;
use crate::swapper::{BoltzSwapper, ReconnectHandler, Swapper, SwapperStatusStream};
use crate::wallet::{LiquidOnchainWallet, OnchainWallet};
use crate::{
    error::{PaymentError, SdkResult},
    event::EventManager,
    model::*,
    persist::Persister,
    utils, *,
};

pub const DEFAULT_DATA_DIR: &str = ".data";
/// Number of blocks to monitor a swap after its timeout block height
pub const CHAIN_SWAP_MONITORING_PERIOD_BITCOIN_BLOCKS: u32 = 4320;

pub struct LiquidSdk {
    config: Config,
    onchain_wallet: Arc<dyn OnchainWallet>,
    persister: Arc<Persister>,
    event_manager: Arc<EventManager>,
    status_stream: Arc<dyn SwapperStatusStream>,
    swapper: Arc<dyn Swapper>,
    liquid_chain_service: Arc<Mutex<dyn LiquidChainService>>,
    bitcoin_chain_service: Arc<Mutex<dyn BitcoinChainService>>,
    fiat_api: Arc<dyn FiatAPI>,
    is_started: RwLock<bool>,
    shutdown_sender: watch::Sender<()>,
    shutdown_receiver: watch::Receiver<()>,
    send_swap_state_handler: SendSwapStateHandler,
    receive_swap_state_handler: ReceiveSwapStateHandler,
    chain_swap_state_handler: Arc<ChainSwapStateHandler>,
}

impl LiquidSdk {
    pub async fn connect(req: ConnectRequest) -> Result<Arc<LiquidSdk>> {
        let maybe_swapper_proxy_url =
            match BreezServer::new("https://bs1.breez.technology:443".into(), None) {
                Ok(breez_server) => breez_server
                    .fetch_boltz_swapper_urls()
                    .await
                    .ok()
                    .and_then(|swapper_urls| swapper_urls.first().cloned()),
                Err(_) => None,
            };

        let sdk = LiquidSdk::new(req.config, maybe_swapper_proxy_url, req.mnemonic)?;
        sdk.start().await?;

        Ok(sdk)
    }

    fn new(
        config: Config,
        swapper_proxy_url: Option<String>,
        mnemonic: String,
    ) -> Result<Arc<Self>> {
        fs::create_dir_all(&config.working_dir)?;

        let persister = Arc::new(Persister::new(&config.working_dir, config.network)?);
        persister.init()?;

        let event_manager = Arc::new(EventManager::new());
        let (shutdown_sender, shutdown_receiver) = watch::channel::<()>(());

        if let Some(swapper_proxy_url) = swapper_proxy_url {
            persister.set_swapper_proxy_url(swapper_proxy_url)?;
        }
        let cached_swapper_proxy_url = persister.get_swapper_proxy_url()?;
        let swapper = Arc::new(BoltzSwapper::new(config.clone(), cached_swapper_proxy_url));
        let status_stream = Arc::<dyn SwapperStatusStream>::from(swapper.create_status_stream());

        let liquid_chain_service =
            Arc::new(Mutex::new(HybridLiquidChainService::new(config.clone())?));
        let bitcoin_chain_service = Arc::new(Mutex::new(bitcoin::ElectrumClient::new(
            &ElectrumUrl::new(&config.bitcoin_electrum_url, true, true),
        )?));

        let onchain_wallet = Arc::new(LiquidOnchainWallet::new(mnemonic, config.clone())?);

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

        let breez_server = BreezServer::new(PRODUCTION_BREEZSERVER_URL.into(), None)?;

        let sdk = Arc::new(LiquidSdk {
            config: config.clone(),
            onchain_wallet,
            persister: persister.clone(),
            event_manager,
            status_stream: status_stream.clone(),
            swapper,
            bitcoin_chain_service,
            liquid_chain_service,
            fiat_api: Arc::new(breez_server),
            is_started: RwLock::new(false),
            shutdown_sender,
            shutdown_receiver,
            send_swap_state_handler,
            receive_swap_state_handler,
            chain_swap_state_handler,
        });
        Ok(sdk)
    }

    /// Starts an SDK instance.
    ///
    /// Internal method. Should only be called once per instance.
    /// Should only be called as part of [LiquidSdk::connect].
    async fn start(self: &Arc<LiquidSdk>) -> SdkResult<()> {
        let mut is_started = self.is_started.write().await;
        let start_ts = Instant::now();

        self.persister
            .update_send_swaps_by_state(Created, TimedOut)?;
        self.start_background_tasks().await?;
        *is_started = true;

        let start_duration = start_ts.elapsed();
        info!("Liquid SDK initialized in: {start_duration:?}");
        Ok(())
    }

    /// Starts background tasks.
    ///
    /// Internal method. Should only be used as part of [LiquidSdk::start].
    async fn start_background_tasks(self: &Arc<LiquidSdk>) -> SdkResult<()> {
        // Periodically run sync() in the background
        let sdk_clone = self.clone();
        let mut shutdown_rx_sync_loop = self.shutdown_receiver.clone();
        tokio::spawn(async move {
            loop {
                _ = sdk_clone.sync().await;

                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_secs(30)) => {}
                    _ = shutdown_rx_sync_loop.changed() => {
                        info!("Received shutdown signal, exiting periodic sync loop");
                        return;
                    }
                }
            }
        });

        let reconnect_handler = Box::new(SwapperReconnectHandler {
            persister: self.persister.clone(),
            status_stream: self.status_stream.clone(),
        });
        self.status_stream
            .clone()
            .start(reconnect_handler, self.shutdown_receiver.clone())
            .await;
        self.chain_swap_state_handler
            .clone()
            .start(self.shutdown_receiver.clone())
            .await;
        self.track_swap_updates().await;
        self.track_pending_swaps().await;

        Ok(())
    }

    async fn ensure_is_started(&self) -> SdkResult<()> {
        let is_started = self.is_started.read().await;
        ensure_sdk!(*is_started, SdkError::NotStarted);
        Ok(())
    }

    /// Trigger the stopping of background threads for this SDK instance.
    pub async fn disconnect(&self) -> SdkResult<()> {
        self.ensure_is_started().await?;

        let mut is_started = self.is_started.write().await;
        self.shutdown_sender
            .send(())
            .map_err(|e| SdkError::Generic {
                err: format!("Shutdown failed: {e}"),
            })?;
        *is_started = false;
        Ok(())
    }

    async fn track_swap_updates(self: &Arc<LiquidSdk>) {
        let cloned = self.clone();
        tokio::spawn(async move {
            let mut shutdown_receiver = cloned.shutdown_receiver.clone();
            let mut updates_stream = cloned.status_stream.subscribe_swap_updates();
            let swaps_streams = vec![
                cloned.send_swap_state_handler.subscribe_payment_updates(),
                cloned
                    .receive_swap_state_handler
                    .subscribe_payment_updates(),
                cloned.chain_swap_state_handler.subscribe_payment_updates(),
            ];
            let mut combined_swap_streams =
                select_all(swaps_streams.into_iter().map(BroadcastStream::new));
            loop {
                tokio::select! {
                    payment_id = combined_swap_streams.next() => {
                      if let Some(payment_id) = payment_id {
                        match payment_id {
                            Ok(payment_id) => {
                              if let Err(e) = cloned.emit_payment_updated(Some(payment_id)).await {
                                error!("Failed to emit payment update: {e:?}");
                              }
                            }
                            Err(e) => error!("Failed to receive swap state change: {e:?}")
                        }
                      }
                    }
                    update = updates_stream.recv() => match update {
                        Ok(update) => {
                            let _ = cloned.sync().await;
                            let id = &update.id;
                            match cloned.persister.fetch_swap_by_id(id) {
                                Ok(Swap::Send(_)) => match cloned.send_swap_state_handler.on_new_status(&update).await {
                                    Ok(_) => info!("Successfully handled Send Swap {id} update"),
                                    Err(e) => error!("Failed to handle Send Swap {id} update: {e}")
                                },
                                Ok(Swap::Receive(_)) => match cloned.receive_swap_state_handler.on_new_status(&update).await {
                                    Ok(_) => info!("Successfully handled Receive Swap {id} update"),
                                    Err(e) => error!("Failed to handle Receive Swap {id} update: {e}")
                                },
                                Ok(Swap::Chain(_)) => match cloned.chain_swap_state_handler.on_new_status(&update).await {
                                    Ok(_) => info!("Successfully handled Chain Swap {id} update"),
                                    Err(e) => error!("Failed to handle Chain Swap {id} update: {e}")
                                },
                                _ => {
                                    error!("Could not find Swap {id}");
                                }
                            }
                        }
                        Err(e) => error!("Received stream error: {e:?}"),
                    },
                    _ = shutdown_receiver.changed() => {
                        info!("Received shutdown signal, exiting swap updates loop");
                        return;
                    }
                }
            }
        });
    }

    async fn track_pending_swaps(self: &Arc<LiquidSdk>) {
        let cloned = self.clone();
        tokio::spawn(async move {
            let mut shutdown_receiver = cloned.shutdown_receiver.clone();
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        match cloned.persister.list_pending_send_swaps() {
                            Ok(pending_send_swaps) => {
                                for swap in pending_send_swaps {
                                    if let Err(e) = cloned.check_send_swap_expiration(&swap).await {
                                        error!("Error checking expiration for Send Swap {}: {e:?}", swap.id);
                                    }
                                }
                            }
                            Err(e) => error!("Error listing pending send swaps: {e:?}"),
                        }
                        match cloned.persister.list_pending_chain_swaps() {
                            Ok(pending_chain_swaps) => {
                                for swap in pending_chain_swaps {
                                    if let Err(e) = cloned.check_chain_swap_expiration(&swap).await {
                                        error!("Error checking expiration for Chain Swap {}: {e:?}", swap.id);
                                    }
                                }
                            }
                            Err(e) => error!("Error listing pending chain swaps: {e:?}"),
                        }
                    },
                    _ = shutdown_receiver.changed() => {
                        info!("Received shutdown signal, exiting pending swaps loop");
                        return;
                    }
                }
            }
        });
    }

    async fn check_chain_swap_expiration(&self, chain_swap: &ChainSwap) -> Result<()> {
        if chain_swap.user_lockup_tx_id.is_some() && chain_swap.refund_tx_id.is_none() {
            match chain_swap.direction {
                Direction::Incoming => {
                    let swap_script = chain_swap.get_lockup_swap_script()?.as_bitcoin_script()?;
                    let current_height =
                        self.bitcoin_chain_service.lock().await.tip()?.height as u32;
                    let locktime_from_height =
                        LockTime::from_height(current_height).map_err(|e| {
                            PaymentError::Generic {
                                err: format!(
                                    "Error getting locktime from height {current_height:?}: {e}",
                                ),
                            }
                        })?;

                    info!("Checking Chain Swap {} expiration: locktime_from_height = {locktime_from_height:?},  swap_script.locktime = {:?}", chain_swap.id, swap_script.locktime);
                    if swap_script.locktime.is_implied_by(locktime_from_height) {
                        let id: &String = &chain_swap.id;
                        info!("Chain Swap {} user lockup tx was broadcast. Setting the swap to refundable.", id);
                        self.chain_swap_state_handler
                            .update_swap_info(id, Refundable, None, None, None, None)
                            .await?;
                    }
                }
                Direction::Outgoing => {
                    let swap_script = chain_swap.get_lockup_swap_script()?.as_liquid_script()?;
                    let current_height = self.liquid_chain_service.lock().await.tip().await?;
                    let locktime_from_height = elements::LockTime::from_height(current_height)?;

                    info!("Checking Chain Swap {} expiration: locktime_from_height = {locktime_from_height:?},  swap_script.locktime = {:?}", chain_swap.id, swap_script.locktime);
                    if utils::is_locktime_expired(locktime_from_height, swap_script.locktime) {
                        self.chain_swap_state_handler
                            .refund_outgoing_swap(chain_swap)
                            .await?;
                    }
                }
            }
        }
        Ok(())
    }

    async fn check_send_swap_expiration(&self, send_swap: &SendSwap) -> Result<()> {
        if send_swap.lockup_tx_id.is_some() && send_swap.refund_tx_id.is_none() {
            let swap_script = send_swap.get_swap_script()?;
            let current_height = self.liquid_chain_service.lock().await.tip().await?;
            let locktime_from_height = elements::LockTime::from_height(current_height)?;

            info!("Checking Send Swap {} expiration: locktime_from_height = {locktime_from_height:?},  swap_script.locktime = {:?}", send_swap.id, swap_script.locktime);
            if utils::is_locktime_expired(locktime_from_height, swap_script.locktime) {
                let id = &send_swap.id;
                let refund_tx_id = self.refund_send(send_swap).await?;
                info!("Broadcast refund tx for Send Swap {id}. Tx id: {refund_tx_id}");
                self.send_swap_state_handler
                    .update_swap_info(id, Pending, None, None, Some(&refund_tx_id))
                    .await?;
            }
        }
        Ok(())
    }

    async fn notify_event_listeners(&self, e: SdkEvent) -> Result<()> {
        self.event_manager.notify(e).await;
        Ok(())
    }

    pub async fn add_event_listener(&self, listener: Box<dyn EventListener>) -> SdkResult<String> {
        Ok(self.event_manager.add(listener).await?)
    }

    pub async fn remove_event_listener(&self, id: String) -> SdkResult<()> {
        self.event_manager.remove(id).await;
        Ok(())
    }

    async fn emit_payment_updated(&self, payment_id: Option<String>) -> Result<()> {
        if let Some(id) = payment_id {
            match self.persister.get_payment(id.clone())? {
                Some(payment) => {
                    match payment.status {
                        Complete => {
                            self.notify_event_listeners(SdkEvent::PaymentSucceeded {
                                details: payment,
                            })
                            .await?
                        }
                        Pending => {
                            // The swap state has changed to Pending
                            match payment.swap_id.clone() {
                                Some(swap_id) => match self.persister.fetch_swap_by_id(&swap_id)? {
                                    Swap::Chain(ChainSwap { claim_tx_id, .. })
                                    | Swap::Receive(ReceiveSwap { claim_tx_id, .. }) => {
                                        match claim_tx_id {
                                            Some(_) => {
                                                // The claim tx has now been broadcast
                                                self.notify_event_listeners(
                                                    SdkEvent::PaymentWaitingConfirmation {
                                                        details: payment,
                                                    },
                                                )
                                                .await?
                                            }
                                            None => {
                                                // The lockup tx is in the mempool/confirmed
                                                self.notify_event_listeners(
                                                    SdkEvent::PaymentPending { details: payment },
                                                )
                                                .await?
                                            }
                                        }
                                    }
                                    Swap::Send(_) => {
                                        // The lockup tx is in the mempool/confirmed
                                        self.notify_event_listeners(SdkEvent::PaymentPending {
                                            details: payment,
                                        })
                                        .await?
                                    }
                                },
                                None => debug!("Payment has no swap id"),
                            }
                        }
                        RefundPending => {
                            // The swap state has changed to RefundPending
                            self.notify_event_listeners(SdkEvent::PaymentRefundPending {
                                details: payment,
                            })
                            .await?
                        }
                        Failed => match payment.payment_type {
                            PaymentType::Receive => {
                                self.notify_event_listeners(SdkEvent::PaymentFailed {
                                    details: payment,
                                })
                                .await?
                            }
                            PaymentType::Send => {
                                // The refund tx is confirmed
                                self.notify_event_listeners(SdkEvent::PaymentRefunded {
                                    details: payment,
                                })
                                .await?
                            }
                        },
                        _ => (),
                    };
                }
                None => debug!("Payment not found: {id}"),
            }
        }
        Ok(())
    }

    pub async fn get_info(&self) -> Result<GetInfoResponse> {
        self.ensure_is_started().await?;
        debug!(
            "next_unused_address: {}",
            self.onchain_wallet.next_unused_address().await?
        );

        let mut pending_send_sat = 0;
        let mut pending_receive_sat = 0;
        let mut confirmed_sent_sat = 0;
        let mut confirmed_received_sat = 0;

        for p in self.list_payments().await? {
            match p.payment_type {
                PaymentType::Send => match p.status {
                    Complete => confirmed_sent_sat += p.amount_sat,
                    Failed => {
                        confirmed_sent_sat += p.amount_sat;
                        confirmed_received_sat += p.refund_tx_amount_sat.unwrap_or_default();
                    }
                    Pending => match p.refund_tx_amount_sat {
                        Some(refund_tx_amount_sat) => {
                            confirmed_sent_sat += p.amount_sat;
                            pending_receive_sat += refund_tx_amount_sat;
                        }
                        None => pending_send_sat += p.amount_sat,
                    },
                    Created => pending_send_sat += p.amount_sat,
                    Refundable | RefundPending | TimedOut => {}
                },
                PaymentType::Receive => match p.status {
                    Complete => confirmed_received_sat += p.amount_sat,
                    Pending => pending_receive_sat += p.amount_sat,
                    Created | Refundable | RefundPending | Failed | TimedOut => {}
                },
            }
        }

        Ok(GetInfoResponse {
            balance_sat: confirmed_received_sat - confirmed_sent_sat - pending_send_sat,
            pending_send_sat,
            pending_receive_sat,
            pubkey: self.onchain_wallet.pubkey(),
        })
    }

    fn validate_invoice(&self, invoice: &str) -> Result<Bolt11Invoice, PaymentError> {
        let invoice = invoice.trim().parse::<Bolt11Invoice>().map_err(|err| {
            PaymentError::InvalidInvoice {
                err: err.to_string(),
            }
        })?;

        match (invoice.network().to_string().as_str(), self.config.network) {
            ("bitcoin", LiquidNetwork::Mainnet) => {}
            ("testnet", LiquidNetwork::Testnet) => {}
            _ => {
                return Err(PaymentError::InvalidInvoice {
                    err: "Invoice cannot be paid on the current network".to_string(),
                })
            }
        }

        ensure_sdk!(
            !invoice.is_expired(),
            PaymentError::InvalidInvoice {
                err: "Invoice has expired".to_string()
            }
        );

        Ok(invoice)
    }

    fn validate_submarine_pairs(
        &self,
        receiver_amount_sat: u64,
    ) -> Result<SubmarinePair, PaymentError> {
        let lbtc_pair = self
            .swapper
            .get_submarine_pairs()?
            .ok_or(PaymentError::PairsNotFound)?;

        lbtc_pair.limits.within(receiver_amount_sat)?;

        let fees_sat = lbtc_pair.fees.total(receiver_amount_sat);

        ensure_sdk!(
            receiver_amount_sat > fees_sat,
            PaymentError::AmountOutOfRange
        );

        Ok(lbtc_pair)
    }

    fn validate_chain_pairs(
        &self,
        direction: Direction,
        amount_sat: u64,
    ) -> Result<ChainPair, PaymentError> {
        let pair = self
            .swapper
            .get_chain_pairs(direction)?
            .ok_or(PaymentError::PairsNotFound)?;

        pair.limits.within(amount_sat)?;

        let fees_sat = pair.fees.total(amount_sat);

        ensure_sdk!(amount_sat > fees_sat, PaymentError::AmountOutOfRange);

        Ok(pair)
    }

    /// Estimate the onchain fee for sending the given amount to the given destination address
    async fn estimate_onchain_tx_fee(
        &self,
        amount_sat: u64,
        address: &str,
        fee_rate: Option<f32>,
    ) -> Result<u64> {
        Ok(self
            .onchain_wallet
            .build_tx(fee_rate, address, amount_sat)
            .await?
            .all_fees()
            .values()
            .sum())
    }

    async fn estimate_lockup_tx_fee(&self, amount_sat: u64) -> Result<u64> {
        // TODO Replace this with own address when LWK supports taproot
        //  https://github.com/Blockstream/lwk/issues/31
        let temp_p2tr_addr = match self.config.network {
            LiquidNetwork::Mainnet => "lq1pqvzxvqhrf54dd4sny4cag7497pe38252qefk46t92frs7us8r80ja9ha8r5me09nn22m4tmdqp5p4wafq3s59cql3v9n45t5trwtxrmxfsyxjnstkctj",
            LiquidNetwork::Testnet => "tlq1pq0wqu32e2xacxeyps22x8gjre4qk3u6r70pj4r62hzczxeyz8x3yxucrpn79zy28plc4x37aaf33kwt6dz2nn6gtkya6h02mwpzy4eh69zzexq7cf5y5"
        };

        self.estimate_onchain_tx_fee(amount_sat, temp_p2tr_addr, self.config.lowball_fee_rate())
            .await
    }

    pub async fn prepare_send_payment(
        &self,
        req: &PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        self.ensure_is_started().await?;

        self.ensure_send_is_not_self_transfer(&req.invoice)?;
        let invoice = self.validate_invoice(&req.invoice)?;

        let receiver_amount_sat = invoice
            .amount_milli_satoshis()
            .ok_or(PaymentError::AmountOutOfRange)?
            / 1000;
        let lbtc_pair = self.validate_submarine_pairs(receiver_amount_sat)?;

        let fees_sat = match self.swapper.check_for_mrh(&req.invoice)? {
            Some((lbtc_address, _)) => {
                self.estimate_onchain_tx_fee(
                    receiver_amount_sat,
                    &lbtc_address,
                    self.config.lowball_fee_rate(),
                )
                .await?
            }
            None => {
                let lockup_fees_sat = self.estimate_lockup_tx_fee(receiver_amount_sat).await?;
                lbtc_pair.fees.total(receiver_amount_sat) + lockup_fees_sat
            }
        };

        let payer_amount_sat = receiver_amount_sat + fees_sat;
        ensure_sdk!(
            payer_amount_sat <= self.get_info().await?.balance_sat,
            PaymentError::InsufficientFunds
        );

        Ok(PrepareSendResponse {
            invoice: req.invoice.clone(),
            fees_sat,
        })
    }

    async fn refund_send_non_cooperative(
        &self,
        swap: &SendSwap,
        broadcast_fees_sat: u64,
    ) -> Result<String, PaymentError> {
        info!(
            "Initiating non-cooperative refund for Send Swap {}",
            &swap.id
        );

        let current_height = self.onchain_wallet.tip().await.height();
        let output_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let refund_tx_id = self.swapper.refund_send_swap_non_cooperative(
            swap,
            broadcast_fees_sat,
            &output_address,
            current_height,
        )?;

        info!(
            "Successfully broadcast non-cooperative refund for Send Swap {}, tx: {}",
            swap.id, refund_tx_id
        );
        Ok(refund_tx_id)
    }

    async fn refund_send(&self, swap: &SendSwap) -> Result<String, PaymentError> {
        let amount_sat = get_invoice_amount!(swap.invoice);
        let output_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let cooperative_refund_tx_fees_sat = self
            .estimate_onchain_tx_fee(amount_sat, &output_address, self.config.lowball_fee_rate())
            .await?;
        let refund_res = self.swapper.refund_send_swap_cooperative(
            swap,
            &output_address,
            cooperative_refund_tx_fees_sat,
        );
        match refund_res {
            Ok(res) => Ok(res),
            Err(e) => {
                let non_cooperative_refund_tx_fees_sat = self
                    .estimate_onchain_tx_fee(swap.receiver_amount_sat, &output_address, None)
                    .await?;
                warn!("Cooperative refund failed: {:?}", e);
                self.refund_send_non_cooperative(swap, non_cooperative_refund_tx_fees_sat)
                    .await
            }
        }
    }

    fn ensure_send_is_not_self_transfer(&self, invoice: &str) -> Result<(), PaymentError> {
        match self.persister.fetch_receive_swap_by_invoice(invoice)? {
            None => Ok(()),
            Some(_) => Err(PaymentError::SelfTransferNotSupported),
        }
    }

    /// Creates, initiates and starts monitoring the progress of a Send Payment.
    ///
    /// Depending on [Config]'s `payment_timeout_sec`, this function will return:
    /// - a [PaymentError::PaymentTimeout], if the payment could not be initiated in this time
    /// - a [PaymentState::Pending] payment, if the payment could be initiated, but didn't yet
    /// complete in this time
    /// - a [PaymentState::Complete] payment, if the payment was successfully completed in this time
    pub async fn send_payment(
        &self,
        req: &PrepareSendResponse,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.ensure_is_started().await?;

        self.ensure_send_is_not_self_transfer(&req.invoice)?;
        self.validate_invoice(&req.invoice)?;

        let amount_sat = get_invoice_amount!(&req.invoice);
        let payer_amount_sat = amount_sat + req.fees_sat;
        ensure_sdk!(
            payer_amount_sat <= self.get_info().await?.balance_sat,
            PaymentError::InsufficientFunds
        );

        match self.swapper.check_for_mrh(&req.invoice)? {
            // If we find a valid MRH, extract the BIP21 amount and address, then pay via onchain tx
            Some((address, amount_btc)) => {
                self.send_payment_via_mrh(req, &address, amount_btc).await
            }

            // If no MRH found, perform usual swap
            None => self.send_payment_via_swap(req).await,
        }
    }

    /// Performs a Send Payment by doing an onchain tx to the L-BTC address in the MRH.
    async fn send_payment_via_mrh(
        &self,
        req: &PrepareSendResponse,
        lbtc_address: &str,
        amount_btc: f64,
    ) -> Result<SendPaymentResponse, PaymentError> {
        let amount_sat: u64 = (amount_btc * 100_000_000.0) as u64;
        info!("Found MRH for L-BTC address {lbtc_address} and amount_sat {amount_sat}");

        let receiver_amount_sat = get_invoice_amount!(req.invoice);
        let tx = self
            .onchain_wallet
            .build_tx(None, lbtc_address, receiver_amount_sat)
            .await?;
        let onchain_fees_sat: u64 = tx.all_fees().values().sum();
        let payer_amount_sat = receiver_amount_sat + onchain_fees_sat;
        info!("Built onchain L-BTC tx with receiver_amount_sat = {receiver_amount_sat}, fees_sat = {onchain_fees_sat}");
        info!("Built onchain L-BTC tx with ID {}", tx.txid());

        let tx_id = tx.txid().to_string();
        let tx_hex = lwk_wollet::elements::encode::serialize(&tx).to_lower_hex_string();
        self.swapper
            .broadcast_tx(self.config.network.into(), &tx_hex)?;

        // We insert a pseudo-tx in case LWK fails to pick up the new mempool tx for a while
        // This makes the tx known to the SDK (get_info, list_payments) instantly
        let tx_data = PaymentTxData {
            tx_id: tx_id.clone(),
            timestamp: None,
            amount_sat: payer_amount_sat,
            fees_sat: onchain_fees_sat,
            payment_type: PaymentType::Send,
            is_confirmed: false,
        };
        self.persister.insert_or_update_payment(tx_data.clone())?;
        self.emit_payment_updated(Some(tx_id)).await?; // Emit Pending event

        Ok(SendPaymentResponse {
            payment: Payment::from_tx_data(tx_data, None),
        })
    }

    /// Performs a Send Payment by doing a swap (create it, fund it, track it, etc).
    async fn send_payment_via_swap(
        &self,
        req: &PrepareSendResponse,
    ) -> Result<SendPaymentResponse, PaymentError> {
        let receiver_amount_sat = get_invoice_amount!(req.invoice);
        let lbtc_pair = self.validate_submarine_pairs(receiver_amount_sat)?;
        let lockup_tx_fees_sat = self.estimate_lockup_tx_fee(receiver_amount_sat).await?;
        ensure_sdk!(
            req.fees_sat == lbtc_pair.fees.total(receiver_amount_sat) + lockup_tx_fees_sat,
            PaymentError::InvalidOrExpiredFees
        );

        let swap = match self.persister.fetch_send_swap_by_invoice(&req.invoice)? {
            Some(swap) => match swap.state {
                Pending => return Err(PaymentError::PaymentInProgress),
                Complete => return Err(PaymentError::AlreadyPaid),
                RefundPending | Failed => {
                    return Err(PaymentError::InvalidInvoice {
                        err: "Payment has already failed. Please try with another invoice."
                            .to_string(),
                    })
                }
                _ => swap,
            },
            None => {
                let keypair = utils::generate_keypair();
                let refund_public_key = boltz_client::PublicKey {
                    compressed: true,
                    inner: keypair.public_key(),
                };
                let create_response = self.swapper.create_send_swap(CreateSubmarineRequest {
                    from: "L-BTC".to_string(),
                    to: "BTC".to_string(),
                    invoice: req.invoice.to_string(),
                    refund_public_key,
                    pair_hash: Some(lbtc_pair.hash),
                    referral_id: None,
                })?;

                let swap_id = &create_response.id;
                let create_response_json =
                    SendSwap::from_boltz_struct_to_json(&create_response, swap_id)?;

                let payer_amount_sat = req.fees_sat + receiver_amount_sat;
                let swap = SendSwap {
                    id: swap_id.clone(),
                    invoice: req.invoice.clone(),
                    preimage: None,
                    payer_amount_sat,
                    receiver_amount_sat,
                    create_response_json,
                    lockup_tx_id: None,
                    refund_tx_id: None,
                    created_at: utils::now(),
                    state: PaymentState::Created,
                    refund_private_key: keypair.display_secret().to_string(),
                };
                self.persister.insert_send_swap(&swap)?;
                swap
            }
        };
        self.status_stream.track_swap_id(&swap.id)?;

        let accept_zero_conf = swap.get_boltz_create_response()?.accept_zero_conf;
        self.wait_for_payment(Swap::Send(swap), accept_zero_conf)
            .await
            .map(|payment| SendPaymentResponse { payment })
    }

    pub async fn prepare_pay_onchain(
        &self,
        req: &PreparePayOnchainRequest,
    ) -> Result<PreparePayOnchainResponse, PaymentError> {
        self.ensure_is_started().await?;

        let receiver_amount_sat = req.receiver_amount_sat;
        let pair = self.validate_chain_pairs(Direction::Outgoing, receiver_amount_sat)?;
        let claim_fees_sat = pair.fees.claim_estimate();
        let server_fees_sat = pair.fees.server();
        let server_lockup_amount_sat = receiver_amount_sat + claim_fees_sat;
        let lockup_fees_sat = self
            .estimate_lockup_tx_fee(server_lockup_amount_sat)
            .await?;

        let res = PreparePayOnchainResponse {
            receiver_amount_sat,
            fees_sat: pair.fees.boltz(server_lockup_amount_sat)
                + lockup_fees_sat
                + claim_fees_sat
                + server_fees_sat,
        };

        let payer_amount_sat = res.receiver_amount_sat + res.fees_sat;
        ensure_sdk!(
            payer_amount_sat <= self.get_info().await?.balance_sat,
            PaymentError::InsufficientFunds
        );

        Ok(res)
    }

    pub async fn pay_onchain(
        &self,
        req: &PayOnchainRequest,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.ensure_is_started().await?;

        let receiver_amount_sat = req.prepare_res.receiver_amount_sat;
        let pair = self.validate_chain_pairs(Direction::Outgoing, receiver_amount_sat)?;
        let claim_fees_sat = pair.fees.claim_estimate();
        let server_fees_sat = pair.fees.server();
        let server_lockup_amount_sat = receiver_amount_sat + claim_fees_sat;
        let lockup_fees_sat = self
            .estimate_lockup_tx_fee(server_lockup_amount_sat)
            .await?;

        ensure_sdk!(
            req.prepare_res.fees_sat
                == pair.fees.boltz(server_lockup_amount_sat)
                    + lockup_fees_sat
                    + claim_fees_sat
                    + server_fees_sat,
            PaymentError::InvalidOrExpiredFees
        );

        let payer_amount_sat = req.prepare_res.fees_sat + receiver_amount_sat;
        ensure_sdk!(
            payer_amount_sat <= self.get_info().await?.balance_sat,
            PaymentError::InsufficientFunds
        );

        let preimage = Preimage::new();
        let preimage_str = preimage.to_string().ok_or(PaymentError::InvalidPreimage)?;

        let claim_keypair = utils::generate_keypair();
        let claim_public_key = boltz_client::PublicKey {
            compressed: true,
            inner: claim_keypair.public_key(),
        };
        let refund_keypair = utils::generate_keypair();
        let refund_public_key = boltz_client::PublicKey {
            compressed: true,
            inner: refund_keypair.public_key(),
        };
        let create_response = self.swapper.create_chain_swap(CreateChainRequest {
            from: "L-BTC".to_string(),
            to: "BTC".to_string(),
            preimage_hash: preimage.sha256,
            claim_public_key: Some(claim_public_key),
            refund_public_key: Some(refund_public_key),
            user_lock_amount: None,
            server_lock_amount: Some(server_lockup_amount_sat as u32), // TODO update our model
            pair_hash: Some(pair.hash),
            referral_id: None,
        })?;

        let swap_id = &create_response.id;
        let create_response_json = ChainSwap::from_boltz_struct_to_json(&create_response, swap_id)?;

        let accept_zero_conf = server_lockup_amount_sat <= pair.limits.maximal_zero_conf;
        let payer_amount_sat = req.prepare_res.fees_sat + receiver_amount_sat;
        let claim_address = req.address.clone();

        let swap = ChainSwap {
            id: swap_id.clone(),
            direction: Direction::Outgoing,
            claim_address,
            lockup_address: create_response.lockup_details.lockup_address,
            timeout_block_height: create_response.lockup_details.timeout_block_height,
            preimage: preimage_str,
            payer_amount_sat,
            receiver_amount_sat,
            claim_fees_sat,
            accept_zero_conf,
            create_response_json,
            claim_private_key: claim_keypair.display_secret().to_string(),
            refund_private_key: refund_keypair.display_secret().to_string(),
            server_lockup_tx_id: None,
            user_lockup_tx_id: None,
            claim_tx_id: None,
            refund_tx_id: None,
            created_at: utils::now(),
            state: PaymentState::Created,
        };
        self.persister.insert_chain_swap(&swap)?;
        self.status_stream.track_swap_id(&swap.id)?;

        self.wait_for_payment(Swap::Chain(swap), accept_zero_conf)
            .await
            .map(|payment| SendPaymentResponse { payment })
    }

    async fn wait_for_payment(
        &self,
        swap: Swap,
        accept_zero_conf: bool,
    ) -> Result<Payment, PaymentError> {
        let timeout_fut = tokio::time::sleep(Duration::from_secs(self.config.payment_timeout_sec));
        tokio::pin!(timeout_fut);

        let swap_id = swap.id();
        let mut events_stream = self.event_manager.subscribe();
        let mut maybe_payment: Option<Payment> = None;

        loop {
            tokio::select! {
                _ = &mut timeout_fut => match maybe_payment {
                    Some(payment) => return Ok(payment),
                    None => {
                        debug!("Timeout occured without payment, set swap to timed out");
                        match swap {
                            Swap::Send(_) => self.send_swap_state_handler.update_swap_info(&swap_id, TimedOut, None, None, None).await?,
                            Swap::Chain(_) => self.chain_swap_state_handler.update_swap_info(&swap_id, TimedOut, None, None, None, None).await?,
                            _ => ()
                        }
                        return Err(PaymentError::PaymentTimeout)
                    },
                },
                event = events_stream.recv() => match event {
                    Ok(SdkEvent::PaymentPending { details }) => match details.swap_id.clone() {
                        Some(id) if id == swap_id => match accept_zero_conf {
                            true => {
                                debug!("Received Send Payment pending event with zero-conf accepted");
                                return Ok(details)
                            }
                            false => {
                                debug!("Received Send Payment pending event, waiting for confirmation");
                                maybe_payment = Some(details);
                            }
                        },
                        _ => error!("Received Send Payment pending event for payment without swap ID"),
                    },
                    Ok(SdkEvent::PaymentSucceeded { details }) => match details.swap_id.clone()
                    {
                        Some(id) if id == swap_id => {
                            debug!("Received Send Payment succeed event");
                            return Ok(details);
                        }
                        _ => error!("Received Send Payment succeed event for payment without swap ID"),
                    },
                    Ok(event) => debug!("Unhandled event: {event:?}"),
                    Err(e) => debug!("Received error waiting for event: {e:?}"),
                }
            }
        }
    }

    pub async fn prepare_receive_payment(
        &self,
        req: &PrepareReceiveRequest,
    ) -> Result<PrepareReceiveResponse, PaymentError> {
        self.ensure_is_started().await?;
        let reverse_pair = self
            .swapper
            .get_reverse_swap_pairs()?
            .ok_or(PaymentError::PairsNotFound)?;

        let payer_amount_sat = req.payer_amount_sat;
        let fees_sat = reverse_pair.fees.total(req.payer_amount_sat);

        ensure_sdk!(payer_amount_sat > fees_sat, PaymentError::AmountOutOfRange);

        reverse_pair
            .limits
            .within(payer_amount_sat)
            .map_err(|_| PaymentError::AmountOutOfRange)?;

        debug!("Preparing Receive Swap with: payer_amount_sat {payer_amount_sat} sat, fees_sat {fees_sat} sat");

        Ok(PrepareReceiveResponse {
            payer_amount_sat,
            fees_sat,
        })
    }

    pub async fn receive_payment(
        &self,
        req: &PrepareReceiveResponse,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.ensure_is_started().await?;

        let payer_amount_sat = req.payer_amount_sat;
        let fees_sat = req.fees_sat;

        let reverse_pair = self
            .swapper
            .get_reverse_swap_pairs()?
            .ok_or(PaymentError::PairsNotFound)?;
        let new_fees_sat = reverse_pair.fees.total(req.payer_amount_sat);
        ensure_sdk!(fees_sat == new_fees_sat, PaymentError::InvalidOrExpiredFees);

        debug!("Creating Receive Swap with: payer_amount_sat {payer_amount_sat} sat, fees_sat {fees_sat} sat");

        let keypair = utils::generate_keypair();

        let preimage = Preimage::new();
        let preimage_str = preimage.to_string().ok_or(PaymentError::InvalidPreimage)?;
        let preimage_hash = preimage.sha256.to_string();

        // Address to be used for a BIP-21 direct payment
        let mrh_addr = self.onchain_wallet.next_unused_address().await?;

        // Signature of the claim public key of the SHA256 hash of the address for the direct payment
        let mrh_addr_str = mrh_addr.to_string();
        let mrh_addr_hash = sha256::Hash::hash(mrh_addr_str.as_bytes());
        let mrh_addr_hash_sig = keypair.sign_schnorr(mrh_addr_hash.into());

        let v2_req = CreateReverseRequest {
            invoice_amount: req.payer_amount_sat as u32, // TODO update our model
            from: "BTC".to_string(),
            to: "L-BTC".to_string(),
            preimage_hash: preimage.sha256,
            claim_public_key: keypair.public_key().into(),
            address: Some(mrh_addr_str.clone()),
            address_signature: Some(mrh_addr_hash_sig.to_hex()),
            referral_id: None,
        };
        let create_response = self.swapper.create_receive_swap(v2_req)?;

        // Check if correct MRH was added to the invoice by Boltz
        let (bip21_lbtc_address, bip21_amount_btc) = self
            .swapper
            .check_for_mrh(&create_response.invoice)?
            .ok_or(PaymentError::receive_error("Invoice has no MRH"))?;
        let received_bip21_amount_sat: u64 = (bip21_amount_btc * 100_000_000.0) as u64;
        ensure_sdk!(
            bip21_lbtc_address == mrh_addr_str,
            PaymentError::receive_error("Invoice has incorrect address in MRH")
        );
        // The swap fee savings are passed on to the Sender: MRH amount = invoice amount - fees
        let expected_bip21_amount_sat = req.payer_amount_sat - req.fees_sat;
        ensure_sdk!(
            received_bip21_amount_sat == expected_bip21_amount_sat,
            PaymentError::receive_error(&format!(
                "Invoice has incorrect amount in MRH: expected {expected_bip21_amount_sat} sat, MRH has {received_bip21_amount_sat} sat",
            ))
        );

        let swap_id = create_response.id.clone();
        let invoice = Bolt11Invoice::from_str(&create_response.invoice).map_err(|err| {
            PaymentError::InvalidInvoice {
                err: err.to_string(),
            }
        })?;
        let payer_amount_sat =
            invoice
                .amount_milli_satoshis()
                .ok_or(PaymentError::InvalidInvoice {
                    err: "Invoice does not contain an amount".to_string(),
                })?
                / 1000;

        // Double check that the generated invoice includes our data
        // https://docs.boltz.exchange/v/api/dont-trust-verify#lightning-invoice-verification
        if invoice.payment_hash().to_string() != preimage_hash {
            return Err(PaymentError::InvalidInvoice {
                err: "Invalid preimage returned by swapper".to_string(),
            });
        };

        let create_response_json = ReceiveSwap::from_boltz_struct_to_json(
            &create_response,
            &swap_id,
            &invoice.to_string(),
        )?;
        self.persister
            .insert_receive_swap(&ReceiveSwap {
                id: swap_id.clone(),
                preimage: preimage_str,
                create_response_json,
                claim_private_key: keypair.display_secret().to_string(),
                invoice: invoice.to_string(),
                payer_amount_sat,
                receiver_amount_sat: payer_amount_sat - req.fees_sat,
                claim_fees_sat: reverse_pair.fees.claim_estimate(),
                claim_tx_id: None,
                created_at: utils::now(),
                state: PaymentState::Created,
            })
            .map_err(|_| PaymentError::PersistError)?;
        self.status_stream.track_swap_id(&swap_id)?;

        Ok(ReceivePaymentResponse {
            id: swap_id,
            invoice: invoice.to_string(),
        })
    }

    pub async fn prepare_receive_onchain(
        &self,
        req: &PrepareReceiveOnchainRequest,
    ) -> Result<PrepareReceiveOnchainResponse, PaymentError> {
        self.ensure_is_started().await?;

        let payer_amount_sat = req.payer_amount_sat;
        let pair = self.validate_chain_pairs(Direction::Incoming, payer_amount_sat)?;
        let claim_fees_sat = pair.fees.claim_estimate();
        let server_fees_sat = pair.fees.server();

        Ok(PrepareReceiveOnchainResponse {
            payer_amount_sat,
            fees_sat: pair.fees.boltz(payer_amount_sat) + claim_fees_sat + server_fees_sat,
        })
    }

    pub async fn receive_onchain(
        &self,
        req: &PrepareReceiveOnchainResponse,
    ) -> Result<ReceiveOnchainResponse, PaymentError> {
        self.ensure_is_started().await?;

        let payer_amount_sat = req.payer_amount_sat;
        let pair = self.validate_chain_pairs(Direction::Incoming, payer_amount_sat)?;
        let claim_fees_sat = pair.fees.claim_estimate();
        let server_fees_sat = pair.fees.server();

        ensure_sdk!(
            req.fees_sat == pair.fees.boltz(payer_amount_sat) + claim_fees_sat + server_fees_sat,
            PaymentError::InvalidOrExpiredFees
        );

        let preimage = Preimage::new();
        let preimage_str = preimage.to_string().ok_or(PaymentError::InvalidPreimage)?;

        let claim_keypair = utils::generate_keypair();
        let claim_public_key = boltz_client::PublicKey {
            compressed: true,
            inner: claim_keypair.public_key(),
        };
        let refund_keypair = utils::generate_keypair();
        let refund_public_key = boltz_client::PublicKey {
            compressed: true,
            inner: refund_keypair.public_key(),
        };
        let create_response = self.swapper.create_chain_swap(CreateChainRequest {
            from: "BTC".to_string(),
            to: "L-BTC".to_string(),
            preimage_hash: preimage.sha256,
            claim_public_key: Some(claim_public_key),
            refund_public_key: Some(refund_public_key),
            user_lock_amount: Some(payer_amount_sat as u32), // TODO update our model
            server_lock_amount: None,
            pair_hash: Some(pair.hash),
            referral_id: None,
        })?;

        let swap_id = create_response.id.clone();
        let create_response_json =
            ChainSwap::from_boltz_struct_to_json(&create_response, &swap_id)?;

        let accept_zero_conf = payer_amount_sat <= pair.limits.maximal_zero_conf;
        let receiver_amount_sat = payer_amount_sat - req.fees_sat;
        let claim_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let lockup_address = create_response.lockup_details.lockup_address;

        let swap = ChainSwap {
            id: swap_id.clone(),
            direction: Direction::Incoming,
            claim_address,
            lockup_address: lockup_address.clone(),
            timeout_block_height: create_response.lockup_details.timeout_block_height,
            preimage: preimage_str,
            payer_amount_sat,
            receiver_amount_sat,
            claim_fees_sat,
            accept_zero_conf,
            create_response_json,
            claim_private_key: claim_keypair.display_secret().to_string(),
            refund_private_key: refund_keypair.display_secret().to_string(),
            server_lockup_tx_id: None,
            user_lockup_tx_id: None,
            claim_tx_id: None,
            refund_tx_id: None,
            created_at: utils::now(),
            state: PaymentState::Created,
        };
        self.persister.insert_chain_swap(&swap)?;
        self.status_stream.track_swap_id(&swap.id)?;

        let address = lockup_address;
        let amount = create_response.lockup_details.amount as f64 / 100_000_000.0;
        let bip21 = create_response.lockup_details.bip21.unwrap_or(format!(
            "bitcoin:{address}?amount={amount}&label=Send%20to%20L-BTC%20address"
        ));

        Ok(ReceiveOnchainResponse { address, bip21 })
    }

    pub async fn list_refundables(&self) -> SdkResult<Vec<RefundableSwap>> {
        Ok(self
            .persister
            .list_refundable_chain_swaps()?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    pub async fn prepare_refund(
        &self,
        req: &PrepareRefundRequest,
    ) -> SdkResult<PrepareRefundResponse> {
        let (tx_vsize, tx_fee_sat, refund_tx_id) = self.chain_swap_state_handler.prepare_refund(
            &req.swap_address,
            &req.refund_address,
            req.sat_per_vbyte,
        )?;
        Ok(PrepareRefundResponse {
            tx_vsize,
            tx_fee_sat,
            refund_tx_id,
        })
    }

    pub async fn refund(&self, req: &RefundRequest) -> Result<RefundResponse, PaymentError> {
        let refund_tx_id = self
            .chain_swap_state_handler
            .refund_incoming_swap(&req.swap_address, &req.refund_address, req.sat_per_vbyte)
            .await?;
        Ok(RefundResponse { refund_tx_id })
    }

    pub async fn rescan_onchain_swaps(&self) -> SdkResult<()> {
        self.chain_swap_state_handler
            .rescan_incoming_chain_swaps()
            .await?;
        Ok(())
    }

    /// This method fetches the chain tx data (onchain and mempool) using LWK. For every wallet tx,
    /// it inserts or updates a corresponding entry in our Payments table.
    async fn sync_payments_with_chain_data(&self, with_scan: bool) -> Result<()> {
        let payments_before_sync: HashMap<String, Payment> = self
            .list_payments()
            .await?
            .into_iter()
            .filter_map(|payment| {
                let tx_id = payment.tx_id.clone();
                tx_id.map(|tx_id| (tx_id, payment))
            })
            .collect();
        if with_scan {
            self.onchain_wallet.full_scan().await?;
        }

        let pending_receive_swaps_by_claim_tx_id =
            self.persister.list_pending_receive_swaps_by_claim_tx_id()?;
        let pending_send_swaps_by_refund_tx_id =
            self.persister.list_pending_send_swaps_by_refund_tx_id()?;
        let pending_chain_swaps_by_refund_tx_id =
            self.persister.list_pending_chain_swaps_by_refund_tx_id()?;

        for tx in self.onchain_wallet.transactions().await? {
            let tx_id = tx.txid.to_string();
            let is_tx_confirmed = tx.height.is_some();
            let amount_sat = tx.balance.values().sum::<i64>();

            self.persister.insert_or_update_payment(PaymentTxData {
                tx_id: tx_id.clone(),
                timestamp: tx.timestamp,
                amount_sat: amount_sat.unsigned_abs(),
                fees_sat: tx.fee,
                payment_type: match amount_sat >= 0 {
                    true => PaymentType::Receive,
                    false => PaymentType::Send,
                },
                is_confirmed: is_tx_confirmed,
            })?;

            if let Some(swap) = pending_receive_swaps_by_claim_tx_id.get(&tx_id) {
                if is_tx_confirmed {
                    self.receive_swap_state_handler
                        .update_swap_info(&swap.id, Complete, None, None)
                        .await?;
                }
            } else if let Some(swap) = pending_send_swaps_by_refund_tx_id.get(&tx_id) {
                if is_tx_confirmed {
                    self.send_swap_state_handler
                        .update_swap_info(&swap.id, Failed, None, None, None)
                        .await?;
                }
            } else if let Some(swap) = pending_chain_swaps_by_refund_tx_id.get(&tx_id) {
                if is_tx_confirmed {
                    self.chain_swap_state_handler
                        .update_swap_info(&swap.id, Failed, None, None, None, None)
                        .await?;
                }
            } else {
                // Payments that are not directly associated with a swap (e.g. direct onchain payments using MRH)

                match payments_before_sync.get(&tx_id) {
                    None => {
                        // A completely new payment brought in by this sync, in mempool or confirmed
                        // Covers events:
                        // - onchain Receive Pending and Complete
                        // - onchain Send Complete
                        self.emit_payment_updated(Some(tx_id)).await?;
                    }
                    Some(payment_before_sync) => {
                        if payment_before_sync.status == Pending && is_tx_confirmed {
                            // A know payment that was in the mempool, but is now confirmed
                            // Covers events: Send and Receive direct onchain payments transitioning to Complete
                            self.emit_payment_updated(Some(tx_id)).await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Lists the SDK payments. The payments are determined based on onchain transactions and swaps.
    pub async fn list_payments(&self) -> Result<Vec<Payment>, PaymentError> {
        self.ensure_is_started().await?;

        let mut payments: Vec<Payment> = self.persister.get_payments()?;
        payments.sort_by_key(|p| p.timestamp);
        Ok(payments)
    }

    /// Empties all Liquid Wallet caches for this network type.
    pub fn empty_wallet_cache(&self) -> Result<()> {
        let mut path = PathBuf::from(self.config.working_dir.clone());
        path.push(Into::<ElementsNetwork>::into(self.config.network).as_str());
        path.push("enc_cache");

        fs::remove_dir_all(&path)?;
        fs::create_dir_all(path)?;

        Ok(())
    }

    /// Synchronize the DB with mempool and onchain data
    pub async fn sync(&self) -> SdkResult<()> {
        self.ensure_is_started().await?;

        let t0 = Instant::now();
        self.sync_payments_with_chain_data(true).await?;
        let duration_ms = Instant::now().duration_since(t0).as_millis();
        info!("Synchronized with mempool and onchain data (t = {duration_ms} ms)");

        self.notify_event_listeners(SdkEvent::Synced).await?;
        Ok(())
    }

    pub fn backup(&self, req: BackupRequest) -> Result<()> {
        let backup_path = req
            .backup_path
            .map(PathBuf::from)
            .unwrap_or(self.persister.get_default_backup_path());
        self.persister.backup(backup_path)
    }

    pub fn restore(&self, req: RestoreRequest) -> Result<()> {
        let backup_path = req
            .backup_path
            .map(PathBuf::from)
            .unwrap_or(self.persister.get_default_backup_path());
        self.persister.restore_from_backup(backup_path)
    }

    /// Second step of LNURL-pay. The first step is `parse()`, which also validates the LNURL destination
    /// and generates the `LnUrlPayRequest` payload needed here.
    ///
    /// This call will validate the `amount_msat` and `comment` parameters of `req` against the parameters
    /// of the LNURL endpoint (`req_data`). If they match the endpoint requirements, the LNURL payment
    /// is made.
    pub async fn lnurl_pay(&self, req: LnUrlPayRequest) -> Result<LnUrlPayResult, LnUrlPayError> {
        match validate_lnurl_pay(
            req.amount_msat,
            &req.comment,
            &req.data,
            self.config.network.into(),
        )
        .await?
        {
            ValidatedCallbackResponse::EndpointError { data: e } => {
                Ok(LnUrlPayResult::EndpointError { data: e })
            }
            ValidatedCallbackResponse::EndpointSuccess { data: cb } => {
                let pay_req = self
                    .prepare_send_payment(&PrepareSendRequest {
                        invoice: cb.pr.clone(),
                    })
                    .await?;

                let payment = self.send_payment(&pay_req).await?.payment;

                let maybe_sa_processed: Option<SuccessActionProcessed> = match cb.success_action {
                    Some(sa) => {
                        let processed_sa = match sa {
                            // For AES, we decrypt the contents on the fly
                            SuccessAction::Aes(data) => {
                                let preimage_str = payment
                                    .preimage
                                    .clone()
                                    .ok_or(SdkError::Generic {
                                        err: "Payment successful but no preimage found".to_string(),
                                    })
                                    .unwrap();
                                let preimage =
                                    sha256::Hash::from_str(&preimage_str).map_err(|_| {
                                        LnUrlPayError::Generic {
                                            err: "Invalid preimage".to_string(),
                                        }
                                    })?;
                                let preimage_arr: [u8; 32] = preimage.into_32();
                                let result = match (data, &preimage_arr).try_into() {
                                    Ok(data) => AesSuccessActionDataResult::Decrypted { data },
                                    Err(e) => AesSuccessActionDataResult::ErrorStatus {
                                        reason: e.to_string(),
                                    },
                                };
                                SuccessActionProcessed::Aes { result }
                            }
                            SuccessAction::Message(data) => {
                                SuccessActionProcessed::Message { data }
                            }
                            SuccessAction::Url(data) => SuccessActionProcessed::Url { data },
                        };
                        Some(processed_sa)
                    }
                    None => None,
                };

                Ok(LnUrlPayResult::EndpointSuccess {
                    data: model::LnUrlPaySuccessData {
                        payment,
                        success_action: maybe_sa_processed,
                    },
                })
            }
        }
    }

    /// Second step of LNURL-withdraw. The first step is `parse()`, which also validates the LNURL destination
    /// and generates the `LnUrlWithdrawRequest` payload needed here.
    ///
    /// This call will validate the given `amount_msat` against the parameters
    /// of the LNURL endpoint (`data`). If they match the endpoint requirements, the LNURL withdraw
    /// request is made. A successful result here means the endpoint started the payment.
    pub async fn lnurl_withdraw(
        &self,
        req: LnUrlWithdrawRequest,
    ) -> Result<LnUrlWithdrawResult, LnUrlWithdrawError> {
        let prepare_receive_res = self
            .prepare_receive_payment(&{
                PrepareReceiveRequest {
                    payer_amount_sat: req.amount_msat / 1_000,
                }
            })
            .await?;
        let receive_res = self.receive_payment(&prepare_receive_res).await?;
        let invoice = parse_invoice(&receive_res.invoice)?;

        let res = validate_lnurl_withdraw(req.data, invoice).await?;
        Ok(res)
    }

    /// Third and last step of LNURL-auth. The first step is `parse()`, which also validates the LNURL destination
    /// and generates the `LnUrlAuthRequestData` payload needed here. The second step is user approval of auth action.
    ///
    /// This call will sign `k1` of the LNURL endpoint (`req_data`) on `secp256k1` using `linkingPrivKey` and DER-encodes the signature.
    /// If they match the endpoint requirements, the LNURL auth request is made. A successful result here means the client signature is verified.
    pub async fn lnurl_auth(
        &self,
        req_data: LnUrlAuthRequestData,
    ) -> Result<LnUrlCallbackStatus, LnUrlAuthError> {
        // m/138'/0
        let hashing_key = self.onchain_wallet.derive_bip32_key(vec![
            ChildNumber::from_hardened_idx(138).map_err(Into::<LnUrlError>::into)?,
            ChildNumber::from(0),
        ])?;

        let url =
            Url::from_str(&req_data.url).map_err(|e| LnUrlError::InvalidUri(e.to_string()))?;

        let derivation_path = get_derivation_path(hashing_key, url)?;
        let linking_key = self.onchain_wallet.derive_bip32_key(derivation_path)?;
        let linking_keys = linking_key.to_keypair(&Secp256k1::new());

        Ok(perform_lnurl_auth(linking_keys, req_data).await?)
    }

    /// Fetch live rates of fiat currencies, sorted by name
    pub async fn fetch_fiat_rates(&self) -> Result<Vec<Rate>, SdkError> {
        self.fiat_api.fetch_fiat_rates().await.map_err(Into::into)
    }

    /// List all supported fiat currencies for which there is a known exchange rate.
    /// List is sorted by the canonical name of the currency
    pub async fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>, SdkError> {
        self.fiat_api
            .list_fiat_currencies()
            .await
            .map_err(Into::into)
    }

    pub fn default_config(network: LiquidNetwork) -> Config {
        match network {
            LiquidNetwork::Mainnet => Config::mainnet(),
            LiquidNetwork::Testnet => Config::testnet(),
        }
    }

    pub async fn parse(input: &str) -> Result<InputType, PaymentError> {
        parse(input)
            .await
            .map_err(|e| PaymentError::Generic { err: e.to_string() })
    }

    pub fn parse_invoice(input: &str) -> Result<LNInvoice, PaymentError> {
        parse_invoice(input).map_err(|e| PaymentError::InvalidInvoice { err: e.to_string() })
    }

    /// Configures a global SDK logger that will log to file and will forward log events to
    /// an optional application-specific logger.
    ///
    /// If called, it should be called before any SDK methods (for example, before `connect`).
    ///
    /// It must be called only once in the application lifecycle. Alternatively, If the application
    /// already uses a globally-registered logger, this method shouldn't be called at all.
    ///
    /// ### Arguments
    ///
    /// - `log_dir`: Location where the the SDK log file will be created. The directory must already exist.
    ///
    /// - `app_logger`: Optional application logger.
    ///
    /// If the application is to use it's own logger, but would also like the SDK to log SDK-specific
    /// log output to a file in the configured `log_dir`, then do not register the
    /// app-specific logger as a global logger and instead call this method with the app logger as an arg.
    ///
    /// ### Errors
    ///
    /// An error is thrown if the log file cannot be created in the working directory.
    ///
    /// An error is thrown if a global logger is already configured.
    pub fn init_logging(log_dir: &str, app_logger: Option<Box<dyn log::Log>>) -> Result<()> {
        crate::logger::init_logging(log_dir, app_logger)
    }
}

struct SwapperReconnectHandler {
    persister: Arc<Persister>,
    status_stream: Arc<dyn SwapperStatusStream>,
}

#[async_trait]
impl ReconnectHandler for SwapperReconnectHandler {
    async fn on_stream_reconnect(&self) {
        match self.persister.list_ongoing_swaps() {
            Ok(initial_ongoing_swaps) => {
                info!(
                    "On stream reconnection, got {} initial ongoing swaps",
                    initial_ongoing_swaps.len()
                );
                for ongoing_swap in initial_ongoing_swaps {
                    match self.status_stream.track_swap_id(&ongoing_swap.id()) {
                        Ok(_) => info!("Tracking ongoing swap: {}", ongoing_swap.id()),
                        Err(e) => error!("Failed to track ongoing swap: {e:?}"),
                    }
                }
            }
            Err(e) => error!("Failed to list initial ongoing swaps: {e:?}"),
        }
    }
}
