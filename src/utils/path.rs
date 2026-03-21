//! # Path
//!
//! Path related utilities

use std::path::{Component, Path, PathBuf};

/// Absolutize target path if relative.
/// For example:
///
/// ```rust
/// assert_eq!(absolutize(&Path::new("/home/omar"), &Path::new("readme.txt")).as_path(), Path::new("/home/omar/readme.txt"));
/// assert_eq!(absolutize(&Path::new("/home/omar"), &Path::new("/tmp/readme.txt")).as_path(), Path::new("/tmp/readme.txt"));
/// ```
pub fn absolutize(wrkdir: &Path, target: &Path) -> PathBuf {
    let raw = match target.is_absolute() {
        true => target.to_path_buf(),
        false => {
            let mut p: PathBuf = wrkdir.to_path_buf();
            p.push(target);
            p
        }
    };
    normalize(&raw)
}

/// Normalize a path by resolving `.` and `..` components lexically
/// (without touching the filesystem).
fn normalize(path: &Path) -> PathBuf {
    let mut parts: Vec<Component> = Vec::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                // Pop the last normal component; never pop past root
                if let Some(Component::Normal(_)) = parts.last() {
                    parts.pop();
                }
            }
            Component::CurDir => {} // skip `.`
            other => parts.push(other),
        }
    }
    parts.iter().collect()
}

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
pub fn diff_paths<P>(path: P, base: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
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
                (Some(a), Some(Component::CurDir)) => comps.push(a),
                (Some(_), Some(Component::ParentDir)) => return None,
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

/// Returns whether `p` is child (direct/indirect) of ancestor `ancestor`
pub fn is_child_of<P: AsRef<Path>>(p: P, ancestor: P) -> bool {
    p.as_ref().ancestors().any(|x| x == ancestor.as_ref())
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

    #[test]
    fn absolutize_resolves_parent_dir() {
        assert_eq!(
            absolutize(Path::new("/home/omar"), Path::new("..")).as_path(),
            Path::new("/home")
        );
        assert_eq!(
            absolutize(Path::new("/home/omar"), Path::new("../docs")).as_path(),
            Path::new("/home/docs")
        );
        assert_eq!(
            absolutize(Path::new("/home"), Path::new("..")).as_path(),
            Path::new("/")
        );
    }

    #[test]
    fn absolutize_resolves_current_dir() {
        assert_eq!(
            absolutize(Path::new("/home/omar"), Path::new(".")).as_path(),
            Path::new("/home/omar")
        );
        assert_eq!(
            absolutize(Path::new("/home/omar"), Path::new("./docs")).as_path(),
            Path::new("/home/omar/docs")
        );
    }

    #[test]
    fn normalize_path() {
        assert_eq!(normalize(Path::new("/a/b/../c")), Path::new("/a/c"));
        assert_eq!(normalize(Path::new("/a/b/./c")), Path::new("/a/b/c"));
        assert_eq!(normalize(Path::new("/a/b/c/../..")), Path::new("/a"));
        assert_eq!(normalize(Path::new("/")), Path::new("/"));
        // Parent beyond root stays at root
        assert_eq!(normalize(Path::new("/..")), Path::new("/"));
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

    #[test]
    fn should_tell_whether_path_is_child_of() {
        assert!(is_child_of(
            Path::new("/home/foo/foo.txt"),
            Path::new("/home"),
        ));
        assert!(is_child_of(
            Path::new("/home/foo/foo.txt"),
            Path::new("/home/foo/"),
        ));
        assert!(is_child_of(
            Path::new("/home/foo/foo.txt"),
            Path::new("/home/foo/foo.txt"),
        ));
        assert!(!is_child_of(
            Path::new("/home/foo/foo.txt"),
            Path::new("/tmp"),
        ));
    }
}
