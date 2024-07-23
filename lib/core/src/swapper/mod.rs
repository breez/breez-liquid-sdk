mod boltz_status_stream;

use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use boltz_client::elements::secp256k1_zkp::{MusigPartialSignature, MusigPubNonce};
use boltz_client::error::Error;
use boltz_client::network::electrum::ElectrumConfig;
use boltz_client::network::Chain;
use boltz_client::swaps::boltz::{
    self, BoltzApiClientV2, ChainPair, Cooperative, CreateChainRequest, CreateChainResponse,
    CreateReverseRequest, CreateReverseResponse, CreateSubmarineRequest, CreateSubmarineResponse,
    ReversePair, SubmarineClaimTxResponse, SubmarinePair, BOLTZ_MAINNET_URL_V2,
    BOLTZ_TESTNET_URL_V2,
};
use boltz_client::util::secrets::Preimage;
use boltz_client::{Amount, Bolt11Invoice, BtcSwapTx, Keypair, LBtcSwapTx, LockTime};
use boltz_status_stream::BoltzStatusStream;
use log::{debug, info};
use lwk_wollet::elements;
use serde_json::Value;
use tokio::sync::{broadcast, watch};
use url::Url;

use crate::error::{PaymentError, SdkError};
use crate::model::{
    ChainSwap, Config, Direction, LiquidNetwork, ReceiveSwap, SendSwap, SwapScriptV2, SwapTxV2,
};
use crate::utils;

#[async_trait]
pub trait ReconnectHandler: Send + Sync {
    async fn on_stream_reconnect(&self);
}

#[async_trait]
pub trait SwapperStatusStream: Send + Sync {
    async fn start(
        self: Arc<Self>,
        callback: Box<dyn ReconnectHandler>,
        shutdown: watch::Receiver<()>,
    );
    fn track_swap_id(&self, swap_id: &str) -> Result<()>;
    fn subscribe_swap_updates(&self) -> broadcast::Receiver<boltz::Update>;
}

pub trait Swapper: Send + Sync {
    /// Create a new chain swap
    fn create_chain_swap(
        &self,
        req: CreateChainRequest,
    ) -> Result<CreateChainResponse, PaymentError>;

    /// Create a new send swap
    fn create_send_swap(
        &self,
        req: CreateSubmarineRequest,
    ) -> Result<CreateSubmarineResponse, PaymentError>;

    /// Get the current rate, limits and fees for a given swap direction
    fn get_chain_pair(&self, direction: Direction) -> Result<Option<ChainPair>, PaymentError>;

    /// Get the current rate, limits and fees for both swap directions
    fn get_chain_pairs(&self) -> Result<(Option<ChainPair>, Option<ChainPair>), PaymentError>;

    /// Get a submarine pair information
    fn get_submarine_pairs(&self) -> Result<Option<SubmarinePair>, PaymentError>;

    /// Prepare the chain swap refund
    fn prepare_chain_swap_refund(
        &self,
        swap: &ChainSwap,
        output_address: &str,
        sat_per_vbyte: f32,
    ) -> Result<(u32, u64), SdkError>;

    /// Refund a cooperatively chain swap  
    fn refund_chain_swap_cooperative(
        &self,
        swap: &ChainSwap,
        output_address: &str,
        broadcast_fees_sat: u64,
    ) -> Result<String, PaymentError>;

    /// Refund a cooperatively send swap
    fn refund_send_swap_cooperative(
        &self,
        swap: &SendSwap,
        output_address: &str,
        broadcast_fees_sat: u64,
    ) -> Result<String, PaymentError>;

    /// Refund non-cooperatively chain swap
    fn refund_chain_swap_non_cooperative(
        &self,
        swap: &ChainSwap,
        broadcast_fees_sat: u64,
        output_address: &str,
        current_height: u32,
    ) -> Result<String, PaymentError>;

    /// Refund non-cooperatively send swap
    fn refund_send_swap_non_cooperative(
        &self,
        swap: &SendSwap,
        broadcast_fees_sat: u64,
        output_address: &str,
        current_height: u32,
    ) -> Result<String, PaymentError>;

    /// Get send swap claim tx details which includes the preimage as a proof of payment.
    /// It is used to validate the preimage before claiming which is the reason why we need to separate
    /// the claim into two steps.
    fn get_send_claim_tx_details(
        &self,
        swap: &SendSwap,
    ) -> Result<SubmarineClaimTxResponse, PaymentError>;

    /// Claim chain swap.
    fn claim_chain_swap(&self, swap: &ChainSwap) -> Result<String, PaymentError>;

    /// Claim send swap cooperatively. Here the remote swapper is the one that claims.
    /// We are helping to use key spend path for cheaper fees.
    fn claim_send_swap_cooperative(
        &self,
        swap: &SendSwap,
        claim_tx_response: SubmarineClaimTxResponse,
        refund_address: &str,
    ) -> Result<(), PaymentError>;

    /// Create a new receive swap
    fn create_receive_swap(
        &self,
        req: CreateReverseRequest,
    ) -> Result<CreateReverseResponse, PaymentError>;

    /// Get a reverse pair information
    fn get_reverse_swap_pairs(&self) -> Result<Option<ReversePair>, PaymentError>;

    /// Claim receive swap. Here the local swapper is the one that claims.
    fn claim_receive_swap(
        &self,
        swap: &ReceiveSwap,
        claim_address: String,
    ) -> Result<String, PaymentError>;

    /// Chain broadcast
    fn broadcast_tx(&self, chain: Chain, tx_hex: &str) -> Result<Value, PaymentError>;

    fn create_status_stream(&self) -> Box<dyn SwapperStatusStream>;

    /// Look for a valid Magic Routing Hint. If found, validate it and extract the BIP21 info (amount, address).
    fn check_for_mrh(&self, invoice: &str) -> Result<Option<(String, f64)>, PaymentError>;
}

pub struct BoltzSwapper {
    client: BoltzApiClientV2,
    boltz_url: String,
    referral_id: Option<String>,
    config: Config,
    liquid_electrum_config: ElectrumConfig,
    bitcoin_electrum_config: ElectrumConfig,
}

impl BoltzSwapper {
    pub fn new(config: Config, swapper_proxy_url: Option<String>) -> BoltzSwapper {
        let (boltz_api_base_url, referral_id) = match &config.network {
            LiquidNetwork::Testnet => (None, None),
            LiquidNetwork::Mainnet => match &swapper_proxy_url {
                Some(swapper_proxy_url) => Url::parse(swapper_proxy_url)
                    .map(|url| match url.query() {
                        None => (None, None),
                        Some(query) => {
                            let api_base_url =
                                url.domain().map(|domain| format!("https://{domain}/v2"));
                            let parts: Vec<String> = query.split('=').map(Into::into).collect();
                            let referral_id = parts.get(1).cloned();
                            (api_base_url, referral_id)
                        }
                    })
                    .unwrap_or_default(),
                None => (None, None),
            },
        };

        let boltz_url = boltz_api_base_url.unwrap_or(
            match config.network {
                LiquidNetwork::Mainnet => BOLTZ_MAINNET_URL_V2,
                LiquidNetwork::Testnet => BOLTZ_TESTNET_URL_V2,
            }
            .to_string(),
        );

        BoltzSwapper {
            client: BoltzApiClientV2::new(&boltz_url),
            boltz_url,
            referral_id,
            config: config.clone(),
            liquid_electrum_config: ElectrumConfig::new(
                config.network.into(),
                &config.liquid_electrum_url,
                true,
                true,
                100,
            ),
            bitcoin_electrum_config: ElectrumConfig::new(
                config.network.as_bitcoin_chain(),
                &config.bitcoin_electrum_url,
                true,
                true,
                100,
            ),
        }
    }

    fn new_refund_tx(
        &self,
        swap_id: String,
        swap_script: SwapScriptV2,
        refund_address: &String,
    ) -> Result<SwapTxV2, SdkError> {
        let swap_tx = match swap_script {
            SwapScriptV2::Bitcoin(swap_script) => SwapTxV2::Bitcoin(BtcSwapTx::new_refund(
                swap_script.clone(),
                refund_address,
                &self.bitcoin_electrum_config,
            )?),
            SwapScriptV2::Liquid(swap_script) => SwapTxV2::Liquid(LBtcSwapTx::new_refund(
                swap_script.clone(),
                refund_address,
                &self.liquid_electrum_config,
                self.boltz_url.clone(),
                swap_id,
            )?),
        };
        Ok(swap_tx)
    }

    fn validate_send_swap_preimage(
        &self,
        swap_id: &str,
        invoice: &str,
        preimage: &str,
    ) -> Result<(), PaymentError> {
        Self::verify_payment_hash(preimage, invoice)?;
        info!("Preimage is valid for Send Swap {swap_id}");
        Ok(())
    }

    fn verify_payment_hash(preimage: &str, invoice: &str) -> Result<(), PaymentError> {
        let preimage = Preimage::from_str(preimage)?;
        let preimage_hash = preimage.sha256.to_string();
        let invoice =
            Bolt11Invoice::from_str(invoice).map_err(|e| Error::Generic(e.to_string()))?;
        let invoice_payment_hash = invoice.payment_hash();

        (invoice_payment_hash.to_string() == preimage_hash)
            .then_some(())
            .ok_or(PaymentError::InvalidPreimage)
    }

    fn get_claim_partial_sig(
        &self,
        swap: &ChainSwap,
    ) -> Result<(MusigPartialSignature, MusigPubNonce), PaymentError> {
        let refund_keypair = swap.get_refund_keypair()?;
        let lockup_swap_script = swap.get_lockup_swap_script()?;

        // Create a temporary refund tx to an address from the swap lockup chain
        // We need it to calculate the musig partial sig for the claim tx from the other chain
        let lockup_address = &swap.lockup_address;
        let refund_tx_wrapper =
            self.new_refund_tx(swap.id.clone(), lockup_swap_script, lockup_address)?;

        let claim_tx_details = self.client.get_chain_claim_tx_details(&swap.id)?;
        match swap.direction {
            Direction::Incoming => refund_tx_wrapper.as_bitcoin_tx()?.partial_sign(
                &refund_keypair,
                &claim_tx_details.pub_nonce,
                &claim_tx_details.transaction_hash,
            ),
            Direction::Outgoing => refund_tx_wrapper.as_liquid_tx()?.partial_sign(
                &refund_keypair,
                &claim_tx_details.pub_nonce,
                &claim_tx_details.transaction_hash,
            ),
        }
        .map_err(Into::into)
    }

    fn claim_outgoing_chain_swap(&self, swap: &ChainSwap) -> Result<String, PaymentError> {
        let claim_keypair = swap.get_claim_keypair()?;
        let claim_swap_script = swap.get_claim_swap_script()?.as_bitcoin_script()?;
        let claim_tx_wrapper = BtcSwapTx::new_claim(
            claim_swap_script,
            swap.claim_address.clone(),
            &self.bitcoin_electrum_config,
        )?;

        let (partial_sig, pub_nonce) = self.get_claim_partial_sig(swap)?;

        let claim_tx = claim_tx_wrapper.sign_claim(
            &claim_keypair,
            &Preimage::from_str(&swap.preimage)?,
            swap.claim_fees_sat,
            Some(Cooperative {
                boltz_api: &self.client,
                swap_id: swap.id.clone(),
                pub_nonce: Some(pub_nonce),
                partial_sig: Some(partial_sig),
            }),
        )?;
        debug!("Claim Tx {:?}", claim_tx);

        let claim_tx_id = claim_tx_wrapper
            .broadcast(&claim_tx, &self.bitcoin_electrum_config)?
            .to_string();
        Ok(claim_tx_id)
    }

    fn claim_incoming_chain_swap(&self, swap: &ChainSwap) -> Result<String, PaymentError> {
        let claim_keypair = swap.get_claim_keypair()?;
        let swap_script = swap.get_claim_swap_script()?.as_liquid_script()?;
        let claim_tx_wrapper = LBtcSwapTx::new_claim(
            swap_script,
            swap.claim_address.clone(),
            &self.liquid_electrum_config,
            self.boltz_url.clone(),
            swap.id.clone(),
        )?;

        let (partial_sig, pub_nonce) = self.get_claim_partial_sig(swap)?;

        let claim_tx = claim_tx_wrapper.sign_claim(
            &claim_keypair,
            &Preimage::from_str(&swap.preimage)?,
            Amount::from_sat(swap.claim_fees_sat),
            Some(Cooperative {
                boltz_api: &self.client,
                swap_id: swap.id.clone(),
                pub_nonce: Some(pub_nonce),
                partial_sig: Some(partial_sig),
            }),
        )?;
        debug!("Claim Tx {:?}", claim_tx);
        let claim_tx_id = claim_tx_wrapper.broadcast(
            &claim_tx,
            &self.liquid_electrum_config,
            Some((&self.client, self.config.network.into())),
        )?;
        Ok(claim_tx_id)
    }

    fn refund_swap_cooperative(
        &self,
        swap_id: String,
        swap_script: SwapScriptV2,
        refund_keypair: &Keypair,
        refund_address: &str,
        broadcast_fees_sat: u64,
    ) -> Result<String, PaymentError> {
        info!("Initiating cooperative refund for Swap {}", &swap_id);
        let is_cooperative = Some(Cooperative {
            boltz_api: &self.client,
            swap_id: swap_id.clone(),
            pub_nonce: None,
            partial_sig: None,
        });
        let refund_tx_id = match swap_script.clone() {
            SwapScriptV2::Bitcoin(_) => {
                let refund_tx = self
                    .new_refund_tx(swap_id.clone(), swap_script, &refund_address.into())?
                    .as_bitcoin_tx()?;
                let signed_tx =
                    refund_tx.sign_refund(refund_keypair, broadcast_fees_sat, is_cooperative)?;
                refund_tx
                    .broadcast(&signed_tx, &self.bitcoin_electrum_config)?
                    .to_string()
            }
            SwapScriptV2::Liquid(_) => {
                let refund_tx = self
                    .new_refund_tx(swap_id.clone(), swap_script, &refund_address.into())?
                    .as_liquid_tx()?;
                let signed_tx = refund_tx.sign_refund(
                    refund_keypair,
                    Amount::from_sat(broadcast_fees_sat),
                    is_cooperative,
                )?;
                // We attempt lowball broadcast when constructing the tx cooperatively
                let lowball = Some((&self.client, self.config.network.into()));
                refund_tx.broadcast(&signed_tx, &self.liquid_electrum_config, lowball)?
            }
        };
        info!(
            "Successfully broadcast cooperative refund for Swap {}",
            &swap_id
        );
        Ok(refund_tx_id)
    }

    fn refund_swap_non_cooperative(
        &self,
        swap_id: String,
        swap_script: SwapScriptV2,
        refund_keypair: &Keypair,
        broadcast_fees_sat: u64,
        refund_address: &str,
        current_height: u32,
    ) -> Result<String, PaymentError> {
        let refund_tx_id = match swap_script.clone() {
            SwapScriptV2::Bitcoin(script) => {
                let locktime_from_height =
                    LockTime::from_height(current_height).map_err(|e| PaymentError::Generic {
                        err: format!("Error getting locktime from height {current_height:?}: {e}",),
                    })?;

                info!("locktime info: locktime_from_height = {locktime_from_height:?},  swap_script.locktime = {:?}", script.locktime);
                if !script.locktime.is_implied_by(locktime_from_height) {
                    return Err(PaymentError::Generic {
                        err: format!(
                            "Cannot refund non-cooperatively. Lock time not elapsed yet. Current tip: {:?}. Script lock time: {:?}",
                            locktime_from_height, script.locktime
                        )
                    });
                }

                let refund_tx = self
                    .new_refund_tx(swap_id.clone(), swap_script, &refund_address.into())?
                    .as_bitcoin_tx()?;
                let signed_tx = refund_tx.sign_refund(refund_keypair, broadcast_fees_sat, None)?;
                refund_tx
                    .broadcast(&signed_tx, &self.bitcoin_electrum_config)?
                    .to_string()
            }
            SwapScriptV2::Liquid(script) => {
                let locktime_from_height = elements::LockTime::from_height(current_height)
                    .map_err(|e| PaymentError::Generic {
                        err: format!("Cannot convert current block height to lock time: {e:?}"),
                    })?;

                info!("locktime info: locktime_from_height = {locktime_from_height:?},  swap_script.locktime = {:?}",  script.locktime);
                if !utils::is_locktime_expired(locktime_from_height, script.locktime) {
                    return Err(PaymentError::Generic {
                        err: format!(
                            "Cannot refund non-cooperatively. Lock time not elapsed yet. Current tip: {:?}. Script lock time: {:?}",
                            locktime_from_height, script.locktime
                        )
                    });
                }

                let refund_tx = self
                    .new_refund_tx(swap_id.clone(), swap_script, &refund_address.into())?
                    .as_liquid_tx()?;
                let signed_tx = refund_tx.sign_refund(
                    refund_keypair,
                    Amount::from_sat(broadcast_fees_sat),
                    None,
                )?;
                // We cannot broadcast lowball when constructing a non-cooperative tx
                refund_tx.broadcast(&signed_tx, &self.liquid_electrum_config, None)?
            }
        };
        info!(
            "Successfully broadcast non-cooperative refund for Swap {}",
            swap_id
        );
        Ok(refund_tx_id)
    }
}

impl Swapper for BoltzSwapper {
    /// Create a new chain swap
    fn create_chain_swap(
        &self,
        req: CreateChainRequest,
    ) -> Result<CreateChainResponse, PaymentError> {
        let modified_req = CreateChainRequest {
            referral_id: self.referral_id.clone(),
            ..req.clone()
        };
        Ok(self.client.post_chain_req(modified_req)?)
    }

    /// Create a new send swap
    fn create_send_swap(
        &self,
        req: CreateSubmarineRequest,
    ) -> Result<CreateSubmarineResponse, PaymentError> {
        let modified_req = CreateSubmarineRequest {
            referral_id: self.referral_id.clone(),
            ..req.clone()
        };
        Ok(self.client.post_swap_req(&modified_req)?)
    }

    fn get_chain_pair(&self, direction: Direction) -> Result<Option<ChainPair>, PaymentError> {
        let pairs = self.client.get_chain_pairs()?;
        let pair = match direction {
            Direction::Incoming => pairs.get_btc_to_lbtc_pair(),
            Direction::Outgoing => pairs.get_lbtc_to_btc_pair(),
        };
        Ok(pair)
    }

    fn get_chain_pairs(&self) -> Result<(Option<ChainPair>, Option<ChainPair>), PaymentError> {
        let pairs = self.client.get_chain_pairs()?;
        let pair_outgoing = pairs.get_lbtc_to_btc_pair();
        let pair_incoming = pairs.get_btc_to_lbtc_pair();
        Ok((pair_outgoing, pair_incoming))
    }

    /// Get a submarine pair information
    fn get_submarine_pairs(&self) -> Result<Option<SubmarinePair>, PaymentError> {
        Ok(self.client.get_submarine_pairs()?.get_lbtc_to_btc_pair())
    }

    /// Prepare the chain swap refund
    fn prepare_chain_swap_refund(
        &self,
        swap: &ChainSwap,
        output_address: &str,
        sat_per_vbyte: f32,
    ) -> Result<(u32, u64), SdkError> {
        let refund_keypair = swap.get_refund_keypair()?;
        let preimage = Preimage::from_str(&swap.preimage)?;
        let swap_script = swap.get_lockup_swap_script()?;
        let refund_tx_vsize = match swap.direction {
            Direction::Incoming => {
                let refund_tx = self
                    .new_refund_tx(swap.id.clone(), swap_script, &output_address.into())?
                    .as_bitcoin_tx()?;
                refund_tx.size(&refund_keypair, &preimage)? as u32
            }
            Direction::Outgoing => {
                let refund_tx = self
                    .new_refund_tx(swap.id.clone(), swap_script, &output_address.into())?
                    .as_liquid_tx()?;
                refund_tx.size(&refund_keypair, &preimage)? as u32
            }
        };
        let refund_tx_fee_sat = (refund_tx_vsize as f32 * sat_per_vbyte).ceil() as u64;
        Ok((refund_tx_vsize, refund_tx_fee_sat))
    }

    /// Refund a cooperatively chain swap  
    fn refund_chain_swap_cooperative(
        &self,
        swap: &ChainSwap,
        output_address: &str,
        broadcast_fees_sat: u64,
    ) -> Result<String, PaymentError> {
        let refund_keypair = swap.get_refund_keypair()?;
        let swap_script = swap.get_lockup_swap_script()?;
        info!("Initiating cooperative refund for Chain Swap {}", &swap.id);
        self.refund_swap_cooperative(
            swap.id.clone(),
            swap_script,
            &refund_keypair,
            output_address,
            broadcast_fees_sat,
        )
    }

    /// Refund a cooperatively send swap
    fn refund_send_swap_cooperative(
        &self,
        swap: &SendSwap,
        output_address: &str,
        broadcast_fees_sat: u64,
    ) -> Result<String, PaymentError> {
        info!("Initiating cooperative refund for Send Swap {}", &swap.id);
        let swap_script = SwapScriptV2::Liquid(swap.get_swap_script()?);
        let refund_keypair = swap.get_refund_keypair()?;
        self.refund_swap_cooperative(
            swap.id.clone(),
            swap_script,
            &refund_keypair,
            output_address,
            broadcast_fees_sat,
        )
    }

    /// Refund non-cooperatively chain swap
    fn refund_chain_swap_non_cooperative(
        &self,
        swap: &ChainSwap,
        broadcast_fees_sat: u64,
        output_address: &str,
        current_height: u32,
    ) -> Result<String, PaymentError> {
        info!(
            "Initiating non-cooperative refund for Chain Swap {}",
            &swap.id
        );
        let refund_keypair = swap.get_refund_keypair()?;
        let swap_script = swap.get_lockup_swap_script()?;
        self.refund_swap_non_cooperative(
            swap.id.clone(),
            swap_script,
            &refund_keypair,
            broadcast_fees_sat,
            output_address,
            current_height,
        )
    }

    /// Refund non-cooperatively send swap
    fn refund_send_swap_non_cooperative(
        &self,
        swap: &SendSwap,
        broadcast_fees_sat: u64,
        output_address: &str,
        current_height: u32,
    ) -> Result<String, PaymentError> {
        info!(
            "Initiating non-cooperative refund for Send Swap {}",
            &swap.id
        );
        let swap_script = SwapScriptV2::Liquid(swap.get_swap_script()?);
        let refund_keypair = swap.get_refund_keypair()?;
        self.refund_swap_non_cooperative(
            swap.id.clone(),
            swap_script,
            &refund_keypair,
            broadcast_fees_sat,
            output_address,
            current_height,
        )
    }

    /// Get claim tx details which includes the preimage as a proof of payment.
    /// It is used to validate the preimage before claiming which is the reason why we need to separate
    /// the claim into two steps.
    fn get_send_claim_tx_details(
        &self,
        swap: &SendSwap,
    ) -> Result<SubmarineClaimTxResponse, PaymentError> {
        let claim_tx_response = self.client.get_submarine_claim_tx_details(&swap.id)?;
        info!("Received claim tx details: {:?}", &claim_tx_response);

        self.validate_send_swap_preimage(&swap.id, &swap.invoice, &claim_tx_response.preimage)?;
        Ok(claim_tx_response)
    }

    /// Claim chain swap.
    fn claim_chain_swap(&self, swap: &ChainSwap) -> Result<String, PaymentError> {
        let claim_tx_id = match swap.direction {
            Direction::Incoming => self.claim_incoming_chain_swap(swap),
            Direction::Outgoing => self.claim_outgoing_chain_swap(swap),
        }?;
        info!(
            "Successfully broadcast claim tx {claim_tx_id} for Chain Swap {}",
            swap.id
        );
        Ok(claim_tx_id)
    }

    /// Claim send swap cooperatively. Here the remote swapper is the one that claims.
    /// We are helping to use key spend path for cheaper fees.
    fn claim_send_swap_cooperative(
        &self,
        swap: &SendSwap,
        claim_tx_response: SubmarineClaimTxResponse,
        refund_address: &str,
    ) -> Result<(), PaymentError> {
        let swap_id = &swap.id;
        let keypair = swap.get_refund_keypair()?;
        let swap_script = SwapScriptV2::Liquid(swap.get_swap_script()?);
        let refund_tx = self
            .new_refund_tx(swap.id.clone(), swap_script, &refund_address.into())?
            .as_liquid_tx()?;

        self.validate_send_swap_preimage(swap_id, &swap.invoice, &claim_tx_response.preimage)?;

        let (partial_sig, pub_nonce) = refund_tx.partial_sign(
            &keypair,
            &claim_tx_response.pub_nonce,
            &claim_tx_response.transaction_hash,
        )?;

        self.client.post_submarine_claim_tx_details(
            &swap_id.to_string(),
            pub_nonce,
            partial_sig,
        )?;
        info!("Successfully sent claim details for swap-in {swap_id}");
        Ok(())
    }

    // Create a new receive swap
    fn create_receive_swap(
        &self,
        req: CreateReverseRequest,
    ) -> Result<CreateReverseResponse, PaymentError> {
        let modified_req = CreateReverseRequest {
            referral_id: self.referral_id.clone(),
            ..req.clone()
        };
        Ok(self.client.post_reverse_req(modified_req)?)
    }

    // Get a reverse pair information
    fn get_reverse_swap_pairs(&self) -> Result<Option<ReversePair>, PaymentError> {
        Ok(self.client.get_reverse_pairs()?.get_btc_to_lbtc_pair())
    }

    /// Claim receive swap. Here the local swapper is the one that claims.
    fn claim_receive_swap(
        &self,
        swap: &ReceiveSwap,
        claim_address: String,
    ) -> Result<String, PaymentError> {
        let swap_script = swap.get_swap_script()?;
        let swap_id = &swap.id;
        let claim_tx_wrapper = LBtcSwapTx::new_claim(
            swap_script,
            claim_address,
            &self.liquid_electrum_config,
            self.boltz_url.clone(),
            swap.id.clone(),
        )?;

        let is_cooperative = Some(Cooperative {
            boltz_api: &self.client,
            swap_id: swap.id.clone(),
            pub_nonce: None,
            partial_sig: None,
        });
        let claim_tx = claim_tx_wrapper.sign_claim(
            &swap.get_claim_keypair()?,
            &Preimage::from_str(&swap.preimage)?,
            Amount::from_sat(swap.claim_fees_sat),
            is_cooperative,
        )?;

        let claim_tx_id = claim_tx_wrapper.broadcast(
            &claim_tx,
            &self.liquid_electrum_config,
            Some((&self.client, self.config.network.into())),
        )?;
        info!("Successfully broadcast claim tx {claim_tx_id} for Receive Swap {swap_id}");
        debug!("Claim Tx {:?}", claim_tx);
        Ok(claim_tx_id)
    }

    fn broadcast_tx(&self, chain: Chain, tx_hex: &str) -> Result<Value, PaymentError> {
        Ok(self.client.broadcast_tx(chain, &tx_hex.into())?)
    }

    fn create_status_stream(&self) -> Box<dyn SwapperStatusStream> {
        Box::new(BoltzStatusStream::new(&self.boltz_url))
    }

    fn check_for_mrh(&self, invoice: &str) -> Result<Option<(String, f64)>, PaymentError> {
        boltz_client::swaps::magic_routing::check_for_mrh(
            &self.client,
            invoice,
            self.config.network.into(),
        )
        .map_err(Into::into)
    }
}
