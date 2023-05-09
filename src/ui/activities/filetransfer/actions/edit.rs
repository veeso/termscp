//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::fs::OpenOptions;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

// ext
use remotefs::File;

use super::{FileTransferActivity, LogLevel, SelectedFile, TransferPayload};

impl FileTransferActivity {
    pub(crate) fn action_edit_local_file(&mut self) {
        let entries: Vec<File> = match self.get_local_selected_entries() {
            SelectedFile::One(entry) => vec![entry],
            SelectedFile::Many(entries) => entries,
            SelectedFile::None => vec![],
        };
        // Edit all entries
        for entry in entries.iter() {
            // Check if file
            if entry.is_file() {
                self.log(
                    LogLevel::Info,
                    format!("Opening file \"{}\"…", entry.path().display()),
                );
                // Edit file
                if let Err(err) = self.edit_local_file(entry.path()) {
                    self.log_and_alert(LogLevel::Error, err);
                }
            }
        }
    }

    pub(crate) fn action_edit_remote_file(&mut self) {
        let entries: Vec<File> = match self.get_remote_selected_entries() {
            SelectedFile::One(entry) => vec![entry],
            SelectedFile::Many(entries) => entries,
            SelectedFile::None => vec![],
        };
        // Edit all entries
        for entry in entries.into_iter() {
            // Check if file
            if entry.is_file() {
                self.log(
                    LogLevel::Info,
                    format!("Opening file \"{}\"…", entry.path().display()),
                );
                // Edit file
                if let Err(err) = self.edit_remote_file(entry) {
                    self.log_and_alert(LogLevel::Error, err);
                }
            }
        }
    }

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
                        return Err(format!("Could not read file: {err}"));
                    }
                }
            }
            Err(err) => {
                return Err(format!("Could not read file: {err}"));
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
            Err(err) => return Err(format!("Could not open editor: {err}")),
        }
        if let Some(ctx) = self.context.as_mut() {
            // Enter alternate mode
            if let Err(err) = ctx.terminal().enter_alternate_screen() {
                error!("Could not enter alternate screen: {}", err);
            }
            // Re-enable raw mode
            if let Err(err) = ctx.terminal().enable_raw_mode() {
                error!("Failed to enter raw mode: {}", err);
            }
            // Clear screens
            if let Err(err) = ctx.terminal().clear_screen() {
                error!("Could not clear screen screen: {}", err);
            }
            // Unlock ports
            assert!(self.app.unlock_ports().is_ok());
        }
        Ok(())
    }

    /// Edit file on remote host
    fn edit_remote_file(&mut self, file: File) -> Result<(), String> {
        // Create temp file
        let tmpfile: PathBuf = match self.download_file_as_temp(&file) {
            Ok(p) => p,
            Err(err) => return Err(err),
        };
        // Download file
        let file_name = file.name();
        let file_path = file.path().to_path_buf();
        if let Err(err) = self.filetransfer_recv(
            TransferPayload::File(file),
            tmpfile.as_path(),
            Some(file_name.clone()),
        ) {
            return Err(format!("Could not open file {file_name}: {err}"));
        }
        // Get current file modification time
        let prev_mtime: SystemTime = match self.host.stat(tmpfile.as_path()) {
            Ok(e) => e.metadata().modified.unwrap_or(std::time::UNIX_EPOCH),
            Err(err) => {
                return Err(format!(
                    "Could not stat \"{}\": {}",
                    tmpfile.as_path().display(),
                    err
                ))
            }
        };
        // Edit file
        self.edit_local_file(tmpfile.as_path())?;
        // Get local fs entry
        let tmpfile_entry: File = match self.host.stat(tmpfile.as_path()) {
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
        match prev_mtime
            != tmpfile_entry
                .metadata()
                .modified
                .unwrap_or(std::time::UNIX_EPOCH)
        {
            true => {
                self.log(
                    LogLevel::Info,
                    format!(
                        "File \"{}\" has changed; writing changes to remote",
                        file_path.display()
                    ),
                );
                // Get local fs entry
                let tmpfile_entry = match self.host.stat(tmpfile.as_path()) {
                    Ok(e) => e,
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
