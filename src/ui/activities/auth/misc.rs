//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

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
use super::{AuthActivity, FileTransferParams, FileTransferProtocol};
use crate::filetransfer::params::ProtocolParams;
use crate::system::auto_update::{Release, Update, UpdateStatus};
use crate::system::notifications::Notification;

impl AuthActivity {
    /// Get the default port for protocol
    pub(super) fn get_default_port_for_protocol(protocol: FileTransferProtocol) -> u16 {
        match protocol {
            FileTransferProtocol::Sftp | FileTransferProtocol::Scp => 22,
            FileTransferProtocol::Ftp(_) => 21,
            FileTransferProtocol::AwsS3 => 22, // Doesn't matter, since not used
        }
    }

    /// Returns whether the port is standard or not
    pub(super) fn is_port_standard(port: u16) -> bool {
        port < 1024
    }

    /// Check minimum window size window
    pub(super) fn check_minimum_window_size(&mut self, height: u16) {
        if height < 25 {
            // Mount window error
            self.mount_size_err();
        } else {
            self.umount_size_err();
        }
    }

    /// Collect host params as `FileTransferParams`
    pub(super) fn collect_host_params(&self) -> Result<FileTransferParams, &'static str> {
        match self.protocol {
            FileTransferProtocol::AwsS3 => self.collect_s3_host_params(),
            protocol => self.collect_generic_host_params(protocol),
        }
    }

    /// Get input values from fields or return an error if fields are invalid to work as generic
    pub(super) fn collect_generic_host_params(
        &self,
        protocol: FileTransferProtocol,
    ) -> Result<FileTransferParams, &'static str> {
        let params = self.get_generic_params_input();
        if params.address.is_empty() {
            return Err("Invalid host");
        }
        if params.port == 0 {
            return Err("Invalid port");
        }
        Ok(FileTransferParams {
            protocol,
            params: ProtocolParams::Generic(params),
            entry_directory: None,
        })
    }

    /// Get input values from fields or return an error if fields are invalid to work as aws s3
    pub(super) fn collect_s3_host_params(&self) -> Result<FileTransferParams, &'static str> {
        let params = self.get_s3_params_input();
        if params.bucket_name.is_empty() {
            return Err("Invalid bucket");
        }
        Ok(FileTransferParams {
            protocol: FileTransferProtocol::AwsS3,
            params: ProtocolParams::AwsS3(params),
            entry_directory: None,
        })
    }

    // -- update install

    /// If enabled in configuration, check for updates from Github
    pub(super) fn check_for_updates(&mut self) {
        debug!("Check for updates...");
        // Check version only if unset in the store
        let ctx = self.context_mut();
        if !ctx.store().isset(super::STORE_KEY_LATEST_VERSION) {
            debug!("Version is not set in storage");
            if ctx.config().get_check_for_updates() {
                debug!("Check for updates is enabled");
                // Send request
                match Update::is_new_version_available() {
                    Ok(Some(Release { version, body })) => {
                        // If some, store version and release notes
                        info!("Latest version is: {}", version);
                        if ctx.config().get_notifications() {
                            // Notify new version available
                            Notification::update_available(version.as_str());
                        }
                        // Store info
                        ctx.store_mut()
                            .set_string(super::STORE_KEY_LATEST_VERSION, version);
                        ctx.store_mut()
                            .set_string(super::STORE_KEY_RELEASE_NOTES, body);
                    }
                    Ok(None) => {
                        info!("Latest version is: {} (current)", env!("CARGO_PKG_VERSION"));
                        // Just set flag as check
                        ctx.store_mut().set(super::STORE_KEY_LATEST_VERSION);
                    }
                    Err(err) => {
                        // Report error
                        error!("Failed to get latest version: {}", err);
                        self.mount_error(
                            format!("Could not check for new updates: {}", err).as_str(),
                        );
                    }
                }
            } else {
                info!("Check for updates is disabled");
            }
        }
    }

    /// Install latest termscp version via GUI
    pub(super) fn install_update(&mut self) {
        // Umount release notes
        self.umount_release_notes();
        // Mount wait box
        self.mount_wait("Installing update. Please wait…");
        // Refresh UI
        self.view();
        // Install update
        let result = Update::default().show_progress(false).upgrade();
        // Umount wait
        self.umount_wait();
        // Show outcome
        match result {
            Ok(UpdateStatus::AlreadyUptodate) => self.mount_info("termscp is already up to date!"),
            Ok(UpdateStatus::UpdateInstalled(ver)) => {
                if self.config().get_notifications() {
                    Notification::update_installed(ver.as_str());
                }
                self.mount_info(format!("termscp has been updated to version {}!", ver))
            }
            Err(err) => {
                if self.config().get_notifications() {
                    Notification::update_failed(err.to_string());
                }
                self.mount_error(format!("Could not install update: {}", err))
            }
        }
    }
}
