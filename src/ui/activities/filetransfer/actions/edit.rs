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
use super::{FileTransferActivity, FsEntry, LogLevel, SelectedEntry};
use crate::fs::FsFile;
// ext
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
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
                    format!("Opening file \"{}\"...", entry.get_abs_path().display()),
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
        for entry in entries.iter() {
            // Check if file
            if let FsEntry::File(file) = entry {
                self.log(
                    LogLevel::Info,
                    format!("Opening file \"{}\"...", entry.get_abs_path().display()),
                );
                // Edit file
                if let Err(err) = self.edit_remote_file(&file) {
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
        if let Err(err) = disable_raw_mode() {
            error!("Failed to disable raw mode: {}", err);
        }
        // Leave alternate mode
        if let Some(ctx) = self.context.as_mut() {
            ctx.leave_alternate_screen();
        }
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
            // Clear screen
            ctx.clear_screen();
            // Enter alternate mode
            ctx.enter_alternate_screen();
        }
        // Re-enable raw mode
        let _ = enable_raw_mode();
        Ok(())
    }

    /// ### edit_remote_file
    ///
    /// Edit file on remote host
    fn edit_remote_file(&mut self, file: &FsFile) -> Result<(), String> {
        // Create temp file
        let tmpfile: tempfile::NamedTempFile = match tempfile::NamedTempFile::new() {
            Ok(f) => f,
            Err(err) => {
                return Err(format!("Could not create temporary file: {}", err));
            }
        };
        // Download file
        if let Err(err) = self.filetransfer_recv_one(file, tmpfile.path(), file.name.clone()) {
            return Err(format!("Could not open file {}: {}", file.name, err));
        }
        // Get current file modification time
        let prev_mtime: SystemTime = match self.host.stat(tmpfile.path()) {
            Ok(e) => e.get_last_change_time(),
            Err(err) => {
                return Err(format!(
                    "Could not stat \"{}\": {}",
                    tmpfile.path().display(),
                    err
                ))
            }
        };
        // Edit file
        if let Err(err) = self.edit_local_file(tmpfile.path()) {
            return Err(err);
        }
        // Get local fs entry
        let tmpfile_entry: FsEntry = match self.host.stat(tmpfile.path()) {
            Ok(e) => e,
            Err(err) => {
                return Err(format!(
                    "Could not stat \"{}\": {}",
                    tmpfile.path().display(),
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
                        file.abs_path.display()
                    ),
                );
                // Get local fs entry
                let tmpfile_entry: FsEntry = match self.host.stat(tmpfile.path()) {
                    Ok(e) => e,
                    Err(err) => {
                        return Err(format!(
                            "Could not stat \"{}\": {}",
                            tmpfile.path().display(),
                            err
                        ))
                    }
                };
                // Write file
                let tmpfile_entry: &FsFile = match &tmpfile_entry {
                    FsEntry::Directory(_) => panic!("tempfile is a directory for some reason"),
                    FsEntry::File(f) => f,
                };
                // Send file
                let wrkdir = self.remote().wrkdir.clone();
                if let Err(err) = self.filetransfer_send_one(
                    tmpfile_entry,
                    wrkdir.as_path(),
                    Some(file.name.clone()),
                ) {
                    return Err(format!(
                        "Could not write file {}: {}",
                        file.abs_path.display(),
                        err
                    ));
                }
            }
            false => {
                self.log(
                    LogLevel::Info,
                    format!("File \"{}\" hasn't changed", file.abs_path.display()),
                );
            }
        }
        Ok(())
    }
}
