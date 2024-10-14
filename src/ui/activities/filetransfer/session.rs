//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use bytesize::ByteSize;
use remotefs::fs::{File, Metadata, ReadStream, UnixPex, Welcome, WriteStream};
use remotefs::{RemoteError, RemoteErrorType, RemoteResult};
use thiserror::Error;

use super::{FileTransferActivity, LogLevel};
use crate::host::HostError;
use crate::utils::fmt::fmt_millis;

/// Buffer size for remote I/O
const BUFSIZE: usize = 65535;

/// Describes the reason that caused an error during a file transfer
#[derive(Error, Debug)]
enum TransferErrorReason {
    #[error("File transfer aborted")]
    Abrupted,
    #[error("I/O error on host_bridgehost: {0}")]
    HostIoError(std::io::Error),
    #[error("Host error: {0}")]
    HostError(HostError),
    #[error("I/O error on remote: {0}")]
    RemoteIoError(std::io::Error),
    #[error("File transfer error: {0}")]
    FileTransferError(RemoteError),
}

/// Represents the entity to send or receive during a transfer.
/// - File: describes an individual `File` to send
/// - Any: Can be any kind of `File`, but just one
/// - Many: a list of `File`
#[derive(Debug)]
pub(super) enum TransferPayload {
    File(File),
    Any(File),
    Many(Vec<File>),
}

impl FileTransferActivity {
    pub(super) fn connect_to_host_bridge(&mut self) {
        let ft_params = self.context().remote_params().unwrap().clone();
        let entry_dir: Option<PathBuf> = ft_params.local_path;
        // Connect to host bridge
        match self.host_bridge.connect() {
            Ok(()) => {
                self.host_bridge_connected = self.host_bridge.is_connected();
                if !self.host_bridge_connected {
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
    pub(super) fn connect_to_remote(&mut self) {
        let ft_params = self.context().remote_params().unwrap().clone();
        let entry_dir: Option<PathBuf> = ft_params.remote_path;
        // Connect to remote
        match self.client.connect() {
            Ok(Welcome { banner, .. }) => {
                self.remote_connected = self.client.is_connected();
                if !self.remote_connected {
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
    pub(super) fn disconnect(&mut self) {
        let msg: String = format!("Disconnecting from {}…", self.get_remote_hostname());
        // Show popup disconnecting
        self.mount_wait(msg.as_str());
        // Disconnect
        let _ = self.client.disconnect();
        // Quit
        self.exit_reason = Some(super::ExitReason::Disconnect);
    }

    /// disconnect from remote and then quit
    pub(super) fn disconnect_and_quit(&mut self) {
        self.disconnect();
        self.exit_reason = Some(super::ExitReason::Quit);
    }

    /// Reload remote directory entries and update browser
    pub(super) fn reload_remote_dir(&mut self) {
        if !self.remote_connected {
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
    pub(super) fn reload_host_bridge_dir(&mut self) {
        if !self.host_bridge_connected {
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

    /// Send fs entry to remote.
    /// If dst_name is Some, entry will be saved with a different name.
    /// If entry is a directory, this applies to directory only
    pub(super) fn filetransfer_send(
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
            TransferPayload::Many(ref entries) => {
                self.filetransfer_send_many(entries, curr_remote_path)
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
        let total_transfer_size: usize = self.get_total_transfer_size_host(entry);
        self.transfer.full.init(total_transfer_size);
        // Mount progress bar
        self.mount_progress_bar(format!("Uploading {}…", entry.path().display()));
        // Send recurse
        let result = self.filetransfer_send_recurse(entry, curr_remote_path, dst_name);
        // Umount progress bar
        self.umount_progress_bar();
        result
    }

    /// Send many entries to remote
    fn filetransfer_send_many(
        &mut self,
        entries: &[File],
        curr_remote_path: &Path,
    ) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Calculate total size of transfer
        let total_transfer_size: usize = entries
            .iter()
            .map(|x| self.get_total_transfer_size_host(x))
            .sum();
        self.transfer.full.init(total_transfer_size);
        // Mount progress bar
        self.mount_progress_bar(format!("Uploading {} entries…", entries.len()));
        // Send recurse
        let result = entries
            .iter()
            .map(|x| self.filetransfer_send_recurse(x, curr_remote_path, None))
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
                .client
                .create_dir(remote_path.as_path(), UnixPex::from(0o755))
            {
                Ok(_) => {
                    self.log(
                        LogLevel::Info,
                        format!("Created directory \"{}\"", remote_path.display()),
                    );
                }
                Err(err) if err.kind == RemoteErrorType::DirectoryAlreadyExists => {
                    self.log(
                        LogLevel::Info,
                        format!(
                            "Directory \"{}\" already exists on remote",
                            remote_path.display()
                        ),
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
            match self.host_bridge.list_dir(entry.path()) {
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
                        match self.client.stat(remote_path.as_path()) {
                            Err(err) => self.log(
                                LogLevel::Error,
                                format!(
                                    "Could not remove created file {}: {}",
                                    remote_path.display(),
                                    err
                                ),
                            ),
                            Ok(entry) => {
                                if let Err(err) = self.client.remove_file(entry.path()) {
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
            .host_bridge
            .stat(host_bridge.path.as_path())
            .map_err(TransferErrorReason::HostError)
            .map(|x| x.metadata().clone())?;

        if !self.has_remote_file_changed(remote, &metadata) {
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
        // Try to open host_bridge file
        match self.host_bridge.open_file(host_bridge.path.as_path()) {
            Ok(host_bridge_read) => match self.client.create(remote, &metadata) {
                Ok(rhnd) => self.filetransfer_send_one_with_stream(
                    host_bridge,
                    remote,
                    file_name,
                    host_bridge_read,
                    rhnd,
                ),
                Err(err) if err.kind == RemoteErrorType::UnsupportedFeature => self
                    .filetransfer_send_one_wno_stream(
                        host_bridge,
                        remote,
                        file_name,
                        host_bridge_read,
                    ),
                Err(err) => Err(TransferErrorReason::FileTransferError(err)),
            },
            Err(err) => Err(TransferErrorReason::HostError(err)),
        }
    }

    /// Send file to remote using stream
    fn filetransfer_send_one_with_stream(
        &mut self,
        host: &File,
        remote: &Path,
        file_name: String,
        mut reader: Box<dyn Read + Send>,
        mut writer: WriteStream,
    ) -> Result<(), TransferErrorReason> {
        // Write file
        let file_size = self
            .host_bridge
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
        if let Err(err) = self.client.on_written(writer) {
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
        if let Err(err) = self.client.setstat(remote, host.metadata().clone()) {
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

    /// Send an `File` to remote without using streams.
    fn filetransfer_send_one_wno_stream(
        &mut self,
        host: &File,
        remote: &Path,
        file_name: String,
        reader: Box<dyn Read + Send>,
    ) -> Result<(), TransferErrorReason> {
        // Sync file size and attributes before transfer
        let metadata = self
            .host_bridge
            .stat(host.path.as_path())
            .map_err(TransferErrorReason::HostError)
            .map(|x| x.metadata().clone())?;
        // Write file
        let file_size = self
            .host_bridge
            .stat(host.path())
            .map_err(TransferErrorReason::HostError)
            .map(|x| x.metadata().size as usize)?;
        // Init transfer
        self.transfer.partial.init(file_size);

        // Draw before
        self.update_progress_bar(format!("Uploading \"{file_name}\"…"));
        self.view();
        // Send file
        if let Err(err) = self.client.create_file(remote, &metadata, reader) {
            return Err(TransferErrorReason::FileTransferError(err));
        }
        // set stat
        if let Err(err) = self.client.setstat(remote, metadata) {
            error!("failed to set stat for {}: {}", remote.display(), err);
        }
        // Set transfer size ok
        self.transfer.partial.update_progress(file_size);
        self.transfer.full.update_progress(file_size);
        // Draw again after
        self.update_progress_bar(format!("Uploading \"{file_name}\"…"));
        self.view();
        // log and return Ok
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
    pub(super) fn filetransfer_recv(
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
            TransferPayload::Many(ref entries) => {
                self.filetransfer_recv_many(entries, host_bridge_path)
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
        let total_transfer_size: usize = self.get_total_transfer_size_remote(entry);
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

    /// Send many entries to remote
    fn filetransfer_recv_many(
        &mut self,
        entries: &[File],
        curr_remote_path: &Path,
    ) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Calculate total size of transfer
        let total_transfer_size: usize = entries
            .iter()
            .map(|x| self.get_total_transfer_size_remote(x))
            .sum();
        self.transfer.full.init(total_transfer_size);
        // Mount progress bar
        self.mount_progress_bar(format!("Downloading {} entries…", entries.len()));
        // Send recurse
        let result = entries
            .iter()
            .map(|x| self.filetransfer_recv_recurse(x, curr_remote_path, None))
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
                .host_bridge
                .mkdir_ex(host_bridge_dir_path.as_path(), true)
            {
                Ok(_) => {
                    // Apply file mode to directory
                    if let Err(err) = self
                        .host_bridge
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
                    // Get files in dir
                    match self.client.list_dir(entry.path()) {
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
                    match self.host_bridge.stat(host_bridge_file_path.as_path()) {
                        Err(err) => self.log(
                            LogLevel::Error,
                            format!(
                                "Could not remove created file {}: {}",
                                host_bridge_file_path.display(),
                                err
                            ),
                        ),
                        Ok(entry) => {
                            if let Err(err) = self.host_bridge.remove(&entry) {
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
        if !self.has_host_bridge_file_changed(host_bridge, remote) {
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

        // Try to open host_bridge file
        match self.host_bridge.create_file(host_bridge, &remote.metadata) {
            Ok(writer) => {
                // Download file from remote
                match self.client.open(remote.path.as_path()) {
                    Ok(rhnd) => self.filetransfer_recv_one_with_stream(
                        host_bridge,
                        remote,
                        file_name,
                        rhnd,
                        writer,
                    ),
                    Err(err) if err.kind == RemoteErrorType::UnsupportedFeature => {
                        self.filetransfer_recv_one_wno_stream(host_bridge, remote, file_name)
                    }
                    Err(err) => Err(TransferErrorReason::FileTransferError(err)),
                }
            }
            Err(err) => Err(TransferErrorReason::HostError(err)),
        }
    }

    /// Receive an `File` from remote using stream
    fn filetransfer_recv_one_with_stream(
        &mut self,
        host_bridge: &Path,
        remote: &File,
        file_name: String,
        mut reader: ReadStream,
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
        // Finalize stream
        if let Err(err) = self.client.on_read(reader) {
            self.log(
                LogLevel::Warn,
                format!("Could not finalize remote stream: \"{err}\""),
            );
        }
        // If download was abrupted, return Error
        if self.transfer.aborted() {
            return Err(TransferErrorReason::Abrupted);
        }

        // finalize write
        self.host_bridge
            .finalize_write(writer)
            .map_err(TransferErrorReason::HostError)?;

        // Apply file mode to file
        if let Err(err) = self.host_bridge.setstat(host_bridge, remote.metadata()) {
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

    /// Receive an `File` from remote without using stream
    fn filetransfer_recv_one_wno_stream(
        &mut self,
        host_bridge: &Path,
        remote: &File,
        file_name: String,
    ) -> Result<(), TransferErrorReason> {
        // Open host_bridge file
        let reader = self
            .host_bridge
            .create_file(host_bridge, &remote.metadata)
            .map_err(TransferErrorReason::HostError)
            .map(Box::new)?;
        // Init transfer
        self.transfer.partial.init(remote.metadata.size as usize);
        // Draw before transfer
        self.update_progress_bar(format!("Downloading \"{file_name}\""));
        self.view();
        // recv wno stream
        if let Err(err) = self.client.open_file(remote.path.as_path(), reader) {
            return Err(TransferErrorReason::FileTransferError(err));
        }
        // Update progress at the end
        self.transfer
            .partial
            .update_progress(remote.metadata.size as usize);
        self.transfer
            .full
            .update_progress(remote.metadata.size as usize);
        // Draw after transfer
        self.update_progress_bar(format!("Downloading \"{file_name}\""));
        self.view();
        // Apply file mode to file
        if let Err(err) = self.host_bridge.setstat(host_bridge, remote.metadata()) {
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

    /// Change directory for host_bridge
    pub(super) fn host_bridge_changedir(&mut self, path: &Path, push: bool) {
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

    pub(super) fn local_changedir(&mut self, path: &Path, push: bool) {
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

    pub(super) fn remote_changedir(&mut self, path: &Path, push: bool) {
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
    pub(super) fn download_file_as_temp(&mut self, file: &File) -> Result<PathBuf, String> {
        let tmpfile: PathBuf = match self.cache.as_ref() {
            Some(cache) => {
                let mut p: PathBuf = cache.path().to_path_buf();
                p.push(file.name());
                p
            }
            None => {
                return Err(String::from(
                    "Could not create tempfile: cache not available",
                ))
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
    fn get_total_transfer_size_host(&mut self, entry: &File) -> usize {
        if entry.is_dir() {
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
        }
    }

    /// Get total size of transfer for remote host
    fn get_total_transfer_size_remote(&mut self, entry: &File) -> usize {
        if entry.is_dir() {
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
        }
    }

    // file changed

    /// Check whether provided file has changed on host_bridge disk, compared to remote file
    fn has_host_bridge_file_changed(&mut self, host_bridge: &Path, remote: &File) -> bool {
        // check if files are equal (in case, don't transfer)
        if let Ok(host_bridge_file) = self.host_bridge.stat(host_bridge) {
            host_bridge_file.metadata().modified != remote.metadata().modified
                || host_bridge_file.metadata().size != remote.metadata().size
        } else {
            true
        }
    }

    /// Checks whether remote file has changed compared to host_bridge file
    fn has_remote_file_changed(&mut self, remote: &Path, host_bridge_metadata: &Metadata) -> bool {
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
