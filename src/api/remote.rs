use bytes::Bytes;
use reqwest::Client;
use thiserror::Error;

use crate::utils::hashing::{self, Hash};

pub mod client;
pub mod models;

#[derive(Error, Debug)]
pub enum RemoteError {
    #[error("bad hash")]
    HashError(#[from] hashing::HashingError),

    #[error("reqwest error")]
    Http(#[from] reqwest::Error),
}

pub async fn get_and_verify_hash<'a>(
    client: &'a Client,
    url: &'a str,
    hash: &'a Hash,
) -> Result<Bytes, RemoteError> {
    let result = client.get(url).send().await?.bytes().await?;

    // verify hash
    hashing::compare_hash(&result, hash).await.map_err(RemoteError::from)?;

    Ok(result)
}
