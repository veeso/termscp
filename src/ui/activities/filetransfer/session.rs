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
// Locals
use super::{FileTransferActivity, LogLevel};
use crate::host::HostError;
use crate::utils::fmt::fmt_millis;

// Ext
use bytesize::ByteSize;
use remotefs::fs::{Entry, File, UnixPex, Welcome};
use remotefs::{RemoteError, RemoteErrorType};
use std::fs::File as StdFile;
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use thiserror::Error;

/// Describes the reason that caused an error during a file transfer
#[derive(Error, Debug)]
enum TransferErrorReason {
    #[error("File transfer aborted")]
    Abrupted,
    #[error("Failed to seek file: {0}")]
    CouldNotRewind(std::io::Error),
    #[error("I/O error on localhost: {0}")]
    LocalIoError(std::io::Error),
    #[error("Host error: {0}")]
    HostError(HostError),
    #[error("I/O error on remote: {0}")]
    RemoteIoError(std::io::Error),
    #[error("File transfer error: {0}")]
    FileTransferError(RemoteError),
}

/// Represents the entity to send or receive during a transfer.
/// - File: describes an individual `File` to send
/// - Any: Can be any kind of `Entry`, but just one
/// - Many: a list of `Entry`
#[derive(Debug)]
pub(super) enum TransferPayload {
    File(File),
    Any(Entry),
    Many(Vec<Entry>),
}

impl FileTransferActivity {
    /// Connect to remote
    pub(super) fn connect(&mut self) {
        let ft_params = self.context().ft_params().unwrap().clone();
        let entry_dir: Option<PathBuf> = ft_params.entry_directory;
        // Connect to remote
        match self.client.connect() {
            Ok(Welcome { banner, .. }) => {
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
                }
                // Try to change directory to entry directory
                let mut remote_chdir: Option<PathBuf> = None;
                if let Some(entry_directory) = &entry_dir {
                    remote_chdir = Some(entry_directory.clone());
                }
                if let Some(entry_directory) = remote_chdir {
                    self.remote_changedir(entry_directory.as_path(), false);
                }
                // Set state to explorer
                self.umount_wait();
                self.reload_remote_dir();
                // Update file lists
                self.update_local_filelist();
                self.update_remote_filelist();
            }
            Err(err) => {
                // Set popup fatal error
                self.umount_wait();
                self.mount_fatal(&err.to_string());
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
        // Get current entries
        if let Ok(wrkdir) = self.client.pwd() {
            self.remote_scan(wrkdir.as_path());
            // Set wrkdir
            self.remote_mut().wrkdir = wrkdir;
        }
    }

    /// Reload local directory entries and update browser
    pub(super) fn reload_local_dir(&mut self) {
        let wrkdir: PathBuf = self.host.pwd();
        self.local_scan(wrkdir.as_path());
        self.local_mut().wrkdir = wrkdir;
    }

    /// Scan current local directory
    fn local_scan(&mut self, path: &Path) {
        match self.host.scan_dir(path) {
            Ok(files) => {
                // Set files and sort (sorting is implicit)
                self.local_mut().set_files(files);
            }
            Err(err) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not scan current directory: {}", err),
                );
            }
        }
    }

    /// Scan current remote directory
    fn remote_scan(&mut self, path: &Path) {
        match self.client.list_dir(path) {
            Ok(files) => {
                // Set files and sort (sorting is implicit)
                self.remote_mut().set_files(files);
            }
            Err(err) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not scan current directory: {}", err),
                );
            }
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
        let file_name: String = file.name.clone();
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
        entry: &Entry,
        curr_remote_path: &Path,
        dst_name: Option<String>,
    ) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Calculate total size of transfer
        let total_transfer_size: usize = self.get_total_transfer_size_local(entry);
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
        entries: &[Entry],
        curr_remote_path: &Path,
    ) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Calculate total size of transfer
        let total_transfer_size: usize = entries
            .iter()
            .map(|x| self.get_total_transfer_size_local(x))
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
        entry: &Entry,
        curr_remote_path: &Path,
        dst_name: Option<String>,
    ) -> Result<(), String> {
        // Write popup
        let file_name: String = match entry {
            Entry::Directory(dir) => dir.name.clone(),
            Entry::File(file) => file.name.clone(),
        };
        // Get remote path
        let mut remote_path: PathBuf = PathBuf::from(curr_remote_path);
        let remote_file_name: PathBuf = match dst_name {
            Some(s) => PathBuf::from(s.as_str()),
            None => PathBuf::from(file_name.as_str()),
        };
        remote_path.push(remote_file_name);
        // Match entry
        let result: Result<(), String> = match entry {
            Entry::File(file) => {
                match self.filetransfer_send_one(file, remote_path.as_path(), file_name) {
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
            }
            Entry::Directory(dir) => {
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
                match self.host.scan_dir(dir.path.as_path()) {
                    Ok(entries) => {
                        // Iterate over files
                        for entry in entries.iter() {
                            // If aborted; break
                            if self.transfer.aborted() {
                                break;
                            }
                            // Send entry; name is always None after first call
                            if let Err(err) =
                                self.filetransfer_send_recurse(entry, remote_path.as_path(), None)
                            {
                                return Err(err);
                            }
                        }
                        Ok(())
                    }
                    Err(err) => {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!(
                                "Could not scan directory \"{}\": {}",
                                dir.path.display(),
                                err
                            ),
                        );
                        Err(err.to_string())
                    }
                }
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

    /// Send local file and write it to remote path
    fn filetransfer_send_one(
        &mut self,
        local: &File,
        remote: &Path,
        file_name: String,
    ) -> Result<(), TransferErrorReason> {
        // Sync file size and attributes before transfer
        let metadata = self
            .host
            .stat(local.path.as_path())
            .map_err(TransferErrorReason::HostError)
            .map(|x| x.metadata().clone())?;
        // Upload file
        // Try to open local file
        match self.host.open_file_read(local.path.as_path()) {
            Ok(fhnd) => match self.client.create(remote, &metadata) {
                Ok(rhnd) => {
                    self.filetransfer_send_one_with_stream(local, remote, file_name, fhnd, rhnd)
                }
                Err(err) if err.kind == RemoteErrorType::UnsupportedFeature => {
                    self.filetransfer_send_one_wno_stream(local, remote, file_name, fhnd)
                }
                Err(err) => Err(TransferErrorReason::FileTransferError(err)),
            },
            Err(err) => Err(TransferErrorReason::HostError(err)),
        }
    }

    /// Send file to remote using stream
    fn filetransfer_send_one_with_stream(
        &mut self,
        local: &File,
        remote: &Path,
        file_name: String,
        mut reader: StdFile,
        mut writer: Box<dyn Write>,
    ) -> Result<(), TransferErrorReason> {
        // Write file
        let file_size: usize = reader.seek(std::io::SeekFrom::End(0)).unwrap_or(0) as usize;
        // Init transfer
        self.transfer.partial.init(file_size);
        // rewind
        if let Err(err) = reader.seek(std::io::SeekFrom::Start(0)) {
            return Err(TransferErrorReason::CouldNotRewind(err));
        }
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
            let mut buffer: [u8; 65536] = [0; 65536];
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
                    return Err(TransferErrorReason::LocalIoError(err));
                }
            };
            // Increase progress
            self.transfer.partial.update_progress(delta);
            self.transfer.full.update_progress(delta);
            // Draw only if a significant progress has been made (performance improvement)
            if last_progress_val < self.transfer.partial.calc_progress() - 0.01 {
                // Draw
                self.update_progress_bar(format!("Uploading \"{}\"…", file_name));
                self.view();
                last_progress_val = self.transfer.partial.calc_progress();
            }
        }
        // Finalize stream
        if let Err(err) = self.client.on_written(writer) {
            self.log(
                LogLevel::Warn,
                format!("Could not finalize remote stream: \"{}\"", err),
            );
        }
        // if upload was abrupted, return error
        if self.transfer.aborted() {
            return Err(TransferErrorReason::Abrupted);
        }
        self.log(
            LogLevel::Info,
            format!(
                "Saved file \"{}\" to \"{}\" (took {} seconds; at {}/s)",
                local.path.display(),
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
        local: &File,
        remote: &Path,
        file_name: String,
        mut reader: StdFile,
    ) -> Result<(), TransferErrorReason> {
        // Sync file size and attributes before transfer
        let metadata = self
            .host
            .stat(local.path.as_path())
            .map_err(TransferErrorReason::HostError)
            .map(|x| x.metadata().clone())?;
        // Write file
        let file_size: usize = reader.seek(std::io::SeekFrom::End(0)).unwrap_or(0) as usize;
        // Init transfer
        self.transfer.partial.init(file_size);
        // rewind
        if let Err(err) = reader.seek(std::io::SeekFrom::Start(0)) {
            return Err(TransferErrorReason::CouldNotRewind(err));
        }
        // Draw before
        self.update_progress_bar(format!("Uploading \"{}\"…", file_name));
        self.view();
        // Send file
        if let Err(err) = self.client.create_file(remote, &metadata, Box::new(reader)) {
            return Err(TransferErrorReason::FileTransferError(err));
        }
        // Set transfer size ok
        self.transfer.partial.update_progress(file_size);
        self.transfer.full.update_progress(file_size);
        // Draw again after
        self.update_progress_bar(format!("Uploading \"{}\"…", file_name));
        self.view();
        // log and return Ok
        self.log(
            LogLevel::Info,
            format!(
                "Saved file \"{}\" to \"{}\" (took {} seconds; at {}/s)",
                local.path.display(),
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
        local_path: &Path,
        dst_name: Option<String>,
    ) -> Result<(), String> {
        let result = match payload {
            TransferPayload::Any(ref entry) => {
                self.filetransfer_recv_any(entry, local_path, dst_name)
            }
            TransferPayload::File(ref file) => self.filetransfer_recv_file(file, local_path),
            TransferPayload::Many(ref entries) => self.filetransfer_recv_many(entries, local_path),
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
        entry: &Entry,
        local_path: &Path,
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
        let result = self.filetransfer_recv_recurse(entry, local_path, dst_name);
        // Umount progress bar
        self.umount_progress_bar();
        result
    }

    /// Receive a single file from remote.
    fn filetransfer_recv_file(&mut self, entry: &File, local_path: &Path) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Calculate total transfer size
        let total_transfer_size: usize = entry.metadata.size as usize;
        self.transfer.full.init(total_transfer_size);
        // Mount progress bar
        self.mount_progress_bar(format!("Downloading {}…", entry.path.display()));
        // Receive
        let result = self.filetransfer_recv_one(local_path, entry, entry.name.clone());
        // Umount progress bar
        self.umount_progress_bar();
        // Return result
        result.map_err(|x| x.to_string())
    }

    /// Send many entries to remote
    fn filetransfer_recv_many(
        &mut self,
        entries: &[Entry],
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
        entry: &Entry,
        local_path: &Path,
        dst_name: Option<String>,
    ) -> Result<(), String> {
        // Write popup
        let file_name: String = match entry {
            Entry::Directory(dir) => dir.name.clone(),
            Entry::File(file) => file.name.clone(),
        };
        // Match entry
        let result: Result<(), String> = match entry {
            Entry::File(file) => {
                // Get local file
                let mut local_file_path: PathBuf = PathBuf::from(local_path);
                let local_file_name: String = match dst_name {
                    Some(n) => n,
                    None => file.name.clone(),
                };
                local_file_path.push(local_file_name.as_str());
                // Download file
                if let Err(err) =
                    self.filetransfer_recv_one(local_file_path.as_path(), file, file_name)
                {
                    // If transfer was abrupted or there was an IO error on remote, remove file
                    if matches!(
                        err,
                        TransferErrorReason::Abrupted | TransferErrorReason::LocalIoError(_)
                    ) {
                        // Stat file
                        match self.host.stat(local_file_path.as_path()) {
                            Err(err) => self.log(
                                LogLevel::Error,
                                format!(
                                    "Could not remove created file {}: {}",
                                    local_file_path.display(),
                                    err
                                ),
                            ),
                            Ok(entry) => {
                                if let Err(err) = self.host.remove(&entry) {
                                    self.log(
                                        LogLevel::Error,
                                        format!(
                                            "Could not remove created file {}: {}",
                                            local_file_path.display(),
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
            }
            Entry::Directory(dir) => {
                // Get dir name
                let mut local_dir_path: PathBuf = PathBuf::from(local_path);
                match dst_name {
                    Some(name) => local_dir_path.push(name),
                    None => local_dir_path.push(dir.name.as_str()),
                }
                // Create directory on local
                match self.host.mkdir_ex(local_dir_path.as_path(), true) {
                    Ok(_) => {
                        // Apply file mode to directory
                        #[cfg(any(
                            target_family = "unix",
                            target_os = "macos",
                            target_os = "linux"
                        ))]
                        if let Some(mode) = dir.metadata.mode {
                            if let Err(err) = self.host.chmod(local_dir_path.as_path(), mode) {
                                self.log(
                                    LogLevel::Error,
                                    format!(
                                        "Could not apply file mode {:o} to \"{}\": {}",
                                        u32::from(mode),
                                        local_dir_path.display(),
                                        err
                                    ),
                                );
                            }
                        }
                        self.log(
                            LogLevel::Info,
                            format!("Created directory \"{}\"", local_dir_path.display()),
                        );
                        // Get files in dir
                        match self.client.list_dir(dir.path.as_path()) {
                            Ok(entries) => {
                                // Iterate over files
                                for entry in entries.iter() {
                                    // If transfer has been aborted; break
                                    if self.transfer.aborted() {
                                        break;
                                    }
                                    // Receive entry; name is always None after first call
                                    // Local path becomes local_dir_path
                                    if let Err(err) = self.filetransfer_recv_recurse(
                                        entry,
                                        local_dir_path.as_path(),
                                        None,
                                    ) {
                                        return Err(err);
                                    }
                                }
                                Ok(())
                            }
                            Err(err) => {
                                self.log_and_alert(
                                    LogLevel::Error,
                                    format!(
                                        "Could not scan directory \"{}\": {}",
                                        dir.path.display(),
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
                                local_dir_path.display(),
                                err
                            ),
                        );
                        Err(err.to_string())
                    }
                }
            }
        };
        // Reload directory on local
        self.reload_local_dir();
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

    /// Receive file from remote and write it to local path
    fn filetransfer_recv_one(
        &mut self,
        local: &Path,
        remote: &File,
        file_name: String,
    ) -> Result<(), TransferErrorReason> {
        // Try to open local file
        match self.host.open_file_write(local) {
            Ok(local_file) => {
                // Download file from remote
                match self.client.open(remote.path.as_path()) {
                    Ok(rhnd) => self.filetransfer_recv_one_with_stream(
                        local, remote, file_name, rhnd, local_file,
                    ),
                    Err(err) if err.kind == RemoteErrorType::UnsupportedFeature => {
                        self.filetransfer_recv_one_wno_stream(local, remote, file_name)
                    }
                    Err(err) => Err(TransferErrorReason::FileTransferError(err)),
                }
            }
            Err(err) => Err(TransferErrorReason::HostError(err)),
        }
    }

    /// Receive an `Entry` from remote using stream
    fn filetransfer_recv_one_with_stream(
        &mut self,
        local: &Path,
        remote: &File,
        file_name: String,
        mut reader: Box<dyn Read>,
        mut writer: StdFile,
    ) -> Result<(), TransferErrorReason> {
        let mut total_bytes_written: usize = 0;
        // Init transfer
        self.transfer.partial.init(remote.metadata.size as usize);
        // Write local file
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
            let mut buffer: [u8; 65536] = [0; 65536];
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
                                    return Err(TransferErrorReason::LocalIoError(err));
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
                self.update_progress_bar(format!("Downloading \"{}\"", file_name));
                self.view();
                last_progress_val = self.transfer.partial.calc_progress();
            }
        }
        // Finalize stream
        if let Err(err) = self.client.on_read(reader) {
            self.log(
                LogLevel::Warn,
                format!("Could not finalize remote stream: \"{}\"", err),
            );
        }
        // If download was abrupted, return Error
        if self.transfer.aborted() {
            return Err(TransferErrorReason::Abrupted);
        }
        // Apply file mode to file
        #[cfg(target_family = "unix")]
        if let Some(mode) = remote.metadata.mode {
            if let Err(err) = self.host.chmod(local, mode) {
                self.log(
                    LogLevel::Error,
                    format!(
                        "Could not apply file mode {:o} to \"{}\": {}",
                        u32::from(mode),
                        local.display(),
                        err
                    ),
                );
            }
        }
        // Log
        self.log(
            LogLevel::Info,
            format!(
                "Saved file \"{}\" to \"{}\" (took {} seconds; at {}/s)",
                remote.path.display(),
                local.display(),
                fmt_millis(self.transfer.partial.started().elapsed()),
                ByteSize(self.transfer.partial.calc_bytes_per_second()),
            ),
        );
        Ok(())
    }

    /// Receive an `Entry` from remote without using stream
    fn filetransfer_recv_one_wno_stream(
        &mut self,
        local: &Path,
        remote: &File,
        file_name: String,
    ) -> Result<(), TransferErrorReason> {
        // Open local file
        let reader = self
            .host
            .open_file_write(local)
            .map_err(TransferErrorReason::HostError)
            .map(Box::new)?;
        // Init transfer
        self.transfer.partial.init(remote.metadata.size as usize);
        // Draw before transfer
        self.update_progress_bar(format!("Downloading \"{}\"", file_name));
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
        self.update_progress_bar(format!("Downloading \"{}\"", file_name));
        self.view();
        // Apply file mode to file
        #[cfg(target_family = "unix")]
        if let Some(mode) = remote.metadata.mode {
            if let Err(err) = self.host.chmod(local, mode) {
                self.log(
                    LogLevel::Error,
                    format!(
                        "Could not apply file mode {:o} to \"{}\": {}",
                        u32::from(mode),
                        local.display(),
                        err
                    ),
                );
            }
        }
        // Log
        self.log(
            LogLevel::Info,
            format!(
                "Saved file \"{}\" to \"{}\" (took {} seconds; at {}/s)",
                remote.path.display(),
                local.display(),
                fmt_millis(self.transfer.partial.started().elapsed()),
                ByteSize(self.transfer.partial.calc_bytes_per_second()),
            ),
        );
        Ok(())
    }

    /// Change directory for local
    pub(super) fn local_changedir(&mut self, path: &Path, push: bool) {
        // Get current directory
        let prev_dir: PathBuf = self.local().wrkdir.clone();
        // Change directory
        match self.host.change_wrkdir(path) {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!("Changed directory on local: {}", path.display()),
                );
                // Push prev_dir to stack
                if push {
                    self.local_mut().pushd(prev_dir.as_path())
                }
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not change working directory: {}", err),
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
                    format!("Could not change working directory: {}", err),
                );
            }
        }
    }

    /// Download provided file as a temporary file
    pub(super) fn download_file_as_temp(&mut self, file: &File) -> Result<PathBuf, String> {
        let tmpfile: PathBuf = match self.cache.as_ref() {
            Some(cache) => {
                let mut p: PathBuf = cache.path().to_path_buf();
                p.push(file.name.as_str());
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
            Some(file.name.clone()),
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

    /// Get total size of transfer for localhost
    fn get_total_transfer_size_local(&mut self, entry: &Entry) -> usize {
        match entry {
            Entry::File(file) => file.metadata.size as usize,
            Entry::Directory(dir) => {
                // List dir
                match self.host.scan_dir(dir.path.as_path()) {
                    Ok(files) => files
                        .iter()
                        .map(|x| self.get_total_transfer_size_local(x))
                        .sum(),
                    Err(err) => {
                        self.log(
                            LogLevel::Error,
                            format!("Could not list directory {}: {}", dir.path.display(), err),
                        );
                        0
                    }
                }
            }
        }
    }

    /// Get total size of transfer for remote host
    fn get_total_transfer_size_remote(&mut self, entry: &Entry) -> usize {
        match entry {
            Entry::File(file) => file.metadata.size as usize,
            Entry::Directory(dir) => {
                // List directory
                match self.client.list_dir(dir.path.as_path()) {
                    Ok(files) => files
                        .iter()
                        .map(|x| self.get_total_transfer_size_remote(x))
                        .sum(),
                    Err(err) => {
                        self.log(
                            LogLevel::Error,
                            format!("Could not list directory {}: {}", dir.path.display(), err),
                        );
                        0
                    }
                }
            }
        }
    }

    // -- file exist

    pub(crate) fn local_file_exists(&mut self, p: &Path) -> bool {
        self.host.file_exists(p)
    }

    pub(crate) fn remote_file_exists(&mut self, p: &Path) -> bool {
        self.client.stat(p).is_ok()
    }
}
