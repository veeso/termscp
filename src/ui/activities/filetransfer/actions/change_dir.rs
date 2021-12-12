//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

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
// locals
use super::{FileExplorerTab, FileTransferActivity, LogLevel, Msg, PendingActionMsg};

use remotefs::Directory;
use std::path::PathBuf;

/// Describes destination for sync browsing
enum SyncBrowsingDestination {
    Path(String),
    ParentDir,
    PreviousDir,
}

impl FileTransferActivity {
    /// Enter a directory on local host from entry
    pub(crate) fn action_enter_local_dir(&mut self, dir: Directory) {
        self.local_changedir(dir.path.as_path(), true);
        if self.browser.sync_browsing {
            self.synchronize_browsing(SyncBrowsingDestination::Path(dir.name));
        }
    }

    /// Enter a directory on local host from entry
    pub(crate) fn action_enter_remote_dir(&mut self, dir: Directory) {
        self.remote_changedir(dir.path.as_path(), true);
        if self.browser.sync_browsing {
            self.synchronize_browsing(SyncBrowsingDestination::Path(dir.name));
        }
    }

    /// Change local directory reading value from input
    pub(crate) fn action_change_local_dir(&mut self, input: String) {
        let dir_path: PathBuf = self.local_to_abs_path(PathBuf::from(input.as_str()).as_path());
        self.local_changedir(dir_path.as_path(), true);
        // Check whether to sync
        if self.browser.sync_browsing {
            self.synchronize_browsing(SyncBrowsingDestination::Path(input));
        }
    }

    /// Change remote directory reading value from input
    pub(crate) fn action_change_remote_dir(&mut self, input: String) {
        let dir_path: PathBuf = self.remote_to_abs_path(PathBuf::from(input.as_str()).as_path());
        self.remote_changedir(dir_path.as_path(), true);
        // Check whether to sync
        if self.browser.sync_browsing {
            self.synchronize_browsing(SyncBrowsingDestination::Path(input));
        }
    }

    /// Go to previous directory from localhost
    pub(crate) fn action_go_to_previous_local_dir(&mut self) {
        if let Some(d) = self.local_mut().popd() {
            self.local_changedir(d.as_path(), false);
            // Check whether to sync
            if self.browser.sync_browsing {
                self.synchronize_browsing(SyncBrowsingDestination::PreviousDir);
            }
        }
    }

    /// Go to previous directory from remote host
    pub(crate) fn action_go_to_previous_remote_dir(&mut self) {
        if let Some(d) = self.remote_mut().popd() {
            self.remote_changedir(d.as_path(), false);
            // Check whether to sync
            if self.browser.sync_browsing {
                self.synchronize_browsing(SyncBrowsingDestination::PreviousDir);
            }
        }
    }

    /// Go to upper directory on local host
    pub(crate) fn action_go_to_local_upper_dir(&mut self) {
        // Get pwd
        let path: PathBuf = self.local().wrkdir.clone();
        // Go to parent directory
        if let Some(parent) = path.as_path().parent() {
            self.local_changedir(parent, true);
            // If sync is enabled update remote too
            if self.browser.sync_browsing {
                self.synchronize_browsing(SyncBrowsingDestination::ParentDir);
            }
        }
    }

    /// #### action_go_to_remote_upper_dir
    ///
    /// Go to upper directory on remote host
    pub(crate) fn action_go_to_remote_upper_dir(&mut self) {
        // Get pwd
        let path: PathBuf = self.remote().wrkdir.clone();
        // Go to parent directory
        if let Some(parent) = path.as_path().parent() {
            self.remote_changedir(parent, true);
            // If sync is enabled update local too
            if self.browser.sync_browsing {
                self.synchronize_browsing(SyncBrowsingDestination::ParentDir);
            }
        }
    }

    // -- sync browsing

    /// Synchronize browsing on the target browser.
    /// If destination doesn't exist, then prompt for directory creation.
    fn synchronize_browsing(&mut self, destination: SyncBrowsingDestination) {
        // Get destination path
        let path = match self.resolve_sync_browsing_destination(&destination) {
            Some(p) => p,
            None => return,
        };
        trace!("Synchronizing browsing to path {}", path.display());
        // Check whether destination exists on host
        let exists = match self.browser.tab() {
            FileExplorerTab::Local => match self.client.exists(path.as_path()) {
                Ok(e) => e,
                Err(err) => {
                    error!(
                        "Failed to check whether {} exists on remote: {}",
                        path.display(),
                        err
                    );
                    return;
                }
            },
            FileExplorerTab::Remote => self.host.file_exists(path.as_path()),
            _ => return,
        };
        let name = path
            .file_name()
            .map(|x| x.to_string_lossy().to_string())
            .unwrap();
        // If file doesn't exist, ask whether to create directory
        if !exists {
            trace!("Directory doesn't exist; asking to user if I should create it");
            // Mount dialog
            self.mount_sync_browsing_mkdir_popup(&name);
            // Wait for dialog dismiss
            if self.wait_for_pending_msg(&[
                Msg::PendingAction(PendingActionMsg::MakePendingDirectory),
                Msg::PendingAction(PendingActionMsg::CloseSyncBrowsingMkdirPopup),
            ]) == Msg::PendingAction(PendingActionMsg::MakePendingDirectory)
            {
                trace!("User wants to create the unexisting directory");
                // Make directory
                match self.browser.tab() {
                    FileExplorerTab::Local => self.action_remote_mkdir(name.clone()),
                    FileExplorerTab::Remote => self.action_local_mkdir(name.clone()),
                    _ => {}
                }
            } else {
                // Do not synchronize, disable sync browsing and return
                trace!("The user doesn't want to create the directory; disabling synchronized browsing");
                self.log(
                    LogLevel::Warn,
                    format!(
                        "Refused to create '{}'; synchronized browsing disabled",
                        name
                    ),
                );
                self.browser.toggle_sync_browsing();
                self.refresh_remote_status_bar();
                self.umount_sync_browsing_mkdir_popup();
                return;
            }
            // Umount dialog
            self.umount_sync_browsing_mkdir_popup();
        }
        trace!("Entering on the other explorer directory {}", name);
        // Enter directory
        match destination {
            SyncBrowsingDestination::ParentDir => match self.browser.tab() {
                FileExplorerTab::Local => self.remote_changedir(path.as_path(), true),
                FileExplorerTab::Remote => self.local_changedir(path.as_path(), true),
                _ => {}
            },
            SyncBrowsingDestination::Path(_) => match self.browser.tab() {
                FileExplorerTab::Local => self.remote_changedir(path.as_path(), true),
                FileExplorerTab::Remote => self.local_changedir(path.as_path(), true),
                _ => {}
            },
            SyncBrowsingDestination::PreviousDir => match self.browser.tab() {
                FileExplorerTab::Local => self.remote_changedir(path.as_path(), false),
                FileExplorerTab::Remote => self.local_changedir(path.as_path(), false),
                _ => {}
            },
        }
    }

    /// Resolve synchronized browsing destination
    fn resolve_sync_browsing_destination(
        &mut self,
        destination: &SyncBrowsingDestination,
    ) -> Option<PathBuf> {
        match (destination, self.browser.tab()) {
            // NOTE: tab and methods are switched on purpose
            (SyncBrowsingDestination::ParentDir, FileExplorerTab::Local) => {
                self.remote().wrkdir.parent().map(|x| x.to_path_buf())
            }
            (SyncBrowsingDestination::ParentDir, FileExplorerTab::Remote) => {
                self.local().wrkdir.parent().map(|x| x.to_path_buf())
            }
            (SyncBrowsingDestination::PreviousDir, FileExplorerTab::Local) => {
                if let Some(p) = self.remote_mut().popd() {
                    Some(p)
                } else {
                    warn!("Cannot synchronize browsing: remote has no previous directory in stack");
                    None
                }
            }
            (SyncBrowsingDestination::PreviousDir, FileExplorerTab::Remote) => {
                if let Some(p) = self.local_mut().popd() {
                    Some(p)
                } else {
                    warn!("Cannot synchronize browsing: local has no previous directory in stack");
                    None
                }
            }
            (SyncBrowsingDestination::Path(p), _) => Some(PathBuf::from(p.as_str())),
            _ => {
                warn!("Cannot synchronize browsing for current explorer");
                None
            }
        }
    }
}
