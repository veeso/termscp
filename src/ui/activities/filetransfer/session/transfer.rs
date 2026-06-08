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

/// A single planned transfer action produced by the pre-scan.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::ui::activities::filetransfer) enum WorkItem {
    /// Create a directory at the destination path. No progress weight.
    /// `src` carries the source `File` so its metadata can be mirrored onto the
    /// created directory (used by downloads to restore mode/timestamps).
    Mkdir { src: File, dst: PathBuf },
    /// Copy one file from `src` to `dst`. Weighted as one file.
    CopyFile { src: File, dst: PathBuf },
}

/// Recursively flatten `entries` into an ordered work-queue.
///
/// `list_dir` lists the children of a directory `File`. `on_progress` is called
/// after each item is appended with `(dirs_seen, files_seen)` and returns `false`
/// to abort the walk early.
///
/// Pure over its closures so it can be unit-tested without a real filesystem.
///
/// The production scan lives in [`FileTransferActivity::scan_worklist`], which
/// inlines the same stack-walk to avoid a double `&mut self` borrow; this pure
/// version exists to validate the flattening algorithm in isolation.
#[cfg(test)]
fn flatten_worklist<L, P>(
    entries: &[(File, PathBuf)],
    list_dir: &mut L,
    on_progress: &mut P,
) -> Result<Vec<WorkItem>, String>
where
    L: FnMut(&Path) -> Result<Vec<File>, String>,
    P: FnMut(usize, usize) -> bool,
{
    let mut out: Vec<WorkItem> = Vec::new();
    let mut dirs = 0usize;
    let mut files = 0usize;
    let mut stack: Vec<(File, PathBuf)> = entries.iter().rev().cloned().collect();
    while let Some((entry, dst)) = stack.pop() {
        if entry.is_dir() {
            out.push(WorkItem::Mkdir {
                src: entry.clone(),
                dst: dst.clone(),
            });
            dirs += 1;
            if !on_progress(dirs, files) {
                return Err("aborted".to_string());
            }
            let children = list_dir(entry.path())?;
            for child in children.into_iter().rev() {
                let mut child_dst = dst.clone();
                child_dst.push(child.name());
                stack.push((child, child_dst));
            }
        } else {
            out.push(WorkItem::CopyFile {
                src: entry.clone(),
                dst: dst.clone(),
            });
            files += 1;
            if !on_progress(dirs, files) {
                return Err("aborted".to_string());
            }
        }
    }
    Ok(out)
}

/// Returns whether `entries` contains at least one directory, in which case a
/// pre-scan is required to build the work-queue.
fn selection_has_dirs(entries: &[(File, PathBuf)]) -> bool {
    entries.iter().any(|(f, _)| f.is_dir())
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
        // Single file = 1 entry
        self.transfer.progress.init(1);
        // Mount progress bar
        self.mount_progress_bar(format!("Uploading {}…", file.path.display()));
        self.view();
        // Get remote path
        let file_name: String = file.name();
        let mut remote_path: PathBuf = PathBuf::from(curr_remote_path);
        let remote_file_name: PathBuf = match dst_name {
            Some(s) => PathBuf::from(s.as_str()),
            None => PathBuf::from(file_name.as_str()),
        };
        remote_path.push(remote_file_name);
        // Send (counting is owned by `filetransfer_send_one` / `_with_stream`)
        let result = self.filetransfer_send_one(file, remote_path.as_path(), file_name);
        // Umount progress bar
        self.umount_progress_bar();
        // Return result
        result.map_err(|x| x.to_string())
    }

    /// Send a `TransferPayload` of type `Any` by delegating to the work-queue.
    fn filetransfer_send_any(
        &mut self,
        entry: &File,
        curr_remote_path: &Path,
        dst_name: Option<String>,
    ) -> Result<(), String> {
        let mut dst = PathBuf::from(curr_remote_path);
        dst.push(dst_name.unwrap_or_else(|| entry.name()));
        self.filetransfer_send_transfer_queue(&[(entry.clone(), dst)])
    }

    /// Build the upload work-queue by scanning the local selection.
    fn build_send_worklist(
        &mut self,
        entries: &[(File, PathBuf)],
    ) -> Result<Vec<WorkItem>, String> {
        self.scan_worklist(entries, false)
    }

    /// Build the download work-queue by scanning the remote selection.
    fn build_recv_worklist(
        &mut self,
        entries: &[(File, PathBuf)],
    ) -> Result<Vec<WorkItem>, String> {
        self.scan_worklist(entries, true)
    }

    /// Shared scan walk. `remote_side` selects which pane lists directories
    /// (remote for downloads, local for uploads).
    ///
    /// The walk is abortable via [`crate::ui::activities::filetransfer::lib::TransferStates::aborted`]
    /// and periodically redraws a "Scanning…" popup to keep the UI responsive.
    fn scan_worklist(
        &mut self,
        entries: &[(File, PathBuf)],
        remote_side: bool,
    ) -> Result<Vec<WorkItem>, String> {
        let mut out: Vec<WorkItem> = Vec::new();
        let mut dirs = 0usize;
        let mut files = 0usize;
        let mut last_redraw = Instant::now();
        let mut stack: Vec<(File, PathBuf)> = entries.iter().rev().cloned().collect();
        while let Some((entry, dst)) = stack.pop() {
            if self.transfer.aborted() {
                return Err("Scan aborted".to_string());
            }
            if entry.is_dir() {
                out.push(WorkItem::Mkdir {
                    src: entry.clone(),
                    dst: dst.clone(),
                });
                dirs += 1;
                let listed = if remote_side {
                    self.browser.remote_pane_mut().fs.list_dir(entry.path())
                } else {
                    self.browser.local_pane_mut().fs.list_dir(entry.path())
                };
                match listed {
                    Ok(children) => {
                        for child in children.into_iter().rev() {
                            let mut child_dst = dst.clone();
                            child_dst.push(child.name());
                            stack.push((child, child_dst));
                        }
                    }
                    Err(err) => {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not scan \"{}\": {err}", entry.path().display()),
                        );
                        return Err(err.to_string());
                    }
                }
            } else {
                out.push(WorkItem::CopyFile {
                    src: entry.clone(),
                    dst: dst.clone(),
                });
                files += 1;
            }
            // Redraw at most every 100ms to keep the UI responsive while scanning.
            if last_redraw.elapsed().as_millis() >= 100 {
                self.tick();
                self.update_scan_progress(dirs, files);
                self.view();
                last_redraw = Instant::now();
            }
        }
        Ok(out)
    }

    /// Send transfer queue entries to remote.
    ///
    /// When the selection contains directories the tree is scanned first to build
    /// an ordered work-queue, giving an exact file count up front; otherwise the
    /// flat file selection is mapped directly without scanning.
    fn filetransfer_send_transfer_queue(
        &mut self,
        entries: &[(File, PathBuf)],
    ) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Zero the counters so that during the pre-scan `files_total == 0`, making
        // `is_single_file()` deterministically true. This forces the layout to render
        // the single (partial) bar during the "Preparing/Scanning" phase, where the
        // scan text is written. The real `init(total_files)` runs after the scan.
        self.transfer.progress.init(0);
        // Build the work-queue: scan the tree only when directories are involved.
        let worklist = if selection_has_dirs(entries) {
            self.mount_progress_bar(String::from("Preparing transfer…"));
            self.view();
            match self.build_send_worklist(entries) {
                Ok(worklist) => worklist,
                Err(err) => {
                    self.umount_progress_bar();
                    return Err(err);
                }
            }
        } else {
            entries
                .iter()
                .map(|(src, dst)| WorkItem::CopyFile {
                    src: src.clone(),
                    dst: dst.clone(),
                })
                .collect()
        };
        // Total = number of files to copy
        let total_files = worklist
            .iter()
            .filter(|item| matches!(item, WorkItem::CopyFile { .. }))
            .count();
        self.transfer.progress.init(total_files);
        // Mount progress bar
        self.mount_progress_bar(format!("Uploading {total_files} files…"));
        self.view();
        // Iterate the work-queue
        let mut result = Ok(());
        for item in &worklist {
            if self.transfer.aborted() {
                self.log_and_alert(LogLevel::Warn, "Upload aborted!".to_string());
                break;
            }
            match item {
                WorkItem::Mkdir { src: _, dst } => {
                    match self
                        .browser
                        .remote_pane_mut()
                        .fs
                        .mkdir_ex(dst.as_path(), true)
                    {
                        Ok(_) => {
                            self.log(
                                LogLevel::Info,
                                format!("Created directory \"{}\"", dst.display()),
                            );
                            self.reload_remote_dir();
                        }
                        Err(err) => {
                            self.log_and_alert(
                                LogLevel::Error,
                                format!("Failed to create directory \"{}\": {err}", dst.display()),
                            );
                            result = Err(err.to_string());
                            break;
                        }
                    }
                }
                WorkItem::CopyFile { src, dst } => {
                    if let Err(err) = self.filetransfer_send_one(src, dst.as_path(), src.name()) {
                        // If transfer was abrupted or there was an IO error on remote, remove file
                        if matches!(
                            err,
                            TransferErrorReason::Abrupted | TransferErrorReason::RemoteIoError(_)
                        ) {
                            // Stat file on remote and remove it if exists
                            match self.browser.remote_pane_mut().fs.stat(dst.as_path()) {
                                Err(err) => self.log(
                                    LogLevel::Error,
                                    format!(
                                        "Could not remove created file {}: {}",
                                        dst.display(),
                                        err
                                    ),
                                ),
                                Ok(entry) => {
                                    if let Err(err) =
                                        self.browser.remote_pane_mut().fs.remove(&entry)
                                    {
                                        self.log(
                                            LogLevel::Error,
                                            format!(
                                                "Could not remove created file {}: {}",
                                                dst.display(),
                                                err
                                            ),
                                        );
                                    }
                                }
                            }
                        }
                        result = Err(err.to_string());
                        break;
                    }
                    // Counting is owned by `filetransfer_send_one` / `_with_stream`.
                    self.reload_remote_dir();
                    // Redraw on the file boundary so the full bar's (N/total) counter
                    // advances after every completed file, including small files that
                    // finish within a single in-loop redraw interval.
                    self.update_progress_bar(format!("Uploaded \"{}\"", src.name()));
                    self.view();
                }
            }
        }
        // Umount progress bar
        self.umount_progress_bar();
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
            self.transfer.progress.skip_file();
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
        self.transfer.progress.start_file(file_size);
        let file_started = Instant::now();

        // Write remote file
        let mut total_bytes_written: usize = 0;
        let mut last_redraw: Instant = Instant::now();
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
            self.transfer.progress.add_bytes(delta);
            // Redraw at most every 100ms to keep UI responsive for large files
            if last_redraw.elapsed().as_millis() >= 100 {
                self.update_progress_bar(format!("Uploading \"{file_name}\"…"));
                self.view();
                last_redraw = Instant::now();
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
                fmt_millis(file_started.elapsed()),
                ByteSize(self.transfer.progress.calc_bytes_per_second()),
            ),
        );
        // Count this file as completed (owns the per-file count for the success path).
        self.transfer.progress.finish_file();
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
    /// If entry is a directory, this applies to directory only.
    fn filetransfer_recv_any(
        &mut self,
        entry: &File,
        host_path: &Path,
        dst_name: Option<String>,
    ) -> Result<(), String> {
        let mut dst = PathBuf::from(host_path);
        dst.push(dst_name.unwrap_or_else(|| entry.name()));
        self.filetransfer_recv_transfer_queue(&[(entry.clone(), dst)])
    }

    /// Receive a single file from remote.
    fn filetransfer_recv_file(
        &mut self,
        entry: &File,
        host_bridge_path: &Path,
    ) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Single file = 1 entry
        self.transfer.progress.init(1);
        // Mount progress bar
        self.mount_progress_bar(format!("Downloading {}…", entry.path.display()));
        self.view();
        // Receive (counting is owned by `filetransfer_recv_one` / `_with_stream`)
        let result = self.filetransfer_recv_one(host_bridge_path, entry, entry.name());
        // Umount progress bar
        self.umount_progress_bar();
        // Return result
        result.map_err(|x| x.to_string())
    }

    /// Receive transfer queue from remote.
    ///
    /// When the selection contains directories the tree is scanned first to build
    /// an ordered work-queue, giving an exact file count up front; otherwise the
    /// flat file selection is mapped directly without scanning.
    fn filetransfer_recv_transfer_queue(
        &mut self,
        entries: &[(File, PathBuf)],
    ) -> Result<(), String> {
        // Reset states
        self.transfer.reset();
        // Zero the counters so that during the pre-scan `files_total == 0`, making
        // `is_single_file()` deterministically true. This forces the layout to render
        // the single (partial) bar during the "Preparing/Scanning" phase, where the
        // scan text is written. The real `init(total_files)` runs after the scan.
        self.transfer.progress.init(0);
        // Build the work-queue: scan the tree only when directories are involved.
        let worklist = if selection_has_dirs(entries) {
            self.mount_progress_bar(String::from("Preparing transfer…"));
            self.view();
            match self.build_recv_worklist(entries) {
                Ok(worklist) => worklist,
                Err(err) => {
                    self.umount_progress_bar();
                    return Err(err);
                }
            }
        } else {
            entries
                .iter()
                .map(|(src, dst)| WorkItem::CopyFile {
                    src: src.clone(),
                    dst: dst.clone(),
                })
                .collect()
        };
        // Total = number of files to copy
        let total_files = worklist
            .iter()
            .filter(|item| matches!(item, WorkItem::CopyFile { .. }))
            .count();
        self.transfer.progress.init(total_files);
        // Mount progress bar
        self.mount_progress_bar(format!("Downloading {total_files} files…"));
        self.view();
        // Iterate the work-queue
        let mut result = Ok(());
        for item in &worklist {
            if self.transfer.aborted() {
                self.log_and_alert(LogLevel::Warn, "Download aborted!".to_string());
                break;
            }
            match item {
                WorkItem::Mkdir { src, dst } => {
                    match self
                        .browser
                        .local_pane_mut()
                        .fs
                        .mkdir_ex(dst.as_path(), true)
                    {
                        Ok(_) => {
                            // Apply remote dir mode/timestamps to the created local dir
                            if let Err(err) = self
                                .browser
                                .local_pane_mut()
                                .fs
                                .setstat(dst.as_path(), src.metadata())
                            {
                                self.log(
                                    LogLevel::Error,
                                    format!(
                                        "Could not set stat to directory {:?} to \"{}\": {}",
                                        src.metadata(),
                                        dst.display(),
                                        err
                                    ),
                                );
                            }
                            self.log(
                                LogLevel::Info,
                                format!("Created directory \"{}\"", dst.display()),
                            );
                            self.reload_host_bridge_dir();
                        }
                        Err(err) => {
                            self.log_and_alert(
                                LogLevel::Error,
                                format!("Failed to create directory \"{}\": {err}", dst.display()),
                            );
                            result = Err(err.to_string());
                            break;
                        }
                    }
                }
                WorkItem::CopyFile { src, dst } => {
                    if let Err(err) = self.filetransfer_recv_one(dst.as_path(), src, src.name()) {
                        // If transfer was abrupted or there was an IO error on host, remove file
                        if matches!(
                            err,
                            TransferErrorReason::Abrupted | TransferErrorReason::HostIoError(_)
                        ) {
                            // Stat file and remove it if exists
                            match self.browser.local_pane_mut().fs.stat(dst.as_path()) {
                                Err(err) => self.log(
                                    LogLevel::Error,
                                    format!(
                                        "Could not remove created file {}: {}",
                                        dst.display(),
                                        err
                                    ),
                                ),
                                Ok(entry) => {
                                    if let Err(err) =
                                        self.browser.local_pane_mut().fs.remove(&entry)
                                    {
                                        self.log(
                                            LogLevel::Error,
                                            format!(
                                                "Could not remove created file {}: {}",
                                                dst.display(),
                                                err
                                            ),
                                        );
                                    }
                                }
                            }
                        }
                        result = Err(err.to_string());
                        break;
                    }
                    // Counting is owned by `filetransfer_recv_one` / `_with_stream`.
                    self.reload_host_bridge_dir();
                    // Redraw on the file boundary so the full bar's (N/total) counter
                    // advances after every completed file, including small files that
                    // finish within a single in-loop redraw interval.
                    self.update_progress_bar(format!("Downloaded \"{}\"", src.name()));
                    self.view();
                }
            }
        }
        // Umount progress bar
        self.umount_progress_bar();
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
            self.transfer.progress.skip_file();
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
        self.transfer
            .progress
            .start_file(remote.metadata.size as usize);
        let file_started = Instant::now();
        // Write host_bridge file
        let mut last_redraw: Instant = Instant::now();
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
            self.transfer.progress.add_bytes(delta);
            // Redraw at most every 100ms to keep UI responsive for large files
            if last_redraw.elapsed().as_millis() >= 100 {
                self.update_progress_bar(format!("Downloading \"{file_name}\""));
                self.view();
                last_redraw = Instant::now();
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
                fmt_millis(file_started.elapsed()),
                ByteSize(self.transfer.progress.calc_bytes_per_second()),
            ),
        );

        // Count this file as completed (owns the per-file count for the success path).
        self.transfer.progress.finish_file();
        Ok(())
    }
}

#[cfg(test)]
mod worklist_test {
    use std::time::SystemTime;

    use pretty_assertions::assert_eq;
    use remotefs::fs::{FileType, Metadata, UnixPex};

    use super::*;

    fn make_entry(path: &str, is_dir: bool) -> File {
        // Use a fixed timestamp so two entries built for the same path compare
        // equal (`File` derives `PartialEq` over its full metadata, timestamps
        // included), allowing direct equality assertions against the worklist.
        let t = SystemTime::UNIX_EPOCH;
        let metadata = Metadata {
            accessed: Some(t),
            created: Some(t),
            modified: Some(t),
            file_type: if is_dir {
                FileType::Directory
            } else {
                FileType::File
            },
            symlink: None,
            gid: Some(0),
            uid: Some(0),
            mode: Some(UnixPex::from(if is_dir { 0o755 } else { 0o644 })),
            size: 64,
        };
        File {
            path: PathBuf::from(path),
            metadata,
        }
    }

    #[test]
    fn should_flatten_nested_tree_in_order() {
        // Tree: /a -> [f1, /b -> [f2]]
        let dir_a = make_entry("/src/a", true);
        let file_f1 = make_entry("/src/a/f1", false);
        let dir_b = make_entry("/src/a/b", true);
        let file_f2 = make_entry("/src/a/b/f2", false);

        let entries = vec![(dir_a, PathBuf::from("/dst/a"))];

        let mut list_dir = |path: &Path| -> Result<Vec<File>, String> {
            match path.to_string_lossy().as_ref() {
                "/src/a" => Ok(vec![file_f1.clone(), dir_b.clone()]),
                "/src/a/b" => Ok(vec![file_f2.clone()]),
                other => Err(format!("unexpected list_dir on {other}")),
            }
        };
        let mut on_progress = |_dirs: usize, _files: usize| true;

        let out = flatten_worklist(&entries, &mut list_dir, &mut on_progress).unwrap();

        assert_eq!(
            out,
            vec![
                WorkItem::Mkdir {
                    src: make_entry("/src/a", true),
                    dst: PathBuf::from("/dst/a"),
                },
                WorkItem::CopyFile {
                    src: make_entry("/src/a/f1", false),
                    dst: PathBuf::from("/dst/a/f1"),
                },
                WorkItem::Mkdir {
                    src: make_entry("/src/a/b", true),
                    dst: PathBuf::from("/dst/a/b"),
                },
                WorkItem::CopyFile {
                    src: make_entry("/src/a/b/f2", false),
                    dst: PathBuf::from("/dst/a/b/f2"),
                },
            ]
        );
        // Exactly two CopyFile items
        let copy_files = out
            .iter()
            .filter(|item| matches!(item, WorkItem::CopyFile { .. }))
            .count();
        assert_eq!(copy_files, 2);
    }

    #[test]
    fn should_flatten_flat_selection_without_listing_dirs() {
        let entries = vec![
            (make_entry("/src/f1", false), PathBuf::from("/dst/f1")),
            (make_entry("/src/f2", false), PathBuf::from("/dst/f2")),
        ];

        // `list_dir` must never be called for a flat file selection.
        let mut list_dir =
            |_path: &Path| -> Result<Vec<File>, String> { panic!("list_dir must not be called") };
        let mut on_progress = |_dirs: usize, _files: usize| true;

        let out = flatten_worklist(&entries, &mut list_dir, &mut on_progress).unwrap();

        assert_eq!(
            out,
            vec![
                WorkItem::CopyFile {
                    src: make_entry("/src/f1", false),
                    dst: PathBuf::from("/dst/f1"),
                },
                WorkItem::CopyFile {
                    src: make_entry("/src/f2", false),
                    dst: PathBuf::from("/dst/f2"),
                },
            ]
        );
        assert!(
            out.iter()
                .all(|item| matches!(item, WorkItem::CopyFile { .. }))
        );
    }

    #[test]
    fn should_abort_when_on_progress_returns_false() {
        let entries = vec![(make_entry("/src/f1", false), PathBuf::from("/dst/f1"))];

        let mut list_dir = |_path: &Path| -> Result<Vec<File>, String> { Ok(vec![]) };
        let mut on_progress = |_dirs: usize, _files: usize| false;

        let result = flatten_worklist(&entries, &mut list_dir, &mut on_progress);
        assert!(result.is_err());
    }
}
