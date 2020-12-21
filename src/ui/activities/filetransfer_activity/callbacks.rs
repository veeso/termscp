/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
*/

use super::{FileExplorerTab, FileTransferActivity, FsEntry, LogLevel};

use std::path::PathBuf;

impl FileTransferActivity {
    /// ### callback_nothing_to_do
    ///
    /// Self titled
    pub(super) fn callback_nothing_to_do(&mut self) {}

    /// ### callback_change_directory
    ///
    /// Callback for GOTO command
    pub(super) fn callback_change_directory(&mut self, input: String) {
        let dir_path: PathBuf = PathBuf::from(input);
        match self.tab {
            FileExplorerTab::Local => {
                // If path is relative, concat pwd
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
            FileExplorerTab::Remote => {
                // If path is relative, concat pwd
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
        }
    }

    /// ### callback_copy
    ///
    /// Callback for COPY command (both from local and remote)
    pub(super) fn callback_copy(&mut self, input: String) {
        let dest_path: PathBuf = PathBuf::from(input);
        match self.tab {
            FileExplorerTab::Local => {
                // Get selected entry
                if self.local.files.get(self.local.index).is_some() {
                    let entry: FsEntry = self.local.files.get(self.local.index).unwrap().clone();
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
            FileExplorerTab::Remote => {
                // Get selected entry
                if self.remote.files.get(self.remote.index).is_some() {
                    let entry: FsEntry = self.remote.files.get(self.remote.index).unwrap().clone();
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
        }
    }

    /// ### callback_mkdir
    ///
    /// Callback for MKDIR command (supports both local and remote)
    pub(super) fn callback_mkdir(&mut self, input: String) {
        match self.tab {
            FileExplorerTab::Local => {
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
            FileExplorerTab::Remote => {
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
        }
    }

    /// ### callback_rename
    ///
    /// Callback for RENAME command (supports borth local and remote)
    pub(super) fn callback_rename(&mut self, input: String) {
        match self.tab {
            FileExplorerTab::Local => {
                let mut dst_path: PathBuf = PathBuf::from(input);
                // Check if path is relative
                if dst_path.as_path().is_relative() {
                    let mut wrkdir: PathBuf = self.local.wrkdir.clone();
                    wrkdir.push(dst_path);
                    dst_path = wrkdir;
                }
                // Check if file entry exists
                if let Some(entry) = self.local.files.get(self.local.index) {
                    let full_path: PathBuf = entry.get_abs_path();
                    // Rename file or directory and report status as popup
                    match self
                        .context
                        .as_mut()
                        .unwrap()
                        .local
                        .rename(entry, dst_path.as_path())
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
                                format!(
                                    "Could not rename file \"{}\": {}",
                                    full_path.display(),
                                    err
                                ),
                            );
                        }
                    }
                }
            }
            FileExplorerTab::Remote => {
                // Check if file entry exists
                if let Some(entry) = self.remote.files.get(self.remote.index) {
                    let full_path: PathBuf = entry.get_abs_path();
                    // Rename file or directory and report status as popup
                    let dst_path: PathBuf = PathBuf::from(input);
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
                                format!(
                                    "Could not rename file \"{}\": {}",
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

    /// ### callback_delete_fsentry
    ///
    /// Delete current selected fsentry in the currently selected TAB
    pub(super) fn callback_delete_fsentry(&mut self) {
        // Match current selected tab
        match self.tab {
            FileExplorerTab::Local => {
                // Check if file entry exists
                if let Some(entry) = self.local.files.get(self.local.index) {
                    let full_path: PathBuf = entry.get_abs_path();
                    // Delete file or directory and report status as popup
                    match self.context.as_mut().unwrap().local.remove(entry) {
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
            }
            FileExplorerTab::Remote => {
                // Check if file entry exists
                if let Some(entry) = self.remote.files.get(self.remote.index) {
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

    /// ### callback_save_as
    ///
    /// Call file upload, but save with input as name
    /// Handled both local and remote tab
    pub(super) fn callback_save_as(&mut self, input: String) {
        match self.tab {
            FileExplorerTab::Local => {
                // Get pwd
                let wrkdir: PathBuf = self.remote.wrkdir.clone();
                // Get file and clone (due to mutable / immutable stuff...)
                if self.local.files.get(self.local.index).is_some() {
                    let file: FsEntry = self.local.files.get(self.local.index).unwrap().clone();
                    // Call upload; pass realfile, keep link name
                    self.filetransfer_send(&file.get_realfile(), wrkdir.as_path(), Some(input));
                }
            }
            FileExplorerTab::Remote => {
                // Get file and clone (due to mutable / immutable stuff...)
                if self.remote.files.get(self.remote.index).is_some() {
                    let file: FsEntry = self.remote.files.get(self.remote.index).unwrap().clone();
                    // Call upload; pass realfile, keep link name
                    let wrkdir: PathBuf = self.local.wrkdir.clone();
                    self.filetransfer_recv(&file.get_realfile(), wrkdir.as_path(), Some(input));
                }
            }
        }
    }
}
