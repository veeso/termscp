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
use super::{FileExplorerTab, FileTransferActivity, FsEntry, LogLevel};
use tuirealm::{Payload, Value};
// externals
use std::path::PathBuf;

impl FileTransferActivity {
    /// ### action_change_local_dir
    ///
    /// Change local directory reading value from input
    pub(super) fn action_change_local_dir(&mut self, input: String) {
        let dir_path: PathBuf = PathBuf::from(input.as_str());
        let abs_dir_path: PathBuf = match dir_path.is_relative() {
            true => {
                let mut d: PathBuf = self.local.wrkdir.clone();
                d.push(dir_path);
                d
            }
            false => dir_path,
        };
        self.local_changedir(abs_dir_path.as_path(), true);
    }

    /// ### action_change_remote_dir
    ///
    /// Change remote directory reading value from input
    pub(super) fn action_change_remote_dir(&mut self, input: String) {
        let dir_path: PathBuf = PathBuf::from(input.as_str());
        let abs_dir_path: PathBuf = match dir_path.is_relative() {
            true => {
                let mut wrkdir: PathBuf = self.remote.wrkdir.clone();
                wrkdir.push(dir_path);
                wrkdir
            }
            false => dir_path,
        };
        self.remote_changedir(abs_dir_path.as_path(), true);
    }

    /// ### action_local_copy
    ///
    /// Copy file on local
    pub(super) fn action_local_copy(&mut self, input: String) {
        if let Some(idx) = self.get_local_file_idx() {
            let dest_path: PathBuf = PathBuf::from(input);
            let entry: FsEntry = self.local.get(idx).unwrap().clone();
            if let Some(ctx) = self.context.as_mut() {
                match ctx.local.copy(&entry, dest_path.as_path()) {
                    Ok(_) => {
                        self.log(
                            LogLevel::Info,
                            format!(
                                "Copied \"{}\" to \"{}\"",
                                entry.get_abs_path().display(),
                                dest_path.display()
                            )
                            .as_str(),
                        );
                        // Reload entries
                        let wrkdir: PathBuf = self.local.wrkdir.clone();
                        self.local_scan(wrkdir.as_path());
                    }
                    Err(err) => self.log_and_alert(
                        LogLevel::Error,
                        format!(
                            "Could not copy \"{}\" to \"{}\": {}",
                            entry.get_abs_path().display(),
                            dest_path.display(),
                            err
                        ),
                    ),
                }
            }
        }
    }

    /// ### action_remote_copy
    ///
    /// Copy file on remote
    pub(super) fn action_remote_copy(&mut self, input: String) {
        if let Some(idx) = self.get_remote_file_idx() {
            let dest_path: PathBuf = PathBuf::from(input);
            let entry: FsEntry = self.remote.get(idx).unwrap().clone();
            match self.client.as_mut().copy(&entry, dest_path.as_path()) {
                Ok(_) => {
                    self.log(
                        LogLevel::Info,
                        format!(
                            "Copied \"{}\" to \"{}\"",
                            entry.get_abs_path().display(),
                            dest_path.display()
                        )
                        .as_str(),
                    );
                    self.reload_remote_dir();
                }
                Err(err) => self.log_and_alert(
                    LogLevel::Error,
                    format!(
                        "Could not copy \"{}\" to \"{}\": {}",
                        entry.get_abs_path().display(),
                        dest_path.display(),
                        err
                    ),
                ),
            }
        }
    }

    pub(super) fn action_local_mkdir(&mut self, input: String) {
        match self
            .context
            .as_mut()
            .unwrap()
            .local
            .mkdir(PathBuf::from(input.as_str()).as_path())
        {
            Ok(_) => {
                // Reload files
                self.log(
                    LogLevel::Info,
                    format!("Created directory \"{}\"", input).as_ref(),
                );
                let wrkdir: PathBuf = self.local.wrkdir.clone();
                self.local_scan(wrkdir.as_path());
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not create directory \"{}\": {}", input, err),
                );
            }
        }
    }
    pub(super) fn action_remote_mkdir(&mut self, input: String) {
        match self
            .client
            .as_mut()
            .mkdir(PathBuf::from(input.as_str()).as_path())
        {
            Ok(_) => {
                // Reload files
                self.log(
                    LogLevel::Info,
                    format!("Created directory \"{}\"", input).as_ref(),
                );
                self.reload_remote_dir();
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not create directory \"{}\": {}", input, err),
                );
            }
        }
    }

    pub(super) fn action_local_rename(&mut self, input: String) {
        let entry: Option<FsEntry> = self.get_local_file_entry().cloned();
        if let Some(entry) = entry {
            let mut dst_path: PathBuf = PathBuf::from(input);
            // Check if path is relative
            if dst_path.as_path().is_relative() {
                let mut wrkdir: PathBuf = self.local.wrkdir.clone();
                wrkdir.push(dst_path);
                dst_path = wrkdir;
            }
            let full_path: PathBuf = entry.get_abs_path();
            // Rename file or directory and report status as popup
            match self
                .context
                .as_mut()
                .unwrap()
                .local
                .rename(&entry, dst_path.as_path())
            {
                Ok(_) => {
                    // Reload files
                    let path: PathBuf = self.local.wrkdir.clone();
                    self.local_scan(path.as_path());
                    // Log
                    self.log(
                        LogLevel::Info,
                        format!(
                            "Renamed file \"{}\" to \"{}\"",
                            full_path.display(),
                            dst_path.display()
                        )
                        .as_ref(),
                    );
                }
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!("Could not rename file \"{}\": {}", full_path.display(), err),
                    );
                }
            }
        }
    }

    pub(super) fn action_remote_rename(&mut self, input: String) {
        if let Some(idx) = self.get_remote_file_idx() {
            if let Some(entry) = self.remote.get(idx) {
                let dst_path: PathBuf = PathBuf::from(input);
                let full_path: PathBuf = entry.get_abs_path();
                // Rename file or directory and report status as popup
                match self.client.as_mut().rename(entry, dst_path.as_path()) {
                    Ok(_) => {
                        // Reload files
                        let path: PathBuf = self.remote.wrkdir.clone();
                        self.remote_scan(path.as_path());
                        // Log
                        self.log(
                            LogLevel::Info,
                            format!(
                                "Renamed file \"{}\" to \"{}\"",
                                full_path.display(),
                                dst_path.display()
                            )
                            .as_ref(),
                        );
                    }
                    Err(err) => {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not rename file \"{}\": {}", full_path.display(), err),
                        );
                    }
                }
            }
        }
    }

    pub(super) fn action_local_delete(&mut self) {
        let entry: Option<FsEntry> = self.get_local_file_entry().cloned();
        if let Some(entry) = entry {
            let full_path: PathBuf = entry.get_abs_path();
            // Delete file or directory and report status as popup
            match self.context.as_mut().unwrap().local.remove(&entry) {
                Ok(_) => {
                    // Reload files
                    let p: PathBuf = self.local.wrkdir.clone();
                    self.local_scan(p.as_path());
                    // Log
                    self.log(
                        LogLevel::Info,
                        format!("Removed file \"{}\"", full_path.display()).as_ref(),
                    );
                }
                Err(err) => {
                    self.log_and_alert(
                        LogLevel::Error,
                        format!("Could not delete file \"{}\": {}", full_path.display(), err),
                    );
                }
            }
        }
    }

    pub(super) fn action_remote_delete(&mut self) {
        if let Some(idx) = self.get_remote_file_idx() {
            // Check if file entry exists
            if let Some(entry) = self.remote.get(idx) {
                let full_path: PathBuf = entry.get_abs_path();
                // Delete file
                match self.client.remove(entry) {
                    Ok(_) => {
                        self.reload_remote_dir();
                        self.log(
                            LogLevel::Info,
                            format!("Removed file \"{}\"", full_path.display()).as_ref(),
                        );
                    }
                    Err(err) => {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not delete file \"{}\": {}", full_path.display(), err),
                        );
                    }
                }
            }
        }
    }

    pub(super) fn action_local_saveas(&mut self, input: String) {
        if let Some(idx) = self.get_local_file_idx() {
            // Get pwd
            let wrkdir: PathBuf = self.remote.wrkdir.clone();
            if self.local.get(idx).is_some() {
                let file: FsEntry = self.local.get(idx).unwrap().clone();
                // Call upload; pass realfile, keep link name
                self.filetransfer_send(&file.get_realfile(), wrkdir.as_path(), Some(input));
            }
        }
    }

    pub(super) fn action_remote_saveas(&mut self, input: String) {
        if let Some(idx) = self.get_remote_file_idx() {
            // Get pwd
            let wrkdir: PathBuf = self.local.wrkdir.clone();
            if self.remote.get(idx).is_some() {
                let file: FsEntry = self.remote.get(idx).unwrap().clone();
                // Call upload; pass realfile, keep link name
                self.filetransfer_recv(&file.get_realfile(), wrkdir.as_path(), Some(input));
            }
        }
    }

    pub(super) fn action_local_newfile(&mut self, input: String) {
        // Check if file exists
        let mut file_exists: bool = false;
        for file in self.local.iter_files_all() {
            if input == file.get_name() {
                file_exists = true;
            }
        }
        if file_exists {
            self.log_and_alert(
                LogLevel::Warn,
                format!("File \"{}\" already exists", input,),
            );
            return;
        }
        // Create file
        let file_path: PathBuf = PathBuf::from(input.as_str());
        if let Some(ctx) = self.context.as_mut() {
            if let Err(err) = ctx.local.open_file_write(file_path.as_path()) {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not create file \"{}\": {}", file_path.display(), err),
                );
            } else {
                self.log(
                    LogLevel::Info,
                    format!("Created file \"{}\"", file_path.display()).as_str(),
                );
            }
            // Reload files
            let path: PathBuf = self.local.wrkdir.clone();
            self.local_scan(path.as_path());
        }
    }

    pub(super) fn action_remote_newfile(&mut self, input: String) {
        // Check if file exists
        let mut file_exists: bool = false;
        for file in self.remote.iter_files_all() {
            if input == file.get_name() {
                file_exists = true;
            }
        }
        if file_exists {
            self.log_and_alert(
                LogLevel::Warn,
                format!("File \"{}\" already exists", input,),
            );
            return;
        }
        // Get path on remote
        let file_path: PathBuf = PathBuf::from(input.as_str());
        // Create file (on local)
        match tempfile::NamedTempFile::new() {
            Err(err) => self.log_and_alert(
                LogLevel::Error,
                format!("Could not create tempfile: {}", err),
            ),
            Ok(tfile) => {
                // Stat tempfile
                if let Some(ctx) = self.context.as_mut() {
                    let local_file: FsEntry = match ctx.local.stat(tfile.path()) {
                        Err(err) => {
                            self.log_and_alert(
                                LogLevel::Error,
                                format!("Could not stat tempfile: {}", err),
                            );
                            return;
                        }
                        Ok(f) => f,
                    };
                    if let FsEntry::File(local_file) = local_file {
                        // Create file
                        match self.client.send_file(&local_file, file_path.as_path()) {
                            Err(err) => self.log_and_alert(
                                LogLevel::Error,
                                format!(
                                    "Could not create file \"{}\": {}",
                                    file_path.display(),
                                    err
                                ),
                            ),
                            Ok(writer) => {
                                // Finalize write
                                if let Err(err) = self.client.on_sent(writer) {
                                    self.log_and_alert(
                                        LogLevel::Warn,
                                        format!("Could not finalize file: {}", err),
                                    );
                                } else {
                                    self.log(
                                        LogLevel::Info,
                                        format!("Created file \"{}\"", file_path.display())
                                            .as_str(),
                                    );
                                }
                                // Reload files
                                let path: PathBuf = self.remote.wrkdir.clone();
                                self.remote_scan(path.as_path());
                            }
                        }
                    }
                }
            }
        }
    }

    pub(super) fn action_local_exec(&mut self, input: String) {
        match self.context.as_mut().unwrap().local.exec(input.as_str()) {
            Ok(output) => {
                // Reload files
                self.log(
                    LogLevel::Info,
                    format!("\"{}\": {}", input, output).as_ref(),
                );
                let wrkdir: PathBuf = self.local.wrkdir.clone();
                self.local_scan(wrkdir.as_path());
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not execute command \"{}\": {}", input, err),
                );
            }
        }
    }

    pub(super) fn action_remote_exec(&mut self, input: String) {
        match self.client.as_mut().exec(input.as_str()) {
            Ok(output) => {
                // Reload files
                self.log(
                    LogLevel::Info,
                    format!("\"{}\": {}", input, output).as_ref(),
                );
                self.reload_remote_dir();
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not execute command \"{}\": {}", input, err),
                );
            }
        }
    }

    pub(super) fn action_local_find(&mut self, input: String) -> Result<Vec<FsEntry>, String> {
        match self.context.as_mut().unwrap().local.find(input.as_str()) {
            Ok(entries) => Ok(entries),
            Err(err) => Err(format!("Could not search for files: {}", err)),
        }
    }

    pub(super) fn action_remote_find(&mut self, input: String) -> Result<Vec<FsEntry>, String> {
        match self.client.as_mut().find(input.as_str()) {
            Ok(entries) => Ok(entries),
            Err(err) => Err(format!("Could not search for files: {}", err)),
        }
    }

    pub(super) fn action_find_changedir(&mut self, idx: usize) {
        // Match entry
        if let Some(entry) = self.found.as_ref().unwrap().get(idx) {
            // Get path: if a directory, use directory path; if it is a File, get parent path
            let path: PathBuf = match entry {
                FsEntry::Directory(dir) => dir.abs_path.clone(),
                FsEntry::File(file) => match file.abs_path.parent() {
                    None => PathBuf::from("."),
                    Some(p) => p.to_path_buf(),
                },
            };
            // Change directory
            match self.tab {
                FileExplorerTab::FindLocal | FileExplorerTab::Local => {
                    self.local_changedir(path.as_path(), true)
                }
                FileExplorerTab::FindRemote | FileExplorerTab::Remote => {
                    self.remote_changedir(path.as_path(), true)
                }
            }
        }
    }

    pub(super) fn action_find_transfer(&mut self, idx: usize, name: Option<String>) {
        let entry: Option<FsEntry> = self.found.as_ref().unwrap().get(idx).cloned();
        if let Some(entry) = entry {
            // Download file
            match self.tab {
                FileExplorerTab::FindLocal | FileExplorerTab::Local => {
                    let wrkdir: PathBuf = self.remote.wrkdir.clone();
                    self.filetransfer_send(&entry.get_realfile(), wrkdir.as_path(), name);
                }
                FileExplorerTab::FindRemote | FileExplorerTab::Remote => {
                    let wrkdir: PathBuf = self.local.wrkdir.clone();
                    self.filetransfer_recv(&entry.get_realfile(), wrkdir.as_path(), name);
                }
            }
        }
    }

    pub(super) fn action_find_delete(&mut self, idx: usize) {
        let entry: Option<FsEntry> = self.found.as_ref().unwrap().get(idx).cloned();
        if let Some(entry) = entry {
            // Download file
            match self.tab {
                FileExplorerTab::FindLocal | FileExplorerTab::Local => {
                    let full_path: PathBuf = entry.get_abs_path();
                    // Delete file or directory and report status as popup
                    match self.context.as_mut().unwrap().local.remove(&entry) {
                        Ok(_) => {
                            // Reload files
                            let p: PathBuf = self.local.wrkdir.clone();
                            self.local_scan(p.as_path());
                            // Log
                            self.log(
                                LogLevel::Info,
                                format!("Removed file \"{}\"", full_path.display()).as_ref(),
                            );
                        }
                        Err(err) => {
                            self.log_and_alert(
                                LogLevel::Error,
                                format!(
                                    "Could not delete file \"{}\": {}",
                                    full_path.display(),
                                    err
                                ),
                            );
                        }
                    }
                }
                FileExplorerTab::FindRemote | FileExplorerTab::Remote => {
                    let full_path: PathBuf = entry.get_abs_path();
                    // Delete file
                    match self.client.remove(&entry) {
                        Ok(_) => {
                            self.reload_remote_dir();
                            self.log(
                                LogLevel::Info,
                                format!("Removed file \"{}\"", full_path.display()).as_ref(),
                            );
                        }
                        Err(err) => {
                            self.log_and_alert(
                                LogLevel::Error,
                                format!(
                                    "Could not delete file \"{}\": {}",
                                    full_path.display(),
                                    err
                                ),
                            );
                        }
                    }
                }
            }
        }
    }

    /// ### get_local_file_entry
    ///
    /// Get local file entry
    pub(super) fn get_local_file_entry(&self) -> Option<&FsEntry> {
        match self.get_local_file_idx() {
            None => None,
            Some(idx) => self.local.get(idx),
        }
    }

    /// ### get_remote_file_entry
    ///
    /// Get remote file entry
    pub(super) fn get_remote_file_entry(&self) -> Option<&FsEntry> {
        match self.get_remote_file_idx() {
            None => None,
            Some(idx) => self.remote.get(idx),
        }
    }

    // -- private

    /// ### get_local_file_idx
    ///
    /// Get index of selected file in the local tab
    fn get_local_file_idx(&self) -> Option<usize> {
        match self.view.get_state(super::COMPONENT_EXPLORER_LOCAL) {
            Some(Payload::One(Value::Usize(idx))) => Some(idx),
            _ => None,
        }
    }

    /// ### get_remote_file_idx
    ///
    /// Get index of selected file in the remote file
    fn get_remote_file_idx(&self) -> Option<usize> {
        match self.view.get_state(super::COMPONENT_EXPLORER_REMOTE) {
            Some(Payload::One(Value::Usize(idx))) => Some(idx),
            _ => None,
        }
    }
}
