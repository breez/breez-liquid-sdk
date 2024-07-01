use std::time::Duration;
use std::{str::FromStr, sync::Arc};

use anyhow::{anyhow, Result};
use boltz_client::swaps::boltzv2::{self, SwapUpdateTxDetails};
use boltz_client::swaps::{boltz::ChainSwapStates, boltzv2::CreateChainResponse};
use boltz_client::Secp256k1;
use log::{debug, error, info, warn};
use lwk_wollet::elements::Transaction;
use tokio::sync::{broadcast, watch, Mutex};
use tokio::time::MissedTickBehavior;

use crate::chain::bitcoin::BitcoinChainService;
use crate::chain::liquid::LiquidChainService;
use crate::error::{SdkError, SdkResult};
use crate::model::PaymentState::{
    Complete, Created, Failed, Pending, RefundPending, Refundable, TimedOut,
};
use crate::model::{ChainSwap, Config, Direction, PaymentTxData, PaymentType};
use crate::sdk::CHAIN_SWAP_MONITORING_PERIOD_BITCOIN_BLOCKS;
use crate::swapper::Swapper;
use crate::wallet::OnchainWallet;
use crate::{error::PaymentError, model::PaymentState, persist::Persister};

pub(crate) struct ChainSwapStateHandler {
    config: Config,
    onchain_wallet: Arc<dyn OnchainWallet>,
    persister: Arc<Persister>,
    swapper: Arc<dyn Swapper>,
    liquid_chain_service: Arc<Mutex<dyn LiquidChainService>>,
    bitcoin_chain_service: Arc<Mutex<dyn BitcoinChainService>>,
    subscription_notifier: broadcast::Sender<String>,
}

impl ChainSwapStateHandler {
    pub(crate) fn new(
        config: Config,
        onchain_wallet: Arc<dyn OnchainWallet>,
        persister: Arc<Persister>,
        swapper: Arc<dyn Swapper>,
        liquid_chain_service: Arc<Mutex<dyn LiquidChainService>>,
        bitcoin_chain_service: Arc<Mutex<dyn BitcoinChainService>>,
    ) -> Result<Self> {
        let (subscription_notifier, _) = broadcast::channel::<String>(30);
        Ok(Self {
            config,
            onchain_wallet,
            persister,
            swapper,
            liquid_chain_service,
            bitcoin_chain_service,
            subscription_notifier,
        })
    }

    pub(crate) async fn start(self: Arc<Self>, mut shutdown: watch::Receiver<()>) {
        let cloned = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60 * 10));
            interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Err(e) = cloned.rescan_incoming_chain_swaps().await {
                            error!("Error checking chain swaps: {e:?}");
                        }
                    },
                    _ = shutdown.changed() => {
                        info!("Received shutdown signal, exiting chain swap loop");
                        return;
                    }
                }
            }
        });
    }

    pub(crate) fn subscribe_payment_updates(&self) -> broadcast::Receiver<String> {
        self.subscription_notifier.subscribe()
    }

    /// Handles status updates from Boltz for Chain swaps
    pub(crate) async fn on_new_status(&self, update: &boltzv2::Update) -> Result<()> {
        let id = &update.id;
        let swap = self
            .persister
            .fetch_chain_swap_by_id(id)?
            .ok_or(anyhow!("No ongoing Chain Swap found for ID {id}"))?;

        match swap.direction {
            Direction::Incoming => self.on_new_incoming_status(&swap, update).await,
            Direction::Outgoing => self.on_new_outgoing_status(&swap, update).await,
        }
    }

    pub(crate) async fn rescan_incoming_chain_swaps(&self) -> Result<()> {
        let current_height = self.bitcoin_chain_service.lock().await.tip()?.height as u32;
        let chain_swaps: Vec<ChainSwap> = self
            .persister
            .list_chain_swaps()?
            .into_iter()
            .filter(|s| s.direction == Direction::Incoming)
            .collect();
        info!(
            "Rescanning {} Chain Swap(s) at height {}",
            chain_swaps.len(),
            current_height
        );
        for swap in chain_swaps {
            if let Err(e) = self.rescan_incoming_chain_swap(&swap, current_height).await {
                error!("Error rescanning Chain Swap {}: {e:?}", swap.id);
            }
        }
        Ok(())
    }

    async fn rescan_incoming_chain_swap(
        &self,
        swap: &ChainSwap,
        current_height: u32,
    ) -> Result<()> {
        let monitoring_block_height =
            swap.timeout_block_height + CHAIN_SWAP_MONITORING_PERIOD_BITCOIN_BLOCKS;
        let is_swap_expired = current_height > swap.timeout_block_height;
        let is_monitoring_expired = current_height > monitoring_block_height;

        if (is_swap_expired && !is_monitoring_expired) || swap.state == RefundPending {
            let swap_script = swap.get_lockup_swap_script()?.as_bitcoin_script()?;
            let script_pubkey = swap_script
                .to_address(self.config.network.as_bitcoin_chain())
                .map_err(|e| anyhow!("Error getting script address: {e:?}"))?
                .script_pubkey();
            let script_balance = self
                .bitcoin_chain_service
                .lock()
                .await
                .script_get_balance(script_pubkey.as_script())?;
            info!(
                "Chain Swap {} has {} confirmed and {} unconfirmed sats",
                swap.id, script_balance.confirmed, script_balance.unconfirmed
            );

            if script_balance.confirmed > 0
                && script_balance.unconfirmed == 0
                && swap.state != Refundable
            {
                // If there are unspent funds sent to the lockup script address then set
                // the state to Refundable.
                info!(
                    "Chain Swap {} has {} unspent sats. Setting the swap to refundable",
                    swap.id, script_balance.confirmed
                );
                self.update_swap_info(&swap.id, Refundable, None, None, None, None)
                    .await?;
            } else if script_balance.confirmed == 0 {
                // If the funds sent to the lockup script address are spent then set the
                // state back to Complete/Failed.
                let to_state = match swap.claim_tx_id {
                    Some(_) => Complete,
                    None => Failed,
                };

                if to_state != swap.state {
                    info!(
                        "Chain Swap {} has 0 unspent sats. Setting the swap to {:?}",
                        swap.id, to_state
                    );
                    self.update_swap_info(&swap.id, to_state, None, None, None, None)
                        .await?;
                }
            }
        }
        Ok(())
    }

    async fn on_new_incoming_status(
        &self,
        swap: &ChainSwap,
        update: &boltzv2::Update,
    ) -> Result<()> {
        let id = &update.id;
        let status = &update.status;
        let swap_state = ChainSwapStates::from_str(status)
            .map_err(|_| anyhow!("Invalid ChainSwapState for Chain Swap {id}: {status}"))?;

        info!("Handling incoming Chain Swap transition to {status:?} for swap {id}");
        // See https://docs.boltz.exchange/v/api/lifecycle#chain-swaps
        match swap_state {
            // Boltz announced the user lockup tx is in the mempool or has been confirmed.
            ChainSwapStates::TransactionMempool | ChainSwapStates::TransactionConfirmed => {
                if let Some(zero_conf_rejected) = update.zero_conf_rejected {
                    info!("Is zero conf rejected for Chain Swap {id}: {zero_conf_rejected}");
                    self.persister
                        .update_chain_swap_accept_zero_conf(id, !zero_conf_rejected)?;
                }
                if let Some(transaction) = update.transaction.clone() {
                    self.update_swap_info(id, Pending, None, Some(&transaction.id), None, None)
                        .await?;
                }
                Ok(())
            }

            // Boltz announced the server lockup tx is in the mempool.
            // Verify the transaction and claim if zero-conf
            ChainSwapStates::TransactionServerMempool => {
                match swap.claim_tx_id.clone() {
                    None => {
                        let Some(transaction) = update.transaction.clone() else {
                            return Err(anyhow!("Unexpected payload from Boltz status stream"));
                        };

                        if let Err(e) = self.verify_lockup_tx(swap, &transaction, false).await {
                            warn!("Server lockup mempool transaction for incoming Chain Swap {} could not be verified. txid: {}, err: {}",
                                swap.id,
                                transaction.id,
                                e);
                            return Err(anyhow!(
                                "Could not verify transaction {}: {e}",
                                transaction.id
                            ));
                        }

                        info!("Server lockup mempool transaction was verified for incoming Chain Swap {}", swap.id);
                        self.update_swap_info(id, Pending, Some(&transaction.id), None, None, None)
                            .await?;

                        if swap.accept_zero_conf {
                            self.claim(swap).await.map_err(|e| {
                                error!("Could not cooperate Chain Swap {id} claim: {e}");
                                anyhow!("Could not post claim details. Err: {e:?}")
                            })?;
                        }
                    }
                    Some(claim_tx_id) => {
                        warn!("Claim tx for Chain Swap {id} was already broadcast: txid {claim_tx_id}")
                    }
                };
                Ok(())
            }

            // Boltz announced the server lockup tx has been confirmed.
            // Verify the transaction and claim
            ChainSwapStates::TransactionServerConfirmed => {
                match swap.claim_tx_id.clone() {
                    None => {
                        let Some(transaction) = update.transaction.clone() else {
                            return Err(anyhow!("Unexpected payload from Boltz status stream"));
                        };

                        if let Err(e) = self.verify_lockup_tx(swap, &transaction, true).await {
                            warn!("Server lockup transaction for incoming Chain Swap {} could not be verified. txid: {}, err: {}",
                                swap.id,
                                transaction.id,
                                e);
                            return Err(anyhow!(
                                "Could not verify transaction {}: {e}",
                                transaction.id
                            ));
                        }

                        info!(
                            "Server lockup transaction was verified for incoming Chain Swap {}",
                            swap.id
                        );
                        self.update_swap_info(id, Pending, Some(&transaction.id), None, None, None)
                            .await?;
                        self.claim(swap).await.map_err(|e| {
                            error!("Could not cooperate Chain Swap {id} claim: {e}");
                            anyhow!("Could not post claim details. Err: {e:?}")
                        })?;
                    }
                    Some(claim_tx_id) => {
                        warn!("Claim tx for Chain Swap {id} was already broadcast: txid {claim_tx_id}")
                    }
                };
                Ok(())
            }

            // If swap state is unrecoverable, either:
            // 1. The transaction failed
            // 2. Lockup failed (too little funds were sent)
            // 3. The claim lockup was refunded
            // 4. The swap has expired (>24h)
            // We initiate a cooperative refund, and then fallback to a regular one
            ChainSwapStates::TransactionFailed
            | ChainSwapStates::TransactionLockupFailed
            | ChainSwapStates::TransactionRefunded
            | ChainSwapStates::SwapExpired => {
                match swap.refund_tx_id.clone() {
                    None => {
                        warn!("Chain Swap {id} is in an unrecoverable state: {swap_state:?}");
                        match (swap.user_lockup_tx_id.clone(), swap_state) {
                            (Some(_), _) => {
                                info!("Chain Swap {id} user lockup tx was broadcast. Setting the swap to refundable.");
                                self.update_swap_info(id, Refundable, None, None, None, None)
                                    .await?;
                            }
                            (None, ChainSwapStates::TransactionLockupFailed) => {
                                info!("Chain Swap {id} user lockup tx was broadcast but lockup has failed. Setting the swap to refundable.");
                                self.update_swap_info(id, Refundable, None, None, None, None)
                                    .await?;
                            }
                            (None, _) => {
                                info!("Chain Swap {id} user lockup tx was never broadcast. Resolving payment as failed.");
                                self.update_swap_info(id, Failed, None, None, None, None)
                                    .await?;
                            }
                        }
                    }
                    Some(refund_tx_id) => warn!(
                        "Refund tx for Chain Swap {id} was already broadcast: txid {refund_tx_id}"
                    ),
                };
                Ok(())
            }

            _ => {
                debug!("Unhandled state for Chain Swap {id}: {swap_state:?}");
                Ok(())
            }
        }
    }

    async fn on_new_outgoing_status(
        &self,
        swap: &ChainSwap,
        update: &boltzv2::Update,
    ) -> Result<()> {
        let id = &update.id;
        let status = &update.status;
        let swap_state = ChainSwapStates::from_str(status)
            .map_err(|_| anyhow!("Invalid ChainSwapState for Chain Swap {id}: {status}"))?;

        info!("Handling outgoing Chain Swap transition to {status:?} for swap {id}");
        // See https://docs.boltz.exchange/v/api/lifecycle#chain-swaps
        match swap_state {
            // The swap is created
            ChainSwapStates::Created => {
                match (swap.state, swap.user_lockup_tx_id.clone()) {
                    // The swap timed out before receiving this status
                    (TimedOut, _) => warn!("Chain Swap {id} timed out, do not broadcast a lockup tx"),

                    // Create the user lockup tx
                    (_, None) => {
                        let create_response = swap.get_boltz_create_response()?;
                        let user_lockup_tx = self.lockup_funds(id, &create_response).await?;
                        let lockup_tx_id = user_lockup_tx.txid().to_string();
                        let lockup_tx_fees_sat: u64 = user_lockup_tx.all_fees().values().sum();

                        // We insert a pseudo-lockup-tx in case LWK fails to pick up the new mempool tx for a while
                        // This makes the tx known to the SDK (get_info, list_payments) instantly
                        self.persister.insert_or_update_payment(PaymentTxData {
                            tx_id: lockup_tx_id.clone(),
                            timestamp: None,
                            amount_sat: swap.receiver_amount_sat,
                            // This should be: boltz fee + lockup fee + claim fee
                            fees_sat: lockup_tx_fees_sat + swap.claim_fees_sat,
                            payment_type: PaymentType::Send,
                            is_confirmed: false,
                        })?;

                        self.update_swap_info(id, Pending, None, Some(&lockup_tx_id), None, None)
                            .await?;
                    },

                    // Lockup tx already exists
                    (_, Some(lockup_tx_id)) => warn!("User lockup tx for Chain Swap {id} was already broadcast: txid {lockup_tx_id}"),
                };
                Ok(())
            }

            // Boltz announced the user lockup tx is in the mempool or has been confirmed.
            ChainSwapStates::TransactionMempool | ChainSwapStates::TransactionConfirmed => {
                if let Some(zero_conf_rejected) = update.zero_conf_rejected {
                    info!("Is zero conf rejected for Chain Swap {id}: {zero_conf_rejected}");
                    self.persister
                        .update_chain_swap_accept_zero_conf(id, !zero_conf_rejected)?;
                }
                if let Some(transaction) = update.transaction.clone() {
                    self.update_swap_info(id, Pending, None, Some(&transaction.id), None, None)
                        .await?;
                }
                Ok(())
            }

            // Boltz announced the server lockup tx is in the mempool.
            // Verify the transaction and claim if zero-conf
            ChainSwapStates::TransactionServerMempool => {
                match swap.claim_tx_id.clone() {
                    None => {
                        let Some(transaction) = update.transaction.clone() else {
                            return Err(anyhow!("Unexpected payload from Boltz status stream"));
                        };

                        if let Err(e) = self.verify_lockup_tx(swap, &transaction, false).await {
                            warn!("Server lockup mempool transaction for outgoing Chain Swap {} could not be verified. txid: {}, err: {}",
                                swap.id,
                                transaction.id,
                                e);
                            return Err(anyhow!(
                                "Could not verify transaction {}: {e}",
                                transaction.id
                            ));
                        }

                        info!("Server lockup mempool transaction was verified for outgoing Chain Swap {}", swap.id);
                        self.update_swap_info(id, Pending, Some(&transaction.id), None, None, None)
                            .await?;

                        if swap.accept_zero_conf {
                            self.claim(swap).await.map_err(|e| {
                                error!("Could not cooperate Chain Swap {id} claim: {e}");
                                anyhow!("Could not post claim details. Err: {e:?}")
                            })?;
                        }
                    }
                    Some(claim_tx_id) => {
                        warn!("Claim tx for Chain Swap {id} was already broadcast: txid {claim_tx_id}")
                    }
                };
                Ok(())
            }

            // Boltz announced the server lockup tx has been confirmed.
            // Verify the transaction and claim
            ChainSwapStates::TransactionServerConfirmed => {
                match swap.claim_tx_id.clone() {
                    None => {
                        let Some(transaction) = update.transaction.clone() else {
                            return Err(anyhow!("Unexpected payload from Boltz status stream"));
                        };

                        if let Err(e) = self.verify_lockup_tx(swap, &transaction, true).await {
                            warn!("Server lockup transaction for outgoing Chain Swap {} could not be verified. txid: {}, err: {}",
                                swap.id,
                                transaction.id,
                                e);
                            return Err(anyhow!(
                                "Could not verify transaction {}: {e}",
                                transaction.id
                            ));
                        }

                        info!(
                            "Server lockup transaction was verified for outgoing Chain Swap {}",
                            swap.id
                        );
                        self.update_swap_info(id, Pending, Some(&transaction.id), None, None, None)
                            .await?;
                        self.claim(swap).await.map_err(|e| {
                            error!("Could not cooperate Chain Swap {id} claim: {e}");
                            anyhow!("Could not post claim details. Err: {e:?}")
                        })?;
                    }
                    Some(claim_tx_id) => {
                        warn!("Claim tx for Chain Swap {id} was already broadcast: txid {claim_tx_id}")
                    }
                };
                Ok(())
            }

            // If swap state is unrecoverable, either:
            // 1. The transaction failed
            // 2. Lockup failed (too little funds were sent)
            // 3. The claim lockup was refunded
            // 4. The swap has expired (>24h)
            // We initiate a cooperative refund, and then fallback to a regular one
            ChainSwapStates::TransactionFailed
            | ChainSwapStates::TransactionLockupFailed
            | ChainSwapStates::TransactionRefunded
            | ChainSwapStates::SwapExpired => {
                match swap.refund_tx_id.clone() {
                    None => {
                        warn!("Chain Swap {id} is in an unrecoverable state: {swap_state:?}");
                        match swap.user_lockup_tx_id.clone() {
                            Some(_) => {
                                warn!("Chain Swap {id} user lockup tx has been broadcast. Attempting refund.");
                                let refund_tx_id = self.refund_outgoing_swap(swap).await?;
                                info!("Broadcast refund tx for Chain Swap {id}. Tx id: {refund_tx_id}");
                                self.update_swap_info(
                                    id,
                                    RefundPending,
                                    None,
                                    None,
                                    None,
                                    Some(&refund_tx_id),
                                )
                                .await?;
                            }
                            None => {
                                warn!("Chain Swap {id} user lockup tx was never broadcast. Resolving payment as failed.");
                                self.update_swap_info(id, Failed, None, None, None, None)
                                    .await?;
                            }
                        }
                    }
                    Some(refund_tx_id) => warn!(
                        "Refund tx for Chain Swap {id} was already broadcast: txid {refund_tx_id}"
                    ),
                };
                Ok(())
            }

            _ => {
                debug!("Unhandled state for Chain Swap {id}: {swap_state:?}");
                Ok(())
            }
        }
    }

    async fn lockup_funds(
        &self,
        swap_id: &str,
        create_response: &CreateChainResponse,
    ) -> Result<Transaction, PaymentError> {
        let lockup_details = create_response.lockup_details.clone();

        debug!(
            "Initiated Chain Swap: send {} sats to liquid address {}",
            lockup_details.amount, lockup_details.lockup_address
        );

        let lockup_tx = self
            .onchain_wallet
            .build_tx(
                None,
                &lockup_details.lockup_address,
                lockup_details.amount as u64,
            )
            .await?;

        let lockup_tx_id = self
            .liquid_chain_service
            .lock()
            .await
            .broadcast(&lockup_tx, Some(swap_id))
            .await?
            .to_string();

        debug!(
          "Successfully broadcast lockup transaction for Chain Swap {swap_id}. Lockup tx id: {lockup_tx_id}"
        );
        Ok(lockup_tx)
    }

    /// Transitions a Chain swap to a new state
    pub(crate) async fn update_swap_info(
        &self,
        swap_id: &str,
        to_state: PaymentState,
        server_lockup_tx_id: Option<&str>,
        user_lockup_tx_id: Option<&str>,
        claim_tx_id: Option<&str>,
        refund_tx_id: Option<&str>,
    ) -> Result<(), PaymentError> {
        info!("Transitioning Chain swap {swap_id} to {to_state:?} (server_lockup_tx_id = {:?}, user_lockup_tx_id = {:?}, claim_tx_id = {:?}), refund_tx_id = {:?})", server_lockup_tx_id, user_lockup_tx_id, claim_tx_id, refund_tx_id);

        let swap: ChainSwap = self
            .persister
            .fetch_chain_swap_by_id(swap_id)
            .map_err(|_| PaymentError::PersistError)?
            .ok_or(PaymentError::Generic {
                err: format!("Chain Swap not found {swap_id}"),
            })?;
        let payment_id = match swap.direction {
            Direction::Incoming => claim_tx_id.map(|c| c.to_string()).or(swap.claim_tx_id),
            Direction::Outgoing => user_lockup_tx_id
                .map(|c| c.to_string())
                .or(swap.user_lockup_tx_id),
        };

        Self::validate_state_transition(swap.state, to_state)?;
        self.persister.try_handle_chain_swap_update(
            swap_id,
            to_state,
            server_lockup_tx_id,
            user_lockup_tx_id,
            claim_tx_id,
            refund_tx_id,
        )?;
        if let Some(payment_id) = payment_id {
            let _ = self.subscription_notifier.send(payment_id);
        }
        Ok(())
    }

    async fn claim(&self, chain_swap: &ChainSwap) -> Result<(), PaymentError> {
        debug!("Initiating claim for Chain Swap {}", &chain_swap.id);
        let claim_tx_id = self.swapper.claim_chain_swap(chain_swap)?;

        if chain_swap.direction == Direction::Incoming {
            // We insert a pseudo-claim-tx in case LWK fails to pick up the new mempool tx for a while
            // This makes the tx known to the SDK (get_info, list_payments) instantly
            self.persister.insert_or_update_payment(PaymentTxData {
                tx_id: claim_tx_id.clone(),
                timestamp: None,
                amount_sat: chain_swap.receiver_amount_sat,
                fees_sat: 0,
                payment_type: PaymentType::Receive,
                is_confirmed: false,
            })?;
        }

        self.update_swap_info(
            &chain_swap.id,
            Complete,
            None,
            None,
            Some(&claim_tx_id),
            None,
        )
        .await?;
        Ok(())
    }

    pub fn prepare_refund(
        &self,
        lockup_address: &str,
        output_address: &str,
        sat_per_vbyte: u32,
    ) -> SdkResult<(u32, u64, Option<String>)> {
        let swap = self
            .persister
            .fetch_chain_swap_by_lockup_address(lockup_address)?
            .ok_or(SdkError::Generic {
                err: format!("Swap {} not found", lockup_address),
            })?;
        if let Some(refund_tx_id) = swap.refund_tx_id.clone() {
            warn!(
                "A refund tx for Chain Swap {} was already broadcast: txid {refund_tx_id}",
                swap.id
            );
        }
        let (tx_vsize, tx_fee_sat) =
            self.swapper
                .prepare_chain_swap_refund(&swap, output_address, sat_per_vbyte as f32)?;
        Ok((tx_vsize, tx_fee_sat, swap.refund_tx_id))
    }

    pub(crate) async fn refund_incoming_swap(
        &self,
        lockup_address: &str,
        output_address: &str,
        sat_per_vbyte: u32,
    ) -> Result<String, PaymentError> {
        let swap = self
            .persister
            .fetch_chain_swap_by_lockup_address(lockup_address)?
            .ok_or(PaymentError::Generic {
                err: format!("Swap {} not found", lockup_address),
            })?;
        if let Some(refund_tx_id) = swap.refund_tx_id.clone() {
            warn!(
                "A refund tx for Chain Swap {} was already broadcast: txid {refund_tx_id}",
                swap.id
            );
        }

        let (_, broadcast_fees_sat) =
            self.swapper
                .prepare_chain_swap_refund(&swap, output_address, sat_per_vbyte as f32)?;
        let refund_res =
            self.swapper
                .refund_chain_swap_cooperative(&swap, output_address, broadcast_fees_sat);
        let refund_tx_id = match refund_res {
            Ok(res) => Ok(res),
            Err(e) => {
                warn!("Cooperative refund failed: {:?}", e);
                let current_height = self.bitcoin_chain_service.lock().await.tip()?.height as u32;
                self.swapper.refund_chain_swap_non_cooperative(
                    &swap,
                    broadcast_fees_sat,
                    output_address,
                    current_height,
                )
            }
        }?;

        info!(
            "Broadcast refund tx for Chain Swap {}. Tx id: {refund_tx_id}",
            swap.id
        );
        self.update_swap_info(
            &swap.id,
            RefundPending,
            None,
            None,
            None,
            Some(&refund_tx_id),
        )
        .await?;
        Ok(refund_tx_id)
    }

    pub(crate) async fn refund_outgoing_swap(
        &self,
        swap: &ChainSwap,
    ) -> Result<String, PaymentError> {
        match swap.refund_tx_id.clone() {
            Some(refund_tx_id) => Err(PaymentError::Generic {
                err: format!(
                    "Refund tx for Chain Swap {} was already broadcast: txid {refund_tx_id}",
                    swap.id
                ),
            }),
            None => {
                let output_address = self.onchain_wallet.next_unused_address().await?.to_string();
                let (_, broadcast_fees_sat) =
                    self.swapper
                        .prepare_chain_swap_refund(swap, &output_address, 0.1)?;
                let refund_res = self.swapper.refund_chain_swap_cooperative(
                    swap,
                    &output_address,
                    broadcast_fees_sat,
                );
                let refund_tx_id = match refund_res {
                    Ok(res) => Ok(res),
                    Err(e) => {
                        warn!("Cooperative refund failed: {:?}", e);
                        let current_height = self.liquid_chain_service.lock().await.tip().await?;
                        self.swapper.refund_chain_swap_non_cooperative(
                            swap,
                            broadcast_fees_sat,
                            &output_address,
                            current_height,
                        )
                    }
                }?;

                info!(
                    "Broadcast refund tx for Chain Swap {}. Tx id: {refund_tx_id}",
                    swap.id
                );
                self.update_swap_info(
                    &swap.id,
                    RefundPending,
                    None,
                    None,
                    None,
                    Some(&refund_tx_id),
                )
                .await?;
                Ok(refund_tx_id)
            }
        }
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
            (_, Pending) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Pending state"),
            }),

            (Created | Pending | RefundPending, Complete) => Ok(()),
            (_, Complete) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Complete state"),
            }),

            (Created, TimedOut) => Ok(()),
            (_, TimedOut) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to TimedOut state"),
            }),

            (Created | Pending | RefundPending | Failed | Complete, Refundable) => Ok(()),
            (_, Refundable) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Refundable state"),
            }),

            (Pending | Refundable, RefundPending) => Ok(()),
            (_, RefundPending) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to RefundPending state"),
            }),

            (Complete, Failed) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Failed state"),
            }),

            (_, Failed) => Ok(()),
        }
    }

    async fn verify_lockup_tx(
        &self,
        chain_swap: &ChainSwap,
        swap_update_tx: &SwapUpdateTxDetails,
        verify_confirmation: bool,
    ) -> Result<()> {
        match chain_swap.direction {
            Direction::Incoming => {
                self.verify_incoming_lockup_tx(chain_swap, swap_update_tx, verify_confirmation)
                    .await
            }
            Direction::Outgoing => {
                self.verify_outgoing_lockup_tx(chain_swap, swap_update_tx, verify_confirmation)
                    .await
            }
        }
    }

    async fn verify_incoming_lockup_tx(
        &self,
        chain_swap: &ChainSwap,
        swap_update_tx: &SwapUpdateTxDetails,
        verify_confirmation: bool,
    ) -> Result<()> {
        let swap_script = chain_swap.get_claim_swap_script()?;
        let claim_details = chain_swap.get_boltz_create_response()?.claim_details;
        // Verify transaction
        let liquid_swap_script = swap_script.as_liquid_script()?;
        let address = liquid_swap_script
            .to_address(self.config.network.into())
            .map_err(|e| anyhow!("Failed to get swap script address {e:?}"))?;
        let tx = self
            .liquid_chain_service
            .lock()
            .await
            .verify_tx(
                &address,
                &swap_update_tx.id,
                &swap_update_tx.hex,
                verify_confirmation,
            )
            .await?;
        // Verify RBF
        let rbf_explicit = tx.input.iter().any(|tx_in| tx_in.sequence.is_rbf());
        if !verify_confirmation && rbf_explicit {
            return Err(anyhow!("Transaction signals RBF"));
        }
        // Verify amount
        let secp = Secp256k1::new();
        let to_address_output = tx
            .output
            .iter()
            .filter(|tx_out| tx_out.script_pubkey == address.script_pubkey());
        let mut value = 0;
        for tx_out in to_address_output {
            value += tx_out
                .unblind(&secp, liquid_swap_script.blinding_key.secret_key())?
                .value;
        }
        if value < claim_details.amount as u64 {
            return Err(anyhow!(
                "Transaction value {value} sats is less than {} sats",
                claim_details.amount
            ));
        }
        Ok(())
    }

    async fn verify_outgoing_lockup_tx(
        &self,
        chain_swap: &ChainSwap,
        swap_update_tx: &SwapUpdateTxDetails,
        verify_confirmation: bool,
    ) -> Result<()> {
        let swap_script = chain_swap.get_claim_swap_script()?;
        let claim_details = chain_swap.get_boltz_create_response()?.claim_details;
        // Verify transaction
        let address = swap_script
            .as_bitcoin_script()?
            .to_address(self.config.network.as_bitcoin_chain())
            .map_err(|e| anyhow!("Failed to get swap script address {e:?}"))?;
        let tx = self
            .bitcoin_chain_service
            .lock()
            .await
            .verify_tx(
                &address,
                &swap_update_tx.id,
                &swap_update_tx.hex,
                verify_confirmation,
            )
            .await?;
        // Verify RBF
        let rbf_explicit = tx.input.iter().any(|input| input.sequence.is_rbf());
        if !verify_confirmation && rbf_explicit {
            return Err(anyhow!("Transaction signals RBF"));
        }
        // Verify amount
        let value: u64 = tx
            .output
            .iter()
            .filter(|tx_out| tx_out.script_pubkey == address.script_pubkey())
            .map(|tx_out| tx_out.value.to_sat())
            .sum();
        if value < claim_details.amount as u64 {
            return Err(anyhow!(
                "Transaction value {value} sats is less than {} sats",
                claim_details.amount
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        sync::Arc,
    };

    use anyhow::Result;

    use crate::{
        model::{
            Direction,
            PaymentState::{self, *},
        },
        test_utils::{
            chain_swap::{new_chain_swap, new_chain_swap_state_handler},
            persist::new_persister,
        },
    };

    #[tokio::test]
    async fn test_chain_swap_state_transitions() -> Result<()> {
        let (_temp_dir, storage) = new_persister()?;
        let storage = Arc::new(storage);

        let chain_swap_state_handler = new_chain_swap_state_handler(storage.clone())?;

        // Test valid combinations of states
        let all_states = HashSet::from([Created, Pending, Complete, TimedOut, Failed]);
        let valid_combinations = HashMap::from([
            (
                Created,
                HashSet::from([Pending, Complete, TimedOut, Refundable, Failed]),
            ),
            (
                Pending,
                HashSet::from([Pending, Complete, Refundable, RefundPending, Failed]),
            ),
            (TimedOut, HashSet::from([Failed])),
            (Complete, HashSet::from([Refundable])),
            (Refundable, HashSet::from([RefundPending, Failed])),
            (RefundPending, HashSet::from([Refundable, Complete, Failed])),
            (Failed, HashSet::from([Failed, Refundable])),
        ]);

        for (first_state, allowed_states) in valid_combinations.iter() {
            for allowed_state in allowed_states {
                let chain_swap = new_chain_swap(Direction::Incoming, Some(*first_state));
                storage.insert_chain_swap(&chain_swap)?;

                assert!(chain_swap_state_handler
                    .update_swap_info(&chain_swap.id, *allowed_state, None, None, None, None)
                    .await
                    .is_ok());
            }
        }

        // Test invalid combinations of states
        let invalid_combinations: HashMap<PaymentState, HashSet<PaymentState>> = valid_combinations
            .iter()
            .map(|(first_state, allowed_states)| {
                (
                    *first_state,
                    all_states.difference(allowed_states).cloned().collect(),
                )
            })
            .collect();

        for (first_state, disallowed_states) in invalid_combinations.iter() {
            for disallowed_state in disallowed_states {
                let chain_swap = new_chain_swap(Direction::Incoming, Some(*first_state));
                storage.insert_chain_swap(&chain_swap)?;

                assert!(chain_swap_state_handler
                    .update_swap_info(&chain_swap.id, *disallowed_state, None, None, None, None)
                    .await
                    .is_err());
            }
        }

        Ok(())
    }
}
