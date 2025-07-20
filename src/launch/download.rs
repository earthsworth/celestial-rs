use log::{error, info, warn};
use std::path::PathBuf;

use thiserror::Error;

use crate::{
    launch::local::Resource,
    utils::hashing::{HashingError, compare_file_hash},
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
}

/// Downlaod resource to target_dir/<hash_type>-<hash>
///
/// * `target_dir`: The target dir to store files
/// * `resource`: The resource to download
///
/// Return a vector if the operation produces new resources (it doesn't contains the provided
/// resource)
pub async fn download_resource(
    target_dir: &PathBuf,
    resource: &Resource,
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

    // TODO: download file
    // TODO: extract the archive (optional)
    // TODO: add extern resources to vec

    Ok(Vec::new())
}
