//! ## TestHelpers
//!
//! contains helper functions for tests

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use remotefs::fs::{File, FileType, Metadata};
// ext
use std::fs::File as StdFile;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

pub fn create_sample_file_entry() -> (File, NamedTempFile) {
    // Write
    let tmpfile = create_sample_file();
    (
        File {
            path: tmpfile.path().to_path_buf(),
            metadata: Metadata::default(),
        },
        tmpfile,
    )
}

pub fn create_sample_file() -> NamedTempFile {
    // Write
    let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
    writeln!(
        tmpfile,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.Mauris ultricies consequat eros,nec scelerisque magna imperdiet metus."
    )
    .unwrap();
    tmpfile
}

/// ### make_file_at
///
/// Make a file with `name` at specified path
pub fn make_file_at(dir: &Path, filename: &str) -> std::io::Result<()> {
    let mut p: PathBuf = PathBuf::from(dir);
    p.push(filename);
    let mut file = StdFile::create(p.as_path())?;
    writeln!(
        file,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.Mauris ultricies consequat eros,nec scelerisque magna imperdiet metus."
    )?;
    Ok(())
}

/// ### make_dir_at
///
/// Make a directory in `dir`
pub fn make_dir_at(dir: &Path, dirname: &str) -> std::io::Result<()> {
    let mut p: PathBuf = PathBuf::from(dir);
    p.push(dirname);
    std::fs::create_dir(p.as_path())
}

/// Create a File at specified path
pub fn make_fsentry<P: AsRef<Path>>(path: P, is_dir: bool) -> File {
    let path: PathBuf = path.as_ref().to_path_buf();
    File {
        path,
        metadata: Metadata::default().file_type(if is_dir {
            FileType::Directory
        } else {
            FileType::File
        }),
    }
}

/// ### create_file_ioers
///
/// Open a file with two handlers, the first is to read, the second is to write
pub fn create_file_ioers(p: &Path) -> (StdFile, StdFile) {
    (
        StdFile::open(p).ok().unwrap(),
        StdFile::create(p).ok().unwrap(),
    )
}

mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_utils_test_helpers_sample_file() {
        let _ = create_sample_file_entry();
    }

    #[test]
    fn test_utils_test_helpers_make_fsentry() {
        assert_eq!(
            make_fsentry(PathBuf::from("/tmp/omar.txt"), false)
                .name()
                .as_str(),
            "omar.txt"
        );
        assert_eq!(
            make_fsentry(PathBuf::from("/tmp/cards"), true)
                .name()
                .as_str(),
            "cards"
        );
    }

    #[test]
    fn test_utils_test_helpers_make_samples() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        assert!(make_file_at(tmpdir.path(), "omaroni.txt").is_ok());
        assert!(make_file_at(PathBuf::from("/aaaaa/bbbbb/cccc").as_path(), "readme.txt").is_err());
        assert!(make_dir_at(tmpdir.path(), "docs").is_ok());
        assert!(make_dir_at(PathBuf::from("/aaaaa/bbbbb/cccc").as_path(), "docs").is_err());
    }

    #[test]
    fn test_utils_test_helpers_create_file_ioers() {
        let (_, tmp) = create_sample_file_entry();
        let _ = create_file_ioers(tmp.path());
    }
}
