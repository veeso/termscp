//! ## Hash
//!
//! `hash` is the module which provides utilities for calculating digests

/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
*/

extern crate data_encoding;
extern crate ring;

use data_encoding::HEXLOWER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// ### hash_sha256_file
///
/// Get SHA256 of provided path
pub fn hash_sha256_file(file: &Path) -> Result<String, std::io::Error> {
    // Open file
    let mut reader: File = File::open(file)?;
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 8192];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    // Finish context
    let digest: Digest = context.finish();
    Ok(HEXLOWER.encode(digest.as_ref()))
}

#[cfg(test)]
mod tests {

    use super::*;

    use std::io::Write;

    #[test]
    fn test_utils_hash_sha256() {
        let tmp: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        // Write
        let mut fhnd: File = File::create(tmp.path()).unwrap();
        assert!(fhnd.write_all(b"Hello world!\n").is_ok());
        assert_eq!(
            *hash_sha256_file(tmp.path()).ok().as_ref().unwrap(),
            String::from("0ba904eae8773b70c75333db4de2f3ac45a8ad4ddba1b242f0b3cfc199391dd8")
        );
        // Bad file
        assert!(hash_sha256_file(Path::new("/tmp/oiojjt5ig/aiehgoiwg")).is_err());
    }
}
