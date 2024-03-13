mod commands;
mod persist;

use anyhow::{anyhow, Result};
use lwk_common::{singlesig_desc, Singlesig};
use lwk_signer::SwSigner;
use std::{fs, path::PathBuf, sync::Arc};

use clap::Parser;
use log::{error, info};
use rustyline::{error::ReadlineError, hint::HistoryHinter, Editor};

use breez_sdk_liquid::{Network, Wallet, WalletOptions};
use commands::{handle_command, CliHelper, Command};
use persist::CliPersistence;

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[clap(name = "data_dir", short = 'd', long = "data_dir")]
    pub(crate) data_dir: Option<String>,
}

fn show_results(res: Result<String>) {
    match res {
        Ok(inner) => println!("{inner}"),
        Err(err) => eprintln!("Error: {err}"),
    }
}

fn init_persistence(args: &Args) -> Result<CliPersistence> {
    let data_dir = args.data_dir.clone().unwrap_or(".data".to_string());
    let data_dir = PathBuf::from(&data_dir);

    fs::create_dir_all(&data_dir)?;

    Ok(CliPersistence { data_dir })
}

fn init_wallet(persistence: &CliPersistence) -> Result<Arc<Wallet>> {
    let mnemonic = persistence.get_or_create_mnemonic()?;
    let signer = SwSigner::new(&mnemonic.to_string(), false)?;
    let desc = singlesig_desc(
        &signer,
        Singlesig::Wpkh,
        lwk_common::DescriptorBlindingKey::Elip151,
        false,
    )
    .expect("Expected valid descriptor");

    Wallet::new(WalletOptions {
        signer,
        desc,
        electrum_url: None,
        db_root_dir: None,
        network: Network::LiquidTestnet,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let persistence = init_persistence(&args)?;
    let history_file = &persistence.history_file();

    let rl = &mut Editor::new()?;
    rl.set_helper(Some(CliHelper {
        hinter: HistoryHinter {},
    }));

    if rl.load_history(history_file).is_err() {
        info!("No history found");
    }

    let wallet = init_wallet(&persistence)?;

    loop {
        let readline = rl.readline("breez-liquid> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let mut vec: Vec<&str> = line.as_str().split_whitespace().collect();
                vec.insert(0, "");
                let cli_res = Command::try_parse_from(vec);
                if cli_res.is_err() {
                    println!("{}", cli_res.unwrap_err());
                    continue;
                }
                let res = handle_command(rl, &wallet, cli_res.unwrap()).await;
                show_results(res);
            }
            Err(ReadlineError::Interrupted) => {
                info!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                info!("CTRL-D");
                break;
            }
            Err(err) => {
                error!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(history_file).map_err(|e| anyhow!(e))
}
