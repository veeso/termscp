//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

use std::path::{Path, PathBuf};

use remotefs::fs::{File, Metadata};

use super::transfer::TransferPayload;
use crate::host::HostError;
use crate::ui::activities::filetransfer::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    // -- reload directory --

    /// Reload remote directory entries and update browser
    pub(in crate::ui::activities::filetransfer) fn reload_remote_dir(&mut self) {
        self.reload_dir_on(false);
    }

    /// Reload host_bridge directory entries and update browser
    pub(in crate::ui::activities::filetransfer) fn reload_host_bridge_dir(&mut self) {
        self.reload_dir_on(true);
    }

    /// Reload directory entries for the specified side.
    fn reload_dir_on(&mut self, local: bool) {
        let pane = if local {
            self.browser.local_pane()
        } else {
            self.browser.remote_pane()
        };
        if !pane.connected {
            return;
        }

        self.mount_blocking_wait("Loading directory...");

        let pane = if local {
            self.browser.local_pane_mut()
        } else {
            self.browser.remote_pane_mut()
        };
        let wrkdir = match pane.fs.pwd() {
            Ok(wrkdir) => wrkdir,
            Err(err) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not scan current directory: {err}"),
                );
                self.umount_wait();
                return;
            }
        };

        let res = self.scan_on(local, wrkdir.as_path());

        self.umount_wait();

        match res {
            Ok(_) => {
                let explorer = if local {
                    self.host_bridge_mut()
                } else {
                    self.remote_mut()
                };
                explorer.wrkdir = wrkdir;
            }
            Err(err) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not scan current directory: {err}"),
                );
            }
        }
    }

    /// Scan a directory on the specified side and update the explorer file list.
    fn scan_on(&mut self, local: bool, path: &Path) -> Result<(), HostError> {
        let pane = if local {
            self.browser.local_pane_mut()
        } else {
            self.browser.remote_pane_mut()
        };
        let files = pane.fs.list_dir(path)?;

        let explorer = if local {
            self.host_bridge_mut()
        } else {
            self.remote_mut()
        };
        explorer.set_files(files);
        Ok(())
    }

    // -- change directory --

    /// Change directory on the current tab's pane (no reload).
    pub(in crate::ui::activities::filetransfer) fn pane_changedir(
        &mut self,
        path: &Path,
        push: bool,
    ) {
        let prev_dir: PathBuf = self.browser.fs_pane().explorer.wrkdir.clone();
        match self.browser.fs_pane_mut().fs.change_wrkdir(path) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!("Changed directory: {}", path.display()),
                );
                if push {
                    self.browser
                        .fs_pane_mut()
                        .explorer
                        .pushd(prev_dir.as_path());
                }
            }
            Err(err) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not change working directory: {err}"),
                );
            }
        }
    }

    /// Change directory on the local pane and reload.
    pub(in crate::ui::activities::filetransfer) fn local_changedir(
        &mut self,
        path: &Path,
        push: bool,
    ) {
        let prev_dir: PathBuf = self.host_bridge().wrkdir.clone();
        match self.browser.local_pane_mut().fs.change_wrkdir(path) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!("Changed directory on host bridge: {}", path.display()),
                );
                self.reload_host_bridge_dir();
                if push {
                    self.host_bridge_mut().pushd(prev_dir.as_path())
                }
            }
            Err(err) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not change working directory: {err}"),
                );
            }
        }
    }

    /// Change directory on the remote pane and reload.
    pub(in crate::ui::activities::filetransfer) fn remote_changedir(
        &mut self,
        path: &Path,
        push: bool,
    ) {
        let prev_dir: PathBuf = self.remote().wrkdir.clone();
        match self.browser.remote_pane_mut().fs.change_wrkdir(path) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!("Changed directory on remote: {}", path.display()),
                );
                self.reload_remote_dir();
                if push {
                    self.remote_mut().pushd(prev_dir.as_path())
                }
            }
            Err(err) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not change working directory: {err}"),
                );
            }
        }
    }

    // -- temporary file download --

    /// Download provided file as a temporary file
    pub(in crate::ui::activities::filetransfer) fn download_file_as_temp(
        &mut self,
        file: &File,
    ) -> Result<PathBuf, String> {
        let tmpfile: PathBuf = match self.cache.as_ref() {
            Some(cache) => {
                let mut p: PathBuf = cache.path().to_path_buf();
                p.push(file.name());
                p
            }
            None => {
                return Err(String::from(
                    "Could not create tempfile: cache not available",
                ));
            }
        };
        match self.filetransfer_recv(
            TransferPayload::File(file.clone()),
            tmpfile.as_path(),
            Some(file.name()),
        ) {
            Err(err) => Err(format!(
                "Could not download {} to temporary file: {}",
                file.path.display(),
                err
            )),
            Ok(()) => Ok(tmpfile),
        }
    }

    // -- transfer sizes --

    /// Get total size of transfer for the specified side.
    pub(super) fn get_total_transfer_size(&mut self, entry: &File, local: bool) -> usize {
        self.mount_blocking_wait("Calculating transfer size…");

        let sz = if entry.is_dir() {
            let list_result = if local {
                self.browser.local_pane_mut().fs.list_dir(entry.path())
            } else {
                self.browser.remote_pane_mut().fs.list_dir(entry.path())
            };
            match list_result {
                Ok(files) => files
                    .iter()
                    .map(|x| self.get_total_transfer_size(x, local))
                    .sum(),
                Err(err) => {
                    self.log(
                        LogLevel::Error,
                        format!(
                            "Could not list directory {}: {}",
                            entry.path().display(),
                            err
                        ),
                    );
                    0
                }
            }
        } else {
            entry.metadata.size as usize
        };

        self.umount_wait();
        sz
    }

    // -- file changed --

    /// Check whether a file has changed on the specified side, compared to the given metadata.
    pub(super) fn has_file_changed(
        &mut self,
        path: &Path,
        other_metadata: &Metadata,
        local: bool,
    ) -> bool {
        let stat_result = if local {
            self.browser.local_pane_mut().fs.stat(path)
        } else {
            self.browser.remote_pane_mut().fs.stat(path)
        };
        if let Ok(file) = stat_result {
            other_metadata.modified != file.metadata().modified
                || other_metadata.size != file.metadata().size
        } else {
            true
        }
    }

    // -- file exist --

    /// Check whether a file exists on the specified side.
    pub(crate) fn file_exists(&mut self, p: &Path, local: bool) -> bool {
        let pane = if local {
            self.browser.local_pane_mut()
        } else {
            self.browser.remote_pane_mut()
        };
        pane.fs.exists(p).unwrap_or_default()
    }
}
