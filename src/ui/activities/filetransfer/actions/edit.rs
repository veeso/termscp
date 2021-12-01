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
use super::{FileTransferActivity, FsEntry, LogLevel, SelectedEntry, TransferPayload};
use crate::fs::FsFile;
// ext
use std::fs::OpenOptions;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

impl FileTransferActivity {
    pub(crate) fn action_edit_local_file(&mut self) {
        let entries: Vec<FsEntry> = match self.get_local_selected_entries() {
            SelectedEntry::One(entry) => vec![entry],
            SelectedEntry::Many(entries) => entries,
            SelectedEntry::None => vec![],
        };
        // Edit all entries
        for entry in entries.iter() {
            // Check if file
            if entry.is_file() {
                self.log(
                    LogLevel::Info,
                    format!("Opening file \"{}\"…", entry.get_abs_path().display()),
                );
                // Edit file
                if let Err(err) = self.edit_local_file(entry.get_abs_path().as_path()) {
                    self.log_and_alert(LogLevel::Error, err);
                }
            }
        }
        // Reload entries
        self.reload_local_dir();
    }

    pub(crate) fn action_edit_remote_file(&mut self) {
        let entries: Vec<FsEntry> = match self.get_remote_selected_entries() {
            SelectedEntry::One(entry) => vec![entry],
            SelectedEntry::Many(entries) => entries,
            SelectedEntry::None => vec![],
        };
        // Edit all entries
        for entry in entries.into_iter() {
            // Check if file
            if let FsEntry::File(file) = entry {
                self.log(
                    LogLevel::Info,
                    format!("Opening file \"{}\"…", file.abs_path.display()),
                );
                // Edit file
                if let Err(err) = self.edit_remote_file(file) {
                    self.log_and_alert(LogLevel::Error, err);
                }
            }
        }
        // Reload entries
        self.reload_remote_dir();
    }

    /// ### edit_local_file
    ///
    /// Edit a file on localhost
    fn edit_local_file(&mut self, path: &Path) -> Result<(), String> {
        // Read first 2048 bytes or less from file to check if it is textual
        match OpenOptions::new().read(true).open(path) {
            Ok(mut f) => {
                // Read
                let mut buff: [u8; 2048] = [0; 2048];
                match f.read(&mut buff) {
                    Ok(size) => {
                        if content_inspector::inspect(&buff[0..size]).is_binary() {
                            return Err("Could not open file in editor: file is binary".to_string());
                        }
                    }
                    Err(err) => {
                        return Err(format!("Could not read file: {}", err));
                    }
                }
            }
            Err(err) => {
                return Err(format!("Could not read file: {}", err));
            }
        }
        // Put input mode back to normal
        if let Err(err) = self.context_mut().terminal().disable_raw_mode() {
            error!("Failed to disable raw mode: {}", err);
        }
        // Leave alternate mode
        if let Err(err) = self.context_mut().terminal().leave_alternate_screen() {
            error!("Could not leave alternate screen: {}", err);
        }
        // Lock ports
        assert!(self.app.lock_ports().is_ok());
        // Open editor
        match edit::edit_file(path) {
            Ok(_) => self.log(
                LogLevel::Info,
                format!(
                    "Changes performed through editor saved to \"{}\"!",
                    path.display()
                ),
            ),
            Err(err) => return Err(format!("Could not open editor: {}", err)),
        }
        if let Some(ctx) = self.context.as_mut() {
            if let Err(err) = ctx.terminal().clear_screen() {
                error!("Could not clear screen screen: {}", err);
            }
            // Enter alternate mode
            if let Err(err) = ctx.terminal().enter_alternate_screen() {
                error!("Could not enter alternate screen: {}", err);
            }
            // Re-enable raw mode
            if let Err(err) = ctx.terminal().enable_raw_mode() {
                error!("Failed to enter raw mode: {}", err);
            }
            // Unlock ports
            assert!(self.app.unlock_ports().is_ok());
        }
        Ok(())
    }

    /// ### edit_remote_file
    ///
    /// Edit file on remote host
    fn edit_remote_file(&mut self, file: FsFile) -> Result<(), String> {
        // Create temp file
        let tmpfile: PathBuf = match self.download_file_as_temp(&file) {
            Ok(p) => p,
            Err(err) => return Err(err),
        };
        // Download file
        let file_name = file.name.clone();
        let file_path = file.abs_path.clone();
        if let Err(err) = self.filetransfer_recv(
            TransferPayload::File(file),
            tmpfile.as_path(),
            Some(file_name.clone()),
        ) {
            return Err(format!("Could not open file {}: {}", file_name, err));
        }
        // Get current file modification time
        let prev_mtime: SystemTime = match self.host.stat(tmpfile.as_path()) {
            Ok(e) => e.get_last_change_time(),
            Err(err) => {
                return Err(format!(
                    "Could not stat \"{}\": {}",
                    tmpfile.as_path().display(),
                    err
                ))
            }
        };
        // Edit file
        if let Err(err) = self.edit_local_file(tmpfile.as_path()) {
            return Err(err);
        }
        // Get local fs entry
        let tmpfile_entry: FsEntry = match self.host.stat(tmpfile.as_path()) {
            Ok(e) => e,
            Err(err) => {
                return Err(format!(
                    "Could not stat \"{}\": {}",
                    tmpfile.as_path().display(),
                    err
                ))
            }
        };
        // Check if file has changed
        match prev_mtime != tmpfile_entry.get_last_change_time() {
            true => {
                self.log(
                    LogLevel::Info,
                    format!(
                        "File \"{}\" has changed; writing changes to remote",
                        file_path.display()
                    ),
                );
                // Get local fs entry
                let tmpfile_entry: FsFile = match self.host.stat(tmpfile.as_path()) {
                    Ok(e) => e.unwrap_file(),
                    Err(err) => {
                        return Err(format!(
                            "Could not stat \"{}\": {}",
                            tmpfile.as_path().display(),
                            err
                        ))
                    }
                };
                // Send file
                let wrkdir = self.remote().wrkdir.clone();
                if let Err(err) = self.filetransfer_send(
                    TransferPayload::File(tmpfile_entry),
                    wrkdir.as_path(),
                    Some(file_name),
                ) {
                    return Err(format!(
                        "Could not write file {}: {}",
                        file_path.display(),
                        err
                    ));
                }
            }
            false => {
                self.log(
                    LogLevel::Info,
                    format!("File \"{}\" hasn't changed", file_path.display()),
                );
            }
        }
        Ok(())
    }
}
