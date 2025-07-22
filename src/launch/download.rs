use async_zip::{error::ZipError, tokio::read::seek::ZipFileReader};
use digest::DynDigest;
use futures_util::StreamExt;
use log::{error, info, warn};
use md5::Digest;
use reqwest::Client;
use sha2::Sha256;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use std::{backtrace::Backtrace, ops::Range, path::PathBuf, sync::Arc};
use tokio::{
    fs::{self, File},
    io::{AsyncWriteExt, BufReader},
    sync::mpsc,
};

use thiserror::Error;

use crate::{
    launch::local::Resource,
    utils::{
        hashing::{Hash, HashingError, compare_file_hash},
        stream::stream_write_and_calculate_hash,
        tempfile_async,
    },
};

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Failed to calculate hashcode")]
    Hashing(#[from] HashingError),

    #[error("Failed to unzip")]
    Unarchive(#[from] ZipError),

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
    concurrency: usize,
    max_retries: u32,
) -> Result<Vec<Resource>, DownloadError> {
    let file_name = format!(
        "{}-{}",
        resource.file_hash.hash_type(),
        resource.file_hash.value()
    );
    let file_path = target_dir.join(&file_name);

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
    info!(
        "Start download file {} with {} hash {}",
        &resource.file_relative_path,
        &resource.file_hash.hash_type(),
        &resource.file_hash.value()
    );

    // download file
    download_parallelly(
        client,
        remote_url,
        &mut file,
        Some(&resource.file_hash),
        concurrency,
        max_retries,
    )
    .await?;

    let mut addition_resources = Vec::new();

    if resource.is_archive {
        // try to read the archive
        // TODO: multiple archive format support
        let mut buffer = BufReader::new(file);
        let mut zip_archive = ZipFileReader::with_tokio(&mut buffer).await?;

        let mut v = Vec::new();

        for (index, entry) in zip_archive.file().entries().iter().enumerate() {
            if !entry.dir()? {
                let file_name = entry.filename();
                let final_path = format!(
                    "{}/{}",
                    resource.file_relative_path.trim_end_matches("/"),
                    file_name.as_str()?
                );
                // we will extract later
                v.push((final_path, index));
            }
        }

        // extract entries
        for (path, index) in v.into_iter() {
            let reader = zip_archive.reader_with_entry(index).await?;
            let mut hasher: Box<dyn DynDigest> = Box::new(Sha256::new());
            // create file
            let extracted_path = target_dir.join(format!("{}-part{index}", &file_name));
            let mut file = fs::File::create(&extracted_path).await?;

            // TODO: fix stream write and calculate hash

            stream_write_and_calculate_hash(&mut reader.compat(), &mut file, &mut Some(&mut hasher)).await?;

            let sha256_hash = hex::encode(hasher.finalize());

            // rename the file
            let path_with_hash = extracted_path
                .parent()
                .unwrap()
                .join(format!("sha256-{sha256_hash}"));
            fs::rename(extracted_path, path_with_hash).await?;

            // create resource obj
            let resource = Resource {
                file_relative_path: path,
                file_hash: Hash::Sha256(sha256_hash),
                remote_url: None,
                is_archive: false,
                from_archive: Some(Box::new(resource.to_owned())),
            };
            // add extern resources to vec
            addition_resources.push(resource)
        }
    }

    Ok(addition_resources)
}

pub async fn download_parallelly(
    client: &Client,
    url: &str,
    file: &mut File,
    expected_file_hash: Option<&Hash>,
    concurrency: usize,
    max_retries: u32,
) -> Result<(), DownloadError> {
    // fetch file size
    let res = client.head(url).send().await?;
    let total_size: Option<usize> = res
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok());

    let Some(total_size) = total_size else {
        // download in the single thread since Celestial cannot know the size of the file
        return download_single_thread(client, url, file, expected_file_hash, max_retries).await;
    };

    if total_size <= 5120 {
        // file too small, do parallel download is expensize
        return download_single_thread(client, url, file, expected_file_hash, max_retries).await;
    }

    let concurrency = if concurrency >= total_size {
        concurrency / 2
    } else {
        concurrency
    };

    let single_chunk_max_size = total_size / concurrency;
    // split chunks
    let mut chunk_ranges: Vec<Range<usize>> = Vec::new();
    let mut counter: usize = 0;
    loop {
        let chunk_start = counter * single_chunk_max_size;
        let chunk_end = (counter + 1) * single_chunk_max_size - 1;

        // chunk the border
        let chunk_end = if total_size > chunk_end {
            chunk_end
        } else {
            // EOF
            total_size
        };

        // add chunk
        chunk_ranges.push(chunk_start..chunk_end);

        if chunk_end == total_size {
            // the last chunk
            break;
        }
        counter += 1;
    }

    let client = Arc::new(client.clone());

    // order, file
    let (tx, mut rx) = mpsc::channel(20);

    // start download tasks
    for (chunk_num, chunk_range) in chunk_ranges.into_iter().enumerate() {
        // TODO: use map
        let client = Arc::clone(&client);
        let url = url;
        let url = url.to_string();
        let tx = tx.clone();
        tokio::spawn(async move {
            for _retry_count in 1..max_retries {
                let result: anyhow::Result<()> = async {
                    // create temp file
                    let (mut chunk_file_handle, chunk_file_path) =
                        tempfile_async::tempfile().await?;

                    let range = format!("bytes={}-{}", chunk_range.start, chunk_range.end);
                    // download chunk
                    let mut stream = client
                        .get(&url)
                        .header("Range", range)
                        .send()
                        .await?
                        .bytes_stream();

                    // write stream to chunk_file
                    while let Some(chunk) = stream.next().await {
                        let chunk = chunk?;
                        chunk_file_handle.write_all(&chunk).await?;
                    }

                    // now this chunk is download successfully
                    // add chunk_file_handle and path to completed files (with order)
                    tx.send((chunk_num, chunk_file_handle, chunk_file_path))
                        .await?;
                    Ok(())
                }
                .await;

                if let Ok(()) = result {
                    // download successfully
                    break;
                }
            }
        });
    }

    let mut completed_tasks = Vec::new();

    while let Some(completed_task) = rx.recv().await {
        // add to vec
        completed_tasks.push(completed_task);
    }

    // sort completed tasks
    completed_tasks.sort_by(|task_a, task_b| task_a.0.cmp(&task_b.0));

    let mut hasher = expected_file_hash.map(|hash| hash.create_hasher());

    // join chunks
    for (_, chunk_file, chunk_path) in completed_tasks.into_iter() {
        let mut reader = BufReader::new(chunk_file);

        stream_write_and_calculate_hash(&mut reader, file, &mut hasher.as_mut()).await?;
        // This chunk was dumped into the target file
        // so delete it
        fs::remove_file(chunk_path).await?;
    }

    // verify hash
    if let Some(hasher) = hasher {
        let actual_hash = hex::encode(hasher.finalize());
        let expected_hash = expected_file_hash.unwrap();
        if actual_hash != expected_hash.value() {
            return Err(DownloadError::Hashing(HashingError::HashNotMatch {
                expected_hash: expected_hash.to_owned(),
                actual_hash: actual_hash,
            }));
        }
    }

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

            let mut hasher = file_hash.map(|hash| hash.create_hasher());
            // stream write file
            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                file.write_all(&chunk).await?;

                // update hasher if possible
                hasher
                    .iter_mut()
                    .next()
                    .map(|hasher| hasher.update(&chunk))
                    .unwrap_or(());
            }
            // check hash
            if let Some(file_hash) = file_hash {
                // compare hash
                let hasher = hasher.unwrap();
                let actual_hash = hex::encode(hasher.finalize());
                if file_hash.value() != actual_hash {
                    return Err(DownloadError::Hashing(HashingError::HashNotMatch {
                        expected_hash: file_hash.to_owned(),
                        actual_hash: actual_hash,
                    }));
                }
            }
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
        file_hash
            .map(|hash| hash.value())
            .unwrap_or("<unknown hash>")
    );
    Err(DownloadError::MaxRetriesExceeded {
        url: url.to_string(),
        max_retries,
    })
}
