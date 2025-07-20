use futures_util::StreamExt;
use log::{error, info, warn};
use reqwest::Client;
use std::{backtrace::Backtrace, path::PathBuf};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use thiserror::Error;

use crate::{
    launch::local::Resource,
    utils::hashing::{Hash, HashingError, compare_file_hash},
};

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Failed to calculate hashcode")]
    Hashing(#[from] HashingError),

    #[error(
        "Resource {} with {} hash {} not found and there is no remote URL provided.",
        .resource.file_relative_path, .resource.file_hash.hash_type(),
        .resource.file_hash.value()
    )]
    NoRemoteUrlProvided { resource: Resource },

    #[error("Error fetching file")]
    Http(#[from] reqwest::Error),

    #[error("IO Error")]
    Io(#[from] std::io::Error),

    #[error("Max retries exceeded when requesting to URL {url} (max_retries = {max_retries})")]
    MaxRetriesExceeded { url: String, max_retries: u32 },

    #[error("Failed to create parent folders of the path {0}")]
    FailedCreateParentFolders(PathBuf),
}

/// Downlaod resource to target_dir/<hash_type>-<hash>
///
/// * `target_dir`: The target dir to store files
/// * `resource`: The resource to download
///
/// Return a vector if the operation produces new resources (it doesn't contains the provided
/// resource)
pub async fn download_resource(
    client: &Client,
    target_dir: &PathBuf,
    resource: &Resource,
    max_retries: u32,
) -> Result<Vec<Resource>, DownloadError> {
    let file_name = format!(
        "{}-{}",
        resource.file_hash.hash_type(),
        resource.file_hash.value()
    );
    let file_path = target_dir.join(file_name);

    if file_path.exists() {
        // if the hash matches, skip downloading, otherwise redownload it
        // compare hash
        match compare_file_hash(&file_path, &resource.file_hash).await {
            Ok(_) => {
                // the hash matches, skip downloading
                // FIXME: archives may produce extern files, must check them later
                return Ok(Vec::new());
            }
            Err(err) => match err {
                // it's ok
                HashingError::HashNotMatch {
                    expected_hash,
                    actual_hash,
                } => {
                    warn!(
                        "File {} exists on file system, but the hash doesn't match. expected_hash: {} ({}) actual_hash: {}",
                        file_path.to_string_lossy(),
                        expected_hash.value(),
                        expected_hash.hash_type(),
                        actual_hash,
                    );
                }
                // bad!
                _ => return Err(err.into()),
            },
        };
    }
    if resource.remote_url.is_none() {
        error!(
            "File {} with {} hash {} not found on filesystem and there is no remote_url provided.",
            &resource.file_relative_path,
            &resource.file_hash.hash_type(),
            &resource.file_hash.value()
        );
        return Err(DownloadError::NoRemoteUrlProvided {
            resource: resource.to_owned(),
        });
    }

    let remote_url = resource.remote_url.iter().next().unwrap();

    info!(
        "Download resource {} ({} hash: {}) from URL {}",
        resource.file_relative_path,
        resource.file_hash.hash_type(),
        resource.file_hash.value(),
        remote_url
    );

    // create parent folders of the file
    fs::create_dir_all(
        &file_path
            .parent()
            .ok_or(DownloadError::FailedCreateParentFolders(
                file_path.to_owned(),
            ))?,
    )
    .await?;

    // create file
    let mut file = File::create(&file_path).await?;

    // download file
    download_parallelly(
        client,
        remote_url,
        &mut file,
        Some(&resource.file_hash),
        max_retries,
    )
    .await?;

    // TODO: extract the archive (optional)
    // TODO: add extern resources to vec

    Ok(Vec::new())
}

pub async fn download_parallelly(
    client: &Client,
    url: &str,
    file: &mut File,
    file_hash: Option<&Hash>,
    max_retries: u32,
) -> Result<(), DownloadError> {
    // fetch file size
    let res = client.head(url).send().await?;
    let total_size: Option<usize> = res
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok());

    if total_size.is_none() {
        // download in the single thread since Celestial cannot know the size of the file
        return download_single_thread(client, url, file, file_hash, max_retries).await;
    }

    // TODO: parallelly download
    Ok(())
}

pub async fn download_single_thread(
    client: &Client,
    url: &str,
    file: &mut File,
    file_hash: Option<&Hash>,
    max_retries: u32,
) -> Result<(), DownloadError> {
    for retry_count in 1..=max_retries {
        // get file
        let result: anyhow::Result<()> = {
            let mut stream = client.get(url).send().await?.bytes_stream();

            // stream write file
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                file.write_all(&chunk).await?;
            }
            // TODO: check hash
            Ok(())
        };

        if let Err(err) = result {
            error!(
                "Error happened when download file (retry {retry_count}/{max_retries}): {err}, \n{}",
                Backtrace::capture()
            );
        } else {
            // operation success
            return Ok(());
        }
    }
    error!(
        "Failed to download file with hash {}: Max retries exceeded",
        file_hash.map(|hash| hash.value()).unwrap_or("<unknown hash>")
    );
    Err(DownloadError::MaxRetriesExceeded {
        url: url.to_string(),
        max_retries,
    })
}
