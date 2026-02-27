//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

use std::path::{Path, PathBuf};

use remotefs::RemoteResult;
use remotefs::fs::{File, Metadata};

use super::transfer::TransferPayload;
use crate::host::HostError;
use crate::ui::activities::filetransfer::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    /// Reload remote directory entries and update browser
    pub(in crate::ui::activities::filetransfer) fn reload_remote_dir(&mut self) {
        if !self.browser.remote_pane().connected {
            return;
        }
        // Get current entries
        if let Ok(wrkdir) = self.client.pwd() {
            self.mount_blocking_wait("Loading remote directory...");

            let res = self.remote_scan(wrkdir.as_path());

            self.umount_wait();

            match res {
                Ok(_) => {
                    self.remote_mut().wrkdir = wrkdir;
                }
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!("Could not scan current remote directory: {err}"),
                    );
                }
            }
        }
    }

    /// Reload host_bridge directory entries and update browser
    pub(in crate::ui::activities::filetransfer) fn reload_host_bridge_dir(&mut self) {
        if !self.browser.local_pane().connected {
            return;
        }

        self.mount_blocking_wait("Loading host bridge directory...");

        let wrkdir = match self.host_bridge.pwd() {
            Ok(wrkdir) => wrkdir,
            Err(err) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not scan current host bridge directory: {err}"),
                );
                return;
            }
        };

        let res = self.host_bridge_scan(wrkdir.as_path());

        self.umount_wait();

        match res {
            Ok(_) => {
                self.host_bridge_mut().wrkdir = wrkdir;
            }
            Err(err) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not scan current host bridge directory: {err}"),
                );
            }
        }
    }

    /// Scan current host bridge directory
    fn host_bridge_scan(&mut self, path: &Path) -> Result<(), HostError> {
        match self.host_bridge.list_dir(path) {
            Ok(files) => {
                // Set files and sort (sorting is implicit)
                self.host_bridge_mut().set_files(files);

                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    /// Scan current remote directory
    fn remote_scan(&mut self, path: &Path) -> RemoteResult<()> {
        match self.client.list_dir(path) {
            Ok(files) => {
                // Set files and sort (sorting is implicit)
                self.remote_mut().set_files(files);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    /// Change directory for host_bridge
    pub(in crate::ui::activities::filetransfer) fn host_bridge_changedir(
        &mut self,
        path: &Path,
        push: bool,
    ) {
        // Get current directory
        let prev_dir: PathBuf = self.host_bridge().wrkdir.clone();
        // Change directory
        match self.host_bridge.change_wrkdir(path) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!("Changed directory on host_bridge: {}", path.display()),
                );
                // Push prev_dir to stack
                if push {
                    self.host_bridge_mut().pushd(prev_dir.as_path())
                }
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not change working directory: {err}"),
                );
            }
        }
    }

    pub(in crate::ui::activities::filetransfer) fn local_changedir(
        &mut self,
        path: &Path,
        push: bool,
    ) {
        // Get current directory
        let prev_dir: PathBuf = self.host_bridge().wrkdir.clone();
        // Change directory
        match self.host_bridge.change_wrkdir(path) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!("Changed directory on host bridge: {}", path.display()),
                );
                // Update files
                self.reload_host_bridge_dir();
                // Push prev_dir to stack
                if push {
                    self.host_bridge_mut().pushd(prev_dir.as_path())
                }
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not change working directory: {err}"),
                );
            }
        }
    }

    pub(in crate::ui::activities::filetransfer) fn remote_changedir(
        &mut self,
        path: &Path,
        push: bool,
    ) {
        // Get current directory
        let prev_dir: PathBuf = self.remote().wrkdir.clone();
        // Change directory
        match self.client.as_mut().change_dir(path) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!("Changed directory on remote: {}", path.display()),
                );
                // Update files
                self.reload_remote_dir();
                // Push prev_dir to stack
                if push {
                    self.remote_mut().pushd(prev_dir.as_path())
                }
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not change working directory: {err}"),
                );
            }
        }
    }

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
        // Download file
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

    // -- transfer sizes

    /// Get total size of transfer for host_bridgehost
    pub(super) fn get_total_transfer_size_host(&mut self, entry: &File) -> usize {
        // mount message to tell we are calculating size
        self.mount_blocking_wait("Calculating transfer size…");

        let sz = if entry.is_dir() {
            // List dir
            match self.host_bridge.list_dir(entry.path()) {
                Ok(files) => files
                    .iter()
                    .map(|x| self.get_total_transfer_size_host(x))
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

    /// Get total size of transfer for remote host
    pub(super) fn get_total_transfer_size_remote(&mut self, entry: &File) -> usize {
        // mount message to tell we are calculating size
        self.mount_blocking_wait("Calculating transfer size…");

        let sz = if entry.is_dir() {
            // List directory
            match self.client.list_dir(entry.path()) {
                Ok(files) => files
                    .iter()
                    .map(|x| self.get_total_transfer_size_remote(x))
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

    // file changed

    /// Check whether provided file has changed on host_bridge disk, compared to remote file
    pub(super) fn has_host_bridge_file_changed(
        &mut self,
        host_bridge: &Path,
        remote: &File,
    ) -> bool {
        // check if files are equal (in case, don't transfer)
        if let Ok(host_bridge_file) = self.host_bridge.stat(host_bridge) {
            host_bridge_file.metadata().modified != remote.metadata().modified
                || host_bridge_file.metadata().size != remote.metadata().size
        } else {
            true
        }
    }

    /// Checks whether remote file has changed compared to host_bridge file
    pub(super) fn has_remote_file_changed(
        &mut self,
        remote: &Path,
        host_bridge_metadata: &Metadata,
    ) -> bool {
        // check if files are equal (in case, don't transfer)
        if let Ok(remote_file) = self.client.stat(remote) {
            host_bridge_metadata.modified != remote_file.metadata().modified
                || host_bridge_metadata.size != remote_file.metadata().size
        } else {
            true
        }
    }

    // -- file exist

    pub(crate) fn host_bridge_file_exists(&mut self, p: &Path) -> bool {
        self.host_bridge.exists(p).unwrap_or_default()
    }

    pub(crate) fn remote_file_exists(&mut self, p: &Path) -> bool {
        self.client.exists(p).unwrap_or_default()
    }
}
