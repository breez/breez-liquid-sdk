use anyhow::{anyhow, Result};
use bitcoincore_rpc::{
    bitcoin::{absolute::LockTime, Address, Amount, Network},
    bitcoincore_rpc_json::ListReceivedByAddressResult,
    RpcApi,
};
use cln_rpc::{
    model::requests::{self, NewaddrAddresstype},
    ClnRpc, Response,
};
use elements::{
    address::Address as EAddress,
    opcodes::all::{OP_CHECKSIG, OP_CLTV, OP_DROP, OP_ELSE, OP_ENDIF, OP_EQUAL, OP_HASH160, OP_IF},
    script::Builder,
    AddressParams,
};
use log::debug;

use std::{
    env,
    path::Path,
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use lwk_common::{singlesig_desc, Singlesig};
use lwk_signer::{
    bip39::{Language, Mnemonic},
    SwSigner,
};
use lwk_wollet::{
    full_scan_with_electrum_client, BlockchainBackend, ElectrumClient, ElectrumUrl,
    ElementsNetwork, NoPersist, Wollet,
};

use crate::utils::check_var_set;

const BLOCKSTREAM_ELECTRUM_URL: &str = "blockstream.info:465";

const FUNDING_ATTEMPTS_MAX: u8 = 20;
const CONFIRMATION_SLEEP_TIME: u64 = 45;

const SWAPPER_ENV_CFG: ParticipantConfig = ParticipantConfig {
    mnemonic_env: "SWAPPER_MNEMONIC",
    lnsocket_env: "SWAPPER_LNSOCKET",
};

const USER_ENV_CFG: ParticipantConfig = ParticipantConfig {
    mnemonic_env: "USER_MNEMONIC",
    lnsocket_env: "USER_LNSOCKET",
};

pub struct Participant {
    pub signer: SwSigner,
    pub liquid_wallet: Arc<Mutex<Wollet>>,
    pub ln_rpc: Arc<async_mutex::Mutex<ClnRpc>>,
}

pub struct Simulator {
    pub swapper: Participant,
    pub user: Participant,
    pub electrum_client: Arc<Mutex<ElectrumClient>>,
    pub bitcoind_client: Arc<bitcoincore_rpc::Client>,
}

pub enum ParticipantType {
    Swapper,
    User,
}

impl ToString for ParticipantType {
    fn to_string(&self) -> String {
        match self {
            ParticipantType::Swapper => "swapper",
            ParticipantType::User => "user",
        }
        .to_string()
    }
}

struct ParticipantConfig {
    mnemonic_env: &'static str,
    lnsocket_env: &'static str,
}

impl Simulator {
    pub async fn try_init() -> Result<Self> {
        let swapper = Simulator::init_participant(ParticipantType::Swapper).await?;
        let user = Simulator::init_participant(ParticipantType::User).await?;
        let electrum_client = Arc::new(Mutex::new(ElectrumClient::new(&ElectrumUrl::new(
            BLOCKSTREAM_ELECTRUM_URL,
            true,
            true,
        ))?));
        let bitcoind_client = Arc::new(bitcoincore_rpc::Client::new(
            &env::var("BITCOIND_ADDR").unwrap_or("127.0.0.1:18444".to_string()),
            bitcoincore_rpc::Auth::UserPass(
                env::var("BITCOIND_AUTH_USER")?,
                env::var("BITCOIND_AUTH_PASSWORD")?,
            ),
        )?);

        Ok(Simulator {
            swapper,
            user,
            electrum_client,
            bitcoind_client,
        })
    }

    fn init_liquid(cfg: &ParticipantConfig) -> Result<(SwSigner, Wollet)> {
        let mnemonic = match env::var(cfg.mnemonic_env) {
            Ok(m) => Mnemonic::from_str(&m)?,
            Err(_) => dbg!(Mnemonic::generate_in(Language::English, 24)?),
        }
        .to_string();

        let signer = SwSigner::new(&mnemonic, false)?;
        let desc = singlesig_desc(
            &signer,
            Singlesig::ShWpkh,
            lwk_common::DescriptorBlindingKey::Elip151,
            false,
        )
        .map_err(|_| anyhow!("Expected valid descriptor"))?;

        Ok((
            signer,
            Wollet::new(ElementsNetwork::LiquidTestnet, NoPersist::new(), &desc)?,
        ))
    }

    async fn init_ln(cfg: &ParticipantConfig) -> Result<ClnRpc> {
        ClnRpc::new(Path::new(&env::var(cfg.lnsocket_env)?)).await
    }

    pub async fn init_participant(p_type: ParticipantType) -> Result<Participant> {
        let cfg: &ParticipantConfig = match p_type {
            ParticipantType::Swapper => &SWAPPER_ENV_CFG,
            ParticipantType::User => &USER_ENV_CFG,
        };

        let (signer, liquid_wallet) = Simulator::init_liquid(cfg)?;
        let ln_rpc = Simulator::init_ln(cfg).await?;

        Ok(Participant {
            signer,
            ln_rpc: Arc::new(async_mutex::Mutex::new(ln_rpc)),
            liquid_wallet: Arc::new(Mutex::new(liquid_wallet)),
        })
    }

    pub fn scan_liquid_wallet(&self, wallet: &mut Wollet) -> Result<()> {
        let mut client = self.electrum_client.lock().unwrap();
        full_scan_with_electrum_client(wallet, &mut client)?;
        Ok(())
    }

    pub fn fund_liquid(&self, wallet: Arc<Mutex<Wollet>>) -> Result<u64> {
        let mut wallet = wallet.lock().unwrap();

        let address = wallet.address(None)?.address().to_string();
        debug!("Funding liquid wallet at address {address}...");

        // Alternatively, one could also check whether or not the balance has increased
        // by prompting multiple rescans
        let mut attempts = 0;
        while attempts < FUNDING_ATTEMPTS_MAX {
            let body = reqwest::blocking::get(format!(
                "https://liquidtestnet.com/faucet?address={address}&action=lbtc"
            ))?
            .text()?;

            // Pretty rudimental rest, but unfortunately the API does not allow us to check this in any
            // other way
            if body.contains(&address[1..9]) {
                break;
            }

            debug!("Could not contact faucet. Reattempting in 5 seconds...");
            attempts += 1;
            thread::sleep(Duration::from_secs(5));
        }

        if attempts == FUNDING_ATTEMPTS_MAX {
            panic!("Could not fund liquid wallet! Max retries exceeded.");
        }

        debug!("Wallet has been funded! Awaiting confirmation (sleeping for {CONFIRMATION_SLEEP_TIME} seconds)...");
        self.liquid_await_n_confirmations(1)?;

        let total_balance = self.get_liquid_balance(&mut wallet)?;

        self.get_liquid_balance(&mut wallet)
    }

    pub async fn fund_ln(
        &self,
        rpc: Arc<async_mutex::Mutex<ClnRpc>>,
        amount_sat: u64,
    ) -> Result<u64> {
        let mut cln_rpc = rpc.lock().await;
        let bitcoind_rpc = &self.bitcoind_client;

        // Create bitcoind address (if necessary) and mine funds
        if check_var_set("FUND_BTC") {
            if bitcoind_rpc.list_wallets()?.is_empty() {
                debug!("Creating wallet 'default'...");
                bitcoind_rpc.create_wallet("default", None, None, None, None)?;
            }

            debug!("Adding an address to wallet 'default'...");
            let btc_address = match bitcoind_rpc
                .list_received_by_address(None, Some(1), Some(true), None)?
                .first()
            {
                Some(ListReceivedByAddressResult { address, .. }) => address.clone(),
                None => bitcoind_rpc.get_new_address(None, None)?,
            }
            .require_network(Network::Regtest)?;

            debug!("Mining 150 blocks...");
            bitcoind_rpc.generate_to_address(150, &btc_address)?;

            while bitcoind_rpc.get_balance(Some(1), None)?.to_sat() == 0 {
                debug!("Checking if block has been mined...");
                thread::sleep(Duration::from_secs(5));
            }
            debug!("Funds added successfully!");
        }

        debug!("Creating new LN address...");
        // Create ln address and send funds
        let response = cln_rpc
            .call(cln_rpc::Request::NewAddr(requests::NewaddrRequest {
                addresstype: Some(NewaddrAddresstype::BECH32),
            }))
            .await?;

        // TODO turn unwrapping into macro
        let ln_address = match response {
            Response::NewAddr(r) => r.bech32.expect("Expecting valid bech32 address"),
            _ => return Err(anyhow!("Received invalid response type")),
        };
        let ln_address = Address::from_str(&ln_address)?.require_network(Network::Regtest)?;

        debug!("Create success! ({}) Funding address...", &ln_address);
        bitcoind_rpc.send_to_address(
            &ln_address,
            Amount::from_sat(amount_sat),
            None,
            None,
            None,
            None,
            None,
            None,
        )?;
        debug!("Fund successful! Verifying on CLN...");

        // Wait until funds are registered
        let mut funds = 0;
        while funds == 0 {
            funds = self.get_ln_balance(&mut cln_rpc).await?;
            thread::sleep(Duration::from_secs(5));
        }

        debug!("Node funded successfully! ({}) msat", funds);

        Ok(funds)
    }

    pub async fn get_ln_balance(&self, rpc: &mut ClnRpc) -> Result<u64> {
        let response = rpc
            .call(cln_rpc::Request::ListFunds(requests::ListfundsRequest {
                spent: Some(false),
            }))
            .await?;

        match response {
            Response::ListFunds(res) => Ok(res
                .outputs
                .iter()
                .map(|output| output.amount_msat.msat())
                .sum()),
            _ => Err(anyhow!("Could not retrieve funds")),
        }
    }

    pub fn get_liquid_balance(&self, wallet: &mut Wollet) -> Result<u64> {
        self.scan_liquid_wallet(wallet)?;
        Ok(wallet.balance()?.values().sum::<u64>())
    }

    // Taken from https://github.com/SatoshiPortal/boltz-rust/blob/trunk/src/swaps/liquid.rs
    pub fn create_redeem_address(payment_hash: String) -> Result<String> {
        let receiver_pubkey = todo!();
        let locktime: LockTime = todo!();
        let sender_pubkey = todo!();

        let script = Builder::new()
            .push_opcode(OP_HASH160)
            .push_slice(payment_hash.as_bytes())
            .push_opcode(OP_EQUAL)
            .push_opcode(OP_IF)
            .push_key(&receiver_pubkey)
            .push_opcode(OP_ELSE)
            .push_int(locktime.to_consensus_u32() as i64)
            .push_opcode(OP_CLTV)
            .push_opcode(OP_DROP)
            .push_key(&sender_pubkey)
            .push_opcode(OP_ENDIF)
            .push_opcode(OP_CHECKSIG)
            .into_script();

        let blinding_key = todo!();

        Ok(EAddress::p2wsh(&script, blinding_key, &AddressParams::LIQUID_TESTNET).to_string())
    }

    pub fn liquid_await_n_confirmations(&self, n: u32) -> Result<()> {
        let mut electrum_client = self.electrum_client.lock().unwrap();
        let mut current_tip = electrum_client.tip()?.height;

        while current_tip < current_tip + n {
            current_tip = electrum_client.tip()?.height;
            thread::sleep(Duration::from_secs(CONFIRMATION_SLEEP_TIME));
        }

        Ok(())
    }
}
