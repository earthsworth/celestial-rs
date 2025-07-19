use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Hash {
    Md5(String),
    Sha1(String),
    Sha256(String),
    Sha512(String),
}

impl Hash {
    pub fn hash_type(&self) -> &'static str {
        match self {
            Hash::Md5(_) => "MD5",
            Hash::Sha1(_) => "SHA1",
            Hash::Sha256(_) => "SHA256",
            Hash::Sha512(_) => "SHA512",
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Hash::Md5(v) | Hash::Sha1(v) | Hash::Sha256(v) | Hash::Sha512(v) => v,
        }
    }
}

/// Compare file hash with expected hash
/// Returns Ok(()) if hash matches, Err if file doesn't exist or hash doesn't match
pub async fn compare_hash(file_path: &Path, expected_hash: &Hash) -> Result<()> {
    // Check if file exists
    if !file_path.exists() {
        return Err(anyhow!("File does not exist: {}", file_path.display()));
    }

    // Read file content
    let file_content = fs::read(file_path)
        .await
        .map_err(|e| anyhow!("Failed to read file {}: {}", file_path.display(), e))?;

    // Calculate hash based on expected hash type
    let calculated_hash = match expected_hash {
        Hash::Md5(_) => {
            use md5::compute;
            let result = compute(file_content);
            format!("{result:x}")
        }
        Hash::Sha1(_) => {
            use sha1::{Digest, Sha1};
            let mut hasher = Sha1::new();
            hasher.update(&file_content);
            format!("{:x}", hasher.finalize())
        }
        Hash::Sha256(_) => {
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(&file_content);
            format!("{:x}", hasher.finalize())
        }
        Hash::Sha512(_) => {
            use sha2::{Digest, Sha512};
            let mut hasher = Sha512::new();
            hasher.update(&file_content);
            format!("{:x}", hasher.finalize())
        }
    };

    // Compare hashes (case-insensitive)
    if calculated_hash.to_lowercase() == expected_hash.value().to_lowercase() {
        Ok(())
    } else {
        Err(anyhow!(
            "Hash mismatch for file {}: expected {} {}, got {}",
            file_path.display(),
            expected_hash.hash_type(),
            expected_hash.value(),
            calculated_hash
        ))
    }
}

/// Calculate hash for a file
pub async fn calculate_file_hash(file_path: &Path, hash_type: &str) -> Result<Hash> {
    if !file_path.exists() {
        return Err(anyhow!("File does not exist: {}", file_path.display()));
    }

    let file_content = fs::read(file_path)
        .await
        .map_err(|e| anyhow!("Failed to read file {}: {}", file_path.display(), e))?;

    let hash = match hash_type.to_uppercase().as_str() {
        "MD5" => {
            use md5::compute;
            let result = compute(file_content);
            Hash::Md5(format!("{result:x}"))
        }
        "SHA1" => {
            use sha1::{Digest, Sha1};
            let mut hasher = Sha1::new();
            hasher.update(&file_content);
            Hash::Sha1(format!("{:x}", hasher.finalize()))
        }
        "SHA256" => {
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(&file_content);
            Hash::Sha256(format!("{:x}", hasher.finalize()))
        }
        "SHA512" => {
            use sha2::{Digest, Sha512};
            let mut hasher = Sha512::new();
            hasher.update(&file_content);
            Hash::Sha512(format!("{:x}", hasher.finalize()))
        }
        _ => return Err(anyhow!("Unsupported hash type: {}", hash_type)),
    };

    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_compare_hash_success() {
        // Create a temporary file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"hello world").unwrap();

        // Calculate expected hash
        let expected_hash = Hash::Sha256(
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9".to_string(),
        );

        // Test comparison
        let result = compare_hash(temp_file.path(), &expected_hash).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_compare_hash_mismatch() {
        // Create a temporary file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"hello world").unwrap();

        // Wrong hash
        let wrong_hash = Hash::Sha256("wrong_hash".to_string());

        // Test comparison
        let result = compare_hash(temp_file.path(), &wrong_hash).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_calculate_file_hash() {
        // Create a temporary file
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"hello world").unwrap();

        // Calculate hash
        let hash = calculate_file_hash(temp_file.path(), "SHA256")
            .await
            .unwrap();

        match hash {
            Hash::Sha256(value) => {
                assert_eq!(
                    value,
                    "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
                );
            }
            _ => panic!("Expected SHA256 hash"),
        }
    }
}
