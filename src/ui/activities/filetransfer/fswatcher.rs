use std::path::Path;

use super::{FileTransferActivity, LogLevel, TransferPayload};
use crate::system::watcher::FsChange;

impl FileTransferActivity {
    /// poll file watcher
    pub(super) fn poll_watcher(&mut self) {
        if self.fswatcher.is_none() {
            return;
        }
        let watcher = self.fswatcher.as_mut().unwrap();
        match watcher.poll() {
            Ok(None) => {}
            Ok(Some(FsChange::Move(mov))) => {
                debug!(
                    "fs watcher reported a `Move` from {} to {}",
                    mov.source().display(),
                    mov.destination().display()
                );
                self.move_watched_file(mov.source(), mov.destination());
            }
            Ok(Some(FsChange::Remove(remove))) => {
                debug!(
                    "fs watcher reported a `Remove` of {}",
                    remove.path().display()
                );
                self.remove_watched_file(remove.path());
            }
            Ok(Some(FsChange::Update(update))) => {
                debug!(
                    "fs watcher reported an `Update` from {} to {}",
                    update.host_bridge().display(),
                    update.remote().display()
                );
                self.upload_watched_file(update.host_bridge(), update.remote());
            }
            Err(err) => {
                self.log(
                    LogLevel::Error,
                    format!("error while polling file watcher: {err}"),
                );
            }
        }
    }

    fn move_watched_file(&mut self, source: &Path, destination: &Path) {
        // stat remote file
        trace!(
            "renaming watched file {} to {}",
            source.display(),
            destination.display()
        );
        // stat fs entry
        let origin = match self.browser.remote_pane_mut().fs.stat(source) {
            Ok(f) => f,
            Err(err) => {
                self.log(
                    LogLevel::Error,
                    format!(
                        "failed to stat file to rename {}: {}",
                        source.display(),
                        err
                    ),
                );
                return;
            }
        };
        // rename using action
        self.remote_rename_file(&origin, destination)
    }

    fn remove_watched_file(&mut self, file: &Path) {
        // stat the file first to use HostBridge::remove
        let entry = match self.browser.remote_pane_mut().fs.stat(file) {
            Ok(e) => e,
            Err(err) => {
                self.log(
                    LogLevel::Error,
                    format!("failed to stat watched file {}: {}", file.display(), err),
                );
                return;
            }
        };
        match self.browser.remote_pane_mut().fs.remove(&entry) {
            Ok(()) => {
                self.log(
                    LogLevel::Info,
                    format!("removed watched file at {}", file.display()),
                );
            }
            Err(err) => {
                self.log(
                    LogLevel::Error,
                    format!("failed to remove watched file {}: {}", file.display(), err),
                );
            }
        }
    }

    fn upload_watched_file(&mut self, host: &Path, remote: &Path) {
        // stat host file
        let entry = match self.browser.local_pane_mut().fs.stat(host) {
            Ok(e) => e,
            Err(err) => {
                self.log(
                    LogLevel::Error,
                    format!(
                        "failed to sync file {} with remote (stat failed): {}",
                        remote.display(),
                        err
                    ),
                );
                return;
            }
        };
        // send
        trace!(
            "syncing host file {} with remote {}",
            host.display(),
            remote.display()
        );
        let remote_path = remote.parent().unwrap_or_else(|| Path::new("/"));
        match self.filetransfer_send(TransferPayload::Any(entry), remote_path, None) {
            Ok(()) => {
                self.log(
                    LogLevel::Info,
                    format!(
                        "synched watched file {} with {}",
                        host.display(),
                        remote.display()
                    ),
                );
            }
            Err(err) => {
                self.log(
                    LogLevel::Error,
                    format!("failed to sync watched file {}: {}", remote.display(), err),
                );
            }
        }
    }
}
