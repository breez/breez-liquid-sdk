use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use lwk_wollet::elements::{LockTime, LockTime::*};
use log::debug;
use anyhow::{anyhow, Result};
use reqwest::StatusCode;

use crate::error::{LiquidSdkError, PaymentError};

pub(crate) fn now() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}

pub(crate) fn json_to_pubkey(json: &str) -> Result<boltz_client::PublicKey, PaymentError> {
    boltz_client::PublicKey::from_str(json).map_err(|e| PaymentError::Generic {
        err: format!("Failed to deserialize PublicKey: {e:?}"),
    })
}

pub(crate) fn generate_keypair() -> boltz_client::Keypair {
    let secp = boltz_client::Secp256k1::new();
    let mut rng = bip39::rand::rngs::OsRng;
    let secret_key = lwk_wollet::secp256k1::SecretKey::new(&mut rng);
    boltz_client::Keypair::from_secret_key(&secp, &secret_key)
}

pub(crate) fn decode_keypair(secret_key: &str) -> Result<boltz_client::Keypair, lwk_wollet::Error> {
    let secp = boltz_client::Secp256k1::new();
    let secret_key = lwk_wollet::secp256k1::SecretKey::from_str(secret_key)?;
    Ok(boltz_client::Keypair::from_secret_key(&secp, &secret_key))
}

pub(crate) fn is_locktime_expired(current_locktime: LockTime, expiry_locktime: LockTime) -> bool {
    match (current_locktime, expiry_locktime) {
        (Blocks(n), Blocks(lock_time)) => n >= lock_time,
        (Seconds(n), Seconds(lock_time)) => n >= lock_time,
        _ => false, // Not using the same units
    }
}

pub(crate) fn get_reqwest_client() -> Result<reqwest::Client, LiquidSdkError> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| LiquidSdkError::ServiceConnectivity { err: e.to_string() })
}

pub(crate) async fn get_and_log_response(
    url: &str,
) -> Result<(String, StatusCode), LiquidSdkError> {
    debug!("Making GET request to: {url}");

    let response = get_reqwest_client()?
        .get(url)
        .send()
        .await
        .map_err(|e| LiquidSdkError::ServiceConnectivity { err: e.to_string() })?;
    let status = response.status();
    let raw_body = response
        .text()
        .await
        .map_err(|e| LiquidSdkError::ServiceConnectivity { err: e.to_string() })?;
    debug!("Received response, status: {status}, raw response body: {raw_body}");

    Ok((raw_body, status))
}

// pub(crate) async fn get_parse_and_log_response<T>(
//     url: &str,
//     enforce_status_check: bool,
// ) -> Result<T, LiquidSdkError>
// where
//     for<'a> T: serde::de::Deserialize<'a>,
// {
//     let (raw_body, status) = get_and_log_response(url).await?;
//     if enforce_status_check && !status.is_success() {
//         let err = format!("GET request {url} failed with status: {status}");
//         log::error!("{err}");
//         return Err(LiquidSdkError::ServiceConnectivity { err });
//     }
//
//     serde_json::from_str::<T>(&raw_body).map_err(Into::into)
// }

