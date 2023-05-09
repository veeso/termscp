//! ## File system watcher
//!
//! A watcher for file system paths, which reports changes on local fs

mod change;

// -- export
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError};
use std::time::Duration;

pub use change::FsChange;
use notify::{
    watcher, DebouncedEvent, Error as WatcherError, RecommendedWatcher, RecursiveMode, Watcher,
};
use thiserror::Error;

use crate::utils::path as path_utils;

type FsWatcherResult<T> = Result<T, FsWatcherError>;

/// Describes an error returned by the `FsWatcher`
#[derive(Debug, Error)]
pub enum FsWatcherError {
    #[error("unable to unwatch this path, since is not currently watched")]
    PathNotWatched,
    #[error("unable to watch path, since it's already watched")]
    PathAlreadyWatched,
    #[error("worker error: {0}")]
    WorkerError(WatcherError),
}

impl From<WatcherError> for FsWatcherError {
    fn from(err: WatcherError) -> Self {
        Self::WorkerError(err)
    }
}

/// File system watcher
pub struct FsWatcher {
    paths: HashMap<PathBuf, PathBuf>,
    receiver: Receiver<DebouncedEvent>,
    watcher: RecommendedWatcher,
}

impl FsWatcher {
    /// Initialize a new `FsWatcher`
    pub fn init(delay: Duration) -> FsWatcherResult<Self> {
        let (tx, receiver) = channel();

        Ok(Self {
            paths: HashMap::default(),
            receiver,
            watcher: watcher(tx, delay)?,
        })
    }

    /// Poll searching for the first available disk change
    pub fn poll(&self) -> FsWatcherResult<Option<FsChange>> {
        match self.receiver.recv_timeout(Duration::from_millis(1)) {
            Ok(DebouncedEvent::Rename(source, dest)) => Ok(self.build_fs_move(source, dest)),
            Ok(DebouncedEvent::Remove(p)) => Ok(self.build_fs_remove(p)),
            Ok(DebouncedEvent::Chmod(p) | DebouncedEvent::Create(p) | DebouncedEvent::Write(p)) => {
                Ok(self.build_fs_update(p))
            }
            Ok(
                DebouncedEvent::Rescan
                | DebouncedEvent::NoticeRemove(_)
                | DebouncedEvent::NoticeWrite(_),
            ) => Ok(None),
            Ok(DebouncedEvent::Error(e, _)) => {
                error!("FsWatcher reported error: {}", e);
                Err(e.into())
            }
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(RecvTimeoutError::Disconnected) => panic!("File watcher died"),
        }
    }

    /// Watch `local` path on localhost
    pub fn watch(&mut self, local: &Path, remote: &Path) -> FsWatcherResult<()> {
        // Start watcher if unwatched
        if !self.watched(local) {
            self.watcher.watch(local, RecursiveMode::Recursive)?;
            // Insert new path to paths
            self.paths.insert(local.to_path_buf(), remote.to_path_buf());
            Ok(())
        } else {
            Err(FsWatcherError::PathAlreadyWatched)
        }
    }

    /// Returns whether `path` is currently watched.
    /// This method looks also in path ancestors.
    ///
    /// Example:
    /// if `/home` is watched, then if we call `watched("/home/foo/file.txt")` will return `true`
    pub fn watched(&self, path: &Path) -> bool {
        self.find_watched_path(path).is_some()
    }

    /// Returns the list of watched paths
    pub fn watched_paths(&self) -> Vec<&Path> {
        Vec::from_iter(self.paths.keys().map(|x| x.as_path()))
    }

    /// Unwatch provided path.
    /// When unwatching the path, it searches for the ancestor watched path if any.
    /// Returns the unwatched resolved path
    pub fn unwatch(&mut self, path: &Path) -> FsWatcherResult<PathBuf> {
        let watched_path = self.find_watched_path(path).map(|x| x.0.to_path_buf());
        if let Some(watched_path) = watched_path {
            self.watcher.unwatch(watched_path.as_path())?;
            self.paths.remove(watched_path.as_path());
            Ok(watched_path)
        } else {
            Err(FsWatcherError::PathNotWatched)
        }
    }

    /// Given a certain path, returns the path data associated to the path which
    /// is ancestor of that path in the current watched path
    fn find_watched_path(&self, p: &Path) -> Option<(&Path, &Path)> {
        self.paths
            .iter()
            .find(|(k, _)| path_utils::is_child_of(p, k))
            .map(|(k, v)| (k.as_path(), v.as_path()))
    }

    /// Build `FsChange` from path to local `changed_file`
    fn build_fs_move(&self, source: PathBuf, destination: PathBuf) -> Option<FsChange> {
        if let Some((watched_local, watched_remote)) = self.find_watched_path(&source) {
            Some(FsChange::mov(
                source,
                destination,
                watched_local,
                watched_remote,
            ))
        } else {
            None
        }
    }

    /// Build `FsChange` from path to local `changed_file`
    fn build_fs_remove(&self, removed_path: PathBuf) -> Option<FsChange> {
        if let Some((watched_local, watched_remote)) = self.find_watched_path(&removed_path) {
            Some(FsChange::remove(
                removed_path,
                watched_local,
                watched_remote,
            ))
        } else {
            None
        }
    }

    /// Build `FsChange` from path to local `changed_file`
    fn build_fs_update(&self, changed_file: PathBuf) -> Option<FsChange> {
        if let Some((watched_local, watched_remote)) = self.find_watched_path(&changed_file) {
            Some(FsChange::update(
                changed_file,
                watched_local,
                watched_remote,
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;
    use tempfile::TempDir;

    use super::*;
    #[cfg(target_os = "macos")]
    use crate::utils::test_helpers;

    #[test]
    fn should_init_fswatcher() {
        let watcher = FsWatcher::init(Duration::from_secs(5)).unwrap();
        assert!(watcher.paths.is_empty());
    }

    #[test]
    fn should_watch_path() {
        let mut watcher = FsWatcher::init(Duration::from_secs(5)).unwrap();
        let tempdir = TempDir::new().unwrap();
        assert!(watcher
            .watch(tempdir.path(), Path::new("/tmp/test"))
            .is_ok());
        // check if in paths
        assert_eq!(
            watcher.paths.get(tempdir.path()).unwrap(),
            Path::new("/tmp/test")
        );
        // close tempdir
        assert!(tempdir.close().is_ok());
    }

    #[test]
    fn should_not_watch_path_if_subdir_of_watched_path() {
        let mut watcher = FsWatcher::init(Duration::from_secs(5)).unwrap();
        let tempdir = TempDir::new().unwrap();
        assert!(watcher
            .watch(tempdir.path(), Path::new("/tmp/test"))
            .is_ok());
        // watch subdir
        let mut subdir = tempdir.path().to_path_buf();
        subdir.push("abc/def");
        // should return already watched
        assert!(watcher
            .watch(subdir.as_path(), Path::new("/tmp/test/abc/def"))
            .is_err());
        // close tempdir
        assert!(tempdir.close().is_ok());
    }

    #[test]
    fn should_unwatch_path() {
        let mut watcher = FsWatcher::init(Duration::from_secs(5)).unwrap();
        let tempdir = TempDir::new().unwrap();
        assert!(watcher
            .watch(tempdir.path(), Path::new("/tmp/test"))
            .is_ok());
        // unwatch
        assert!(watcher.unwatch(tempdir.path()).is_ok());
        assert!(watcher.paths.get(tempdir.path()).is_none());
        // close tempdir
        assert!(tempdir.close().is_ok());
    }

    #[test]
    fn should_unwatch_path_when_subdir() {
        let mut watcher = FsWatcher::init(Duration::from_secs(5)).unwrap();
        let tempdir = TempDir::new().unwrap();
        assert!(watcher
            .watch(tempdir.path(), Path::new("/tmp/test"))
            .is_ok());
        // unwatch
        let mut subdir = tempdir.path().to_path_buf();
        subdir.push("abc/def");
        assert_eq!(
            watcher.unwatch(subdir.as_path()).unwrap().as_path(),
            Path::new(tempdir.path())
        );
        assert!(watcher.paths.get(tempdir.path()).is_none());
        // close tempdir
        assert!(tempdir.close().is_ok());
    }

    #[test]
    fn should_return_err_when_unwatching_unwatched_path() {
        let mut watcher = FsWatcher::init(Duration::from_secs(5)).unwrap();
        assert!(watcher.unwatch(Path::new("/tmp")).is_err());
    }

    #[test]
    fn should_tell_whether_path_is_watched() {
        let mut watcher = FsWatcher::init(Duration::from_secs(5)).unwrap();
        let tempdir = TempDir::new().unwrap();
        assert!(watcher
            .watch(tempdir.path(), Path::new("/tmp/test"))
            .is_ok());
        assert_eq!(watcher.watched(tempdir.path()), true);
        let mut subdir = tempdir.path().to_path_buf();
        subdir.push("abc/def");
        assert_eq!(watcher.watched(subdir.as_path()), true);
        assert_eq!(watcher.watched(Path::new("/tmp")), false);
        // close tempdir
        assert!(tempdir.close().is_ok());
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn should_poll_file_update() {
        let mut watcher = FsWatcher::init(Duration::from_millis(100)).unwrap();
        let tempdir = TempDir::new().unwrap();
        let tempdir_path = PathBuf::from(format!("/private{}", tempdir.path().display()));
        assert!(watcher
            .watch(tempdir_path.as_path(), Path::new("/tmp/test"))
            .is_ok());
        // create file
        let file_path = test_helpers::make_file_at(tempdir_path.as_path(), "test.txt").unwrap();
        // wait
        std::thread::sleep(Duration::from_millis(500));
        // wait till update
        loop {
            let fs_change = watcher.poll().unwrap();
            if let Some(FsChange::Update(_)) = fs_change {
                break;
            }
            std::thread::sleep(Duration::from_millis(500));
        }
        assert!(std::fs::remove_file(file_path.as_path()).is_ok());
        // close tempdir
        assert!(tempdir.close().is_ok());
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn should_poll_file_removed() {
        let mut watcher = FsWatcher::init(Duration::from_millis(100)).unwrap();
        let tempdir = TempDir::new().unwrap();
        let tempdir_path = PathBuf::from(format!("/private{}", tempdir.path().display()));
        assert!(watcher
            .watch(tempdir_path.as_path(), Path::new("/tmp/test"))
            .is_ok());
        // create file
        let file_path = test_helpers::make_file_at(tempdir_path.as_path(), "test.txt").unwrap();
        std::thread::sleep(Duration::from_millis(500));
        // wait
        assert!(std::fs::remove_file(file_path.as_path()).is_ok());
        // poll till remove
        loop {
            let fs_change = watcher.poll().unwrap();
            if let Some(FsChange::Remove(remove)) = fs_change {
                assert_eq!(remove.path(), Path::new("/tmp/test/test.txt"));
                break;
            }
            std::thread::sleep(Duration::from_millis(500));
        }
        // close tempdir
        assert!(tempdir.close().is_ok());
    }

    /*
    #[test]
    #[cfg(target_family = "unix")]
    fn should_poll_file_moved() {
        let mut watcher = FsWatcher::init(Duration::from_millis(100)).unwrap();
        let tempdir = TempDir::new().unwrap();
        let tempdir_path = PathBuf::from(format!("/private{}", tempdir.path().display()));
        assert!(watcher
            .watch(tempdir_path.as_path(), Path::new("/tmp/test"))
            .is_ok());
        // create file
        let file_path = test_helpers::make_file_at(tempdir_path.as_path(), "test.txt").unwrap();
        // wait
        std::thread::sleep(Duration::from_millis(500));
        // move file
        let mut new_file_path = tempdir.path().to_path_buf();
        new_file_path.push("new.txt");
        assert!(std::fs::rename(file_path.as_path(), new_file_path.as_path()).is_ok());
        std::thread::sleep(Duration::from_millis(500));
        // wait till rename
        loop {
            let fs_change = watcher.poll().unwrap();
            if let Some(FsChange::Move(mov)) = fs_change {
                assert_eq!(mov.source(), Path::new("/tmp/test/test.txt"));
                assert_eq!(mov.destination(), Path::new("/tmp/test/new.txt"));
                break;
            }
            std::thread::sleep(Duration::from_millis(500));
        }
        // remove file
        assert!(std::fs::remove_file(new_file_path.as_path()).is_ok());
        // close tempdir
        assert!(tempdir.close().is_ok());
    }
     */

    #[test]
    #[cfg(target_os = "macos")]
    fn should_poll_nothing() {
        let mut watcher = FsWatcher::init(Duration::from_secs(5)).unwrap();
        let tempdir = TempDir::new().unwrap();
        assert!(watcher
            .watch(tempdir.path(), Path::new("/tmp/test"))
            .is_ok());
        assert!(watcher.poll().ok().unwrap().is_none());
        // close tempdir
        assert!(tempdir.close().is_ok());
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn should_get_watched_paths() {
        let mut watcher = FsWatcher::init(Duration::from_secs(5)).unwrap();
        assert!(watcher.watch(Path::new("/tmp"), Path::new("/tmp")).is_ok());
        assert!(watcher
            .watch(Path::new("/home"), Path::new("/home"))
            .is_ok());
        let mut watched_paths = watcher.watched_paths();
        watched_paths.sort();
        assert_eq!(watched_paths, vec![Path::new("/home"), Path::new("/tmp")]);
    }
}
