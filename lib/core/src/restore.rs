//! This module provides functionality for restoring the swap tx IDs from onchain data

use std::collections::HashMap;

use anyhow::Result;
use log::{error, info};
use lwk_wollet::elements::Txid;
use lwk_wollet::WalletTx;
use sdk_common::bitcoin::hashes::hex::ToHex;

use crate::prelude::*;
use crate::restore::immutable::*;

/// A map of all our known LWK onchain txs, indexed by tx ID
pub(crate) struct TxMap {
    outgoing_tx_map: HashMap<Txid, WalletTx>,
    incoming_tx_map: HashMap<Txid, WalletTx>,
}
impl TxMap {
    pub(crate) fn from_raw_tx_map(raw_tx_map: HashMap<Txid, WalletTx>) -> Self {
        let (outgoing_tx_map, incoming_tx_map): (HashMap<Txid, WalletTx>, HashMap<Txid, WalletTx>) =
            raw_tx_map
                .into_iter()
                .partition(|(_, tx)| tx.balance.values().sum::<i64>() < 0);

        Self {
            outgoing_tx_map,
            incoming_tx_map,
        }
    }
}

pub(crate) struct RecoveredOnchainDataSend {
    lockup_tx_ids: HashMap<String, HistoryTxId>,
    claim_tx_ids: HashMap<String, HistoryTxId>,
    refund_tx_ids: HashMap<String, HistoryTxId>,
}

pub(crate) struct RecoveredOnchainDataReceive {
    lockup_claim_tx_ids: HashMap<String, (HistoryTxId, HistoryTxId)>,
}

pub(crate) struct RecoveredOnchainDataChainSend {
    lbtc_user_lockup_tx_ids: HashMap<String, HistoryTxId>,
    lbtc_refund_tx_ids: HashMap<String, HistoryTxId>,
    btc_server_lockup_tx_ids: HashMap<String, HistoryTxId>,
    btc_claim_tx_ids: HashMap<String, HistoryTxId>,
}

pub(crate) struct RecoveredOnchainDataChainReceive {
    /// Server lockup tx ID, claim tx ID.
    ///
    /// We store them in a pair because when they are present, we always expect both to be present.
    lbtc_server_lockup_claim_tx_ids: HashMap<String, (HistoryTxId, HistoryTxId)>,

    btc_user_lockup_tx_ids: HashMap<String, HistoryTxId>,
    btc_refund_tx_ids: HashMap<String, HistoryTxId>,
}

pub(crate) struct RecoveredOnchainData {
    send: RecoveredOnchainDataSend,
    receive: RecoveredOnchainDataReceive,
    chain_send: RecoveredOnchainDataChainSend,
    chain_receive: RecoveredOnchainDataChainReceive,
}

impl LiquidSdk {
    pub(crate) async fn recover_from_onchain(&self, tx_map: TxMap) -> Result<()> {
        // Immutable DB, as fetched from endpoint
        let imm_db = self.get_swaps_list().await?;

        // Recovered onchain data (txs) per swap
        let recovered = self.get_onchain_data(tx_map, &imm_db).await?;

        // Persist updated swaps
        // TODO Optimize: persist in batches
        self.event_manager.pause_notifications();
        for send_swap_id in imm_db.send_swap_immutable_db_by_swap_id.keys() {
            self.persist_send_swap(send_swap_id, &recovered.send).await;
        }
        for receive_swap_id in imm_db.receive_swap_immutable_db_by_swap_id_.keys() {
            self.persist_receive_swap(receive_swap_id, &recovered.receive)
                .await;
        }
        for send_chain_swap_id in imm_db.send_chain_swap_immutable_db_by_swap_id.keys() {
            self.persist_send_chain_swap(send_chain_swap_id, &recovered.chain_send)
                .await;
        }
        for receive_chain_swap_id in imm_db.receive_chain_swap_immutable_db_by_swap_id.keys() {
            self.persist_receive_chain_swap(receive_chain_swap_id, &recovered.chain_receive)
                .await;
        }
        self.event_manager.resume_notifications();

        Ok(())
    }

    async fn persist_send_swap(
        &self,
        send_swap_id: &String,
        recovered_send: &RecoveredOnchainDataSend,
    ) {
        let lockup_tx_id = recovered_send.lockup_tx_ids.get(send_swap_id);
        let lockup_tx_id_str = lockup_tx_id.map(|tx| tx.txid.to_hex());
        let claim_tx_id = recovered_send.claim_tx_ids.get(send_swap_id);
        let refund_tx_id = recovered_send.refund_tx_ids.get(send_swap_id);
        let refund_tx_id_str = refund_tx_id.map(|tx| tx.txid.to_hex());

        let state = match lockup_tx_id {
            Some(_) => match claim_tx_id {
                Some(_) => PaymentState::Complete,
                None => match refund_tx_id {
                    Some(refund_tx_id) => match refund_tx_id.confirmed {
                        true => PaymentState::Failed,
                        false => PaymentState::RefundPending,
                    },
                    None => PaymentState::Pending,
                },
            },
            None => PaymentState::TimedOut,
        };

        if let Err(e) = self
            .send_swap_state_handler
            .update_swap_info(
                send_swap_id,
                state,
                None, // TODO Get preimage (from immutable DB?)
                lockup_tx_id_str.as_deref(),
                refund_tx_id_str.as_deref(),
            )
            .await
        {
            error!("Failed to persist Send Swap {send_swap_id}: {e}");
        }
    }

    async fn persist_receive_swap(
        &self,
        receive_swap_id: &String,
        recovered_receive: &RecoveredOnchainDataReceive,
    ) {
        let lockup_claim_tx_ids = recovered_receive.lockup_claim_tx_ids.get(receive_swap_id);
        let (lockup_tx_id, claim_tx_id) = match lockup_claim_tx_ids {
            Some((x, y)) => (Some(x), Some(y)),
            None => (None, None),
        };
        let lockup_tx_id_str = lockup_tx_id.map(|tx| tx.txid.to_hex());
        let claim_tx_id_str = claim_tx_id.map(|tx| tx.txid.to_hex());

        let state = match lockup_claim_tx_ids {
            Some((_lockup_tx_id, claim_tx_id)) => match claim_tx_id.confirmed {
                true => PaymentState::Complete,
                false => PaymentState::Pending,
            },
            // TODO How to distinguish between Failed and Created (if in both cases, no lockup or claim tx present)
            //   See https://docs.boltz.exchange/v/api/lifecycle#reverse-submarine-swaps
            None => PaymentState::Failed,
        };

        if let Err(e) = self
            .receive_swap_state_handler
            .update_swap_info(
                receive_swap_id,
                state,
                lockup_tx_id_str.as_deref(),
                claim_tx_id_str.as_deref(),
            )
            .await
        {
            error!("Failed to persist Receive Swap {receive_swap_id}: {e}");
        }
    }

    async fn persist_send_chain_swap(
        &self,
        send_chain_swap_id: &String,
        recovered_send_chain: &RecoveredOnchainDataChainSend,
    ) {
        let server_lockup_tx_id = recovered_send_chain
            .btc_server_lockup_tx_ids
            .get(send_chain_swap_id);
        let server_lockup_tx_id_str = server_lockup_tx_id.map(|tx| tx.txid.to_hex());
        let user_lockup_tx_id = recovered_send_chain
            .lbtc_user_lockup_tx_ids
            .get(send_chain_swap_id);
        let user_lockup_tx_id_str = user_lockup_tx_id.map(|tx| tx.txid.to_hex());
        let claim_tx_id = recovered_send_chain
            .btc_claim_tx_ids
            .get(send_chain_swap_id);
        let claim_tx_id_str = claim_tx_id.map(|tx| tx.txid.to_hex());
        let refund_tx_id = recovered_send_chain
            .lbtc_refund_tx_ids
            .get(send_chain_swap_id);
        let refund_tx_id_str = refund_tx_id.map(|tx| tx.txid.to_hex());

        // TODO How to detect TimedOut state?
        ///     TimedOut: This covers the case when the swap state is still Created and the swap fails to reach the
        ///     Pending state in time. The TimedOut state indicates the lockup tx should never be broadcast.
        let state = match user_lockup_tx_id {
            Some(_) => {
                match claim_tx_id {
                    Some(_) => PaymentState::Complete,
                    None => {
                        match refund_tx_id {
                            Some(tx) => match tx.confirmed {
                                true => PaymentState::Failed,
                                false => PaymentState::RefundPending,
                            },
                            // TODO Created or TimedOut
                            None => PaymentState::Created,
                        }
                    }
                }
            }
            None => PaymentState::Created, // TODO Does immutable DB include Created swaps?
        };

        if let Err(e) = self
            .chain_swap_state_handler
            .update_swap_info(
                send_chain_swap_id,
                state,
                server_lockup_tx_id_str.as_deref(),
                user_lockup_tx_id_str.as_deref(),
                claim_tx_id_str.as_deref(),
                refund_tx_id_str.as_deref(),
            )
            .await
        {
            error!("Failed to persist Send Chain Swap {send_chain_swap_id}: {e}");
        }
    }

    async fn persist_receive_chain_swap(
        &self,
        receive_chain_swap_id: &String,
        recovered_receive_chain: &RecoveredOnchainDataChainReceive,
    ) {
        let (server_lockup_tx_id, server_claim_tx_id) = match recovered_receive_chain
            .lbtc_server_lockup_claim_tx_ids
            .get(receive_chain_swap_id)
        {
            Some((server_lockup_tx_id, server_claim_tx_id)) => {
                (Some(server_lockup_tx_id), Some(server_claim_tx_id))
            }
            None => (None, None),
        };
        let server_lockup_tx_id_str = server_lockup_tx_id.map(|tx| tx.txid.to_hex());
        let server_claim_tx_id_str = server_claim_tx_id.map(|tx| tx.txid.to_hex());
        let user_lockup_tx_id = recovered_receive_chain
            .btc_user_lockup_tx_ids
            .get(receive_chain_swap_id);
        let user_lockup_tx_id_str = user_lockup_tx_id.map(|tx| tx.txid.to_hex());
        let refund_tx_id = recovered_receive_chain
            .btc_refund_tx_ids
            .get(receive_chain_swap_id);
        let refund_tx_id_str = refund_tx_id.map(|tx| tx.txid.to_hex());

        // // TODO How to detect TimedOut state?
        let state = match user_lockup_tx_id {
            Some(_) => {
                match server_claim_tx_id {
                    Some(_) => PaymentState::Complete,
                    None => {
                        match refund_tx_id {
                            Some(tx) => match tx.confirmed {
                                true => PaymentState::Failed,
                                false => PaymentState::RefundPending,
                            },
                            // TODO Created or TimedOut
                            None => PaymentState::Created,
                        }
                    }
                }
            }
            None => PaymentState::Created, // TODO Does immutable DB include Created swaps?
        };

        if let Err(e) = self
            .chain_swap_state_handler
            .update_swap_info(
                receive_chain_swap_id,
                state,
                server_lockup_tx_id_str.as_deref(),
                user_lockup_tx_id_str.as_deref(),
                server_claim_tx_id_str.as_deref(),
                refund_tx_id_str.as_deref(),
            )
            .await
        {
            error!("Failed to persist Receive Chain Swap {receive_chain_swap_id}: {e}");
        }
    }

    pub(crate) async fn get_onchain_data(
        &self,
        tx_map: TxMap,
        immutable_db: &SwapsList,
    ) -> Result<RecoveredOnchainData> {
        let histories = self.fetch_swaps_histories(immutable_db).await?;

        let recovered_send_data = self
            .recover_send_swap_tx_ids(&tx_map, histories.send)
            .await?;
        let recovered_receive_data = self
            .recover_receive_swap_tx_ids(&tx_map, histories.receive)
            .await?;
        let recovered_chain_send_data = self
            .recover_send_chain_swap_tx_ids(&tx_map, histories.send_chain)
            .await?;
        let recovered_chain_receive_data = self
            .recover_receive_chain_swap_tx_ids(&tx_map, histories.receive_chain)
            .await?;

        Ok(RecoveredOnchainData {
            send: recovered_send_data,
            receive: recovered_receive_data,
            chain_send: recovered_chain_send_data,
            chain_receive: recovered_chain_receive_data,
        })
    }

    /// Reconstruct Send Swap tx IDs from the onchain data and the immutable DB data
    async fn recover_send_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        send_histories_by_swap_id: HashMap<String, SendSwapHistory>,
    ) -> Result<RecoveredOnchainDataSend> {
        let mut lockup_tx_map: HashMap<String, HistoryTxId> = HashMap::new();
        let mut claim_tx_map: HashMap<String, HistoryTxId> = HashMap::new();
        let mut refund_tx_map: HashMap<String, HistoryTxId> = HashMap::new();

        for (swap_id, history) in send_histories_by_swap_id {
            // If a history tx is one of our outgoing txs, it's a lockup tx
            let maybe_lockup_tx_id = history
                .iter()
                .find(|&tx| tx_map.outgoing_tx_map.contains_key::<Txid>(&tx.txid));
            match maybe_lockup_tx_id {
                None => {
                    error!("No lockup tx found when recovering data for Send Swap {swap_id}")
                }
                Some(lockup_tx_id) => {
                    lockup_tx_map.insert(swap_id.clone(), lockup_tx_id.clone());
                }
            }

            // If a history tx is one of our incoming txs, it's a refund tx
            let maybe_refund_tx_id = history
                .iter()
                .find(|&tx| tx_map.incoming_tx_map.contains_key::<Txid>(&tx.txid));
            if let Some(refund_tx_id) = maybe_refund_tx_id {
                refund_tx_map.insert(swap_id.clone(), refund_tx_id.clone());
            }

            // A history tx that is neither a known incoming or outgoing tx is a claim tx
            let maybe_claim_tx_id = history
                .iter()
                .filter(|&tx| !tx_map.incoming_tx_map.contains_key::<Txid>(&tx.txid))
                .find(|&tx| !tx_map.outgoing_tx_map.contains_key::<Txid>(&tx.txid));
            if let Some(claim_tx_id) = maybe_claim_tx_id {
                claim_tx_map.insert(swap_id.clone(), claim_tx_id.clone());
            }
        }

        info!(
            "[Recover Send] Found {} lockup txs from onchain data",
            lockup_tx_map.len()
        );
        info!(
            "[Recover Send] Found {} claim txs from onchain data",
            claim_tx_map.len()
        );
        info!(
            "[Recover Send] Found {} refund txs from onchain data",
            refund_tx_map.len()
        );

        Ok(RecoveredOnchainDataSend {
            lockup_tx_ids: lockup_tx_map,
            claim_tx_ids: claim_tx_map,
            refund_tx_ids: refund_tx_map,
        })
    }

    /// Reconstruct Receive Swap tx IDs from the onchain data and the immutable DB data
    async fn recover_receive_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        receive_histories_by_swap_id: HashMap<String, ReceiveSwapHistory>,
    ) -> Result<RecoveredOnchainDataReceive> {
        let mut lockup_claim_tx_ids_map: HashMap<String, (HistoryTxId, HistoryTxId)> =
            HashMap::new();

        for (swap_id, history) in receive_histories_by_swap_id {
            match history.len() {
                2 => {
                    let first = &history[0];
                    let second = &history[1];

                    // If a history tx is a known incoming txs, it's the claim tx
                    let (lockup_tx_id, claim_tx_id) =
                        match tx_map.incoming_tx_map.contains_key::<Txid>(&first.txid) {
                            true => (second, first),
                            false => (first, second),
                        };
                    lockup_claim_tx_ids_map
                        .insert(swap_id.clone(), (lockup_tx_id.clone(), claim_tx_id.clone()));
                }
                n => {
                    error!("Script history with unexpected length {n} found while recovering data for Receive Swap {swap_id}")
                }
            }
        }

        info!(
            "[Recover Receive] Found {} lockup and claim tx pairs from onchain data",
            lockup_claim_tx_ids_map.len()
        );

        Ok(RecoveredOnchainDataReceive {
            lockup_claim_tx_ids: lockup_claim_tx_ids_map,
        })
    }

    /// Reconstruct Chain Send Swap tx IDs from the onchain data and the immutable DB data
    async fn recover_send_chain_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        chain_send_histories_by_swap_id: HashMap<String, SendChainSwapHistory>,
    ) -> Result<RecoveredOnchainDataChainSend> {
        let mut lbtc_user_lockup_tx_map: HashMap<String, HistoryTxId> = HashMap::new();
        let mut lbtc_refund_tx_map: HashMap<String, HistoryTxId> = HashMap::new();
        let mut btc_server_lockup_tx_map: HashMap<String, HistoryTxId> = HashMap::new();
        let mut btc_claim_tx_map: HashMap<String, HistoryTxId> = HashMap::new();

        for (swap_id, history) in chain_send_histories_by_swap_id {
            info!("[Recover Chain Send] Checking swap {swap_id}");

            // If a history tx is one of our outgoing txs, it's a lockup tx
            let maybe_lockup_tx_id = history
                .lbtc_lockup_script_history
                .iter()
                .find(|&tx| tx_map.outgoing_tx_map.contains_key::<Txid>(&tx.txid));
            match maybe_lockup_tx_id {
                None => {
                    error!("No lockup tx found when recovering data for Chain Send Swap {swap_id}")
                }
                Some(lockup_tx_id) => {
                    lbtc_user_lockup_tx_map.insert(swap_id.clone(), lockup_tx_id.clone());
                }
            }

            // If a history tx is one of our incoming txs, it's a refund tx
            let maybe_refund_tx_id = history
                .lbtc_lockup_script_history
                .iter()
                .find(|&tx| tx_map.incoming_tx_map.contains_key::<Txid>(&tx.txid));
            if let Some(refund_tx_id) = maybe_refund_tx_id {
                lbtc_refund_tx_map.insert(swap_id.clone(), refund_tx_id.clone());
            }

            match history.btc_claim_script_history.len() {
                2 => {
                    // TODO How to tell the claim tx apart from the lockup tx? Is the order in which they're received from Electrum reliable?
                    btc_server_lockup_tx_map
                        .insert(swap_id.clone(), history.btc_claim_script_history[0].clone());
                    btc_claim_tx_map
                        .insert(swap_id.clone(), history.btc_claim_script_history[1].clone());
                }
                n => {
                    error!("BTC script history with unexpected length {n} found while recovering data for Chain Send Swap {swap_id}")
                }
            }
        }

        info!(
            "[Recover Chain Send] Found {} user L-BTC lockup txs from onchain data",
            lbtc_user_lockup_tx_map.len()
        );
        info!(
            "[Recover Chain Send] Found {} L-BTC refund txs from onchain data",
            lbtc_refund_tx_map.len()
        );
        info!(
            "[Recover Chain Send] Found {} server BTC lockup txs from onchain data",
            btc_server_lockup_tx_map.len()
        );
        info!(
            "[Recover Chain Send] Found {} BTC claim txs from onchain data",
            btc_claim_tx_map.len()
        );

        Ok(RecoveredOnchainDataChainSend {
            lbtc_user_lockup_tx_ids: lbtc_user_lockup_tx_map,
            lbtc_refund_tx_ids: lbtc_refund_tx_map,
            btc_server_lockup_tx_ids: btc_server_lockup_tx_map,
            btc_claim_tx_ids: btc_claim_tx_map,
        })
    }

    /// Reconstruct Chain Receive Swap tx IDs from the onchain data and the immutable DB data
    async fn recover_receive_chain_swap_tx_ids(
        &self,
        tx_map: &TxMap,
        chain_receive_histories_by_swap_id: HashMap<String, ReceiveChainSwapHistory>,
    ) -> Result<RecoveredOnchainDataChainReceive> {
        let mut lbtc_server_lockup_claim_tx_ids: HashMap<String, (HistoryTxId, HistoryTxId)> =
            HashMap::new();
        let mut btc_user_lockup_tx_ids: HashMap<String, HistoryTxId> = HashMap::new();
        let mut btc_refund_tx_ids: HashMap<String, HistoryTxId> = HashMap::new();

        for (swap_id, history) in chain_receive_histories_by_swap_id {
            info!("[Recover Chain Receive] Checking swap {swap_id}");

            match history.lbtc_claim_script_history.len() {
                2 => {
                    let first = &history.lbtc_claim_script_history[0];
                    let second = &history.lbtc_claim_script_history[1];

                    // If a history tx is a known incoming txs, it's the claim tx
                    let (lockup_tx_id, claim_tx_id) =
                        match tx_map.incoming_tx_map.contains_key::<Txid>(&first.txid) {
                            true => (second, first),
                            false => (first, second),
                        };
                    lbtc_server_lockup_claim_tx_ids
                        .insert(swap_id.clone(), (lockup_tx_id.clone(), claim_tx_id.clone()));
                }
                n => {
                    error!("L-BTC script history with unexpected length {n} found while recovering data for Chain Receive Swap {swap_id}")
                }
            }

            match history.btc_lockup_script_history.len() {
                // TODO How to treat case when history length > 2? (address re-use)
                x if x >= 2 => {
                    // TODO How to tell the user lockup tx apart from the refund tx? Is the order in which they're received from Electrum reliable?
                    // TODO How to tell BTC refund apart from BTC server claim tx?
                    btc_user_lockup_tx_ids.insert(
                        swap_id.clone(),
                        history.btc_lockup_script_history[0].clone(),
                    );
                    btc_refund_tx_ids.insert(
                        swap_id.clone(),
                        history.btc_lockup_script_history[1].clone(),
                    );
                }
                n => {
                    error!("BTC script history with unexpected length {n} found while recovering data for Chain Receive Swap {swap_id}")
                }
            }
        }

        info!("[Recover Chain Receive] Found {} L-BTC server lockup and claim tx pairs from onchain data", lbtc_server_lockup_claim_tx_ids.len());
        info!(
            "[Recover Chain Receive] Found {} BTC user lockup tx IDs from onchain data",
            btc_user_lockup_tx_ids.len()
        );
        info!(
            "[Recover Chain Receive] Found {} BTC refund tx IDs from onchain data",
            btc_refund_tx_ids.len()
        );

        Ok(RecoveredOnchainDataChainReceive {
            lbtc_server_lockup_claim_tx_ids,
            btc_user_lockup_tx_ids,
            btc_refund_tx_ids,
        })
    }
}

/// Methods to simulate the immutable DB data available from real-time sync
// TODO Remove once real-time sync is integrated
pub(crate) mod immutable {
    use std::collections::HashMap;

    use anyhow::{anyhow, ensure, Result};
    use boltz_client::{BtcSwapScript, LBtcSwapScript};
    use log::{error, info};
    use lwk_wollet::elements::Txid;
    use lwk_wollet::History;

    use crate::prelude::*;
    use crate::sdk::LiquidSdk;

    type BtcScript = lwk_wollet::bitcoin::ScriptBuf;
    type LBtcScript = lwk_wollet::elements::Script;

    pub(crate) type SendSwapHistory = Vec<HistoryTxId>;
    pub(crate) type ReceiveSwapHistory = Vec<HistoryTxId>;

    #[derive(Clone)]
    pub(crate) struct HistoryTxId {
        pub txid: Txid,
        pub confirmed: bool,
    }
    impl From<History> for HistoryTxId {
        fn from(value: History) -> Self {
            Self {
                txid: value.txid,
                confirmed: value.height > 0,
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Clone)]
    pub(crate) struct SendSwapImmutableData {
        pub(crate) swap_id: String,
        pub(crate) swap_script: LBtcSwapScript,
        pub(crate) script: LBtcScript,
    }

    #[allow(dead_code)]
    #[derive(Clone)]
    pub(crate) struct ReceiveSwapImmutableData {
        pub(crate) swap_id: String,
        pub(crate) swap_script: LBtcSwapScript,
        pub(crate) script: LBtcScript,
    }

    #[allow(dead_code)]
    #[derive(Clone)]
    pub(crate) struct SendChainSwapImmutableData {
        swap_id: String,
        lockup_swap_script: LBtcSwapScript,
        lockup_script: LBtcScript,
        claim_swap_script: BtcSwapScript,
        claim_script: BtcScript,
    }

    pub(crate) struct SendChainSwapHistory {
        pub(crate) lbtc_lockup_script_history: Vec<HistoryTxId>,
        pub(crate) btc_claim_script_history: Vec<HistoryTxId>,
    }

    #[allow(dead_code)]
    #[derive(Clone)]
    pub(crate) struct ReceiveChainSwapImmutableData {
        swap_id: String,
        lockup_swap_script: BtcSwapScript,
        lockup_script: BtcScript,
        claim_swap_script: LBtcSwapScript,
        claim_script: LBtcScript,
    }

    pub(crate) struct ReceiveChainSwapHistory {
        pub(crate) lbtc_claim_script_history: Vec<HistoryTxId>,
        pub(crate) btc_lockup_script_history: Vec<HistoryTxId>,
    }

    /// Swap data received from the immutable DB
    pub(crate) struct SwapsList {
        pub(crate) send_swap_immutable_db_by_swap_id: HashMap<String, SendSwapImmutableData>,
        pub(crate) receive_swap_immutable_db_by_swap_id_: HashMap<String, ReceiveSwapImmutableData>,
        pub(crate) send_chain_swap_immutable_db_by_swap_id:
            HashMap<String, SendChainSwapImmutableData>,
        pub(crate) receive_chain_swap_immutable_db_by_swap_id:
            HashMap<String, ReceiveChainSwapImmutableData>,
    }

    impl SwapsList {
        fn init(
            send_swaps: Vec<SendSwap>,
            receive_swaps: Vec<ReceiveSwap>,
            send_chain_swaps: Vec<ChainSwap>,
            receive_chain_swaps: Vec<ChainSwap>,
        ) -> Result<Self> {
            let send_swap_immutable_db_by_swap_id: HashMap<String, SendSwapImmutableData> =
                send_swaps
                    .iter()
                    .filter_map(|swap| match swap.get_swap_script() {
                        Ok(swap_script) => match &swap_script.funding_addrs {
                            Some(address) => Some((
                                swap.id.clone(),
                                SendSwapImmutableData {
                                    swap_id: swap.id.clone(),
                                    swap_script: swap_script.clone(),
                                    script: address.script_pubkey(),
                                },
                            )),
                            None => {
                                error!("No funding address found for Send Swap {}", swap.id);
                                None
                            }
                        },
                        Err(e) => {
                            error!("Failed to get swap script for Send Swap {}: {e}", swap.id);
                            None
                        }
                    })
                    .collect();
            let send_swap_immutable_db_size = send_swap_immutable_db_by_swap_id.len();
            info!("Send Swap immutable DB: {send_swap_immutable_db_size} rows");

            let receive_swap_immutable_db_by_swap_id_: HashMap<String, ReceiveSwapImmutableData> =
                receive_swaps
                    .iter()
                    .filter_map(|swap| {
                        let swap_id = &swap.id;

                        let swap_script = swap
                            .get_swap_script()
                            .map_err(|e| {
                                error!("Failed to get swap script for Receive Swap {swap_id}: {e}")
                            })
                            .ok()?;

                        match &swap_script.funding_addrs {
                            Some(address) => Some((
                                swap.id.clone(),
                                ReceiveSwapImmutableData {
                                    swap_id: swap.id.clone(),
                                    swap_script: swap_script.clone(),
                                    script: address.script_pubkey(),
                                },
                            )),
                            None => {
                                error!("No funding address found for Receive Swap {}", swap.id);
                                None
                            }
                        }
                    })
                    .collect();
            let receive_swap_immutable_db_size = receive_swap_immutable_db_by_swap_id_.len();
            info!("Receive Swap immutable DB: {receive_swap_immutable_db_size} rows");

            let send_chain_swap_immutable_db_by_swap_id: HashMap<String, SendChainSwapImmutableData> =
                send_chain_swaps.iter().filter_map(|swap| {
                    let swap_id = &swap.id;

                    let lockup_swap_script = swap.get_lockup_swap_script()
                        .map_err(|e| error!("Failed to get lockup swap script for swap {swap_id}: {e}"))
                        .map(|s| s.as_liquid_script().ok())
                        .ok()
                        .flatten()?;
                    let claim_swap_script = swap.get_claim_swap_script()
                        .map_err(|e| error!("Failed to get claim swap script for swap {swap_id}: {e}"))
                        .map(|s| s.as_bitcoin_script().ok()).ok().flatten()?;

                    let maybe_lockup_script = lockup_swap_script.clone().funding_addrs.map(|addr| addr.script_pubkey());
                    let maybe_claim_script = claim_swap_script.clone().funding_addrs.map(|addr| addr.script_pubkey());

                    match (maybe_lockup_script, maybe_claim_script) {
                        (Some(lockup_script), Some(claim_script)) => {
                            Some((swap.id.clone(), SendChainSwapImmutableData {
                                swap_id: swap.id.clone(),
                                lockup_swap_script,
                                lockup_script,
                                claim_swap_script,
                                claim_script,
                            }))
                        }
                        (lockup_script, claim_script) => {
                            error!("Failed to get lockup or claim script for swap {swap_id}. Lockup script: {lockup_script:?}. Claim script: {claim_script:?}");
                            None
                        }
                    }
                })
                .collect();
            let send_chain_swap_immutable_db_size = send_chain_swap_immutable_db_by_swap_id.len();
            info!("Send Chain Swap immutable DB: {send_chain_swap_immutable_db_size} rows");

            let receive_chain_swap_immutable_db_by_swap_id: HashMap<String, ReceiveChainSwapImmutableData> =
                receive_chain_swaps.iter().filter_map(|swap| {
                    let swap_id = &swap.id;

                    let lockup_swap_script = swap.get_lockup_swap_script()
                        .map_err(|e| error!("Failed to get lockup swap script for swap {swap_id}: {e}"))
                        .map(|s| s.as_bitcoin_script().ok()).ok().flatten()?;
                    let claim_swap_script = swap.get_claim_swap_script()
                        .map_err(|e| error!("Failed to get claim swap script for swap {swap_id}: {e}"))
                        .map(|s| s.as_liquid_script().ok()).ok().flatten()?;

                    let maybe_lockup_script = lockup_swap_script.clone().funding_addrs.map(|addr| addr.script_pubkey());
                    let maybe_claim_script = claim_swap_script.clone().funding_addrs.map(|addr| addr.script_pubkey());

                    match (maybe_lockup_script, maybe_claim_script) {
                        (Some(lockup_script), Some(claim_script)) => {
                            Some((swap.id.clone(), ReceiveChainSwapImmutableData {
                                swap_id: swap.id.clone(),
                                lockup_swap_script,
                                lockup_script,
                                claim_swap_script,
                                claim_script,
                            }))
                        }
                        (lockup_script, claim_script) => {
                            error!("Failed to get lockup or claim script for swap {swap_id}. Lockup script: {lockup_script:?}. Claim script: {claim_script:?}");
                            None
                        }
                    }
                })
                .collect();
            let receive_chain_swap_immutable_db_size =
                receive_chain_swap_immutable_db_by_swap_id.len();
            info!("Receive Chain Swap immutable DB: {receive_chain_swap_immutable_db_size} rows");

            Ok(SwapsList {
                send_swap_immutable_db_by_swap_id,
                receive_swap_immutable_db_by_swap_id_,
                send_chain_swap_immutable_db_by_swap_id,
                receive_chain_swap_immutable_db_by_swap_id,
            })
        }

        fn send_swaps_by_script(&self) -> HashMap<LBtcScript, SendSwapImmutableData> {
            self.send_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| (imm.script.clone(), imm))
                .collect()
        }

        fn send_histories_by_swap_id(
            &self,
            lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<HistoryTxId>>,
        ) -> HashMap<String, SendSwapHistory> {
            let send_swaps_by_script = self.send_swaps_by_script();

            let mut data: HashMap<String, SendSwapHistory> = HashMap::new();
            lbtc_script_to_history_map
                .iter()
                .for_each(|(lbtc_script, lbtc_script_history)| {
                    if let Some(imm) = send_swaps_by_script.get(lbtc_script) {
                        data.insert(imm.swap_id.clone(), lbtc_script_history.clone());
                    }
                });
            data
        }

        fn receive_swaps_by_script(&self) -> HashMap<LBtcScript, ReceiveSwapImmutableData> {
            self.receive_swap_immutable_db_by_swap_id_
                .clone()
                .into_values()
                .map(|imm| (imm.script.clone(), imm))
                .collect()
        }

        fn receive_histories_by_swap_id(
            &self,
            lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<HistoryTxId>>,
        ) -> HashMap<String, ReceiveSwapHistory> {
            let receive_swaps_by_script = self.receive_swaps_by_script();

            let mut data: HashMap<String, ReceiveSwapHistory> = HashMap::new();
            lbtc_script_to_history_map
                .iter()
                .for_each(|(lbtc_script, lbtc_script_history)| {
                    if let Some(imm) = receive_swaps_by_script.get(lbtc_script) {
                        data.insert(imm.swap_id.clone(), lbtc_script_history.clone());
                    }
                });
            data
        }

        fn send_chain_swaps_by_lbtc_lockup_script(
            &self,
        ) -> HashMap<LBtcScript, SendChainSwapImmutableData> {
            self.send_chain_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| (imm.lockup_script.clone(), imm))
                .collect()
        }

        fn send_chain_histories_by_swap_id(
            &self,
            lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<HistoryTxId>>,
            btc_script_to_history_map: &HashMap<BtcScript, Vec<HistoryTxId>>,
        ) -> HashMap<String, SendChainSwapHistory> {
            let send_chain_swaps_by_lbtc_script = self.send_chain_swaps_by_lbtc_lockup_script();

            let mut data: HashMap<String, SendChainSwapHistory> = HashMap::new();
            lbtc_script_to_history_map.iter().for_each(
                |(lbtc_lockup_script, lbtc_script_history)| {
                    if let Some(imm) = send_chain_swaps_by_lbtc_script.get(lbtc_lockup_script) {
                        let btc_script_history = btc_script_to_history_map
                            .get(&imm.claim_script)
                            .cloned()
                            .unwrap_or_default();

                        data.insert(
                            imm.swap_id.clone(),
                            SendChainSwapHistory {
                                lbtc_lockup_script_history: lbtc_script_history.clone(),
                                btc_claim_script_history: btc_script_history,
                            },
                        );
                    }
                },
            );
            data
        }

        fn receive_chain_swaps_by_lbtc_claim_script(
            &self,
        ) -> HashMap<LBtcScript, ReceiveChainSwapImmutableData> {
            self.receive_chain_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| (imm.claim_script.clone(), imm))
                .collect()
        }

        fn receive_chain_histories_by_swap_id(
            &self,
            lbtc_script_to_history_map: &HashMap<LBtcScript, Vec<HistoryTxId>>,
            btc_script_to_history_map: &HashMap<BtcScript, Vec<HistoryTxId>>,
        ) -> HashMap<String, ReceiveChainSwapHistory> {
            let receive_chain_swaps_by_lbtc_script =
                self.receive_chain_swaps_by_lbtc_claim_script();

            let mut data: HashMap<String, ReceiveChainSwapHistory> = HashMap::new();
            lbtc_script_to_history_map
                .iter()
                .for_each(|(lbtc_script_pk, lbtc_script_history)| {
                    if let Some(imm) = receive_chain_swaps_by_lbtc_script.get(lbtc_script_pk) {
                        let btc_script_history = btc_script_to_history_map
                            .get(&imm.lockup_script)
                            .cloned()
                            .unwrap_or_default();

                        data.insert(
                            imm.swap_id.clone(),
                            ReceiveChainSwapHistory {
                                lbtc_claim_script_history: lbtc_script_history.clone(),
                                btc_lockup_script_history: btc_script_history.clone(),
                            },
                        );
                    }
                });
            data
        }

        fn get_all_swap_lbtc_scripts(&self) -> Vec<LBtcScript> {
            let send_swap_scripts: Vec<LBtcScript> = self
                .send_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.script)
                .collect();
            let receive_swap_scripts: Vec<LBtcScript> = self
                .receive_swap_immutable_db_by_swap_id_
                .clone()
                .into_values()
                .map(|imm| imm.script)
                .collect();
            let send_chain_swap_lbtc_lockup_scripts: Vec<LBtcScript> = self
                .send_chain_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.lockup_script)
                .collect();
            let receive_chain_swap_lbtc_claim_scripts: Vec<LBtcScript> = self
                .receive_chain_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.claim_script)
                .collect();

            let mut swap_scripts = send_swap_scripts.clone();
            swap_scripts.extend(receive_swap_scripts.clone());
            swap_scripts.extend(send_chain_swap_lbtc_lockup_scripts.clone());
            swap_scripts.extend(receive_chain_swap_lbtc_claim_scripts.clone());
            swap_scripts
        }

        fn get_all_swap_btc_scripts(&self) -> Vec<BtcScript> {
            let send_chain_swap_btc_claim_scripts: Vec<BtcScript> = self
                .send_chain_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.claim_script)
                .collect();
            let receive_chain_swap_btc_lockup_scripts: Vec<BtcScript> = self
                .receive_chain_swap_immutable_db_by_swap_id
                .clone()
                .into_values()
                .map(|imm| imm.lockup_script)
                .collect();

            let mut swap_scripts = send_chain_swap_btc_claim_scripts.clone();
            swap_scripts.extend(receive_chain_swap_btc_lockup_scripts.clone());
            swap_scripts
        }
    }

    pub(crate) struct SwapsHistories {
        pub(crate) send: HashMap<String, SendSwapHistory>,
        pub(crate) receive: HashMap<String, ReceiveSwapHistory>,
        pub(crate) send_chain: HashMap<String, SendChainSwapHistory>,
        pub(crate) receive_chain: HashMap<String, ReceiveChainSwapHistory>,
    }

    impl LiquidSdk {
        pub(crate) async fn get_swaps_list(&self) -> Result<SwapsList> {
            let send_swaps = self.persister.list_send_swaps()?;
            let receive_swaps = self.persister.list_receive_swaps()?;
            let chain_swaps = self.persister.list_chain_swaps()?;
            let (send_chain_swaps, receive_chain_swaps): (Vec<ChainSwap>, Vec<ChainSwap>) =
                chain_swaps
                    .into_iter()
                    .partition(|swap| swap.direction == Direction::Outgoing);

            SwapsList::init(
                send_swaps,
                receive_swaps,
                send_chain_swaps,
                receive_chain_swaps,
            )
        }

        /// For a given [SwapList], this fetches the script histories from the chain services
        pub(crate) async fn fetch_swaps_histories(
            &self,
            swaps_list: &SwapsList,
        ) -> Result<SwapsHistories> {
            let swap_lbtc_scripts = swaps_list.get_all_swap_lbtc_scripts();

            let lbtc_script_histories = self
                .liquid_chain_service
                .lock()
                .await
                .get_scripts_history(&swap_lbtc_scripts.iter().collect::<Vec<&LBtcScript>>())
                .await?;
            let lbtc_swap_scripts_len = swap_lbtc_scripts.len();
            let lbtc_script_histories_len = lbtc_script_histories.len();
            ensure!(
                lbtc_swap_scripts_len == lbtc_script_histories_len,
                anyhow!("Got {lbtc_script_histories_len} L-BTC script histories, expected {lbtc_swap_scripts_len}")
            );
            let lbtc_script_to_history_map: HashMap<LBtcScript, Vec<HistoryTxId>> =
                swap_lbtc_scripts
                    .into_iter()
                    .zip(lbtc_script_histories.into_iter())
                    .map(|(k, v)| (k, v.into_iter().map(HistoryTxId::from).collect()))
                    .collect();

            let swap_btc_scripts = swaps_list.get_all_swap_btc_scripts();
            let btc_script_histories = self
                .bitcoin_chain_service
                .lock()
                .await
                .get_scripts_history(
                    &swap_btc_scripts
                        .iter()
                        .map(|x| x.as_script())
                        .collect::<Vec<&lwk_wollet::bitcoin::Script>>(),
                )?;
            let btc_swap_scripts_len = swap_btc_scripts.len();
            let btc_script_histories_len = btc_script_histories.len();
            ensure!(
                btc_swap_scripts_len == btc_script_histories_len,
                anyhow!("Got {btc_script_histories_len} BTC script histories, expected {btc_swap_scripts_len}")
            );
            let btc_script_to_history_map: HashMap<BtcScript, Vec<HistoryTxId>> = swap_btc_scripts
                .into_iter()
                .zip(btc_script_histories.into_iter())
                .map(|(k, v)| (k, v.into_iter().map(HistoryTxId::from).collect()))
                .collect();

            Ok(SwapsHistories {
                send: swaps_list.send_histories_by_swap_id(&lbtc_script_to_history_map),
                receive: swaps_list.receive_histories_by_swap_id(&lbtc_script_to_history_map),
                send_chain: swaps_list.send_chain_histories_by_swap_id(
                    &lbtc_script_to_history_map,
                    &btc_script_to_history_map,
                ),
                receive_chain: swaps_list.receive_chain_histories_by_swap_id(
                    &lbtc_script_to_history_map,
                    &btc_script_to_history_map,
                ),
            })
        }
    }
}
