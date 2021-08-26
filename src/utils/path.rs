//! # Path
//!
//! Path related utilities

use std::path::{Component, Path, PathBuf};

/// ### absolutize
///
/// Absolutize target path if relative.
/// For example:
///
/// ```rust
/// assert_eq!(absolutize(&Path::new("/home/omar"), &Path::new("readme.txt")).as_path(), Path::new("/home/omar/readme.txt"));
/// assert_eq!(absolutize(&Path::new("/home/omar"), &Path::new("/tmp/readme.txt")).as_path(), Path::new("/tmp/readme.txt"));
/// ```
pub fn absolutize(wrkdir: &Path, target: &Path) -> PathBuf {
    match target.is_absolute() {
        true => target.to_path_buf(),
        false => {
            let mut p: PathBuf = wrkdir.to_path_buf();
            p.push(target);
            p
        }
    }
}

/// ### diff_paths
///
/// This function will get the difference from path `path` to `base`. Basically will remove `base` from `path`
///
/// For example:
///
/// ```rust
/// assert_eq!(diff_paths(&Path::new("/foo/bar"), &Path::new("/")).as_path(), Path::new("foo/bar"));
/// assert_eq!(diff_paths(&Path::new("/foo/bar"), &Path::new("/foo")).as_path(), Path::new("bar"));
/// ```
///
/// This function has been written by <https://github.com/Manishearth>
/// and is licensed under the APACHE-2/MIT license <https://github.com/Manishearth/pathdiff>
pub fn diff_paths<P, B>(path: P, base: B) -> Option<PathBuf>
where
    P: AsRef<Path>,
    B: AsRef<Path>,
{
    let path = path.as_ref();
    let base = base.as_ref();

    if path.is_absolute() != base.is_absolute() {
        if path.is_absolute() {
            Some(PathBuf::from(path))
        } else {
            None
        }
    } else {
        let mut ita = path.components();
        let mut itb = base.components();
        let mut comps: Vec<Component> = vec![];
        loop {
            match (ita.next(), itb.next()) {
                (None, None) => break,
                (Some(a), None) => {
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
                (None, _) => comps.push(Component::ParentDir),
                (Some(a), Some(b)) if comps.is_empty() && a == b => (),
                (Some(a), Some(b)) if b == Component::CurDir => comps.push(a),
                (Some(_), Some(b)) if b == Component::ParentDir => return None,
                (Some(a), Some(_)) => {
                    comps.push(Component::ParentDir);
                    for _ in itb {
                        comps.push(Component::ParentDir);
                    }
                    comps.push(a);
                    comps.extend(ita.by_ref());
                    break;
                }
            }
        }
        Some(comps.iter().map(|c| c.as_os_str()).collect())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn absolutize_path() {
        assert_eq!(
            absolutize(&Path::new("/home/omar"), &Path::new("readme.txt")).as_path(),
            Path::new("/home/omar/readme.txt")
        );
        assert_eq!(
            absolutize(&Path::new("/home/omar"), &Path::new("/tmp/readme.txt")).as_path(),
            Path::new("/tmp/readme.txt")
        );
    }

    #[test]
    fn calc_diff_paths() {
        assert_eq!(
            diff_paths(&Path::new("/foo/bar"), &Path::new("/"))
                .unwrap()
                .as_path(),
            Path::new("foo/bar")
        );
        assert_eq!(
            diff_paths(&Path::new("/foo/bar"), &Path::new("/foo"))
                .unwrap()
                .as_path(),
            Path::new("bar")
        );
        assert_eq!(
            diff_paths(&Path::new("/foo/bar/chiedo.gif"), &Path::new("/"))
                .unwrap()
                .as_path(),
            Path::new("foo/bar/chiedo.gif")
        );
    }
}
