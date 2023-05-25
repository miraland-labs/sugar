use std::rc::Rc;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signature::{keypair::Keypair, read_keypair_file},
    },
    Client, Cluster,
};
use anyhow::{anyhow, Result};
use console::style;
use tracing::error;

use crate::{config::data::SugarConfig, constants::DEFAULT_KEYPATH, parse::*};

pub fn setup_client(sugar_config: &SugarConfig) -> Result<Client> {
    let rpc_url = sugar_config.rpc_url.clone();
    let ws_url = rpc_url.replace("http", "ws");
    let cluster = Cluster::Custom(rpc_url, ws_url);

    let key_bytes = sugar_config.keypair.to_bytes();
    let signer = Rc::new(Keypair::from_bytes(&key_bytes)?);

    let opts = CommitmentConfig::confirmed();
    Ok(Client::new_with_options(cluster, signer, opts))
}

pub fn sugar_setup(
    keypair_opt: Option<String>,
    rpc_url_opt: Option<String>,
) -> Result<SugarConfig> {
    let mln_config_option = parse_miraland_config();

    let rpc_url = get_rpc_url(rpc_url_opt);

    let keypair = match keypair_opt {
        Some(keypair_path) => match read_keypair_file(&keypair_path) {
            Ok(keypair) => keypair,
            Err(e) => {
                error!("Failed to read keypair file: {}", e);
                return Err(anyhow!(
                    "Failed to read keypair file: {}, {}",
                    keypair_path,
                    e
                ));
            }
        },

        None => match mln_config_option {
            Some(ref mln_config) => match read_keypair_file(&mln_config.keypair_path) {
                Ok(keypair) => keypair,
                Err(e) => {
                    error!(
                        "Failed to read keypair file: {}, {}",
                        &mln_config.keypair_path, e
                    );
                    return Err(anyhow!(
                        "Failed to read keypair file: {}, {}",
                        &mln_config.keypair_path,
                        e
                    ));
                }
            },
            None => match read_keypair_file(&*shellexpand::tilde(DEFAULT_KEYPATH)) {
                Ok(keypair) => keypair,
                Err(e) => {
                    error!("Failed to read keypair file: {}, {}", DEFAULT_KEYPATH, e);
                    return Err(anyhow!(
                        "Failed to read keypair file: {}, {}",
                        DEFAULT_KEYPATH,
                        e
                    ));
                }
            },
        },
    };

    Ok(SugarConfig { rpc_url, keypair })
}

pub fn get_rpc_url(rpc_url_opt: Option<String>) -> String {
    let mln_config_option = parse_miraland_config();

    match rpc_url_opt {
        Some(rpc_url) => rpc_url,
        None => match mln_config_option {
            Some(ref mln_config) => mln_config.json_rpc_url.clone(),
            None => {
                println!(
                    "{}",
                    style("No RPC URL found in Miraland config file.")
                        .bold()
                        .red(),
                );
                std::process::exit(1);
            }
        },
    }
}
