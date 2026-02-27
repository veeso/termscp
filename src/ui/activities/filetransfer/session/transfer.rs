//! ## FileTransferActivity
//!
//! `filetransfer_activity` is the module which implements the Filetransfer activity, which is the main activity afterall

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use bytesize::ByteSize;
use remotefs::fs::File;
use thiserror::Error;

use crate::host::HostError;
use crate::ui::activities::filetransfer::{FileTransferActivity, LogLevel};
use crate::utils::fmt::fmt_millis;

/// Buffer size for remote I/O
const BUFSIZE: usize = 65535;

/// Describes the reason that caused an error during a file transfer
#[derive(Error, Debug)]
enum TransferErrorReason {
    #[error("File transfer aborted")]
    Abrupted,
    #[error("I/O error on host_bridge: {0}")]
    HostIoError(std::io::Error),
    #[error("Host error: {0}")]
    HostError(HostError),
    #[error("I/O error on remote: {0}")]
    RemoteIoError(std::io::Error),
    #[error("Remote error: {0}")]
    RemoteHostError(HostError),
}

/// Represents the entity to send or receive during a transfer.
/// - File: describes an individual `File` to send
/// - Any: Can be any kind of `File`, but just one
/// - Many: a list of `File`
#[derive(Debug)]
pub(in crate::ui::activities::filetransfer) enum TransferPayload {
    File(File),
    Any(File),
    /// List of file with their destination name
    TransferQueue(Vec<(File, PathBuf)>),
}

impl FileTransferActivity {
    /// Send fs entry to remote.
    /// If dst_name is Some, entry will be saved with a different name.
    /// If entry is a directory, this applies to directory only
    pub(in crate::ui::activities::filetransfer) fn filetransfer_send(
        &mut self,
        payload: TransferPayload,
        curr_remote_path: &Path,
        dst_name: Option<String>,
    ) -> Result<(), String> {
        // Use different method based on payload
        let result = match payload {
            TransferPayload::Any(ref entry) => {
                self.filetransfer_send_any(entry, curr_remote_path, dst_name)
            }
            TransferPayload::File(ref file) => {
                self.filetransfer_send_file(file, curr_remote_path, dst_name)
            }
            TransferPayload::TransferQueue(ref entries) => {
                self.filetransfer_send_transfer_queue(entries)
            }
        };
        // Notify
        match &result {
            Ok(_) => {
                self.notify_transfer_completed(&payload);
            }
            Err(e) => {
                self.notify_transfer_error(e.as_str());
            }
        }
        result
    }

    /// Send one file to remote at specified path.
    fn filetransfer_send_file(
        &mut self,
        file: &File,
        curr_remote_path: &Path,
        dst_name: Option<String>,
    ) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Calculate total size of transfer
        let total_transfer_size: usize = file.metadata.size as usize;
        self.transfer.full.init(total_transfer_size);
        // Mount progress bar
        self.mount_progress_bar(format!("Uploading {}…", file.path.display()));
        // Get remote path
        let file_name: String = file.name();
        let mut remote_path: PathBuf = PathBuf::from(curr_remote_path);
        let remote_file_name: PathBuf = match dst_name {
            Some(s) => PathBuf::from(s.as_str()),
            None => PathBuf::from(file_name.as_str()),
        };
        remote_path.push(remote_file_name);
        // Send
        let result = self.filetransfer_send_one(file, remote_path.as_path(), file_name);
        // Umount progress bar
        self.umount_progress_bar();
        // Return result
        result.map_err(|x| x.to_string())
    }

    /// Send a `TransferPayload` of type `Any`
    fn filetransfer_send_any(
        &mut self,
        entry: &File,
        curr_remote_path: &Path,
        dst_name: Option<String>,
    ) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Calculate total size of transfer
        let total_transfer_size: usize = self.get_total_transfer_size(entry, true);
        self.transfer.full.init(total_transfer_size);
        // Mount progress bar
        self.mount_progress_bar(format!("Uploading {}…", entry.path().display()));
        // Send recurse
        let result = self.filetransfer_send_recurse(entry, curr_remote_path, dst_name);
        // Umount progress bar
        self.umount_progress_bar();
        result
    }

    /// Send transfer queue entries to remote
    fn filetransfer_send_transfer_queue(
        &mut self,
        entries: &[(File, PathBuf)],
    ) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Calculate total size of transfer
        let total_transfer_size: usize = entries
            .iter()
            .map(|(x, _)| self.get_total_transfer_size(x, true))
            .sum();
        self.transfer.full.init(total_transfer_size);
        // Mount progress bar
        self.mount_progress_bar(format!("Uploading {} entries…", entries.len()));
        // Send recurse
        let result = entries
            .iter()
            .map(|(x, remote)| self.filetransfer_send_recurse(x, remote, None))
            .find(|x| x.is_err())
            .unwrap_or(Ok(()));
        // Umount progress bar
        self.umount_progress_bar();
        result
    }

    fn filetransfer_send_recurse(
        &mut self,
        entry: &File,
        curr_remote_path: &Path,
        dst_name: Option<String>,
    ) -> Result<(), String> {
        // Write popup
        let file_name = entry.name();
        // Get remote path
        let mut remote_path: PathBuf = PathBuf::from(curr_remote_path);
        let remote_file_name: PathBuf = match dst_name {
            Some(s) => PathBuf::from(s.as_str()),
            None => PathBuf::from(file_name.as_str()),
        };
        remote_path.push(remote_file_name);
        // Match entry
        let result: Result<(), String> = if entry.is_dir() {
            // Create directory on remote first
            match self
                .browser
                .remote_pane_mut()
                .fs
                .mkdir_ex(remote_path.as_path(), true)
            {
                Ok(_) => {
                    self.log(
                        LogLevel::Info,
                        format!("Created directory \"{}\"", remote_path.display()),
                    );
                }
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!(
                            "Failed to create directory \"{}\": {}",
                            remote_path.display(),
                            err
                        ),
                    );
                    return Err(err.to_string());
                }
            }
            // Get files in dir
            match self.browser.local_pane_mut().fs.list_dir(entry.path()) {
                Ok(entries) => {
                    // Iterate over files
                    for entry in entries.iter() {
                        // If aborted; break
                        if self.transfer.aborted() {
                            break;
                        }
                        // Send entry; name is always None after first call
                        self.filetransfer_send_recurse(entry, remote_path.as_path(), None)?
                    }
                    Ok(())
                }
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!(
                            "Could not scan directory \"{}\": {}",
                            entry.path().display(),
                            err
                        ),
                    );
                    Err(err.to_string())
                }
            }
        } else {
            match self.filetransfer_send_one(entry, remote_path.as_path(), file_name) {
                Err(err) => {
                    // If transfer was abrupted or there was an IO error on remote, remove file
                    if matches!(
                        err,
                        TransferErrorReason::Abrupted | TransferErrorReason::RemoteIoError(_)
                    ) {
                        // Stat file on remote and remove it if exists
                        match self
                            .browser
                            .remote_pane_mut()
                            .fs
                            .stat(remote_path.as_path())
                        {
                            Err(err) => self.log(
                                LogLevel::Error,
                                format!(
                                    "Could not remove created file {}: {}",
                                    remote_path.display(),
                                    err
                                ),
                            ),
                            Ok(entry) => {
                                if let Err(err) = self.browser.remote_pane_mut().fs.remove(&entry) {
                                    self.log(
                                        LogLevel::Error,
                                        format!(
                                            "Could not remove created file {}: {}",
                                            remote_path.display(),
                                            err
                                        ),
                                    );
                                }
                            }
                        }
                    }
                    Err(err.to_string())
                }
                Ok(_) => Ok(()),
            }
        };
        // Scan dir on remote
        self.reload_remote_dir();
        // If aborted; show popup
        if self.transfer.aborted() {
            // Log abort
            self.log_and_alert(
                LogLevel::Warn,
                format!("Upload aborted for \"{}\"!", entry.path().display()),
            );
        }
        result
    }

    /// Send host_bridge file and write it to remote path
    fn filetransfer_send_one(
        &mut self,
        host_bridge: &File,
        remote: &Path,
        file_name: String,
    ) -> Result<(), TransferErrorReason> {
        // Sync file size and attributes before transfer
        let metadata = self
            .browser
            .local_pane_mut()
            .fs
            .stat(host_bridge.path.as_path())
            .map_err(TransferErrorReason::HostError)
            .map(|x| x.metadata().clone())?;

        if !self.has_file_changed(remote, &metadata, false) {
            self.log(
                LogLevel::Info,
                format!(
                    "file {} won't be transferred since hasn't changed",
                    host_bridge.path().display()
                ),
            );
            self.transfer.full.update_progress(metadata.size as usize);
            return Ok(());
        }
        // Upload file
        // Open host_bridge file for reading
        let reader = self
            .browser
            .local_pane_mut()
            .fs
            .open_file(host_bridge.path.as_path())
            .map_err(TransferErrorReason::HostError)?;
        // Open remote file for writing
        let writer = self
            .browser
            .remote_pane_mut()
            .fs
            .create_file(remote, &metadata)
            .map_err(TransferErrorReason::RemoteHostError)?;

        self.filetransfer_send_one_with_stream(host_bridge, remote, file_name, reader, writer)
    }

    /// Send file to remote using stream
    fn filetransfer_send_one_with_stream(
        &mut self,
        host: &File,
        remote: &Path,
        file_name: String,
        mut reader: Box<dyn Read + Send>,
        mut writer: Box<dyn Write + Send>,
    ) -> Result<(), TransferErrorReason> {
        // Write file
        let file_size = self
            .browser
            .local_pane_mut()
            .fs
            .stat(host.path())
            .map_err(TransferErrorReason::HostError)
            .map(|x| x.metadata().size as usize)?;
        // Init transfer
        self.transfer.partial.init(file_size);

        // Write remote file
        let mut total_bytes_written: usize = 0;
        let mut last_progress_val: f64 = 0.0;
        let mut last_input_event_fetch: Option<Instant> = None;
        // While the entire file hasn't been completely written,
        // Or filetransfer has been aborted
        while total_bytes_written < file_size && !self.transfer.aborted() {
            // Handle input events (each 500ms) or if never fetched before
            if last_input_event_fetch.is_none()
                || last_input_event_fetch
                    .unwrap_or_else(Instant::now)
                    .elapsed()
                    .as_millis()
                    >= 500
            {
                // Read events
                self.tick();
                // Reset instant
                last_input_event_fetch = Some(Instant::now());
            }
            // Read till you can
            let mut buffer: [u8; BUFSIZE] = [0; BUFSIZE];
            let delta: usize = match reader.read(&mut buffer) {
                Ok(bytes_read) => {
                    total_bytes_written += bytes_read;
                    if bytes_read == 0 {
                        continue;
                    } else {
                        let mut delta: usize = 0;
                        while delta < bytes_read {
                            // Write bytes
                            match writer.write(&buffer[delta..bytes_read]) {
                                Ok(bytes) => {
                                    delta += bytes;
                                }
                                Err(err) => {
                                    return Err(TransferErrorReason::RemoteIoError(err));
                                }
                            }
                        }
                        delta
                    }
                }
                Err(err) => {
                    return Err(TransferErrorReason::HostIoError(err));
                }
            };
            // Increase progress
            self.transfer.partial.update_progress(delta);
            self.transfer.full.update_progress(delta);
            // Draw only if a significant progress has been made (performance improvement)
            if last_progress_val < self.transfer.partial.calc_progress() - 0.01 {
                // Draw
                self.update_progress_bar(format!("Uploading \"{file_name}\"…"));
                self.view();
                last_progress_val = self.transfer.partial.calc_progress();
            }
        }
        // Finalize stream
        if let Err(err) = self.browser.remote_pane_mut().fs.finalize_write(writer) {
            self.log(
                LogLevel::Warn,
                format!("Could not finalize remote stream: \"{err}\""),
            );
        }
        // if upload was abrupted, return error
        if self.transfer.aborted() {
            return Err(TransferErrorReason::Abrupted);
        }
        // set stat
        if let Err(err) = self
            .browser
            .remote_pane_mut()
            .fs
            .setstat(remote, host.metadata())
        {
            error!("failed to set stat for {}: {}", remote.display(), err);
        }
        self.log(
            LogLevel::Info,
            format!(
                "Saved file \"{}\" to \"{}\" (took {} seconds; at {}/s)",
                host.path.display(),
                remote.display(),
                fmt_millis(self.transfer.partial.started().elapsed()),
                ByteSize(self.transfer.partial.calc_bytes_per_second()),
            ),
        );
        Ok(())
    }

    /// Recv fs entry from remote.
    /// If dst_name is Some, entry will be saved with a different name.
    /// If entry is a directory, this applies to directory only
    pub(in crate::ui::activities::filetransfer) fn filetransfer_recv(
        &mut self,
        payload: TransferPayload,
        host_bridge_path: &Path,
        dst_name: Option<String>,
    ) -> Result<(), String> {
        let result = match payload {
            TransferPayload::Any(ref entry) => {
                self.filetransfer_recv_any(entry, host_bridge_path, dst_name)
            }
            TransferPayload::File(ref file) => self.filetransfer_recv_file(file, host_bridge_path),
            TransferPayload::TransferQueue(ref entries) => {
                self.filetransfer_recv_transfer_queue(entries)
            }
        };
        // Notify
        match &result {
            Ok(_) => {
                self.notify_transfer_completed(&payload);
            }
            Err(e) => {
                self.notify_transfer_error(e.as_str());
            }
        }
        result
    }

    /// Recv fs entry from remote.
    /// If dst_name is Some, entry will be saved with a different name.
    /// If entry is a directory, this applies to directory only
    fn filetransfer_recv_any(
        &mut self,
        entry: &File,
        host_path: &Path,
        dst_name: Option<String>,
    ) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Calculate total transfer size
        let total_transfer_size: usize = self.get_total_transfer_size(entry, false);
        self.transfer.full.init(total_transfer_size);
        // Mount progress bar
        self.mount_progress_bar(format!("Downloading {}…", entry.path().display()));
        // Receive
        let result = self.filetransfer_recv_recurse(entry, host_path, dst_name);
        // Umount progress bar
        self.umount_progress_bar();
        result
    }

    /// Receive a single file from remote.
    fn filetransfer_recv_file(
        &mut self,
        entry: &File,
        host_bridge_path: &Path,
    ) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Calculate total transfer size
        let total_transfer_size: usize = entry.metadata.size as usize;
        self.transfer.full.init(total_transfer_size);
        // Mount progress bar
        self.mount_progress_bar(format!("Downloading {}…", entry.path.display()));
        // Receive
        let result = self.filetransfer_recv_one(host_bridge_path, entry, entry.name());
        // Umount progress bar
        self.umount_progress_bar();
        // Return result
        result.map_err(|x| x.to_string())
    }

    /// Receive transfer queue from remote
    fn filetransfer_recv_transfer_queue(
        &mut self,
        entries: &[(File, PathBuf)],
    ) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Calculate total size of transfer
        let total_transfer_size: usize = entries
            .iter()
            .map(|(x, _)| self.get_total_transfer_size(x, false))
            .sum();
        self.transfer.full.init(total_transfer_size);
        // Mount progress bar
        self.mount_progress_bar(format!("Downloading {} entries…", entries.len()));
        // Send recurse
        let result = entries
            .iter()
            .map(|(x, path)| self.filetransfer_recv_recurse(x, path, None))
            .find(|x| x.is_err())
            .unwrap_or(Ok(()));
        // Umount progress bar
        self.umount_progress_bar();
        result
    }

    fn filetransfer_recv_recurse(
        &mut self,
        entry: &File,
        host_bridge_path: &Path,
        dst_name: Option<String>,
    ) -> Result<(), String> {
        // Write popup
        let file_name = entry.name();
        // Match entry
        let result: Result<(), String> = if entry.is_dir() {
            // Get dir name
            let mut host_bridge_dir_path: PathBuf = PathBuf::from(host_bridge_path);
            match dst_name {
                Some(name) => host_bridge_dir_path.push(name),
                None => host_bridge_dir_path.push(entry.name()),
            }
            // Create directory on host_bridge
            match self
                .browser
                .local_pane_mut()
                .fs
                .mkdir_ex(host_bridge_dir_path.as_path(), true)
            {
                Ok(_) => {
                    // Apply file mode to directory
                    if let Err(err) = self
                        .browser
                        .local_pane_mut()
                        .fs
                        .setstat(host_bridge_dir_path.as_path(), entry.metadata())
                    {
                        self.log(
                            LogLevel::Error,
                            format!(
                                "Could not set stat to directory {:?} to \"{}\": {}",
                                entry.metadata(),
                                host_bridge_dir_path.display(),
                                err
                            ),
                        );
                    }
                    self.log(
                        LogLevel::Info,
                        format!("Created directory \"{}\"", host_bridge_dir_path.display()),
                    );
                    // Get files in dir from remote
                    match self.browser.remote_pane_mut().fs.list_dir(entry.path()) {
                        Ok(entries) => {
                            // Iterate over files
                            for entry in entries.iter() {
                                // If transfer has been aborted; break
                                if self.transfer.aborted() {
                                    break;
                                }
                                // Receive entry; name is always None after first call
                                // Local path becomes host_bridge_dir_path
                                self.filetransfer_recv_recurse(
                                    entry,
                                    host_bridge_dir_path.as_path(),
                                    None,
                                )?
                            }
                            Ok(())
                        }
                        Err(err) => {
                            self.log_and_alert(
                                LogLevel::Error,
                                format!(
                                    "Could not scan directory \"{}\": {}",
                                    entry.path().display(),
                                    err
                                ),
                            );
                            Err(err.to_string())
                        }
                    }
                }
                Err(err) => {
                    self.log(
                        LogLevel::Error,
                        format!(
                            "Failed to create directory \"{}\": {}",
                            host_bridge_dir_path.display(),
                            err
                        ),
                    );
                    Err(err.to_string())
                }
            }
        } else {
            // Get host_bridge file
            let mut host_bridge_file_path: PathBuf = PathBuf::from(host_bridge_path);
            let host_bridge_file_name: String = match dst_name {
                Some(n) => n,
                None => entry.name(),
            };
            host_bridge_file_path.push(host_bridge_file_name.as_str());
            // Download file
            if let Err(err) =
                self.filetransfer_recv_one(host_bridge_file_path.as_path(), entry, file_name)
            {
                // If transfer was abrupted or there was an IO error on remote, remove file
                if matches!(
                    err,
                    TransferErrorReason::Abrupted | TransferErrorReason::HostIoError(_)
                ) {
                    // Stat file
                    match self
                        .browser
                        .local_pane_mut()
                        .fs
                        .stat(host_bridge_file_path.as_path())
                    {
                        Err(err) => self.log(
                            LogLevel::Error,
                            format!(
                                "Could not remove created file {}: {}",
                                host_bridge_file_path.display(),
                                err
                            ),
                        ),
                        Ok(entry) => {
                            if let Err(err) = self.browser.local_pane_mut().fs.remove(&entry) {
                                self.log(
                                    LogLevel::Error,
                                    format!(
                                        "Could not remove created file {}: {}",
                                        host_bridge_file_path.display(),
                                        err
                                    ),
                                );
                            }
                        }
                    }
                }
                Err(err.to_string())
            } else {
                Ok(())
            }
        };
        // Reload directory on host_bridge
        self.reload_host_bridge_dir();
        // if aborted; show alert
        if self.transfer.aborted() {
            // Log abort
            self.log_and_alert(
                LogLevel::Warn,
                format!("Download aborted for \"{}\"!", entry.path().display()),
            );
        }
        result
    }

    /// Receive file from remote and write it to host_bridge path
    fn filetransfer_recv_one(
        &mut self,
        host_bridge: &Path,
        remote: &File,
        file_name: String,
    ) -> Result<(), TransferErrorReason> {
        // check if files are equal (in case, don't transfer)
        if !self.has_file_changed(host_bridge, remote.metadata(), true) {
            self.log(
                LogLevel::Info,
                format!(
                    "file {} won't be transferred since hasn't changed",
                    remote.path().display()
                ),
            );
            self.transfer
                .full
                .update_progress(remote.metadata().size as usize);
            return Ok(());
        }

        // Open host_bridge file for writing
        let writer = self
            .browser
            .local_pane_mut()
            .fs
            .create_file(host_bridge, &remote.metadata)
            .map_err(TransferErrorReason::HostError)?;
        // Open remote file for reading
        let reader = self
            .browser
            .remote_pane_mut()
            .fs
            .open_file(remote.path.as_path())
            .map_err(TransferErrorReason::RemoteHostError)?;

        self.filetransfer_recv_one_with_stream(host_bridge, remote, file_name, reader, writer)
    }

    /// Receive an `File` from remote using stream
    fn filetransfer_recv_one_with_stream(
        &mut self,
        host_bridge: &Path,
        remote: &File,
        file_name: String,
        mut reader: Box<dyn Read + Send>,
        mut writer: Box<dyn Write + Send>,
    ) -> Result<(), TransferErrorReason> {
        let mut total_bytes_written: usize = 0;
        // Init transfer
        self.transfer.partial.init(remote.metadata.size as usize);
        // Write host_bridge file
        let mut last_progress_val: f64 = 0.0;
        let mut last_input_event_fetch: Option<Instant> = None;
        // While the entire file hasn't been completely read,
        // Or filetransfer has been aborted
        while total_bytes_written < remote.metadata.size as usize && !self.transfer.aborted() {
            // Handle input events (each 500 ms) or is None
            if last_input_event_fetch.is_none()
                || last_input_event_fetch
                    .unwrap_or_else(Instant::now)
                    .elapsed()
                    .as_millis()
                    >= 500
            {
                // Read events
                self.tick();
                // Reset instant
                last_input_event_fetch = Some(Instant::now());
            }
            // Read till you can
            let mut buffer: [u8; BUFSIZE] = [0; BUFSIZE];
            let delta: usize = match reader.read(&mut buffer) {
                Ok(bytes_read) => {
                    total_bytes_written += bytes_read;
                    if bytes_read == 0 {
                        continue;
                    } else {
                        let mut delta: usize = 0;
                        while delta < bytes_read {
                            // Write bytes
                            match writer.write(&buffer[delta..bytes_read]) {
                                Ok(bytes) => delta += bytes,
                                Err(err) => {
                                    return Err(TransferErrorReason::HostIoError(err));
                                }
                            }
                        }
                        delta
                    }
                }
                Err(err) => {
                    return Err(TransferErrorReason::RemoteIoError(err));
                }
            };
            // Set progress
            self.transfer.partial.update_progress(delta);
            self.transfer.full.update_progress(delta);
            // Draw only if a significant progress has been made (performance improvement)
            if last_progress_val < self.transfer.partial.calc_progress() - 0.01 {
                // Draw
                self.update_progress_bar(format!("Downloading \"{file_name}\""));
                self.view();
                last_progress_val = self.transfer.partial.calc_progress();
            }
        }
        // If download was abrupted, return Error
        if self.transfer.aborted() {
            return Err(TransferErrorReason::Abrupted);
        }

        // Finalize write
        self.browser
            .local_pane_mut()
            .fs
            .finalize_write(writer)
            .map_err(TransferErrorReason::HostError)?;

        // Apply file mode to file
        if let Err(err) = self
            .browser
            .local_pane_mut()
            .fs
            .setstat(host_bridge, remote.metadata())
        {
            self.log(
                LogLevel::Error,
                format!(
                    "Could not set stat to file {:?} to \"{}\": {}",
                    remote.metadata(),
                    host_bridge.display(),
                    err
                ),
            );
        }
        // Log
        self.log(
            LogLevel::Info,
            format!(
                "Saved file \"{}\" to \"{}\" (took {} seconds; at {}/s)",
                remote.path.display(),
                host_bridge.display(),
                fmt_millis(self.transfer.partial.started().elapsed()),
                ByteSize(self.transfer.partial.calc_bytes_per_second()),
            ),
        );

        Ok(())
    }
}
