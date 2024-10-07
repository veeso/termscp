//! ## File system change
//!
//! this module exposes the types to describe a change to sync on the remote file system

use std::path::{Path, PathBuf};

use crate::utils::path as path_utils;

/// Describes an operation on the remote file system to sync
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FsChange {
    /// Move file on remote
    Move(FileToRename),
    /// Remove file from remote
    Remove(FileToRemove),
    /// Upload file to remote
    Update(FileUpdate),
}

impl FsChange {
    /// Instantiate a new `FsChange::Move`
    pub fn mov(
        source: PathBuf,
        destination: PathBuf,
        local_watched_path: &Path,
        remote_synched_path: &Path,
    ) -> Self {
        Self::Move(FileToRename::new(
            source,
            destination,
            local_watched_path,
            remote_synched_path,
        ))
    }

    /// Instantiate a new `FsChange::Remove`
    pub fn remove(
        removed_path: PathBuf,
        local_watched_path: &Path,
        remote_synched_path: &Path,
    ) -> Self {
        Self::Remove(FileToRemove::new(
            removed_path,
            local_watched_path,
            remote_synched_path,
        ))
    }

    /// Instantiate a new `FsChange::Update`
    pub fn update(
        changed_path: PathBuf,
        local_watched_path: &Path,
        remote_synched_path: &Path,
    ) -> Self {
        Self::Update(FileUpdate::new(
            changed_path,
            local_watched_path,
            remote_synched_path,
        ))
    }
}

/// Describes a file to rename on the remote fs
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileToRename {
    /// Path to file which has to be renamed
    source: PathBuf,
    /// new filename
    destination: PathBuf,
}

impl FileToRename {
    /// Instantiate a new `FileToRename` given
    ///
    /// - the path of the source on local fs
    /// - the path of the destination on local fs
    /// - the path of the file/directory watched on the local fs
    /// - the path of the remote file/directory synched with the local fs
    ///
    /// the `remote` is resolved pushing to `remote_synched_path` the diff between `changed_path` and `local_watched_path`
    fn new(
        source: PathBuf,
        destination: PathBuf,
        local_watched_path: &Path,
        remote_synched_path: &Path,
    ) -> Self {
        Self {
            source: remote_relative_path(&source, local_watched_path, remote_synched_path),
            destination: remote_relative_path(
                &destination,
                local_watched_path,
                remote_synched_path,
            ),
        }
    }

    /// Get path to the source to rename
    pub fn source(&self) -> &Path {
        self.source.as_path()
    }

    /// Get path to the destination name
    pub fn destination(&self) -> &Path {
        self.destination.as_path()
    }
}

/// Describes a file to remove on remote fs
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileToRemove {
    /// Path to the file which has to be removed
    path: PathBuf,
}

impl FileToRemove {
    /// Instantiate a new `FileToRemove` given
    ///
    /// - the path of the file which has been removed on localhost
    /// - the path of the file/directory watched on the local fs
    /// - the path of the remote file/directory synched with the local fs
    ///
    /// the `remote` is resolved pushing to `remote_synched_path` the diff between `removed_path` and `local_watched_path`
    fn new(removed_path: PathBuf, local_watched_path: &Path, remote_synched_path: &Path) -> Self {
        Self {
            path: remote_relative_path(&removed_path, local_watched_path, remote_synched_path),
        }
    }

    /// Get path to the file to unlink
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

/// Describes a file changed to sync
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileUpdate {
    /// Path to file which has changed
    host_bridge: PathBuf,
    /// Path to remote file to update
    remote: PathBuf,
}

impl FileUpdate {
    /// Instantiate a new `FileUpdate` given
    ///
    /// - the path of the file which has changed
    /// - the path of the file/directory watched on the local fs
    /// - the path of the remote file/directory synched with the local fs
    ///
    /// the `remote` is resolved pushing to `remote_synched_path` the diff between `changed_path` and `local_watched_path`
    fn new(changed_path: PathBuf, local_watched_path: &Path, remote_synched_path: &Path) -> Self {
        Self {
            remote: remote_relative_path(&changed_path, local_watched_path, remote_synched_path),
            host_bridge: changed_path,
        }
    }

    /// Get path to local file to sync
    pub fn host_bridge(&self) -> &Path {
        self.host_bridge.as_path()
    }

    /// Get path to remote file to sync
    pub fn remote(&self) -> &Path {
        self.remote.as_path()
    }
}

// -- utils

/// Get remote relative path, given the local target, the path of the local watched path and the path of the remote synched directory/file
fn remote_relative_path(
    target: &Path,
    local_watched_path: &Path,
    remote_synched_path: &Path,
) -> PathBuf {
    let local_diff = path_utils::diff_paths(target, local_watched_path);
    // get absolute path to remote file associated to local file
    match local_diff {
        None => remote_synched_path.to_path_buf(),
        Some(p) => {
            let mut remote = remote_synched_path.to_path_buf();
            remote.push(p);
            remote
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn should_get_remote_relative_path_from_subdir() {
        assert_eq!(
            remote_relative_path(
                Path::new("/tmp/abc/test.txt"),
                Path::new("/tmp"),
                Path::new("/home/foo")
            )
            .as_path(),
            Path::new("/home/foo/abc/test.txt")
        );
    }

    #[test]
    fn should_get_remote_relative_path_same_path() {
        assert_eq!(
            remote_relative_path(
                Path::new("/tmp/abc/test.txt"),
                Path::new("/tmp/abc/test.txt"),
                Path::new("/home/foo/test.txt")
            )
            .as_path(),
            Path::new("/home/foo/test.txt")
        );
    }

    #[test]
    fn should_make_fs_change_move_from_same_directory() {
        let change = FsChange::mov(
            PathBuf::from("/tmp/foo.txt"),
            PathBuf::from("/tmp/bar.txt"),
            Path::new("/tmp"),
            Path::new("/home/foo"),
        );
        if let FsChange::Move(change) = change {
            assert_eq!(change.source(), Path::new("/home/foo/foo.txt"));
            assert_eq!(change.destination(), Path::new("/home/foo/bar.txt"));
        } else {
            panic!("not a Move");
        }
    }

    #[test]
    fn should_make_fs_change_move_from_subdirectory() {
        let change = FsChange::mov(
            PathBuf::from("/tmp/abc/foo.txt"),
            PathBuf::from("/tmp/abc/bar.txt"),
            Path::new("/tmp/abc"),
            Path::new("/home/foo"),
        );
        if let FsChange::Move(change) = change {
            assert_eq!(change.source(), Path::new("/home/foo/foo.txt"));
            assert_eq!(change.destination(), Path::new("/home/foo/bar.txt"));
        } else {
            panic!("not a Move");
        }
    }

    #[test]
    fn should_make_fs_change_remove_from_same_directory() {
        let change = FsChange::remove(
            PathBuf::from("/tmp/bar.txt"),
            Path::new("/tmp/bar.txt"),
            Path::new("/home/foo/bar.txt"),
        );
        if let FsChange::Remove(change) = change {
            assert_eq!(change.path(), Path::new("/home/foo/bar.txt"));
        } else {
            panic!("not a remove");
        }
    }

    #[test]
    fn should_make_fs_change_remove_from_subdirectory() {
        let change = FsChange::remove(
            PathBuf::from("/tmp/abc/bar.txt"),
            Path::new("/tmp/abc"),
            Path::new("/home/foo"),
        );
        if let FsChange::Remove(change) = change {
            assert_eq!(change.path(), Path::new("/home/foo/bar.txt"));
        } else {
            panic!("not a remove");
        }
    }

    #[test]
    fn should_make_fs_change_update_from_same_directory() {
        let change = FsChange::update(
            PathBuf::from("/tmp/bar.txt"),
            Path::new("/tmp/bar.txt"),
            Path::new("/home/foo/bar.txt"),
        );
        if let FsChange::Update(change) = change {
            assert_eq!(change.host_bridge(), Path::new("/tmp/bar.txt"),);
            assert_eq!(change.remote(), Path::new("/home/foo/bar.txt"));
        } else {
            panic!("not an update");
        }
    }

    #[test]
    fn should_make_fs_change_update_from_subdirectory() {
        let change = FsChange::update(
            PathBuf::from("/tmp/abc/foo.txt"),
            Path::new("/tmp"),
            Path::new("/home/foo/temp"),
        );
        if let FsChange::Update(change) = change {
            assert_eq!(change.host_bridge(), Path::new("/tmp/abc/foo.txt"),);
            assert_eq!(change.remote(), Path::new("/home/foo/temp/abc/foo.txt"));
        } else {
            panic!("not an update");
        }
    }
}
