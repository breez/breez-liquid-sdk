use std::{str::FromStr, sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use boltz_client::swaps::boltz;
use boltz_client::swaps::{boltz::CreateSubmarineResponse, boltz::SubSwapStates};
use boltz_client::util::secrets::Preimage;
use boltz_client::{Bolt11Invoice, ToHex};
use log::{debug, error, info, warn};
use lwk_wollet::bitcoin::Witness;
use lwk_wollet::elements::Transaction;
use lwk_wollet::hashes::{sha256, Hash};
use tokio::sync::{broadcast, watch, Mutex};
use tokio::time::MissedTickBehavior;

use crate::chain::liquid::LiquidChainService;
use crate::model::PaymentState::{
    Complete, Created, Failed, Pending, RefundPending, Refundable, TimedOut,
};
use crate::model::{Config, SendSwap};
use crate::swapper::Swapper;
use crate::wallet::OnchainWallet;
use crate::{ensure_sdk, utils};
use crate::{
    error::PaymentError,
    model::{PaymentState, PaymentTxData, PaymentType},
    persist::Persister,
};

#[derive(Clone)]
pub(crate) struct SendSwapStateHandler {
    config: Config,
    onchain_wallet: Arc<dyn OnchainWallet>,
    persister: Arc<Persister>,
    swapper: Arc<dyn Swapper>,
    chain_service: Arc<Mutex<dyn LiquidChainService>>,
    subscription_notifier: broadcast::Sender<String>,
    refundable_swaps: Arc<Mutex<Vec<SendSwap>>>,
}

impl SendSwapStateHandler {
    pub(crate) fn new(
        config: Config,
        onchain_wallet: Arc<dyn OnchainWallet>,
        persister: Arc<Persister>,
        swapper: Arc<dyn Swapper>,
        chain_service: Arc<Mutex<dyn LiquidChainService>>,
    ) -> Self {
        let (subscription_notifier, _) = broadcast::channel::<String>(30);
        Self {
            config,
            onchain_wallet,
            persister,
            swapper,
            chain_service,
            subscription_notifier,
            refundable_swaps: Default::default(),
        }
    }

    pub(crate) async fn start(&self, mut shutdown: watch::Receiver<()>) {
        let cloned = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let swaps = cloned.refundable_swaps.lock().await;
                        for swap in swaps.iter() {
                            if let Err(err) = cloned.refund_cooperative(swap).await {
                                warn!("Could not refund Send swap cooperatively: {err:?}");
                            }
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

    /// Handles status updates from Boltz for Send swaps
    pub(crate) async fn on_new_status(&self, update: &boltz::Update) -> Result<()> {
        let id = &update.id;
        let swap_state = &update.status;
        let swap = self
            .persister
            .fetch_send_swap_by_id(id)?
            .ok_or(anyhow!("No ongoing Send Swap found for ID {id}"))?;

        info!("Handling Send Swap transition to {swap_state:?} for swap {id}");

        // See https://docs.boltz.exchange/v/api/lifecycle#normal-submarine-swaps
        match SubSwapStates::from_str(swap_state) {
            // Boltz has locked the HTLC, we proceed with locking up the funds
            Ok(SubSwapStates::InvoiceSet) => {
                match (swap.state, swap.lockup_tx_id.clone()) {
                    (PaymentState::Created, None) | (PaymentState::TimedOut, None) => {
                        let create_response = swap.get_boltz_create_response()?;
                        let lockup_tx = self.lockup_funds(id, &create_response).await?;
                        let lockup_tx_id = lockup_tx.txid().to_string();
                        let lockup_tx_fees_sat: u64 = lockup_tx.all_fees().values().sum();

                        // We insert a pseudo-lockup-tx in case LWK fails to pick up the new mempool tx for a while
                        // This makes the tx known to the SDK (get_info, list_payments) instantly
                        self.persister.insert_or_update_payment(PaymentTxData {
                            tx_id: lockup_tx_id.clone(),
                            timestamp: Some(utils::now()),
                            amount_sat: swap.payer_amount_sat,
                            fees_sat: lockup_tx_fees_sat,
                            payment_type: PaymentType::Send,
                            is_confirmed: false,
                        })?;

                        self.update_swap_info(id, Pending, None, Some(&lockup_tx_id), None)
                            .await?;
                    }
                    (_, Some(lockup_tx_id)) => {
                        warn!("Lockup tx for Send Swap {id} was already broadcast: txid {lockup_tx_id}")
                    }
                    (state, _) => {
                        debug!("Send Swap {id} is in an invalid state for {swap_state}: {state:?}")
                    }
                }
                Ok(())
            }

            // Boltz has detected the lockup in the mempool, we can speed up
            // the claim by doing so cooperatively
            Ok(SubSwapStates::TransactionClaimPending) => {
                self.cooperate_claim(&swap).await.map_err(|e| {
                    error!("Could not cooperate Send Swap {id} claim: {e}");
                    anyhow!("Could not post claim details. Err: {e:?}")
                })?;

                Ok(())
            }

            // Boltz announced they successfully broadcast the (cooperative or non-cooperative) claim tx
            Ok(SubSwapStates::TransactionClaimed) => {
                debug!("Send Swap {id} has been claimed");

                match swap.preimage {
                    Some(_) => {
                        debug!("The claim tx was a key path spend (cooperative claim)");
                        // Preimage was already validated and stored, PaymentSucceeded event emitted,
                        // when the cooperative claim was handled.
                    }
                    None => {
                        debug!("The claim tx was a script path spend (non-cooperative claim)");
                        let preimage = self
                            .get_preimage_from_script_path_claim_spend(&swap)
                            .await?;
                        self.validate_send_swap_preimage(id, &swap.invoice, &preimage)
                            .await?;
                        self.update_swap_info(id, Complete, Some(&preimage), None, None)
                            .await?;
                    }
                }

                Ok(())
            }

            // If swap state is unrecoverable, either:
            // 1. Boltz failed to pay
            // 2. The swap has expired (>24h)
            // 3. Lockup failed (we sent too little funds)
            // We initiate a cooperative refund, and then fallback to a regular one
            Ok(
                SubSwapStates::TransactionLockupFailed
                | SubSwapStates::InvoiceFailedToPay
                | SubSwapStates::SwapExpired,
            ) => {
                match swap.lockup_tx_id {
                    Some(_) => match swap.refund_tx_id {
                        Some(refund_tx_id) => warn!(
                        "Refund tx for Send Swap {id} was already broadcast: txid {refund_tx_id}"
                    ),
                        None => {
                            warn!("Send Swap {id} is in an unrecoverable state: {swap_state:?}, and lockup tx has been broadcast.");
                            let mut refundable_swaps = self.refundable_swaps.lock().await;
                            refundable_swaps.push(swap);
                        }
                    },
                    // Do not attempt broadcasting a refund if lockup tx was never sent and swap is
                    // unrecoverable. We resolve the payment as failed.
                    None => {
                        warn!("Send Swap {id} is in an unrecoverable state: {swap_state:?}, and lockup tx has never been broadcast. Resolving payment as failed.");
                        self.update_swap_info(id, Failed, None, None, None).await?;
                    }
                }

                Ok(())
            }

            Ok(_) => {
                debug!("Unhandled state for Send Swap {id}: {swap_state}");
                Ok(())
            }

            _ => Err(anyhow!(
                "Invalid SubSwapState for Send Swap {id}: {swap_state}"
            )),
        }
    }

    async fn lockup_funds(
        &self,
        swap_id: &str,
        create_response: &CreateSubmarineResponse,
    ) -> Result<Transaction, PaymentError> {
        debug!(
            "Initiated Send Swap: send {} sats to liquid address {}",
            create_response.expected_amount, create_response.address
        );

        let lockup_tx = self
            .onchain_wallet
            .build_tx(
                self.config.lowball_fee_rate_msat_per_vbyte(),
                &create_response.address,
                create_response.expected_amount,
            )
            .await?;

        info!("broadcasting lockup tx {}", lockup_tx.txid());
        let lockup_tx_id = self
            .chain_service
            .lock()
            .await
            .broadcast(&lockup_tx, Some(swap_id))
            .await?
            .to_string();

        info!("Successfully broadcast lockup tx for Send Swap {swap_id}. Lockup tx id: {lockup_tx_id}");
        Ok(lockup_tx)
    }

    /// Transitions a Send swap to a new state
    pub(crate) async fn update_swap_info(
        &self,
        swap_id: &str,
        to_state: PaymentState,
        preimage: Option<&str>,
        lockup_tx_id: Option<&str>,
        refund_tx_id: Option<&str>,
    ) -> Result<(), PaymentError> {
        info!("Transitioning Send swap {swap_id} to {to_state:?} (lockup_tx_id = {lockup_tx_id:?}, refund_tx_id = {refund_tx_id:?})");

        let swap: SendSwap = self
            .persister
            .fetch_send_swap_by_id(swap_id)
            .map_err(|_| PaymentError::PersistError)?
            .ok_or(PaymentError::Generic {
                err: format!("Send Swap not found {swap_id}"),
            })?;
        let payment_id = lockup_tx_id.map(|c| c.to_string()).or(swap.lockup_tx_id);

        Self::validate_state_transition(swap.state, to_state)?;
        self.persister.try_handle_send_swap_update(
            swap_id,
            to_state,
            preimage,
            lockup_tx_id,
            refund_tx_id,
        )?;
        if let Some(payment_id) = payment_id {
            let _ = self.subscription_notifier.send(payment_id);
        }
        Ok(())
    }

    async fn cooperate_claim(&self, send_swap: &SendSwap) -> Result<(), PaymentError> {
        debug!(
            "Claim is pending for Send Swap {}. Initiating cooperative claim",
            &send_swap.id
        );
        let output_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let claim_tx_details = self.swapper.get_send_claim_tx_details(send_swap)?;
        self.update_swap_info(
            &send_swap.id,
            Complete,
            Some(&claim_tx_details.preimage),
            None,
            None,
        )
        .await?;
        self.swapper
            .claim_send_swap_cooperative(send_swap, claim_tx_details, &output_address)?;
        Ok(())
    }

    async fn get_preimage_from_script_path_claim_spend(
        &self,
        swap: &SendSwap,
    ) -> Result<String, PaymentError> {
        info!("Retrieving preimage from non-cooperative claim tx");

        let id = &swap.id;
        let swap_script = swap.get_swap_script()?;
        let swap_script_pk = swap_script
            .to_address(self.config.network.into())?
            .script_pubkey();
        debug!("Found Send Swap swap_script_pk: {swap_script_pk:?}");

        // Get tx history of the swap script (lockup address)
        let history: Vec<_> = self
            .chain_service
            .lock()
            .await
            .get_script_history(&swap_script_pk)
            .await?;

        // We expect at most 2 txs: lockup and maybe the claim
        ensure_sdk!(
            history.len() <= 2,
            PaymentError::Generic {
                err: "Lockup address history for Send Swap {id} has more than 2 txs".to_string()
            }
        );

        match history.get(1) {
            None => Err(PaymentError::Generic {
                err: format!("Send Swap {id} has no claim tx"),
            }),
            Some(claim_tx_entry) => {
                let claim_tx_id = claim_tx_entry.txid;
                debug!("Send Swap {id} has claim tx {claim_tx_id}");

                let claim_tx = self
                    .chain_service
                    .lock()
                    .await
                    .get_transactions(&[claim_tx_id])
                    .await
                    .map_err(|e| anyhow!("Failed to fetch claim tx {claim_tx_id}: {e}"))?
                    .first()
                    .cloned()
                    .ok_or_else(|| anyhow!("Fetching claim tx returned an empty list"))?;

                let input = claim_tx
                    .input
                    .first()
                    .ok_or_else(|| anyhow!("Found no input for claim tx"))?;

                let script_witness_bytes = input.clone().witness.script_witness;
                info!("Found Send Swap {id} claim tx witness: {script_witness_bytes:?}");
                let script_witness = Witness::from(script_witness_bytes);

                let preimage_bytes = script_witness
                    .nth(1)
                    .ok_or_else(|| anyhow!("Claim tx witness has no preimage"))?;
                let preimage = sha256::Hash::from_slice(preimage_bytes)
                    .map_err(|e| anyhow!("Claim tx witness has invalid preimage: {e}"))?;
                let preimage_hex = preimage.to_hex();
                debug!("Found Send Swap {id} claim tx preimage: {preimage_hex}");

                Ok(preimage_hex)
            }
        }
    }

    async fn validate_send_swap_preimage(
        &self,
        swap_id: &str,
        invoice: &str,
        preimage: &str,
    ) -> Result<(), PaymentError> {
        Self::verify_payment_hash(preimage, invoice)?;
        info!("Preimage is valid for Send Swap {swap_id}");
        Ok(())
    }

    pub(crate) async fn refund_cooperative(&self, swap: &SendSwap) -> Result<String, PaymentError> {
        let output_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let cooperative_refund_tx_fees_sat =
            utils::estimate_refund_fees(swap, &self.config, &output_address, true)?;

        self.swapper.refund_send_swap_cooperative(
            swap,
            &output_address,
            cooperative_refund_tx_fees_sat,
        )
    }

    pub(crate) async fn refund_non_cooperative(
        &self,
        swap: &SendSwap,
    ) -> Result<String, PaymentError> {
        let current_height = self.onchain_wallet.tip().await.height();
        let output_address = self.onchain_wallet.next_unused_address().await?.to_string();
        let non_cooperative_refund_tx_fees_sat =
            utils::estimate_refund_fees(swap, &self.config, &output_address, false)?;

        self.swapper.refund_send_swap_non_cooperative(
            swap,
            non_cooperative_refund_tx_fees_sat,
            &output_address,
            current_height,
        )
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

            (Created | Pending, Complete) => Ok(()),
            (_, Complete) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Complete state"),
            }),

            (Created | TimedOut, TimedOut) => Ok(()),
            (_, TimedOut) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to TimedOut state"),
            }),

            (_, Refundable) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Refundable state"),
            }),

            (Pending, RefundPending) => Ok(()),
            (_, RefundPending) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to RefundPending state"),
            }),

            (Complete, Failed) => Err(PaymentError::Generic {
                err: format!("Cannot transition from {from_state:?} to Failed state"),
            }),
            (_, Failed) => Ok(()),
        }
    }

    fn verify_payment_hash(preimage: &str, invoice: &str) -> Result<(), PaymentError> {
        let preimage = Preimage::from_str(preimage)?;
        let preimage_hash = preimage.sha256.to_string();
        let invoice =
            Bolt11Invoice::from_str(invoice).map_err(|err| PaymentError::InvalidInvoice {
                err: err.to_string(),
            })?;
        let invoice_payment_hash = invoice.payment_hash();

        (invoice_payment_hash.to_string() == preimage_hash)
            .then_some(())
            .ok_or(PaymentError::InvalidPreimage)
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
        model::PaymentState::{self, *},
        test_utils::{
            persist::{new_persister, new_send_swap},
            send_swap::new_send_swap_state_handler,
        },
    };

    #[tokio::test]
    async fn test_send_swap_state_transitions() -> Result<()> {
        let (_temp_dir, storage) = new_persister()?;
        let storage = Arc::new(storage);
        let send_swap_state_handler = new_send_swap_state_handler(storage.clone())?;

        // Test valid combinations of states
        let valid_combinations = HashMap::from([
            (
                Created,
                HashSet::from([Pending, Complete, TimedOut, Failed]),
            ),
            (
                Pending,
                HashSet::from([Pending, RefundPending, Complete, Failed]),
            ),
            (TimedOut, HashSet::from([TimedOut, Failed])),
            (Complete, HashSet::from([])),
            (Refundable, HashSet::from([Failed])),
            (Failed, HashSet::from([Failed])),
        ]);

        for (first_state, allowed_states) in valid_combinations.iter() {
            for allowed_state in allowed_states {
                let send_swap = new_send_swap(Some(*first_state));
                storage.insert_send_swap(&send_swap)?;

                assert!(send_swap_state_handler
                    .update_swap_info(&send_swap.id, *allowed_state, None, None, None)
                    .await
                    .is_ok());
            }
        }

        // Test invalid combinations of states
        let all_states = HashSet::from([Created, Pending, Complete, TimedOut, Failed]);
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
                let send_swap = new_send_swap(Some(*first_state));
                storage.insert_send_swap(&send_swap)?;

                assert!(send_swap_state_handler
                    .update_swap_info(&send_swap.id, *disallowed_state, None, None, None)
                    .await
                    .is_err());
            }
        }

        Ok(())
    }
}
