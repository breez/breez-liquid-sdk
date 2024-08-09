use std::collections::HashMap;
use std::time::Instant;
use std::{fs, path::PathBuf, str::FromStr, sync::Arc, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use boltz_client::{swaps::boltz::*, util::secrets::Preimage};
use boltz_client::{LockTime, ToHex};
use buy::{BuyBitcoinApi, BuyBitcoinService};
use chain::bitcoin::HybridBitcoinChainService;
use chain::liquid::{HybridLiquidChainService, LiquidChainService};
use chain_swap::ESTIMATED_BTC_CLAIM_TX_VSIZE;
use futures_util::stream::select_all;
use futures_util::StreamExt;
use log::{debug, error, info};
use lwk_wollet::bitcoin::hex::DisplayHex;
use lwk_wollet::hashes::{sha256, Hash};
use lwk_wollet::secp256k1::ThirtyTwoByteHash;
use lwk_wollet::{elements, ElementsNetwork};
use sdk_common::bitcoin::secp256k1::Secp256k1;
use sdk_common::bitcoin::util::bip32::ChildNumber;
use sdk_common::ensure_sdk;
use sdk_common::prelude::{FiatAPI, FiatCurrency, LnUrlPayError, LnUrlWithdrawError, Rate};
use tokio::sync::{watch, Mutex, RwLock};
use tokio::time::MissedTickBehavior;
use tokio_stream::wrappers::BroadcastStream;
use url::Url;

use crate::chain::bitcoin::BitcoinChainService;
use crate::chain_swap::ChainSwapStateHandler;
use crate::error::SdkError;
use crate::lightning_invoice::{Bolt11Invoice, Bolt11InvoiceDescription};
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
    pub(crate) config: Config,
    pub(crate) onchain_wallet: Arc<dyn OnchainWallet>,
    pub(crate) persister: Arc<Persister>,
    pub(crate) event_manager: Arc<EventManager>,
    pub(crate) status_stream: Arc<dyn SwapperStatusStream>,
    pub(crate) swapper: Arc<dyn Swapper>,
    pub(crate) liquid_chain_service: Arc<Mutex<dyn LiquidChainService>>,
    pub(crate) bitcoin_chain_service: Arc<Mutex<dyn BitcoinChainService>>,
    pub(crate) fiat_api: Arc<dyn FiatAPI>,
    pub(crate) is_started: RwLock<bool>,
    pub(crate) shutdown_sender: watch::Sender<()>,
    pub(crate) shutdown_receiver: watch::Receiver<()>,
    pub(crate) send_swap_state_handler: SendSwapStateHandler,
    pub(crate) receive_swap_state_handler: ReceiveSwapStateHandler,
    pub(crate) chain_swap_state_handler: Arc<ChainSwapStateHandler>,
    pub(crate) buy_bitcoin_service: Arc<dyn BuyBitcoinApi>,
}

impl LiquidSdk {
    /// Initializes the SDK services and starts the background tasks.
    /// This must be called to create the [LiquidSdk] instance.
    ///
    /// # Arguments
    ///
    /// * `req` - the [ConnectRequest] containing:
    ///     * `mnemonic` - the Liquid wallet mnemonic
    ///     * `config` - the SDK [Config]
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

        let onchain_wallet = Arc::new(LiquidOnchainWallet::new(mnemonic, config.clone())?);

        let persister = Arc::new(Persister::new(
            &config.get_wallet_working_dir(&onchain_wallet.lwk_signer)?,
            config.network,
        )?);
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
        let bitcoin_chain_service =
            Arc::new(Mutex::new(HybridBitcoinChainService::new(config.clone())?));

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

        let breez_server = Arc::new(BreezServer::new(PRODUCTION_BREEZSERVER_URL.into(), None)?);

        let buy_bitcoin_service =
            Arc::new(BuyBitcoinService::new(config.clone(), breez_server.clone()));

        let sdk = Arc::new(LiquidSdk {
            config: config.clone(),
            onchain_wallet,
            persister: persister.clone(),
            event_manager,
            status_stream: status_stream.clone(),
            swapper,
            bitcoin_chain_service,
            liquid_chain_service,
            fiat_api: breez_server,
            is_started: RwLock::new(false),
            shutdown_sender,
            shutdown_receiver,
            send_swap_state_handler,
            receive_swap_state_handler,
            chain_swap_state_handler,
            buy_bitcoin_service,
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

    /// Disconnects the [LiquidSdk] instance and stops the background tasks.
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
                let refund_tx_id = self.send_swap_state_handler.refund(send_swap).await?;
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

    /// Adds an event listener to the [LiquidSdk] instance, where all [SdkEvent]'s will be emitted to.
    /// The event listener can be removed be calling [LiquidSdk::remove_event_listener].
    ///
    /// # Arguments
    ///
    /// * `listener` - The listener which is an implementation of the [EventListener] trait
    pub async fn add_event_listener(&self, listener: Box<dyn EventListener>) -> SdkResult<String> {
        Ok(self.event_manager.add(listener).await?)
    }

    /// Removes an event listener from the [LiquidSdk] instance.
    ///
    /// # Arguments
    ///
    /// * `id` - the event listener id returned by [LiquidSdk::add_event_listener]
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

    /// Get the wallet info, calculating the current pending and confirmed balances.
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

        for p in self
            .list_payments(&ListPaymentsRequest {
                ..Default::default()
            })
            .await?
        {
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
            .get_chain_pair(direction)?
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
    ) -> Result<u64, PaymentError> {
        Ok(self
            .onchain_wallet
            .build_tx(fee_rate, address, amount_sat)
            .await?
            .all_fees()
            .values()
            .sum())
    }

    async fn estimate_lockup_tx_fee(&self, amount_sat: u64) -> Result<u64, PaymentError> {
        // TODO Replace this with own address when LWK supports taproot
        //  https://github.com/Blockstream/lwk/issues/31
        let temp_p2tr_addr = match self.config.network {
            LiquidNetwork::Mainnet => "lq1pqvzxvqhrf54dd4sny4cag7497pe38252qefk46t92frs7us8r80ja9ha8r5me09nn22m4tmdqp5p4wafq3s59cql3v9n45t5trwtxrmxfsyxjnstkctj",
            LiquidNetwork::Testnet => "tlq1pq0wqu32e2xacxeyps22x8gjre4qk3u6r70pj4r62hzczxeyz8x3yxucrpn79zy28plc4x37aaf33kwt6dz2nn6gtkya6h02mwpzy4eh69zzexq7cf5y5"
        };

        self.estimate_onchain_tx_fee(
            amount_sat,
            temp_p2tr_addr,
            self.config.lowball_fee_rate_msat_per_vbyte(),
        )
        .await
    }

    /// Prepares to pay a Lightning invoice via a submarine swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [PrepareSendRequest] containing:
    ///     * `invoice` - the bolt11 Lightning invoice to pay
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
                self.estimate_onchain_tx_fee(receiver_amount_sat, &lbtc_address, None)
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

    fn ensure_send_is_not_self_transfer(&self, invoice: &str) -> Result<(), PaymentError> {
        match self.persister.fetch_receive_swap_by_invoice(invoice)? {
            None => Ok(()),
            Some(_) => Err(PaymentError::SelfTransferNotSupported),
        }
    }

    /// Pays a Lightning invoice via a submarine swap.
    ///
    /// Depending on [Config]'s `payment_timeout_sec`, this function will return:
    /// * [PaymentState::Pending] payment - if the payment could be initiated but didn't yet
    ///   complete in this time
    /// * [PaymentState::Complete] payment - if the payment was successfully completed in this time
    ///
    /// # Arguments
    ///
    /// * `req` - The [PrepareSendResponse] from calling [LiquidSdk::prepare_send_payment]
    ///
    /// # Errors
    ///
    /// * [PaymentError::PaymentTimeout] - if the payment could not be initiated in this time
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
            timestamp: Some(utils::now()),
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
                let webhook = self.persister.get_webhook_url()?.map(|url| Webhook {
                    url,
                    hash_swap_id: Some(true),
                    status: Some(vec![
                        SubSwapStates::InvoiceFailedToPay,
                        SubSwapStates::SwapExpired,
                        SubSwapStates::TransactionClaimPending,
                        SubSwapStates::TransactionLockupFailed,
                    ]),
                });
                let create_response = self.swapper.create_send_swap(CreateSubmarineRequest {
                    from: "L-BTC".to_string(),
                    to: "BTC".to_string(),
                    invoice: req.invoice.to_string(),
                    refund_public_key,
                    pair_hash: Some(lbtc_pair.hash),
                    referral_id: None,
                    webhook,
                })?;

                let swap_id = &create_response.id;
                let create_response_json =
                    SendSwap::from_boltz_struct_to_json(&create_response, swap_id)?;
                let description = get_invoice_description!(req.invoice);

                let payer_amount_sat = req.fees_sat + receiver_amount_sat;
                let swap = SendSwap {
                    id: swap_id.clone(),
                    invoice: req.invoice.clone(),
                    description,
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

    /// Fetch the current payment limits for [LiquidSdk::send_payment] and [LiquidSdk::receive_payment].
    pub async fn fetch_lightning_limits(
        &self,
    ) -> Result<LightningPaymentLimitsResponse, PaymentError> {
        self.ensure_is_started().await?;

        let submarine_pair = self
            .swapper
            .get_submarine_pairs()?
            .ok_or(PaymentError::PairsNotFound)?;
        let send_limits = submarine_pair.limits;

        let reverse_pair = self
            .swapper
            .get_reverse_swap_pairs()?
            .ok_or(PaymentError::PairsNotFound)?;
        let receive_limits = reverse_pair.limits;

        Ok(LightningPaymentLimitsResponse {
            send: Limits {
                min_sat: send_limits.minimal,
                max_sat: send_limits.maximal,
                max_zero_conf_sat: send_limits.maximal_zero_conf,
            },
            receive: Limits {
                min_sat: receive_limits.minimal,
                max_sat: receive_limits.maximal,
                max_zero_conf_sat: self.config.zero_conf_max_amount_sat(),
            },
        })
    }

    /// Fetch the current payment limits for [LiquidSdk::pay_onchain] and [LiquidSdk::receive_onchain].
    pub async fn fetch_onchain_limits(&self) -> Result<OnchainPaymentLimitsResponse, PaymentError> {
        self.ensure_is_started().await?;

        let (pair_outgoing, pair_incoming) = self.swapper.get_chain_pairs()?;
        let send_limits = pair_outgoing
            .ok_or(PaymentError::PairsNotFound)
            .map(|pair| pair.limits)?;
        let receive_limits = pair_incoming
            .ok_or(PaymentError::PairsNotFound)
            .map(|pair| pair.limits)?;

        Ok(OnchainPaymentLimitsResponse {
            send: Limits {
                min_sat: send_limits.minimal,
                max_sat: send_limits.maximal,
                max_zero_conf_sat: send_limits.maximal_zero_conf,
            },
            receive: Limits {
                min_sat: receive_limits.minimal,
                max_sat: receive_limits.maximal,
                max_zero_conf_sat: receive_limits.maximal_zero_conf,
            },
        })
    }

    /// Prepares to pay to a Bitcoin address via a chain swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [PreparePayOnchainRequest] containing:
    ///     * `receiver_amount_sat` - the amount in satoshi that will be received
    ///     * `sat_per_vbyte` - the optional fee rate of the Bitcoin claim transaction. Defaults to the swapper estimated claim fee
    pub async fn prepare_pay_onchain(
        &self,
        req: &PreparePayOnchainRequest,
    ) -> Result<PreparePayOnchainResponse, PaymentError> {
        self.ensure_is_started().await?;

        let receiver_amount_sat = req.receiver_amount_sat;
        let pair = self.validate_chain_pairs(Direction::Outgoing, receiver_amount_sat)?;
        let claim_fees_sat = match req.sat_per_vbyte {
            Some(sat_per_vbyte) => ESTIMATED_BTC_CLAIM_TX_VSIZE * sat_per_vbyte as u64,
            None => pair.fees.claim_estimate(),
        };
        let server_fees_sat = pair.fees.server();
        let server_lockup_amount_sat = receiver_amount_sat + claim_fees_sat;
        let lockup_fees_sat = self
            .estimate_lockup_tx_fee(server_lockup_amount_sat)
            .await?;

        let res = PreparePayOnchainResponse {
            receiver_amount_sat,
            claim_fees_sat,
            total_fees_sat: pair.fees.boltz(server_lockup_amount_sat)
                + lockup_fees_sat
                + claim_fees_sat
                + server_fees_sat,
        };

        let payer_amount_sat = res.receiver_amount_sat + res.total_fees_sat;
        ensure_sdk!(
            payer_amount_sat <= self.get_info().await?.balance_sat,
            PaymentError::InsufficientFunds
        );

        Ok(res)
    }

    /// Pays to a Bitcoin address via a chain swap.
    ///
    /// Depending on [Config]'s `payment_timeout_sec`, this function will return:
    /// * [PaymentState::Pending] payment - if the payment could be initiated but didn't yet
    ///   complete in this time
    /// * [PaymentState::Complete] payment - if the payment was successfully completed in this time
    ///
    /// # Arguments
    ///
    /// * `req` - the [PayOnchainRequest] containing:
    ///     * `address` - the Bitcoin address to pay to
    ///     * `prepare_res` - the [PreparePayOnchainResponse] from calling [LiquidSdk::prepare_pay_onchain]
    ///
    /// # Errors
    ///
    /// * [PaymentError::PaymentTimeout] - if the payment could not be initiated in this time
    pub async fn pay_onchain(
        &self,
        req: &PayOnchainRequest,
    ) -> Result<SendPaymentResponse, PaymentError> {
        self.ensure_is_started().await?;

        let receiver_amount_sat = req.prepare_res.receiver_amount_sat;
        let pair = self.validate_chain_pairs(Direction::Outgoing, receiver_amount_sat)?;
        let claim_fees_sat = req.prepare_res.claim_fees_sat;
        let server_fees_sat = pair.fees.server();
        let server_lockup_amount_sat = receiver_amount_sat + claim_fees_sat;
        let lockup_fees_sat = self
            .estimate_lockup_tx_fee(server_lockup_amount_sat)
            .await?;

        ensure_sdk!(
            req.prepare_res.total_fees_sat
                == pair.fees.boltz(server_lockup_amount_sat)
                    + lockup_fees_sat
                    + claim_fees_sat
                    + server_fees_sat,
            PaymentError::InvalidOrExpiredFees
        );

        let payer_amount_sat = req.prepare_res.total_fees_sat + receiver_amount_sat;
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
        let webhook = self.persister.get_webhook_url()?.map(|url| Webhook {
            url,
            hash_swap_id: Some(true),
            status: Some(vec![
                ChainSwapStates::TransactionFailed,
                ChainSwapStates::TransactionLockupFailed,
                ChainSwapStates::TransactionServerConfirmed,
            ]),
        });
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
            webhook,
        })?;

        let swap_id = &create_response.id;
        let create_response_json = ChainSwap::from_boltz_struct_to_json(&create_response, swap_id)?;

        let accept_zero_conf = server_lockup_amount_sat <= pair.limits.maximal_zero_conf;
        let payer_amount_sat = req.prepare_res.total_fees_sat + receiver_amount_sat;
        let claim_address = req.address.clone();

        let swap = ChainSwap {
            id: swap_id.clone(),
            direction: Direction::Outgoing,
            claim_address,
            lockup_address: create_response.lockup_details.lockup_address,
            timeout_block_height: create_response.lockup_details.timeout_block_height,
            preimage: preimage_str,
            description: Some("Bitcoin transfer".to_string()),
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
                        debug!("Timeout occurred without payment, set swap to timed out");
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

    /// Prepares to receive a Lightning payment via a reverse submarine swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [PrepareReceivePaymentRequest] containing:
    ///     * `payer_amount_sat` - the amount in satoshis to be paid by the payer
    pub async fn prepare_receive_payment(
        &self,
        req: &PrepareReceivePaymentRequest,
    ) -> Result<PrepareReceivePaymentResponse, PaymentError> {
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

        Ok(PrepareReceivePaymentResponse {
            payer_amount_sat,
            fees_sat,
        })
    }

    /// Receive a Lightning payment via a reverse submarine swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [ReceivePaymentRequest] containing:
    ///     * `description` - the optional payment description
    ///     * `prepare_res` - the [PrepareReceivePaymentResponse] from calling [LiquidSdk::prepare_receive_payment]
    ///
    /// # Returns
    ///
    /// * A [ReceivePaymentResponse] containing:
    ///     * `invoice` - the bolt11 Lightning invoice that should be paid
    pub async fn receive_payment(
        &self,
        req: &ReceivePaymentRequest,
    ) -> Result<ReceivePaymentResponse, PaymentError> {
        self.ensure_is_started().await?;

        let payer_amount_sat = req.prepare_res.payer_amount_sat;
        let fees_sat = req.prepare_res.fees_sat;
        let receiver_amount_sat = payer_amount_sat - fees_sat;

        let reverse_pair = self
            .swapper
            .get_reverse_swap_pairs()?
            .ok_or(PaymentError::PairsNotFound)?;
        let new_fees_sat = reverse_pair.fees.total(payer_amount_sat);
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

        let webhook_claim_status =
            match receiver_amount_sat > self.config.zero_conf_max_amount_sat() {
                true => RevSwapStates::TransactionConfirmed,
                false => RevSwapStates::TransactionMempool,
            };
        let webhook = self.persister.get_webhook_url()?.map(|url| Webhook {
            url,
            hash_swap_id: Some(true),
            status: Some(vec![webhook_claim_status]),
        });

        let v2_req = CreateReverseRequest {
            invoice_amount: payer_amount_sat as u32, // TODO update our model
            from: "BTC".to_string(),
            to: "L-BTC".to_string(),
            preimage_hash: preimage.sha256,
            claim_public_key: keypair.public_key().into(),
            description: req.description.clone(),
            address: Some(mrh_addr_str.clone()),
            address_signature: Some(mrh_addr_hash_sig.to_hex()),
            referral_id: None,
            webhook,
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
        let expected_bip21_amount_sat = payer_amount_sat - fees_sat;
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
        let description = match invoice.description() {
            Bolt11InvoiceDescription::Direct(msg) => Some(msg.to_string()),
            Bolt11InvoiceDescription::Hash(_) => None,
        };
        self.persister
            .insert_receive_swap(&ReceiveSwap {
                id: swap_id.clone(),
                preimage: preimage_str,
                create_response_json,
                claim_private_key: keypair.display_secret().to_string(),
                invoice: invoice.to_string(),
                description,
                payer_amount_sat,
                receiver_amount_sat,
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

    /// Prepares to receive from a Bitcoin transaction via a chain swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [PrepareReceiveOnchainRequest] containing:
    ///     * `payer_amount_sat` - the amount in satoshi that will be paid by the payer
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

    async fn create_chain_swap(
        &self,
        payer_amount_sat: u64,
        fees_sat: u64,
    ) -> Result<ChainSwap, PaymentError> {
        let pair = self.validate_chain_pairs(Direction::Incoming, payer_amount_sat)?;
        let claim_fees_sat = pair.fees.claim_estimate();
        let server_fees_sat = pair.fees.server();

        ensure_sdk!(
            fees_sat == pair.fees.boltz(payer_amount_sat) + claim_fees_sat + server_fees_sat,
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
        let webhook = self.persister.get_webhook_url()?.map(|url| Webhook {
            url,
            hash_swap_id: Some(true),
            status: Some(vec![
                ChainSwapStates::TransactionFailed,
                ChainSwapStates::TransactionLockupFailed,
                ChainSwapStates::TransactionServerConfirmed,
            ]),
        });
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
            webhook,
        })?;

        let swap_id = create_response.id.clone();
        let create_response_json =
            ChainSwap::from_boltz_struct_to_json(&create_response, &swap_id)?;

        let accept_zero_conf = payer_amount_sat <= pair.limits.maximal_zero_conf;
        let receiver_amount_sat = payer_amount_sat - fees_sat;
        let claim_address = self.onchain_wallet.next_unused_address().await?.to_string();

        let swap = ChainSwap {
            id: swap_id.clone(),
            direction: Direction::Incoming,
            claim_address,
            lockup_address: create_response.lockup_details.lockup_address,
            timeout_block_height: create_response.lockup_details.timeout_block_height,
            preimage: preimage_str,
            description: Some("Bitcoin transfer".to_string()),
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
        Ok(swap)
    }

    /// Receive from a Bitcoin transaction via a chain swap.
    ///
    /// # Arguments
    ///
    /// * `req` - The [PrepareReceiveOnchainResponse] from calling [LiquidSdk::prepare_receive_onchain]
    ///
    /// # Returns
    ///
    /// * A [ReceiveOnchainResponse] containing:
    ///     * `address` - the Bitcoin address the payer should pay to
    ///     * `bip21` - the BIP-21 URI scheme to pay to. See <https://github.com/bitcoin/bips/blob/master/bip-0021.mediawiki>
    pub async fn receive_onchain(
        &self,
        req: &PrepareReceiveOnchainResponse,
    ) -> Result<ReceiveOnchainResponse, PaymentError> {
        self.ensure_is_started().await?;

        let swap = self
            .create_chain_swap(req.payer_amount_sat, req.fees_sat)
            .await?;
        let create_response = swap.get_boltz_create_response()?;
        let address = create_response.lockup_details.lockup_address;

        let amount = create_response.lockup_details.amount as f64 / 100_000_000.0;
        let bip21 = create_response.lockup_details.bip21.unwrap_or(format!(
            "bitcoin:{address}?amount={amount}&label=Send%20to%20L-BTC%20address"
        ));

        Ok(ReceiveOnchainResponse { address, bip21 })
    }

    /// List all failed chain swaps that need to be refunded.
    /// They can be refunded by calling [LiquidSdk::prepare_refund] then [LiquidSdk::refund].
    pub async fn list_refundables(&self) -> SdkResult<Vec<RefundableSwap>> {
        Ok(self
            .persister
            .list_refundable_chain_swaps()?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    /// Prepares to refund a failed chain swap by calculating the refund transaction size and absolute fee.
    ///
    /// # Arguments
    ///
    /// * `req` - the [PrepareRefundRequest] containing:
    ///     * `swap_address` - the swap address to refund from [RefundableSwap::swap_address]
    ///     * `refund_address` - the Bitcoin address to refund to
    ///     * `sat_per_vbyte` - the fee rate at which to broadcast the refund transaction
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

    /// Refund a failed chain swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [RefundRequest] containing:
    ///     * `swap_address` - the swap address to refund from [RefundableSwap::swap_address]
    ///     * `refund_address` - the Bitcoin address to refund to
    ///     * `sat_per_vbyte` - the fee rate at which to broadcast the refund transaction
    pub async fn refund(&self, req: &RefundRequest) -> Result<RefundResponse, PaymentError> {
        let refund_tx_id = self
            .chain_swap_state_handler
            .refund_incoming_swap(&req.swap_address, &req.refund_address, req.sat_per_vbyte)
            .await?;
        Ok(RefundResponse { refund_tx_id })
    }

    /// Rescans all expired chain swaps created from calling [LiquidSdk::receive_onchain] within
    /// the monitoring period to check if there are any confirmed funds available to refund.
    pub async fn rescan_onchain_swaps(&self) -> SdkResult<()> {
        self.chain_swap_state_handler
            .rescan_incoming_chain_swaps()
            .await?;
        Ok(())
    }

    /// Prepares to buy Bitcoin via a chain swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [PrepareBuyBitcoinRequest] containing:
    ///     * `provider` - the [BuyBitcoinProvider] to use
    ///     * `amount_sat` - the amount in satoshis to buy from the provider
    pub async fn prepare_buy_bitcoin(
        &self,
        req: &PrepareBuyBitcoinRequest,
    ) -> Result<PrepareBuyBitcoinResponse, PaymentError> {
        if self.config.network != LiquidNetwork::Mainnet {
            return Err(PaymentError::Generic {
                err: "Can only buy bitcoin on Mainnet".to_string(),
            });
        }

        let res = self
            .prepare_receive_onchain(&PrepareReceiveOnchainRequest {
                payer_amount_sat: req.amount_sat,
            })
            .await?;
        Ok(PrepareBuyBitcoinResponse {
            provider: req.provider,
            amount_sat: res.payer_amount_sat,
            fees_sat: res.fees_sat,
        })
    }

    /// Generate a URL to a third party provider used to buy Bitcoin via a chain swap.
    ///
    /// # Arguments
    ///
    /// * `req` - the [BuyBitcoinRequest] containing:
    ///     * `prepare_res` - the [PrepareBuyBitcoinResponse] from calling [LiquidSdk::prepare_buy_bitcoin]
    ///     * `redirect_url` - the optional redirect URL the provider should redirect to after purchase
    pub async fn buy_bitcoin(&self, req: &BuyBitcoinRequest) -> Result<String, PaymentError> {
        let swap = self
            .create_chain_swap(req.prepare_res.amount_sat, req.prepare_res.fees_sat)
            .await?;

        Ok(self
            .buy_bitcoin_service
            .buy_bitcoin(req.prepare_res.provider, &swap, req.redirect_url.clone())
            .await?)
    }

    /// This method fetches the chain tx data (onchain and mempool) using LWK. For every wallet tx,
    /// it inserts or updates a corresponding entry in our Payments table.
    async fn sync_payments_with_chain_data(&self, with_scan: bool) -> Result<()> {
        let payments_before_sync: HashMap<String, Payment> = self
            .list_payments(&ListPaymentsRequest::default())
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

    /// Lists the SDK payments in reverse chronological order, from newest to oldest.
    /// The payments are determined based on onchain transactions and swaps.
    pub async fn list_payments(
        &self,
        req: &ListPaymentsRequest,
    ) -> Result<Vec<Payment>, PaymentError> {
        self.ensure_is_started().await?;

        Ok(self.persister.get_payments(req)?)
    }

    /// Empties the Liquid Wallet cache for the [Config::network].
    pub fn empty_wallet_cache(&self) -> Result<()> {
        let mut path = PathBuf::from(self.config.working_dir.clone());
        path.push(Into::<ElementsNetwork>::into(self.config.network).as_str());
        path.push("enc_cache");

        fs::remove_dir_all(&path)?;
        fs::create_dir_all(path)?;

        Ok(())
    }

    /// Synchronizes the local state with the mempool and onchain data.
    pub async fn sync(&self) -> SdkResult<()> {
        self.ensure_is_started().await?;

        let t0 = Instant::now();
        let is_first_sync = !self
            .persister
            .get_is_first_sync_complete()?
            .unwrap_or(false);
        match is_first_sync {
            true => {
                self.event_manager.pause_notifications();
                self.sync_payments_with_chain_data(true).await?;
                self.event_manager.resume_notifications();
                self.persister.set_is_first_sync_complete(true)?;
            }
            false => {
                self.sync_payments_with_chain_data(true).await?;
            }
        }
        let duration_ms = Instant::now().duration_since(t0).as_millis();
        info!("Synchronized with mempool and onchain data (t = {duration_ms} ms)");

        self.notify_event_listeners(SdkEvent::Synced).await?;
        Ok(())
    }

    /// Backup the local state to the provided backup path.
    ///
    /// # Arguments
    ///
    /// * `req` - the [BackupRequest] containing:
    ///     * `backup_path` - the optional backup path. Defaults to [Config::working_dir]
    pub fn backup(&self, req: BackupRequest) -> Result<()> {
        let backup_path = req
            .backup_path
            .map(PathBuf::from)
            .unwrap_or(self.persister.get_default_backup_path());
        self.persister.backup(backup_path)
    }

    /// Restores the local state from the provided backup path.
    ///
    /// # Arguments
    ///
    /// * `req` - the [RestoreRequest] containing:
    ///     * `backup_path` - the optional backup path. Defaults to [Config::working_dir]
    pub fn restore(&self, req: RestoreRequest) -> Result<()> {
        let backup_path = req
            .backup_path
            .map(PathBuf::from)
            .unwrap_or(self.persister.get_default_backup_path());
        ensure_sdk!(
            backup_path.exists(),
            SdkError::Generic {
                err: "Backup file does not exist".to_string()
            }
            .into()
        );
        self.persister.restore_from_backup(backup_path)
    }

    /// Second step of LNURL-pay. The first step is [parse], which also validates the LNURL destination
    /// and generates the [LnUrlPayRequest] payload needed here.
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
            req.validate_success_action_url,
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

    /// Second step of LNURL-withdraw. The first step is [parse], which also validates the LNURL destination
    /// and generates the [LnUrlWithdrawRequest] payload needed here.
    ///
    /// This call will validate the given `amount_msat` against the parameters
    /// of the LNURL endpoint (`data`). If they match the endpoint requirements, the LNURL withdraw
    /// request is made. A successful result here means the endpoint started the payment.
    pub async fn lnurl_withdraw(
        &self,
        req: LnUrlWithdrawRequest,
    ) -> Result<LnUrlWithdrawResult, LnUrlWithdrawError> {
        let prepare_res = self
            .prepare_receive_payment(&{
                PrepareReceivePaymentRequest {
                    payer_amount_sat: req.amount_msat / 1_000,
                }
            })
            .await?;
        let receive_res = self
            .receive_payment(&ReceivePaymentRequest {
                prepare_res,
                description: None,
            })
            .await?;
        let invoice = parse_invoice(&receive_res.invoice)?;

        let res = validate_lnurl_withdraw(req.data, invoice).await?;
        Ok(res)
    }

    /// Third and last step of LNURL-auth. The first step is [parse], which also validates the LNURL destination
    /// and generates the [LnUrlAuthRequestData] payload needed here. The second step is user approval of auth action.
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

    /// Register for webhook callbacks at the given `webhook_url`. Each created swap after registering the
    /// webhook will include the `webhook_url`.
    ///
    /// This method should be called every time the application is started and when the `webhook_url` changes.
    /// For example, if the `webhook_url` contains a push notification token and the token changes after
    /// the application was started, then this method should be called to register for callbacks at
    /// the new correct `webhook_url`. To unregister a webhook call [LiquidSdk::unregister_webhook].
    pub async fn register_webhook(&self, webhook_url: String) -> SdkResult<()> {
        info!("Registering for webhook notifications");
        self.persister.set_webhook_url(webhook_url)?;
        Ok(())
    }

    /// Unregister webhook callbacks. Each swap already created will continue to use the registered
    /// `webhook_url` until complete.
    ///
    /// This can be called when callbacks are no longer needed or the `webhook_url`
    /// has changed such that it needs unregistering. For example, the token is valid but the locale changes.
    /// To register a webhook call [LiquidSdk::register_webhook].
    pub async fn unregister_webhook(&self) -> SdkResult<()> {
        info!("Unregistering for webhook notifications");
        self.persister.remove_webhook_url()?;
        Ok(())
    }

    /// Fetch live rates of fiat currencies, sorted by name.
    pub async fn fetch_fiat_rates(&self) -> Result<Vec<Rate>, SdkError> {
        self.fiat_api.fetch_fiat_rates().await.map_err(Into::into)
    }

    /// List all supported fiat currencies for which there is a known exchange rate.
    /// List is sorted by the canonical name of the currency.
    pub async fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>, SdkError> {
        self.fiat_api
            .list_fiat_currencies()
            .await
            .map_err(Into::into)
    }

    /// Get the recommended BTC fees based on the configured mempool.space instance.
    pub async fn recommended_fees(&self) -> Result<RecommendedFees, SdkError> {
        Ok(self
            .bitcoin_chain_service
            .lock()
            .await
            .recommended_fees()
            .await?)
    }

    /// Get the full default [Config] for specific [LiquidNetwork].
    pub fn default_config(network: LiquidNetwork) -> Config {
        match network {
            LiquidNetwork::Mainnet => Config::mainnet(),
            LiquidNetwork::Testnet => Config::testnet(),
        }
    }

    /// Parses a string into an [InputType]. See [input_parser::parse].
    pub async fn parse(input: &str) -> Result<InputType, PaymentError> {
        parse(input)
            .await
            .map_err(|e| PaymentError::Generic { err: e.to_string() })
    }

    /// Parses a string into an [LNInvoice]. See [invoice::parse_invoice].
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

#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc};

    use anyhow::{anyhow, Result};
    use boltz_client::{
        boltz::{self, SwapUpdateTxDetails},
        swaps::boltz::{ChainSwapStates, RevSwapStates, SubSwapStates},
    };
    use lwk_wollet::{elements::Txid, hashes::hex::DisplayHex};
    use tokio::sync::Mutex;

    use crate::{
        model::{Direction, PaymentState, Swap},
        sdk::LiquidSdk,
        test_utils::{
            chain::{MockBitcoinChainService, MockHistory, MockLiquidChainService},
            chain_swap::{new_chain_swap, TEST_BITCOIN_TX},
            persist::{new_persister, new_receive_swap, new_send_swap},
            sdk::{new_liquid_sdk, new_liquid_sdk_with_chain_services},
            status_stream::MockStatusStream,
            swapper::MockSwapper,
            wallet::TEST_LIQUID_TX,
        },
    };
    use paste::paste;

    struct NewSwapArgs {
        direction: Direction,
        accepts_zero_conf: bool,
        initial_payment_state: Option<PaymentState>,
        user_lockup_tx_id: Option<String>,
    }

    impl Default for NewSwapArgs {
        fn default() -> Self {
            Self {
                accepts_zero_conf: false,
                initial_payment_state: None,
                direction: Direction::Outgoing,
                user_lockup_tx_id: None,
            }
        }
    }

    impl NewSwapArgs {
        pub fn set_direction(mut self, direction: Direction) -> Self {
            self.direction = direction;
            self
        }

        pub fn set_accepts_zero_conf(mut self, accepts_zero_conf: bool) -> Self {
            self.accepts_zero_conf = accepts_zero_conf;
            self
        }

        pub fn set_user_lockup_tx_id(mut self, user_lockup_tx_id: Option<String>) -> Self {
            self.user_lockup_tx_id = user_lockup_tx_id;
            self
        }

        pub fn set_initial_payment_state(mut self, payment_state: PaymentState) -> Self {
            self.initial_payment_state = Some(payment_state);
            self
        }
    }

    macro_rules! trigger_swap_update {
        (
            $type:literal,
            $args:expr,
            $persister:expr,
            $status_stream:expr,
            $status:expr,
            $transaction:expr,
            $zero_conf_rejected:expr
        ) => {{
            let swap = match $type {
                "chain" => {
                    let swap = new_chain_swap(
                        $args.direction,
                        $args.initial_payment_state,
                        $args.accepts_zero_conf,
                        $args.user_lockup_tx_id,
                    );
                    $persister.insert_chain_swap(&swap).unwrap();
                    Swap::Chain(swap)
                }
                "send" => {
                    let swap = new_send_swap($args.initial_payment_state);
                    $persister.insert_send_swap(&swap).unwrap();
                    Swap::Send(swap)
                }
                "receive" => {
                    let swap = new_receive_swap($args.initial_payment_state);
                    $persister.insert_receive_swap(&swap).unwrap();
                    Swap::Receive(swap)
                }
                _ => panic!(),
            };

            $status_stream
                .clone()
                .send_mock_update(boltz::Update {
                    id: swap.id(),
                    status: $status.to_string(),
                    transaction: $transaction,
                    zero_conf_rejected: $zero_conf_rejected,
                })
                .await
                .unwrap();

            paste! {
                $persister.[<fetch _ $type _swap_by_id>](&swap.id())
                    .unwrap()
                    .ok_or(anyhow!("Could not retrieve {} swap", $type))
                    .unwrap()
            }
        }};
    }

    #[tokio::test]
    async fn test_receive_swap_update_tracking() -> Result<()> {
        let (_tmp_dir, persister) = new_persister()?;
        let persister = Arc::new(persister);
        let swapper = Arc::new(MockSwapper::default());
        let status_stream = Arc::new(MockStatusStream::new());

        let sdk = Arc::new(new_liquid_sdk(
            persister.clone(),
            swapper.clone(),
            status_stream.clone(),
        )?);

        LiquidSdk::track_swap_updates(&sdk).await;

        // We spawn a new thread since updates can only be sent when called via async runtimes
        tokio::spawn(async move {
            // Verify the swap becomes invalid after final states are received
            let unrecoverable_states: [RevSwapStates; 4] = [
                RevSwapStates::SwapExpired,
                RevSwapStates::InvoiceExpired,
                RevSwapStates::TransactionFailed,
                RevSwapStates::TransactionRefunded,
            ];

            for status in unrecoverable_states {
                let persisted_swap = trigger_swap_update!(
                    "receive",
                    NewSwapArgs::default(),
                    persister,
                    status_stream,
                    status,
                    None,
                    None
                );
                assert_eq!(persisted_swap.state, PaymentState::Failed);
            }

            // Check that `TransactionMempool` and `TransactionConfirmed` correctly trigger the claim,
            // which in turn sets the `claim_tx_id`
            for status in [
                RevSwapStates::TransactionMempool,
                RevSwapStates::TransactionConfirmed,
            ] {
                let mock_tx = TEST_LIQUID_TX.clone();
                let persisted_swap = trigger_swap_update!(
                    "receive",
                    NewSwapArgs::default(),
                    persister,
                    status_stream,
                    status,
                    Some(SwapUpdateTxDetails {
                        id: mock_tx.txid().to_string(),
                        hex: lwk_wollet::elements::encode::serialize(&mock_tx)
                            .to_lower_hex_string(),
                    }),
                    None
                );
                assert!(persisted_swap.claim_tx_id.is_some());
            }
        })
        .await
        .unwrap();

        Ok(())
    }

    #[tokio::test]
    async fn test_send_swap_update_tracking() -> Result<()> {
        let (_tmp_dir, persister) = new_persister()?;
        let persister = Arc::new(persister);
        let swapper = Arc::new(MockSwapper::default());
        let status_stream = Arc::new(MockStatusStream::new());

        let sdk = Arc::new(new_liquid_sdk(
            persister.clone(),
            swapper.clone(),
            status_stream.clone(),
        )?);

        LiquidSdk::track_swap_updates(&sdk).await;

        // We spawn a new thread since updates can only be sent when called via async runtimes
        tokio::spawn(async move {
            // Verify the swap becomes invalid after final states are received
            let unrecoverable_states: [SubSwapStates; 3] = [
                SubSwapStates::TransactionLockupFailed,
                SubSwapStates::InvoiceFailedToPay,
                SubSwapStates::SwapExpired,
            ];

            for status in unrecoverable_states {
                let persisted_swap = trigger_swap_update!(
                    "send",
                    NewSwapArgs::default(),
                    persister,
                    status_stream,
                    status,
                    None,
                    None
                );
                assert_eq!(persisted_swap.state, PaymentState::Failed);
            }

            // Verify that `InvoiceSet` correctly sets the state to `Pending` and
            // assigns the `lockup_tx_id` to the payment
            let persisted_swap = trigger_swap_update!(
                "send",
                NewSwapArgs::default(),
                persister,
                status_stream,
                SubSwapStates::InvoiceSet,
                None,
                None
            );
            assert_eq!(persisted_swap.state, PaymentState::Pending);
            assert!(persisted_swap.lockup_tx_id.is_some());

            // Verify that `TransactionClaimPending` correctly sets the state to `Complete`
            // and stores the preimage
            let persisted_swap = trigger_swap_update!(
                "send",
                NewSwapArgs::default(),
                persister,
                status_stream,
                SubSwapStates::TransactionClaimPending,
                None,
                None
            );
            assert_eq!(persisted_swap.state, PaymentState::Complete);
            assert!(persisted_swap.preimage.is_some());
        })
        .await
        .unwrap();

        Ok(())
    }

    #[tokio::test]
    async fn test_chain_swap_update_tracking() -> Result<()> {
        let (_tmp_dir, persister) = new_persister()?;
        let persister = Arc::new(persister);
        let swapper = Arc::new(MockSwapper::default());
        let status_stream = Arc::new(MockStatusStream::new());
        let liquid_chain_service = Arc::new(Mutex::new(MockLiquidChainService::new()));
        let bitcoin_chain_service = Arc::new(Mutex::new(MockBitcoinChainService::new()));

        let sdk = Arc::new(new_liquid_sdk_with_chain_services(
            persister.clone(),
            swapper.clone(),
            status_stream.clone(),
            liquid_chain_service.clone(),
            bitcoin_chain_service.clone(),
        )?);

        LiquidSdk::track_swap_updates(&sdk).await;

        // We spawn a new thread since updates can only be sent when called via async runtimes
        tokio::spawn(async move {
            let trigger_failed: [ChainSwapStates; 3] = [
                ChainSwapStates::TransactionFailed,
                ChainSwapStates::SwapExpired,
                ChainSwapStates::TransactionRefunded,
            ];

            // Checks that work for both incoming and outgoing chain swaps
            for direction in [Direction::Incoming, Direction::Outgoing] {
                // Verify the swap becomes invalid after final states are received
                for status in &trigger_failed {
                    let persisted_swap = trigger_swap_update!(
                        "chain",
                        NewSwapArgs::default().set_direction(direction),
                        persister,
                        status_stream,
                        status,
                        None,
                        None
                    );
                    assert_eq!(persisted_swap.state, PaymentState::Failed);
                }

                let (mock_tx_hex, mock_tx_id) = match direction {
                    Direction::Incoming => {
                        let tx = TEST_LIQUID_TX.clone();
                        (
                            lwk_wollet::elements::encode::serialize(&tx).to_lower_hex_string(),
                            tx.txid().to_string(),
                        )
                    }
                    Direction::Outgoing => {
                        let tx = TEST_BITCOIN_TX.clone();
                        (
                            sdk_common::bitcoin::consensus::serialize(&tx).to_lower_hex_string(),
                            tx.txid().to_string(),
                        )
                    }
                };

                // Verify that `TransactionLockupFailed` correctly sets the state as
                // `RefundPending`/`Refundable` or as `Failed` depending on whether or not
                // `user_lockup_tx_id` is present
                for user_lockup_tx_id in &[None, Some(mock_tx_id.clone())] {
                    if let Some(user_lockup_tx_id) = user_lockup_tx_id {
                        match direction {
                            Direction::Incoming => {
                                bitcoin_chain_service
                                    .lock()
                                    .await
                                    .set_history(vec![MockHistory {
                                        txid: Txid::from_str(&user_lockup_tx_id).unwrap(),
                                        height: 0,
                                        block_hash: None,
                                        block_timestamp: None,
                                    }]);
                            }
                            Direction::Outgoing => {
                                liquid_chain_service
                                    .lock()
                                    .await
                                    .set_history(vec![MockHistory {
                                        txid: Txid::from_str(&user_lockup_tx_id).unwrap(),
                                        height: 0,
                                        block_hash: None,
                                        block_timestamp: None,
                                    }]);
                            }
                        }
                    }
                    let persisted_swap = trigger_swap_update!(
                        "chain",
                        NewSwapArgs::default()
                            .set_direction(direction)
                            .set_initial_payment_state(PaymentState::Pending)
                            .set_user_lockup_tx_id(user_lockup_tx_id.clone()),
                        persister,
                        status_stream,
                        ChainSwapStates::TransactionLockupFailed,
                        None,
                        None
                    );
                    let expected_state = if user_lockup_tx_id.is_some() {
                        match direction {
                            Direction::Incoming => PaymentState::Refundable,
                            Direction::Outgoing => PaymentState::RefundPending,
                        }
                    } else {
                        PaymentState::Failed
                    };
                    assert_eq!(persisted_swap.state, expected_state);
                }

                // Verify that `TransactionMempool` and `TransactionConfirmed` correctly set
                // `user_lockup_tx_id` and `accept_zero_conf`
                for status in [
                    ChainSwapStates::TransactionMempool,
                    ChainSwapStates::TransactionConfirmed,
                ] {
                    let persisted_swap = trigger_swap_update!(
                        "chain",
                        NewSwapArgs::default().set_direction(direction),
                        persister,
                        status_stream,
                        status,
                        Some(SwapUpdateTxDetails {
                            id: mock_tx_id.clone(),
                            hex: mock_tx_hex.clone(),
                        }), // sets `update.transaction`
                        Some(true) // sets `update.zero_conf_rejected`
                    );
                    assert_eq!(persisted_swap.user_lockup_tx_id, Some(mock_tx_id.clone()));
                    assert_eq!(persisted_swap.accept_zero_conf, false);
                }

                // Verify that `TransactionServerMempool` correctly:
                // 1. Sets the payment as `Pending` and creates `server_lockup_tx_id` when
                //    `accepts_zero_conf` is false
                // 2. Sets the payment as `Complete` and creates `claim_tx_id` when `accepts_zero_conf`
                //    is true
                for accepts_zero_conf in [false, true] {
                    let persisted_swap = trigger_swap_update!(
                        "chain",
                        NewSwapArgs::default()
                            .set_direction(direction)
                            .set_accepts_zero_conf(accepts_zero_conf),
                        persister,
                        status_stream,
                        ChainSwapStates::TransactionServerMempool,
                        Some(SwapUpdateTxDetails {
                            id: mock_tx_id.clone(),
                            hex: mock_tx_hex.clone(),
                        }),
                        None
                    );
                    match accepts_zero_conf {
                        false => {
                            assert_eq!(persisted_swap.state, PaymentState::Pending);
                            assert!(persisted_swap.server_lockup_tx_id.is_some());
                        }
                        true => {
                            assert_eq!(persisted_swap.state, PaymentState::Complete);
                            assert!(persisted_swap.claim_tx_id.is_some());
                        }
                    };
                }

                // Verify that `TransactionServerConfirmed` correctly
                // sets the payment as `Complete` and creates `claim_tx_id`
                let persisted_swap = trigger_swap_update!(
                    "chain",
                    NewSwapArgs::default().set_direction(direction),
                    persister,
                    status_stream,
                    ChainSwapStates::TransactionServerConfirmed,
                    Some(SwapUpdateTxDetails {
                        id: mock_tx_id,
                        hex: mock_tx_hex,
                    }),
                    None
                );
                assert_eq!(persisted_swap.state, PaymentState::Complete);
                assert!(persisted_swap.claim_tx_id.is_some());
            }

            // For outgoing payments, verify that `Created` correctly sets the payment as `Pending` and creates
            // the `user_lockup_tx_id`
            let persisted_swap = trigger_swap_update!(
                "chain",
                NewSwapArgs::default().set_direction(Direction::Outgoing),
                persister,
                status_stream,
                ChainSwapStates::Created,
                None,
                None
            );
            assert_eq!(persisted_swap.state, PaymentState::Pending);
            assert!(persisted_swap.user_lockup_tx_id.is_some());
        })
        .await
        .unwrap();

        Ok(())
    }
}
