pub(crate) mod hashing;

use std::io;
use std::path::Path;
use symlink;

pub fn create_symlink<P: AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> io::Result<()>{
    symlink::symlink_auto(from, to)?;
    Ok(())
}
