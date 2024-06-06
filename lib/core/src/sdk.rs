use anyhow::{anyhow, Result};
use async_trait::async_trait;
use boltz_client::lightning_invoice::Bolt11InvoiceDescription;
use boltz_client::swaps::boltzv2;
use boltz_client::ToHex;
use boltz_client::{
    swaps::{boltz::RevSwapStates, boltzv2::*},
    util::secrets::Preimage,
    Amount, Bolt11Invoice,
};
use log::{debug, error, info, warn};
use lwk_wollet::{elements::LockTime, ElementsNetwork};
use std::time::Instant;
use std::{
    fs,
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::{Duration, UNIX_EPOCH},
};
use tokio::sync::{watch, RwLock};
use tokio::time::MissedTickBehavior;

use crate::chain::{ChainService, MempoolSpace};
use crate::error::LiquidSdkError;
use crate::model::PaymentState::*;
use crate::send_swap::SendSwapStateHandler;
use crate::swapper::{BoltzSwapper, ReconnectHandler, Swapper, SwapperStatusStream};
use crate::wallet::{LiquidOnchainWallet, OnchainWallet};
use crate::{
    ensure_sdk,
    error::{LiquidSdkResult, PaymentError},
    event::EventManager,
    get_invoice_amount,
    model::*,
    persist::Persister,
    utils,
};

/// Claim tx feerate, in sats per vbyte.
/// Since the  Liquid blocks are consistently empty for now, we hardcode the minimum feerate.
pub const LIQUID_CLAIM_TX_FEERATE_MSAT: f32 = 100.0;

pub const DEFAULT_DATA_DIR: &str = ".data";

pub struct LiquidSdk {
    config: Config,
    onchain_wallet: Arc<dyn OnchainWallet>,
    chain_service: Arc<dyn ChainService>,
    persister: Arc<Persister>,
    event_manager: Arc<EventManager>,
    status_stream: Arc<dyn SwapperStatusStream>,
    swapper: Arc<dyn Swapper>,
    is_started: RwLock<bool>,
    shutdown_sender: watch::Sender<()>,
    shutdown_receiver: watch::Receiver<()>,
}

impl LiquidSdk {
    pub async fn connect(req: ConnectRequest) -> Result<Arc<LiquidSdk>> {
        let config = req.config;
        let sdk = LiquidSdk::new(config, req.mnemonic)?;
        sdk.start().await?;

        Ok(sdk)
    }

    fn new(config: Config, mnemonic: String) -> Result<Arc<Self>> {
        fs::create_dir_all(&config.working_dir)?;

        let persister = Arc::new(Persister::new(&config.working_dir, config.network)?);
        persister.init()?;

        let event_manager = Arc::new(EventManager::new());
        let (shutdown_sender, shutdown_receiver) = watch::channel::<()>(());

        let swapper = Arc::new(BoltzSwapper::new(config.clone()));
        let status_stream = Arc::<dyn SwapperStatusStream>::from(swapper.create_status_stream());

        let chain_service = MempoolSpace::new(config.network);

        let sdk = Arc::new(LiquidSdk {
            config: config.clone(),
            onchain_wallet: Arc::new(LiquidOnchainWallet::new(mnemonic, config)?),
            chain_service: Arc::new(chain_service),
            persister: persister.clone(),
            event_manager,
            status_stream: status_stream.clone(),
            swapper,
            is_started: RwLock::new(false),
            shutdown_sender,
            shutdown_receiver,
        });

        Ok(sdk)
    }

    /// Starts an SDK instance.
    ///
    /// Internal method. Should only be called once per instance.
    /// Should only be called as part of [LiquidSdk::connect].
    async fn start(self: &Arc<LiquidSdk>) -> LiquidSdkResult<()> {
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
    async fn start_background_tasks(self: &Arc<LiquidSdk>) -> LiquidSdkResult<()> {
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
        self.track_swap_updates().await;
        self.track_refundable_swaps().await;

        Ok(())
    }

    async fn ensure_is_started(&self) -> LiquidSdkResult<()> {
        let is_started = self.is_started.read().await;
        ensure_sdk!(*is_started, LiquidSdkError::NotStarted);
        Ok(())
    }

    /// Trigger the stopping of background threads for this SDK instance.
    pub async fn disconnect(&self) -> LiquidSdkResult<()> {
        self.ensure_is_started().await?;

        let mut is_started = self.is_started.write().await;
        self.shutdown_sender
            .send(())
            .map_err(|e| LiquidSdkError::Generic {
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
            let send_swap_state_handler = SendSwapStateHandler::new(
                cloned.config.clone(),
                cloned.onchain_wallet.clone(),
                cloned.persister.clone(),
                cloned.swapper.clone(),
            );
            let mut swap_state_changes = send_swap_state_handler.subscribe_payment_updates();
            loop {
                tokio::select! {
                    payment_id = swap_state_changes.recv() => {
                      match payment_id {
                          Ok(payment_id) => {
                            if let Err(e) = cloned.emit_payment_updated(Some(payment_id)).await {
                              error!("Failed to emit payment update: {e:?}");
                            }
                          }
                          Err(e) => error!("Failed to receive send swap state change: {e:?}")
                      }
                    }
                    update = updates_stream.recv() => match update {
                        Ok(boltzv2::Update { id, status }) => {
                            let _ = cloned.sync().await;
                            match cloned.persister.fetch_send_swap_by_id(&id) {
                                Ok(Some(_)) => {
                                    match send_swap_state_handler.on_new_status(&status, &id).await {
                                        Ok(_) => info!("Succesfully handled Send Swap {id} update"),
                                        Err(e) => error!("Failed to handle Send Swap {id} update: {e}")
                                    }
                                }
                                _ => {
                                    match cloned.persister.fetch_receive_swap(&id) {
                                        Ok(Some(_)) => {
                                            match cloned.try_handle_receive_swap_boltz_status(&status, &id).await {
                                                Ok(_) => info!("Succesfully handled Receive Swap {id} update"),
                                                Err(e) => error!("Failed to handle Receive Swap {id} update: {e}")
                                            }
                                        }
                                        _ => {
                                            error!("Could not find Swap {id}");
                                        }
                                    }
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

    async fn track_refundable_swaps(self: &Arc<LiquidSdk>) {
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
                    },
                    _ = shutdown_receiver.changed() => {
                        info!("Received shutdown signal, exiting refundable swaps loop");
                        return;
                    }
                }
            }
        });
    }

    async fn check_send_swap_expiration(&self, send_swap: &SendSwap) -> Result<()> {
        if send_swap.lockup_tx_id.is_some() && send_swap.refund_tx_id.is_none() {
            let swap_script = send_swap.get_swap_script()?;
            let current_height = self.onchain_wallet.tip().await.height();
            let locktime_from_height = LockTime::from_height(current_height)?;

            info!("Checking Send Swap {} expiration: locktime_from_height = {locktime_from_height:?},  swap_script.locktime = {:?}", send_swap.id, swap_script.locktime);
            if utils::is_locktime_expired(locktime_from_height, swap_script.locktime) {
                let id = &send_swap.id;
                let refund_tx_id = self.try_refund(send_swap).await?;
                info!("Broadcast refund tx for Send Swap {id}. Tx id: {refund_tx_id}");
                let send_swap_state_handler = SendSwapStateHandler::new(
                    self.config.clone(),
                    self.onchain_wallet.clone(),
                    self.persister.clone(),
                    self.swapper.clone(),
                );
                send_swap_state_handler
                    .update_swap_info(id, Pending, None, None, Some(&refund_tx_id))
                    .await?;
            }
        }
        Ok(())
    }

    async fn notify_event_listeners(&self, e: LiquidSdkEvent) -> Result<()> {
        self.event_manager.notify(e).await;
        Ok(())
    }

    pub async fn add_event_listener(
        &self,
        listener: Box<dyn EventListener>,
    ) -> LiquidSdkResult<String> {
        Ok(self.event_manager.add(listener).await?)
    }

    pub async fn remove_event_listener(&self, id: String) -> LiquidSdkResult<()> {
        self.event_manager.remove(id).await;
        Ok(())
    }

    fn validate_state_transition(
        from_state: PaymentState,
        to_state: PaymentState,
    ) -> Result<(), PaymentError> {
        match (from_state, to_state) {
            (_, Created) => Err(PaymentError::Generic {
                err: "Cannot transition to Created state".to_string(),
            }),

            (Created | Pending, Pending) => Ok(()),
            (Complete | Failed | TimedOut, Pending) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Pending state"),
            }),

            (Created | Pending, Complete) => Ok(()),
            (Complete | Failed | TimedOut, Complete) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Complete state"),
            }),

            (Created, TimedOut) => Ok(()),
            (_, TimedOut) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to TimedOut state"),
            }),

            (_, Failed) => Ok(()),
        }
    }

    /// Transitions a Receive swap to a new state
    pub(crate) async fn try_handle_receive_swap_update(
        &self,
        swap_id: &str,
        to_state: PaymentState,
        claim_tx_id: Option<&str>,
    ) -> Result<(), PaymentError> {
        info!(
            "Transitioning Receive swap {swap_id} to {to_state:?} (claim_tx_id = {claim_tx_id:?})"
        );

        let swap = self
            .persister
            .fetch_receive_swap(swap_id)
            .map_err(|_| PaymentError::PersistError)?
            .ok_or(PaymentError::Generic {
                err: format!("Receive Swap not found {swap_id}"),
            })?;
        let payment_id = claim_tx_id.map(|c| c.to_string()).or(swap.claim_tx_id);

        Self::validate_state_transition(swap.state, to_state)?;
        self.persister
            .try_handle_receive_swap_update(swap_id, to_state, claim_tx_id)?;

        Ok(self.emit_payment_updated(payment_id).await?)
    }

    async fn emit_payment_updated(&self, payment_id: Option<String>) -> Result<()> {
        if let Some(id) = payment_id {
            match self.persister.get_payment(id.clone())? {
                Some(payment) => {
                    match payment.status {
                        Complete => {
                            self.notify_event_listeners(LiquidSdkEvent::PaymentSucceed {
                                details: payment,
                            })
                            .await?
                        }
                        Pending => {
                            // The swap state has changed to Pending
                            match payment.swap_id.clone() {
                                Some(swap_id) => match payment.payment_type {
                                    PaymentType::Receive => {
                                        match self.persister.fetch_receive_swap(&swap_id)? {
                                            Some(swap) => match swap.claim_tx_id {
                                                Some(_) => {
                                                    // The claim tx has now been broadcast
                                                    self.notify_event_listeners(
                                                        LiquidSdkEvent::PaymentWaitingConfirmation {
                                                            details: payment,
                                                        },
                                                    )
                                                    .await?
                                                }
                                                None => {
                                                    // The lockup tx is in the mempool/confirmed
                                                    self.notify_event_listeners(
                                                        LiquidSdkEvent::PaymentPending {
                                                            details: payment,
                                                        },
                                                    )
                                                    .await?
                                                }
                                            },
                                            None => debug!("Swap not found: {swap_id}"),
                                        }
                                    }
                                    PaymentType::Send => {
                                        match self.persister.fetch_send_swap_by_id(&swap_id)? {
                                            Some(swap) => match swap.refund_tx_id {
                                                Some(_) => {
                                                    // The refund tx has now been broadcast
                                                    self.notify_event_listeners(
                                                        LiquidSdkEvent::PaymentRefundPending {
                                                            details: payment,
                                                        },
                                                    )
                                                    .await?
                                                }
                                                None => {
                                                    // The lockup tx is in the mempool/confirmed
                                                    self.notify_event_listeners(
                                                        LiquidSdkEvent::PaymentPending {
                                                            details: payment,
                                                        },
                                                    )
                                                    .await?
                                                }
                                            },
                                            None => debug!("Swap not found: {swap_id}"),
                                        }
                                    }
                                },
                                None => debug!("Payment has no swap id"),
                            }
                        }
                        Failed => match payment.payment_type {
                            PaymentType::Receive => {
                                self.notify_event_listeners(LiquidSdkEvent::PaymentFailed {
                                    details: payment,
                                })
                                .await?
                            }
                            PaymentType::Send => {
                                // The refund tx is confirmed
                                self.notify_event_listeners(LiquidSdkEvent::PaymentRefunded {
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

    /// Handles status updates from Boltz for Receive swaps
    pub(crate) async fn try_handle_receive_swap_boltz_status(
        &self,
        swap_state: &str,
        id: &str,
    ) -> Result<()> {
        let receive_swap = self
            .persister
            .fetch_receive_swap(id)?
            .ok_or(anyhow!("No ongoing Receive Swap found for ID {id}"))?;

        info!("Handling Receive Swap transition to {swap_state:?} for swap {id}");

        match RevSwapStates::from_str(swap_state) {
            Ok(RevSwapStates::SwapExpired
            | RevSwapStates::InvoiceExpired
            | RevSwapStates::TransactionFailed
            | RevSwapStates::TransactionRefunded) => {
                error!("Swap {id} entered into an unrecoverable state: {swap_state:?}");
                self.try_handle_receive_swap_update(id, Failed, None).await?;
                Ok(())
            }

            // The lockup tx is in the mempool and we accept 0-conf => try to claim
            // TODO Add 0-conf preconditions check: https://github.com/breez/breez-liquid-sdk/issues/187
            Ok(RevSwapStates::TransactionMempool
            // The lockup tx is confirmed => try to claim
            | RevSwapStates::TransactionConfirmed) => {
                match receive_swap.claim_tx_id {
                    Some(claim_tx_id) => {
                        warn!("Claim tx for Receive Swap {id} was already broadcast: txid {claim_tx_id}")
                    }
                    None => {
                        self.try_handle_receive_swap_update(&receive_swap.id, Pending, None)
                            .await?;
                        match self.try_claim(&receive_swap).await {
                            Ok(_) => {}
                            Err(err) => match err {
                                PaymentError::AlreadyClaimed => warn!("Funds already claimed for Receive Swap {id}"),
                                _ => error!("Claim for Receive Swap {id} failed: {err}")
                            }
                        }
                    }
                }
                Ok(())
            }

            Ok(_) => {
                debug!("Unhandled state for Receive Swap {id}: {swap_state}");
                Ok(())
            },

            _ => Err(anyhow!("Invalid RevSwapState for Receive Swap {id}: {swap_state}")),
        }
    }

    pub async fn get_info(&self, req: GetInfoRequest) -> Result<GetInfoResponse> {
        self.ensure_is_started().await?;
        debug!(
            "next_unused_address: {}",
            self.onchain_wallet.next_unused_address().await?
        );
        if req.with_scan {
            self.sync().await?;
        }

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
                    TimedOut => {}
                },
                PaymentType::Receive => match p.status {
                    Complete => confirmed_received_sat += p.amount_sat,
                    Failed => {}
                    _ => pending_receive_sat += p.amount_sat,
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
        let invoice = invoice
            .trim()
            .parse::<Bolt11Invoice>()
            .map_err(|_| PaymentError::InvalidInvoice)?;

        match (invoice.network().to_string().as_str(), self.config.network) {
            ("bitcoin", Network::Mainnet) => {}
            ("testnet", Network::Testnet) => {}
            _ => return Err(PaymentError::InvalidInvoice),
        }

        ensure_sdk!(!invoice.is_expired(), PaymentError::InvalidInvoice);

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

    /// Estimate the onchain fee for sending the given amount to the given destination address
    async fn estimate_onchain_tx_fee(&self, amount_sat: u64, address: &str) -> Result<u64> {
        Ok(self
            .onchain_wallet
            .build_tx(None, address, amount_sat)
            .await?
            .all_fees()
            .values()
            .sum())
    }

    async fn estimate_lockup_tx_fee(&self, amount_sat: u64) -> Result<u64> {
        // TODO Replace this with own address when LWK supports taproot
        //  https://github.com/Blockstream/lwk/issues/31
        let temp_p2tr_addr = match self.config.network {
            Network::Mainnet => "lq1pqvzxvqhrf54dd4sny4cag7497pe38252qefk46t92frs7us8r80ja9ha8r5me09nn22m4tmdqp5p4wafq3s59cql3v9n45t5trwtxrmxfsyxjnstkctj",
            Network::Testnet => "tlq1pq0wqu32e2xacxeyps22x8gjre4qk3u6r70pj4r62hzczxeyz8x3yxucrpn79zy28plc4x37aaf33kwt6dz2nn6gtkya6h02mwpzy4eh69zzexq7cf5y5"
        };

        self.estimate_onchain_tx_fee(amount_sat, temp_p2tr_addr)
            .await
    }

    pub async fn prepare_send_payment(
        &self,
        req: &PrepareSendRequest,
    ) -> Result<PrepareSendResponse, PaymentError> {
        self.ensure_is_started().await?;

        let invoice = self.validate_invoice(&req.invoice)?;
        let receiver_amount_sat = invoice
            .amount_milli_satoshis()
            .ok_or(PaymentError::AmountOutOfRange)?
            / 1000;

        let lbtc_pair = self.validate_submarine_pairs(receiver_amount_sat)?;

        let lockup_fees_sat = self.estimate_lockup_tx_fee(receiver_amount_sat).await?;

        Ok(PrepareSendResponse {
            invoice: req.invoice.clone(),
            fees_sat: lbtc_pair.fees.total(receiver_amount_sat) + lockup_fees_sat,
        })
    }

    async fn try_refund_non_cooperative(
        &self,
        swap: &SendSwap,
        broadcast_fees_sat: Amount,
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

    async fn try_refund(&self, swap: &SendSwap) -> Result<String, PaymentError> {
        let amount_sat = get_invoice_amount!(swap.invoice);
        let output_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let refund_tx_fees_sat = Amount::from_sat(
            self.estimate_onchain_tx_fee(amount_sat, &output_address)
                .await?,
        );
        let refund_res =
            self.swapper
                .refund_send_swap_cooperative(swap, &output_address, refund_tx_fees_sat);
        match refund_res {
            Ok(res) => Ok(res),
            Err(e) => {
                warn!("Cooperative refund failed: {:?}", e);
                self.try_refund_non_cooperative(swap, refund_tx_fees_sat)
                    .await
            }
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

        self.validate_invoice(&req.invoice)?;
        let receiver_amount_sat = get_invoice_amount!(req.invoice);
        let lbtc_pair = self.validate_submarine_pairs(receiver_amount_sat)?;
        let lockup_tx_fees_sat = self.estimate_lockup_tx_fee(receiver_amount_sat).await?;
        ensure_sdk!(
            req.fees_sat == lbtc_pair.fees.total(receiver_amount_sat) + lockup_tx_fees_sat,
            PaymentError::InvalidOrExpiredFees
        );

        let swap = match self.persister.fetch_send_swap_by_invoice(&req.invoice)? {
            Some(swap) => swap,
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
        self.wait_for_payment(swap.id, accept_zero_conf)
            .await
            .map(|payment| SendPaymentResponse { payment })
    }

    async fn wait_for_payment(
        &self,
        swap_id: String,
        accept_zero_conf: bool,
    ) -> Result<Payment, PaymentError> {
        let timeout_fut = tokio::time::sleep(Duration::from_secs(self.config.payment_timeout_sec));
        tokio::pin!(timeout_fut);

        let mut events_stream = self.event_manager.subscribe();
        let mut maybe_payment: Option<Payment> = None;
        let send_swap_state_handler = SendSwapStateHandler::new(
            self.config.clone(),
            self.onchain_wallet.clone(),
            self.persister.clone(),
            self.swapper.clone(),
        );

        loop {
            tokio::select! {
                _ = &mut timeout_fut => match maybe_payment {
                    Some(payment) => return Ok(payment),
                    None => {
                        debug!("Timeout occured without payment, set swap to timed out");
                        send_swap_state_handler.update_swap_info(&swap_id, TimedOut, None, None, None).await?;
                        return Err(PaymentError::PaymentTimeout)
                    },
                },
                event = events_stream.recv() => match event {
                    Ok(LiquidSdkEvent::PaymentPending { details }) => match details.swap_id.clone() {
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
                    Ok(LiquidSdkEvent::PaymentSucceed { details }) => match details.swap_id.clone()
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

    async fn try_claim(&self, ongoing_receive_swap: &ReceiveSwap) -> Result<(), PaymentError> {
        ensure_sdk!(
            ongoing_receive_swap.claim_tx_id.is_none(),
            PaymentError::AlreadyClaimed
        );
        let swap_id = &ongoing_receive_swap.id;
        let claim_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let claim_tx_id = self
            .swapper
            .claim_receive_swap(ongoing_receive_swap, claim_address)?;

        // We insert a pseudo-claim-tx in case LWK fails to pick up the new mempool tx for a while
        // This makes the tx known to the SDK (get_info, list_payments) instantly
        self.persister.insert_or_update_payment(PaymentTxData {
            tx_id: claim_tx_id.clone(),
            timestamp: None,
            amount_sat: ongoing_receive_swap.receiver_amount_sat,
            payment_type: PaymentType::Receive,
            is_confirmed: false,
        })?;

        self.try_handle_receive_swap_update(swap_id, Pending, Some(&claim_tx_id))
            .await?;

        Ok(())
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

        let v2_req = CreateReverseRequest {
            invoice_amount: req.payer_amount_sat as u32, // TODO update our model
            from: "BTC".to_string(),
            to: "L-BTC".to_string(),
            preimage_hash: preimage.sha256,
            claim_public_key: keypair.public_key().into(),
            address: None,
            address_signature: None,
            referral_id: None,
        };
        let create_response = self.swapper.create_receive_swap(v2_req)?;

        let swap_id = create_response.id.clone();
        let invoice = Bolt11Invoice::from_str(&create_response.invoice)
            .map_err(|_| PaymentError::InvalidInvoice)?;
        let payer_amount_sat = invoice
            .amount_milli_satoshis()
            .ok_or(PaymentError::InvalidInvoice)?
            / 1000;

        // Double check that the generated invoice includes our data
        // https://docs.boltz.exchange/v/api/dont-trust-verify#lightning-invoice-verification
        if invoice.payment_hash().to_string() != preimage_hash {
            return Err(PaymentError::InvalidInvoice);
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

    /// This method fetches the chain tx data (onchain and mempool) using LWK. For every wallet tx,
    /// it inserts or updates a corresponding entry in our Payments table.
    async fn sync_payments_with_chain_data(&self, with_scan: bool) -> Result<()> {
        if with_scan {
            self.onchain_wallet.full_scan().await?;
        }

        let pending_receive_swaps_by_claim_tx_id =
            self.persister.list_pending_receive_swaps_by_claim_tx_id()?;
        let pending_send_swaps_by_refund_tx_id =
            self.persister.list_pending_send_swaps_by_refund_tx_id()?;

        let send_swap_state_handler = SendSwapStateHandler::new(
            self.config.clone(),
            self.onchain_wallet.clone(),
            self.persister.clone(),
            self.swapper.clone(),
        );
        for tx in self.onchain_wallet.transactions().await? {
            let tx_id = tx.txid.to_string();
            let is_tx_confirmed = tx.height.is_some();
            let amount_sat = tx.balance.values().sum::<i64>();

            // Transition the swaps whose state depends on this tx being confirmed
            if is_tx_confirmed {
                if let Some(swap) = pending_receive_swaps_by_claim_tx_id.get(&tx_id) {
                    self.try_handle_receive_swap_update(&swap.id, Complete, None)
                        .await?;
                }
                if let Some(swap) = pending_send_swaps_by_refund_tx_id.get(&tx_id) {
                    send_swap_state_handler
                        .update_swap_info(&swap.id, Failed, None, None, None)
                        .await?;
                }
            }

            self.persister.insert_or_update_payment(PaymentTxData {
                tx_id,
                timestamp: tx.timestamp,
                amount_sat: amount_sat.unsigned_abs(),
                payment_type: match amount_sat >= 0 {
                    true => PaymentType::Receive,
                    false => PaymentType::Send,
                },
                is_confirmed: is_tx_confirmed,
            })?;
        }

        Ok(())
    }

    /// Lists the SDK payments. The payments are determined based on onchain transactions and swaps.
    pub async fn list_payments(&self) -> Result<Vec<Payment>, PaymentError> {
        self.ensure_is_started().await?;

        let mut payments: Vec<Payment> = self.persister.get_payments()?.values().cloned().collect();
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
    pub async fn sync(&self) -> LiquidSdkResult<()> {
        self.ensure_is_started().await?;

        let t0 = Instant::now();
        self.sync_payments_with_chain_data(true).await?;
        let duration_ms = Instant::now().duration_since(t0).as_millis();
        info!("Synchronized with mempool and onchain data (t = {duration_ms} ms)");

        self.notify_event_listeners(LiquidSdkEvent::Synced).await?;
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

    pub fn default_config(network: Network) -> Config {
        match network {
            Network::Mainnet => Config::mainnet(),
            Network::Testnet => Config::testnet(),
        }
    }

    pub fn parse_invoice(input: &str) -> Result<LNInvoice, PaymentError> {
        let input = input
            .strip_prefix("lightning:")
            .or(input.strip_prefix("LIGHTNING:"))
            .unwrap_or(input);
        let invoice = Bolt11Invoice::from_str(input).map_err(|_| PaymentError::InvalidInvoice)?;

        // Try to take payee pubkey from the tagged fields, if doesn't exist recover it from the signature
        let payee_pubkey: String = match invoice.payee_pub_key() {
            Some(key) => key.serialize().to_hex(),
            None => invoice.recover_payee_pub_key().serialize().to_hex(),
        };
        let description = match invoice.description() {
            Bolt11InvoiceDescription::Direct(msg) => Some(msg.to_string()),
            Bolt11InvoiceDescription::Hash(_) => None,
        };
        let description_hash = match invoice.description() {
            Bolt11InvoiceDescription::Direct(_) => None,
            Bolt11InvoiceDescription::Hash(h) => Some(h.0.to_string()),
        };
        let timestamp = invoice
            .timestamp()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| PaymentError::InvalidInvoice)?
            .as_secs();
        let routing_hints = invoice
            .route_hints()
            .iter()
            .map(RouteHint::from_ldk_hint)
            .collect();

        let res = LNInvoice {
            bolt11: input.to_string(),
            network: invoice.currency().try_into()?,
            payee_pubkey,
            payment_hash: invoice.payment_hash().to_hex(),
            description,
            description_hash,
            amount_msat: invoice.amount_milli_satoshis(),
            timestamp,
            expiry: invoice.expiry_time().as_secs(),
            routing_hints,
            payment_secret: invoice.payment_secret().0.to_vec(),
            min_final_cltv_expiry_delta: invoice.min_final_cltv_expiry_delta(),
        };
        Ok(res)
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
    use anyhow::Result;
    use tempdir::TempDir;

    use crate::model::*;
    use crate::sdk::LiquidSdk;

    const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    fn create_temp_dir() -> Result<(TempDir, String)> {
        let data_dir = TempDir::new(&uuid::Uuid::new_v4().to_string())?;
        let data_dir_str = data_dir
            .as_ref()
            .to_path_buf()
            .to_str()
            .expect("Expecting valid temporary path")
            .to_owned();
        Ok((data_dir, data_dir_str))
    }

    async fn list_pending(sdk: &LiquidSdk) -> Result<Vec<Payment>> {
        let payments = sdk.list_payments().await?;

        Ok(payments
            .iter()
            .filter(|p| matches!(&p.status, PaymentState::Pending))
            .cloned()
            .collect())
    }

    #[tokio::test]
    async fn normal_submarine_swap() -> Result<()> {
        let (_data_dir, data_dir_str) = create_temp_dir()?;
        let mut config = Config::testnet();
        config.working_dir = data_dir_str;
        let sdk = LiquidSdk::connect(ConnectRequest {
            mnemonic: TEST_MNEMONIC.to_string(),
            config,
        })
        .await?;

        let invoice = "lntb10u1pnqwkjrpp5j8ucv9mgww0ajk95yfpvuq0gg5825s207clrzl5thvtuzfn68h0sdqqcqzzsxqr23srzjqv8clnrfs9keq3zlg589jvzpw87cqh6rjks0f9g2t9tvuvcqgcl45f6pqqqqqfcqqyqqqqlgqqqqqqgq2qsp5jnuprlxrargr6hgnnahl28nvutj3gkmxmmssu8ztfhmmey3gq2ss9qyyssq9ejvcp6frwklf73xvskzdcuhnnw8dmxag6v44pffwqrxznsly4nqedem3p3zhn6u4ln7k79vk6zv55jjljhnac4gnvr677fyhfgn07qp4x6wrq".to_string();
        sdk.prepare_send_payment(&PrepareSendRequest { invoice })
            .await?;
        assert!(!list_pending(&sdk).await?.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn reverse_submarine_swap() -> Result<()> {
        let (_data_dir, data_dir_str) = create_temp_dir()?;
        let mut config = Config::testnet();
        config.working_dir = data_dir_str;
        let sdk = LiquidSdk::connect(ConnectRequest {
            mnemonic: TEST_MNEMONIC.to_string(),
            config,
        })
        .await?;

        let prepare_response = sdk
            .prepare_receive_payment(&PrepareReceiveRequest {
                payer_amount_sat: 1_000,
            })
            .await?;
        sdk.receive_payment(&prepare_response).await?;
        assert!(!list_pending(&sdk).await?.is_empty());

        Ok(())
    }

    #[test]
    fn reverse_submarine_swap_recovery() -> Result<()> {
        Ok(())
    }
}
