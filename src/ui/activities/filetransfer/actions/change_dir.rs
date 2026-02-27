//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::path::PathBuf;

use remotefs::File;

use super::{FileTransferActivity, LogLevel, Msg, PendingActionMsg};

/// Describes destination for sync browsing
enum SyncBrowsingDestination {
    Path(String),
    ParentDir,
    PreviousDir,
}

impl FileTransferActivity {
    /// Enter a directory from entry, dispatching via the active tab's pane.
    pub(crate) fn action_enter_dir(&mut self, dir: File) {
        self.pane_changedir(dir.path(), true);
        if self.browser.sync_browsing && self.browser.found().is_none() {
            self.synchronize_browsing(SyncBrowsingDestination::Path(dir.name()));
        }
    }

    /// Change directory reading value from input, dispatching via the active tab's pane.
    pub(crate) fn action_change_dir(&mut self, input: String) {
        let dir_path = self.pane_to_abs_path(PathBuf::from(input.as_str()).as_path());
        self.pane_changedir(dir_path.as_path(), true);
        // Check whether to sync
        if self.browser.sync_browsing && self.browser.found().is_none() {
            self.synchronize_browsing(SyncBrowsingDestination::Path(input));
        }
    }

    /// Go to previous directory, dispatching via the active tab's pane.
    pub(crate) fn action_go_to_previous_dir(&mut self) {
        let prev = self.browser.fs_pane_mut().explorer.popd();
        if let Some(d) = prev {
            self.pane_changedir(d.as_path(), false);
            // Check whether to sync
            if self.browser.sync_browsing && self.browser.found().is_none() {
                self.synchronize_browsing(SyncBrowsingDestination::PreviousDir);
            }
        }
    }

    /// Go to upper directory, dispatching via the active tab's pane.
    pub(crate) fn action_go_to_upper_dir(&mut self) {
        let path = self.browser.fs_pane().explorer.wrkdir.clone();
        if let Some(parent) = path.as_path().parent() {
            self.pane_changedir(parent, true);
            // If sync is enabled update the other side too
            if self.browser.sync_browsing && self.browser.found().is_none() {
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
        // Check whether destination exists on the opposite side
        let is_local = self.is_local_tab();
        let exists = if is_local {
            // Current tab is local, so the opposite is remote
            match self.browser.remote_pane_mut().fs.exists(path.as_path()) {
                Ok(e) => e,
                Err(err) => {
                    error!(
                        "Failed to check whether {} exists on remote: {}",
                        path.display(),
                        err
                    );
                    return;
                }
            }
        } else {
            // Current tab is remote, so the opposite is local
            match self.browser.local_pane_mut().fs.exists(path.as_path()) {
                Ok(e) => e,
                Err(err) => {
                    error!(
                        "Failed to check whether {} exists on host: {}",
                        path.display(),
                        err
                    );
                    return;
                }
            }
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
                // Make directory on the opposite side
                self.sync_mkdir_on_opposite(name.clone());
            } else {
                // Do not synchronize, disable sync browsing and return
                trace!(
                    "The user doesn't want to create the directory; disabling synchronized browsing"
                );
                self.log(
                    LogLevel::Warn,
                    format!("Refused to create '{name}'; synchronized browsing disabled"),
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
        // Enter directory on the opposite side
        let push = !matches!(destination, SyncBrowsingDestination::PreviousDir);
        if is_local {
            self.remote_changedir(path.as_path(), push);
        } else {
            self.local_changedir(path.as_path(), push);
        }
    }

    /// Create a directory on the opposite side for sync browsing.
    fn sync_mkdir_on_opposite(&mut self, name: String) {
        if self.is_local_tab() {
            self.action_remote_mkdir(name);
        } else {
            self.action_local_mkdir(name);
        }
    }

    /// Resolve synchronized browsing destination
    fn resolve_sync_browsing_destination(
        &mut self,
        destination: &SyncBrowsingDestination,
    ) -> Option<PathBuf> {
        let is_local = self.is_local_tab();
        match destination {
            // NOTE: tab and methods are switched on purpose (we resolve from the opposite side)
            SyncBrowsingDestination::ParentDir => {
                if is_local {
                    self.remote().wrkdir.parent().map(|x| x.to_path_buf())
                } else {
                    self.host_bridge().wrkdir.parent().map(|x| x.to_path_buf())
                }
            }
            SyncBrowsingDestination::PreviousDir => {
                if is_local {
                    if let Some(p) = self.remote_mut().popd() {
                        Some(p)
                    } else {
                        warn!(
                            "Cannot synchronize browsing: remote has no previous directory in stack"
                        );
                        None
                    }
                } else if let Some(p) = self.host_bridge_mut().popd() {
                    Some(p)
                } else {
                    warn!("Cannot synchronize browsing: local has no previous directory in stack");
                    None
                }
            }
            SyncBrowsingDestination::Path(p) => Some(PathBuf::from(p.as_str())),
        }
    }
}
