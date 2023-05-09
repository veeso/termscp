//! ## File
//!
//! `file` is the module which exposes file related utilities

use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

/// Open file provided as parameter
pub fn open_file<P>(filename: P, create: bool, write: bool, append: bool) -> io::Result<File>
where
    P: AsRef<Path>,
{
    OpenOptions::new()
        .create(create)
        .write(write)
        .append(append)
        .truncate(!append)
        .open(filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utils_file_open() {
        let tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        assert!(open_file(tmpfile.path(), true, true, true).is_ok());
    }
}
