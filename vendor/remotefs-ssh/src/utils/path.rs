//! ## Path
//!
//! path utilities

use std::path::{Path, PathBuf};

#[cfg(target_os = "windows")]
use path_slash::PathExt as _;

/// Absolutize target path if relative.
pub fn absolutize(wrkdir: &Path, target: &Path) -> PathBuf {
    match target.is_absolute() {
        true => target.to_path_buf(),
        false => {
            let mut p: PathBuf = wrkdir.to_path_buf();
            p.push(target);
            resolve(&p)
        }
    }
}

/// Fix provided path; on Windows fixes the backslashes, converting them to slashes
/// While on POSIX does nothing
#[cfg(target_os = "windows")]
fn resolve(p: &Path) -> PathBuf {
    PathBuf::from(p.to_slash_lossy().to_string())
}

#[cfg(target_family = "unix")]
fn resolve(p: &Path) -> PathBuf {
    p.to_path_buf()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn absolutize_path() {
        assert_eq!(
            absolutize(Path::new("/home/omar"), Path::new("readme.txt")).as_path(),
            Path::new("/home/omar/readme.txt")
        );
        assert_eq!(
            absolutize(Path::new("/home/omar"), Path::new("/tmp/readme.txt")).as_path(),
            Path::new("/tmp/readme.txt")
        );
    }
}
