//! ## File system watcher
//!
//! A watcher for file system paths, which reports changes on local fs

mod change;

// -- export
pub use change::FsChange;

use crate::utils::path as path_utils;

use notify::{
    watcher, DebouncedEvent, Error as WatcherError, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError};
use std::time::Duration;

type FsWatcherResult<T> = Result<T, WatcherError>;

/// File system watcher
pub struct FsWatcher {
    paths: HashMap<PathBuf, PathBuf>,
    receiver: Receiver<DebouncedEvent>,
    watcher: RecommendedWatcher,
}

impl FsWatcher {
    /// Initialize a new `FsWatcher`
    pub fn init() -> FsWatcherResult<Self> {
        let (tx, receiver) = channel();

        Ok(Self {
            paths: HashMap::default(),
            receiver,
            watcher: watcher(tx, Duration::from_secs(5))?,
        })
    }

    /// Poll searching for the first available disk change
    pub fn poll(&self) -> FsWatcherResult<Option<FsChange>> {
        match self.receiver.recv_timeout(Duration::from_millis(1)) {
            Ok(DebouncedEvent::Rename(source, dest)) => Ok(self.build_fs_move(source, dest)),
            Ok(DebouncedEvent::NoticeRemove(p) | DebouncedEvent::Remove(p)) => {
                Ok(self.build_fs_remove(p))
            }
            Ok(
                DebouncedEvent::Chmod(p)
                | DebouncedEvent::Create(p)
                | DebouncedEvent::NoticeWrite(p)
                | DebouncedEvent::Write(p),
            ) => Ok(self.build_fs_update(p)),
            Ok(DebouncedEvent::Rescan) => Ok(None),
            Ok(DebouncedEvent::Error(e, _)) => {
                error!("FsWatcher reported error: {}", e);
                Err(e)
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
        }
        Ok(())
    }

    /// Returns whether `path` is currently watched.
    /// This method looks also in path ancestors.
    ///
    /// Example:
    /// if `/home` is watched, then if we call `watched("/home/foo/file.txt")` will return `true`
    pub fn watched(&self, path: &Path) -> bool {
        self.find_watched_path(path).is_some()
    }

    /// Unwatch provided path.
    /// When unwatching the path, it searches for the ancestor watched path if any
    pub fn unwatch(&mut self, path: &Path) -> FsWatcherResult<()> {
        let watched_path = self.find_watched_path(path).map(|x| x.0.to_path_buf());
        if let Some(watched_path) = watched_path {
            self.watcher.unwatch(watched_path.as_path())?;
            self.paths.remove(watched_path.as_path());
        }
        Ok(())
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
