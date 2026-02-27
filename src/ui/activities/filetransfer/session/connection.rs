//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

use std::path::PathBuf;

use remotefs::fs::Welcome;

use crate::ui::activities::filetransfer::{FileTransferActivity, LogLevel};

impl FileTransferActivity {
    pub(in crate::ui::activities::filetransfer) fn connect_to_host_bridge(&mut self) {
        let ft_params = self.context().remote_params().unwrap().clone();
        let entry_dir: Option<PathBuf> = ft_params.local_path;
        // Connect to host bridge
        match self.host_bridge.connect() {
            Ok(()) => {
                self.browser.local_pane_mut().connected = self.host_bridge.is_connected();
                if !self.browser.local_pane().connected {
                    return;
                }

                // Log welcome
                self.log(
                    LogLevel::Info,
                    format!(
                        "Established connection with '{}'",
                        self.get_hostbridge_hostname()
                    ),
                );

                // Try to change directory to entry directory
                let mut remote_chdir: Option<PathBuf> = None;
                if let Some(remote_path) = &entry_dir {
                    remote_chdir = Some(remote_path.clone());
                }
                if let Some(remote_path) = remote_chdir {
                    self.local_changedir(remote_path.as_path(), false);
                }
                // Set state to explorer
                self.umount_wait();
                self.reload_host_bridge_dir();
                // Update file lists
                self.update_host_bridge_filelist();
            }
            Err(err) => {
                // Set popup fatal error
                self.umount_wait();
                self.mount_fatal(err.to_string());
            }
        }
    }

    /// Connect to remote
    pub(in crate::ui::activities::filetransfer) fn connect_to_remote(&mut self) {
        let ft_params = self.context().remote_params().unwrap().clone();
        let entry_dir: Option<PathBuf> = ft_params.remote_path;
        // Connect to remote
        match self.client.connect() {
            Ok(Welcome { banner, .. }) => {
                self.browser.remote_pane_mut().connected = self.client.is_connected();
                if !self.browser.remote_pane().connected {
                    return;
                }

                if let Some(banner) = banner {
                    // Log welcome
                    self.log(
                        LogLevel::Info,
                        format!(
                            "Established connection with '{}': \"{}\"",
                            self.get_remote_hostname(),
                            banner
                        ),
                    );
                } else {
                    // Log welcome
                    self.log(
                        LogLevel::Info,
                        format!(
                            "Established connection with '{}'",
                            self.get_remote_hostname()
                        ),
                    );
                }
                // Try to change directory to entry directory
                let mut remote_chdir: Option<PathBuf> = None;
                if let Some(remote_path) = &entry_dir {
                    remote_chdir = Some(remote_path.clone());
                }
                if let Some(remote_path) = remote_chdir {
                    self.remote_changedir(remote_path.as_path(), false);
                }
                // Set state to explorer
                self.umount_wait();
                self.reload_remote_dir();
                // Update file lists
                self.update_host_bridge_filelist();
                self.update_remote_filelist();
            }
            Err(err) => {
                // Set popup fatal error
                self.umount_wait();
                self.mount_fatal(err.to_string());
            }
        }
    }

    /// disconnect from remote
    pub(in crate::ui::activities::filetransfer) fn disconnect(&mut self) {
        let msg: String = format!("Disconnecting from {}…", self.get_remote_hostname());
        // Show popup disconnecting
        self.mount_wait(msg.as_str());
        // Disconnect
        let _ = self.client.disconnect();
        // Quit
        self.exit_reason = Some(super::super::ExitReason::Disconnect);
    }

    /// disconnect from remote and then quit
    pub(in crate::ui::activities::filetransfer) fn disconnect_and_quit(&mut self) {
        self.disconnect();
        self.exit_reason = Some(super::super::ExitReason::Quit);
    }
}
