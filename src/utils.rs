pub(crate) mod hashing;

use std::io;
use std::path::Path;
use tokio::fs;
use symlink;

#[cfg(unix)]
pub fn symlink_dir<P: AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> std::io::Result<()> {
    std::os::unix::fs::symlink(from, to)?;
    Ok(())
}

#[cfg(windows)]
pub fn symlink_dir<P:AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> std::io::Result<()> {
    symlink::symlink_dir(from, to)?;
    Ok(())
}

#[cfg(unix)]
pub fn symlink_file<P: AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> std::io::Result<()> {
    std::os::unix::fs::symlink(from, to)?;
    Ok(())
}

#[cfg(windows)]
pub fn symlink_file<P:AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> std::io::Result<()> {
    symlink::symlink_file(from, to)?;
    Ok(())
}