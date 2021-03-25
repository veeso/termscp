//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

/*
*
*   Copyright (C) 2020-2021 Christian Visintin - christian.visintin1997@gmail.com
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

// deps
extern crate bytesize;
// locals
use super::{
    FileExplorerTab, FileTransferActivity, LogLevel, COMPONENT_EXPLORER_FIND,
    COMPONENT_EXPLORER_LOCAL, COMPONENT_EXPLORER_REMOTE, COMPONENT_INPUT_COPY,
    COMPONENT_INPUT_EXEC, COMPONENT_INPUT_FIND, COMPONENT_INPUT_GOTO, COMPONENT_INPUT_MKDIR,
    COMPONENT_INPUT_NEWFILE, COMPONENT_INPUT_RENAME, COMPONENT_INPUT_SAVEAS,
    COMPONENT_LIST_FILEINFO, COMPONENT_LOG_BOX, COMPONENT_PROGRESS_BAR, COMPONENT_RADIO_DELETE,
    COMPONENT_RADIO_DISCONNECT, COMPONENT_RADIO_QUIT, COMPONENT_RADIO_SORTING,
    COMPONENT_TEXT_ERROR, COMPONENT_TEXT_FATAL, COMPONENT_TEXT_HELP,
};
use crate::fs::explorer::FileSorting;
use crate::fs::FsEntry;
use crate::ui::activities::keymap::*;
use crate::ui::layout::props::{
    PropValue, PropsBuilder, TableBuilder, TextParts, TextSpan, TextSpanBuilder,
};
use crate::ui::layout::{Msg, Payload};
// externals
use bytesize::ByteSize;
use std::path::{Path, PathBuf};
use tui::style::Color;

impl FileTransferActivity {
    // -- update

    /// ### update
    ///
    /// Update auth activity model based on msg
    /// The function exits when returns None
    pub(super) fn update(&mut self, msg: Option<(String, Msg)>) -> Option<(String, Msg)> {
        let ref_msg: Option<(&str, &Msg)> = match msg.as_ref() {
            None => None,
            Some((s, msg)) => Some((s, msg)),
        };
        // Match msg
        match ref_msg {
            None => None, // Exit after None
            Some(msg) => match msg {
                // -- local tab
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_RIGHT) => {
                    // Change tab
                    self.view.active(COMPONENT_EXPLORER_REMOTE);
                    self.tab = FileExplorerTab::Remote;
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_BACKSPACE) => {
                    // Go to previous directory
                    if let Some(d) = self.local.popd() {
                        self.local_changedir(d.as_path(), false);
                    }
                    // Reload file list component
                    self.update_local_filelist()
                }
                (COMPONENT_EXPLORER_LOCAL, Msg::OnSubmit(Payload::Unsigned(idx))) => {
                    // Match selected file
                    let mut entry: Option<FsEntry> = None;
                    if let Some(e) = self.local.get(*idx) {
                        entry = Some(e.clone());
                    }
                    if let Some(entry) = entry {
                        // If directory, enter directory, otherwise check if symlink
                        match entry {
                            FsEntry::Directory(dir) => {
                                self.local_changedir(dir.abs_path.as_path(), true);
                                self.update_local_filelist()
                            }
                            FsEntry::File(file) => {
                                // Check if symlink
                                match &file.symlink {
                                    Some(pointer) => match &**pointer {
                                        FsEntry::Directory(dir) => {
                                            self.local_changedir(dir.abs_path.as_path(), true);
                                            self.update_local_filelist()
                                        }
                                        _ => None,
                                    },
                                    None => None,
                                }
                            }
                        }
                    } else {
                        None
                    }
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_SPACE) => {
                    // Get pwd
                    let wrkdir: PathBuf = self.remote.wrkdir.clone();
                    // Get file and clone (due to mutable / immutable stuff...)
                    if self.get_local_file_entry().is_some() {
                        let file: FsEntry = self.get_local_file_entry().unwrap().clone();
                        let name: String = file.get_name().to_string();
                        // Call upload; pass realfile, keep link name
                        self.filetransfer_send(&file.get_realfile(), wrkdir.as_path(), Some(name));
                        self.update_remote_filelist()
                    } else {
                        None
                    }
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_A) => {
                    // Toggle hidden files
                    self.local.toggle_hidden_files();
                    // Reload file list component
                    self.update_local_filelist()
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_I) => {
                    let file: Option<FsEntry> = match self.get_local_file_entry() {
                        Some(f) => Some(f.clone()),
                        None => None,
                    };
                    if let Some(file) = file {
                        self.mount_file_info(&file);
                    }
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_L) => {
                    // Reload directory
                    let pwd: PathBuf = self.local.wrkdir.clone();
                    self.local_scan(pwd.as_path());
                    // Reload file list component
                    self.update_local_filelist()
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_O) => {
                    // Clone entry due to mutable stuff...
                    if self.get_local_file_entry().is_some() {
                        let fsentry: FsEntry = self.get_local_file_entry().unwrap().clone();
                        // Check if file
                        if fsentry.is_file() {
                            self.log(
                                LogLevel::Info,
                                format!("Opening file \"{}\"...", fsentry.get_abs_path().display())
                                    .as_str(),
                            );
                            // Edit file
                            match self.edit_local_file(fsentry.get_abs_path().as_path()) {
                                Ok(_) => {
                                    // Reload directory
                                    let pwd: PathBuf = self.local.wrkdir.clone();
                                    self.local_scan(pwd.as_path());
                                }
                                Err(err) => self.log_and_alert(LogLevel::Error, err),
                            }
                        }
                    }
                    // Reload file list component
                    self.update_local_filelist()
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_U) => {
                    // Get pwd
                    let path: PathBuf = self.local.wrkdir.clone();
                    // Go to parent directory
                    if let Some(parent) = path.as_path().parent() {
                        self.local_changedir(parent, true);
                        // Reload file list component
                    }
                    self.update_local_filelist()
                }
                // -- remote tab
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_LEFT) => {
                    // Change tab
                    self.view.active(COMPONENT_EXPLORER_LOCAL);
                    self.tab = FileExplorerTab::Local;
                    None
                }
                (COMPONENT_EXPLORER_REMOTE, Msg::OnSubmit(Payload::Unsigned(idx))) => {
                    // Match selected file
                    let mut entry: Option<FsEntry> = None;
                    if let Some(e) = self.remote.get(*idx) {
                        entry = Some(e.clone());
                    }
                    if let Some(entry) = entry {
                        // If directory, enter directory; if file, check if is symlink
                        match entry {
                            FsEntry::Directory(dir) => {
                                self.remote_changedir(dir.abs_path.as_path(), true);
                                self.update_remote_filelist()
                            }
                            FsEntry::File(file) => {
                                match &file.symlink {
                                    Some(symlink_entry) => {
                                        // If symlink and is directory, point to symlink
                                        match &**symlink_entry {
                                            FsEntry::Directory(dir) => {
                                                self.remote_changedir(dir.abs_path.as_path(), true);
                                                self.update_remote_filelist()
                                            }
                                            _ => None,
                                        }
                                    }
                                    None => None,
                                }
                            }
                        }
                    } else {
                        None
                    }
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_SPACE) => {
                    // Get file and clone (due to mutable / immutable stuff...)
                    if self.get_remote_file_entry().is_some() {
                        let file: FsEntry = self.get_remote_file_entry().unwrap().clone();
                        let name: String = file.get_name().to_string();
                        // Call upload; pass realfile, keep link name
                        let wrkdir: PathBuf = self.local.wrkdir.clone();
                        self.filetransfer_recv(&file.get_realfile(), wrkdir.as_path(), Some(name));
                        self.update_local_filelist()
                    } else {
                        None
                    }
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_BACKSPACE) => {
                    // Go to previous directory
                    if let Some(d) = self.remote.popd() {
                        self.remote_changedir(d.as_path(), false);
                    }
                    // Reload file list component
                    self.update_remote_filelist()
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_A) => {
                    // Toggle hidden files
                    self.remote.toggle_hidden_files();
                    // Reload file list component
                    self.update_remote_filelist()
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_I) => {
                    let file: Option<FsEntry> = match self.get_remote_file_entry() {
                        Some(f) => Some(f.clone()),
                        None => None,
                    };
                    if let Some(file) = file {
                        self.mount_file_info(&file);
                    }
                    None
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_L) => {
                    // Reload directory
                    let pwd: PathBuf = self.remote.wrkdir.clone();
                    self.remote_scan(pwd.as_path());
                    // Reload file list component
                    self.update_remote_filelist()
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_O) => {
                    // Clone entry due to mutable stuff...
                    if self.get_remote_file_entry().is_some() {
                        let fsentry: FsEntry = self.get_remote_file_entry().unwrap().clone();
                        // Check if file
                        if let FsEntry::File(file) = fsentry.clone() {
                            self.log(
                                LogLevel::Info,
                                format!("Opening file \"{}\"...", fsentry.get_abs_path().display())
                                    .as_str(),
                            );
                            // Edit file
                            match self.edit_remote_file(&file) {
                                Ok(_) => {
                                    // Reload directory
                                    let pwd: PathBuf = self.remote.wrkdir.clone();
                                    self.remote_scan(pwd.as_path());
                                }
                                Err(err) => self.log_and_alert(LogLevel::Error, err),
                            }
                        }
                    }
                    // Reload file list component
                    self.update_remote_filelist()
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_U) => {
                    // Get pwd
                    let path: PathBuf = self.remote.wrkdir.clone();
                    // Go to parent directory
                    if let Some(parent) = path.as_path().parent() {
                        self.remote_changedir(parent, true);
                    }
                    // Reload file list component
                    self.update_remote_filelist()
                }
                // -- common explorer keys
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_B)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_B) => {
                    // Show sorting file
                    self.mount_file_sorting();
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_C)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_C) => {
                    self.mount_copy();
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_D)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_D) => {
                    self.mount_mkdir();
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_F)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_F) => {
                    self.mount_find_input();
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_G)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_G) => {
                    self.mount_goto();
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_H)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_H) => {
                    self.mount_help();
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_N)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_N) => {
                    self.mount_newfile();
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_Q)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_Q)
                | (COMPONENT_LOG_BOX, &MSG_KEY_CHAR_Q) => {
                    self.mount_quit();
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_R)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_R) => {
                    // Mount rename
                    self.mount_rename();
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_S)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_S)
                | (COMPONENT_EXPLORER_FIND, &MSG_KEY_CHAR_S) => {
                    // Mount save as
                    self.mount_saveas();
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_X)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_X) => {
                    // Mount exec
                    self.mount_exec();
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_ESC)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_ESC)
                | (COMPONENT_LOG_BOX, &MSG_KEY_ESC) => {
                    self.mount_disconnect();
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_DEL)
                | (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_E)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_DEL)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_E)
                | (COMPONENT_EXPLORER_FIND, &MSG_KEY_DEL)
                | (COMPONENT_EXPLORER_FIND, &MSG_KEY_CHAR_E) => {
                    self.mount_radio_delete();
                    None
                }
                // -- find result explorer
                (COMPONENT_EXPLORER_FIND, &MSG_KEY_ESC) => {
                    // Umount find
                    self.umount_find();
                    // Finalize find
                    self.finalize_find();
                    None
                }
                (COMPONENT_EXPLORER_FIND, Msg::OnSubmit(Payload::Unsigned(idx))) => {
                    // Find changedir
                    self.action_find_changedir(*idx);
                    // Umount find
                    self.umount_find();
                    // Finalize find
                    self.finalize_find();
                    // Reload files
                    match self.tab {
                        FileExplorerTab::Local => self.update_local_filelist(),
                        FileExplorerTab::Remote => self.update_remote_filelist(),
                        _ => None,
                    }
                }
                (COMPONENT_EXPLORER_FIND, &MSG_KEY_SPACE) => {
                    // Get entry
                    match self.view.get_value(COMPONENT_EXPLORER_FIND) {
                        Some(Payload::Unsigned(idx)) => {
                            self.action_find_transfer(idx, None);
                            // Reload files
                            match self.tab {
                                // NOTE: swapped by purpose
                                FileExplorerTab::FindLocal => self.update_remote_filelist(),
                                FileExplorerTab::FindRemote => self.update_local_filelist(),
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                // -- switch to log
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_TAB)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_TAB) => {
                    self.view.active(COMPONENT_LOG_BOX); // Active log box
                    None
                }
                // -- Log box
                (COMPONENT_LOG_BOX, &MSG_KEY_TAB) => {
                    self.view.blur(); // Blur log box
                    None
                }
                // -- copy popup
                (COMPONENT_INPUT_COPY, &MSG_KEY_ESC) => {
                    self.umount_copy();
                    None
                }
                (COMPONENT_INPUT_COPY, Msg::OnSubmit(Payload::Text(input))) => {
                    // Copy file
                    match self.tab {
                        FileExplorerTab::Local => self.action_local_copy(input.to_string()),
                        FileExplorerTab::Remote => self.action_remote_copy(input.to_string()),
                        _ => panic!("Found tab doesn't support COPY"),
                    }
                    self.umount_copy();
                    // Reload files
                    match self.tab {
                        FileExplorerTab::Local => self.update_local_filelist(),
                        FileExplorerTab::Remote => self.update_remote_filelist(),
                        _ => None,
                    }
                }
                // -- exec popup
                (COMPONENT_INPUT_EXEC, &MSG_KEY_ESC) => {
                    self.umount_exec();
                    None
                }
                (COMPONENT_INPUT_EXEC, Msg::OnSubmit(Payload::Text(input))) => {
                    // Exex command
                    match self.tab {
                        FileExplorerTab::Local => self.action_local_exec(input.to_string()),
                        FileExplorerTab::Remote => self.action_remote_exec(input.to_string()),
                        _ => panic!("Found tab doesn't support EXEC"),
                    }
                    self.umount_exec();
                    // Reload files
                    match self.tab {
                        FileExplorerTab::Local => self.update_local_filelist(),
                        FileExplorerTab::Remote => self.update_remote_filelist(),
                        _ => None,
                    }
                }
                // -- find popup
                (COMPONENT_INPUT_FIND, &MSG_KEY_ESC) => {
                    self.umount_find_input();
                    None
                }
                (COMPONENT_INPUT_FIND, Msg::OnSubmit(Payload::Text(input))) => {
                    self.umount_find_input();
                    // Find
                    let res: Result<Vec<FsEntry>, String> = match self.tab {
                        FileExplorerTab::Local => self.action_local_find(input.to_string()),
                        FileExplorerTab::Remote => self.action_remote_find(input.to_string()),
                        _ => panic!("Trying to search for files, while already in a find result"),
                    };
                    // Match result
                    match res {
                        Err(err) => {
                            // Mount error
                            self.mount_error(err.as_str());
                        }
                        Ok(files) => {
                            // Create explorer and load files
                            let mut explorer = Self::build_found_explorer();
                            explorer.set_files(files);
                            self.found = Some(explorer);
                            // Mount result widget
                            self.mount_find(input);
                            self.update_find_list();
                            // Initialize tab
                            self.tab = match self.tab {
                                FileExplorerTab::Local => FileExplorerTab::FindLocal,
                                FileExplorerTab::Remote => FileExplorerTab::FindRemote,
                                _ => FileExplorerTab::FindLocal,
                            };
                        }
                    }
                    None
                }
                // -- goto popup
                (COMPONENT_INPUT_GOTO, &MSG_KEY_ESC) => {
                    self.umount_goto();
                    None
                }
                (COMPONENT_INPUT_GOTO, Msg::OnSubmit(Payload::Text(input))) => {
                    match self.tab {
                        FileExplorerTab::Local => self.action_change_local_dir(input.to_string()),
                        FileExplorerTab::Remote => self.action_change_remote_dir(input.to_string()),
                        _ => panic!("Found tab doesn't support GOTO"),
                    }
                    // Umount
                    self.umount_goto();
                    // Reload files
                    match self.tab {
                        FileExplorerTab::Local => self.update_local_filelist(),
                        FileExplorerTab::Remote => self.update_remote_filelist(),
                        _ => None,
                    }
                }
                // -- make directory
                (COMPONENT_INPUT_MKDIR, &MSG_KEY_ESC) => {
                    self.umount_mkdir();
                    None
                }
                (COMPONENT_INPUT_MKDIR, Msg::OnSubmit(Payload::Text(input))) => {
                    match self.tab {
                        FileExplorerTab::Local => self.action_local_mkdir(input.to_string()),
                        FileExplorerTab::Remote => self.action_remote_mkdir(input.to_string()),
                        _ => panic!("Found tab doesn't support MKDIR"),
                    }
                    self.umount_mkdir();
                    // Reload files
                    match self.tab {
                        FileExplorerTab::Local => self.update_local_filelist(),
                        FileExplorerTab::Remote => self.update_remote_filelist(),
                        _ => None,
                    }
                }
                // -- new file
                (COMPONENT_INPUT_NEWFILE, &MSG_KEY_ESC) => {
                    self.umount_newfile();
                    None
                }
                (COMPONENT_INPUT_NEWFILE, Msg::OnSubmit(Payload::Text(input))) => {
                    match self.tab {
                        FileExplorerTab::Local => self.action_local_newfile(input.to_string()),
                        FileExplorerTab::Remote => self.action_remote_newfile(input.to_string()),
                        _ => panic!("Found tab doesn't support NEWFILE"),
                    }
                    self.umount_newfile();
                    // Reload files
                    match self.tab {
                        FileExplorerTab::Local => self.update_local_filelist(),
                        FileExplorerTab::Remote => self.update_remote_filelist(),
                        _ => None,
                    }
                }
                // -- rename
                (COMPONENT_INPUT_RENAME, &MSG_KEY_ESC) => {
                    self.umount_rename();
                    None
                }
                (COMPONENT_INPUT_RENAME, Msg::OnSubmit(Payload::Text(input))) => {
                    match self.tab {
                        FileExplorerTab::Local => self.action_local_rename(input.to_string()),
                        FileExplorerTab::Remote => self.action_remote_rename(input.to_string()),
                        _ => panic!("Found tab doesn't support RENAME"),
                    }
                    self.umount_rename();
                    // Reload files
                    match self.tab {
                        FileExplorerTab::Local => self.update_local_filelist(),
                        FileExplorerTab::Remote => self.update_remote_filelist(),
                        _ => None,
                    }
                }
                // -- save as
                (COMPONENT_INPUT_SAVEAS, &MSG_KEY_ESC) => {
                    self.umount_saveas();
                    None
                }
                (COMPONENT_INPUT_SAVEAS, Msg::OnSubmit(Payload::Text(input))) => {
                    match self.tab {
                        FileExplorerTab::Local => self.action_local_saveas(input.to_string()),
                        FileExplorerTab::Remote => self.action_remote_saveas(input.to_string()),
                        FileExplorerTab::FindLocal | FileExplorerTab::FindRemote => {
                            // Get entry
                            if let Some(Payload::Unsigned(idx)) =
                                self.view.get_value(COMPONENT_EXPLORER_FIND)
                            {
                                self.action_find_transfer(idx, Some(input.to_string()));
                            }
                        }
                    }
                    self.umount_saveas();
                    // Reload files
                    match self.tab {
                        // NOTE: Swapped is intentional
                        FileExplorerTab::Local => self.update_remote_filelist(),
                        FileExplorerTab::Remote => self.update_local_filelist(),
                        FileExplorerTab::FindLocal => self.update_remote_filelist(),
                        FileExplorerTab::FindRemote => self.update_local_filelist(),
                    }
                }
                // -- fileinfo
                (COMPONENT_LIST_FILEINFO, &MSG_KEY_ENTER)
                | (COMPONENT_LIST_FILEINFO, &MSG_KEY_ESC) => {
                    self.umount_file_info();
                    None
                }
                // -- delete
                (COMPONENT_RADIO_DELETE, &MSG_KEY_ESC)
                | (COMPONENT_RADIO_DELETE, Msg::OnSubmit(Payload::Unsigned(1))) => {
                    self.umount_radio_delete();
                    None
                }
                (COMPONENT_RADIO_DELETE, Msg::OnSubmit(Payload::Unsigned(0))) => {
                    // Choice is 'YES'
                    match self.tab {
                        FileExplorerTab::Local => self.action_local_delete(),
                        FileExplorerTab::Remote => self.action_remote_delete(),
                        FileExplorerTab::FindLocal | FileExplorerTab::FindRemote => {
                            // Get entry
                            if let Some(Payload::Unsigned(idx)) =
                                self.view.get_value(COMPONENT_EXPLORER_FIND)
                            {
                                self.action_find_delete(idx);
                                // Reload entries
                                self.found.as_mut().unwrap().del_entry(idx);
                                self.update_find_list();
                            }
                        }
                    }
                    self.umount_radio_delete();
                    // Reload files
                    match self.tab {
                        FileExplorerTab::Local => self.update_local_filelist(),
                        FileExplorerTab::Remote => self.update_remote_filelist(),
                        FileExplorerTab::FindLocal => self.update_local_filelist(),
                        FileExplorerTab::FindRemote => self.update_remote_filelist(),
                    }
                }
                // -- disconnect
                (COMPONENT_RADIO_DISCONNECT, &MSG_KEY_ESC)
                | (COMPONENT_RADIO_DISCONNECT, Msg::OnSubmit(Payload::Unsigned(1))) => {
                    self.umount_disconnect();
                    None
                }
                (COMPONENT_RADIO_DISCONNECT, Msg::OnSubmit(Payload::Unsigned(0))) => {
                    self.disconnect();
                    self.umount_disconnect();
                    None
                }
                // -- quit
                (COMPONENT_RADIO_QUIT, &MSG_KEY_ESC)
                | (COMPONENT_RADIO_QUIT, Msg::OnSubmit(Payload::Unsigned(1))) => {
                    self.umount_quit();
                    None
                }
                (COMPONENT_RADIO_QUIT, Msg::OnSubmit(Payload::Unsigned(0))) => {
                    self.disconnect_and_quit();
                    self.umount_quit();
                    None
                }
                // -- sorting
                (COMPONENT_RADIO_SORTING, &MSG_KEY_ESC) => {
                    self.umount_file_sorting();
                    None
                }
                (COMPONENT_RADIO_SORTING, Msg::OnSubmit(Payload::Unsigned(mode))) => {
                    // Get sorting mode
                    let sorting: FileSorting = match mode {
                        1 => FileSorting::ByModifyTime,
                        2 => FileSorting::ByCreationTime,
                        3 => FileSorting::BySize,
                        _ => FileSorting::ByName,
                    };
                    match self.tab {
                        FileExplorerTab::Local => self.local.sort_by(sorting),
                        FileExplorerTab::Remote => self.remote.sort_by(sorting),
                        _ => panic!("Found result doesn't support SORTING"),
                    }
                    self.umount_file_sorting();
                    // Reload files
                    match self.tab {
                        FileExplorerTab::Local => self.update_local_filelist(),
                        FileExplorerTab::Remote => self.update_remote_filelist(),
                        _ => None,
                    }
                }
                // -- error
                (COMPONENT_TEXT_ERROR, &MSG_KEY_ESC) | (COMPONENT_TEXT_ERROR, &MSG_KEY_ENTER) => {
                    self.umount_error();
                    None
                }
                // -- fatal
                (COMPONENT_TEXT_FATAL, &MSG_KEY_ESC) | (COMPONENT_TEXT_FATAL, &MSG_KEY_ENTER) => {
                    self.exit_reason = Some(super::ExitReason::Disconnect);
                    None
                }
                // -- help
                (COMPONENT_TEXT_HELP, &MSG_KEY_ESC) | (COMPONENT_TEXT_HELP, &MSG_KEY_ENTER) => {
                    self.umount_help();
                    None
                }
                // -- progress bar
                (COMPONENT_PROGRESS_BAR, &MSG_KEY_CTRL_C) => {
                    // Set transfer aborted to True
                    self.transfer.aborted = true;
                    None
                }
                // -- fallback
                (_, _) => None, // Nothing to do
            },
        }
    }

    /// ### update_local_filelist
    ///
    /// Update local file list
    pub(super) fn update_local_filelist(&mut self) -> Option<(String, Msg)> {
        match self
            .view
            .get_props(super::COMPONENT_EXPLORER_LOCAL)
            .as_mut()
        {
            Some(props) => {
                // Get width
                let width: usize = match self
                    .context
                    .as_ref()
                    .unwrap()
                    .store
                    .get_unsigned(super::STORAGE_EXPLORER_WIDTH)
                {
                    Some(val) => val,
                    None => 256, // Default
                };
                let hostname: String = match hostname::get() {
                    Ok(h) => {
                        let hostname: String = h.as_os_str().to_string_lossy().to_string();
                        let tokens: Vec<&str> = hostname.split('.').collect();
                        String::from(*tokens.get(0).unwrap_or(&"localhost"))
                    }
                    Err(_) => String::from("localhost"),
                };
                let hostname: String = format!(
                    "{}:{} ",
                    hostname,
                    FileTransferActivity::elide_wrkdir_path(
                        self.local.wrkdir.as_path(),
                        hostname.as_str(),
                        width
                    )
                    .display()
                );
                let files: Vec<TextSpan> = self
                    .local
                    .iter_files()
                    .map(|x: &FsEntry| TextSpan::from(self.local.fmt_file(x)))
                    .collect();
                // Update
                let props = props
                    .with_texts(TextParts::new(Some(hostname), Some(files)))
                    .build();
                // Update
                self.view.update(super::COMPONENT_EXPLORER_LOCAL, props)
            }
            None => None,
        }
    }

    /// ### update_remote_filelist
    ///
    /// Update remote file list
    pub(super) fn update_remote_filelist(&mut self) -> Option<(String, Msg)> {
        match self
            .view
            .get_props(super::COMPONENT_EXPLORER_REMOTE)
            .as_mut()
        {
            Some(props) => {
                // Get width
                let width: usize = match self
                    .context
                    .as_ref()
                    .unwrap()
                    .store
                    .get_unsigned(super::STORAGE_EXPLORER_WIDTH)
                {
                    Some(val) => val,
                    None => 256, // Default
                };
                let params = self.context.as_ref().unwrap().ft_params.as_ref().unwrap();
                let hostname: String = format!(
                    "{}:{} ",
                    params.address,
                    FileTransferActivity::elide_wrkdir_path(
                        self.remote.wrkdir.as_path(),
                        params.address.as_str(),
                        width
                    )
                    .display()
                );
                let files: Vec<TextSpan> = self
                    .remote
                    .iter_files()
                    .map(|x: &FsEntry| TextSpan::from(self.remote.fmt_file(x)))
                    .collect();
                // Update
                let props = props
                    .with_texts(TextParts::new(Some(hostname), Some(files)))
                    .build();
                self.view.update(super::COMPONENT_EXPLORER_REMOTE, props)
            }
            None => None,
        }
    }

    /// ### update_logbox
    ///
    /// Update log box
    pub(super) fn update_logbox(&mut self) -> Option<(String, Msg)> {
        match self.view.get_props(super::COMPONENT_LOG_BOX).as_mut() {
            Some(props) => {
                // Get width
                let width: usize = match self
                    .context
                    .as_ref()
                    .unwrap()
                    .store
                    .get_unsigned(super::STORAGE_LOGBOX_WIDTH)
                {
                    Some(val) => val,
                    None => 256, // Default
                };
                // Make log entries
                let mut table: TableBuilder = TableBuilder::default();
                for (idx, record) in self.log_records.iter().enumerate() {
                    let record_rows = textwrap::wrap(record.msg.as_str(), (width as usize) - 38); // -35 'cause log prefix -3 cause of log line cursor
                                                                                                  // Add row if not first row
                    if idx > 0 {
                        table.add_row();
                    }
                    let fg = match record.level {
                        LogLevel::Error => Color::Red,
                        LogLevel::Warn => Color::Yellow,
                        LogLevel::Info => Color::Green,
                    };
                    for (idx, row) in record_rows.iter().enumerate() {
                        match idx {
                            0 => {
                                // First row
                                table
                                    .add_col(TextSpan::from(format!(
                                        "{}",
                                        record.time.format("%Y-%m-%dT%H:%M:%S%Z")
                                    )))
                                    .add_col(TextSpan::from(" ["))
                                    .add_col(
                                        TextSpanBuilder::new(
                                            format!(
                                                "{:5}",
                                                match record.level {
                                                    LogLevel::Error => "ERROR",
                                                    LogLevel::Warn => "WARN",
                                                    LogLevel::Info => "INFO",
                                                }
                                            )
                                            .as_str(),
                                        )
                                        .with_foreground(fg)
                                        .build(),
                                    )
                                    .add_col(TextSpan::from("]: "))
                                    .add_col(TextSpan::from(row.as_ref()));
                            }
                            _ => {
                                table.add_col(TextSpan::from(textwrap::indent(
                                    row.as_ref(),
                                    "                                   ",
                                )));
                            }
                        }
                    }
                }
                let table = table.build();
                let props = props
                    .with_texts(TextParts::table(Some(String::from("Log")), table))
                    .build();
                self.view.update(super::COMPONENT_LOG_BOX, props)
            }
            None => None,
        }
    }

    pub(super) fn update_progress_bar(&mut self, text: String) -> Option<(String, Msg)> {
        match self.view.get_props(COMPONENT_PROGRESS_BAR).as_mut() {
            Some(props) => {
                // Calculate ETA
                let elapsed_secs: u64 = self.transfer.started.elapsed().as_secs();
                let eta: String = match self.transfer.progress as u64 {
                    0 => String::from("--:--"), // NOTE: would divide by 0 :D
                    _ => {
                        let eta: u64 =
                            ((elapsed_secs * 100) / (self.transfer.progress as u64)) - elapsed_secs;
                        format!("{:0width$}:{:0width$}", (eta / 60), (eta % 60), width = 2)
                    }
                };
                // Calculate bytes/s
                let label = format!(
                    "{:.2}% - ETA {} ({}/s)",
                    self.transfer.progress,
                    eta,
                    ByteSize(self.transfer.bytes_per_second())
                );
                let props = props
                    .with_texts(TextParts::new(
                        Some(text),
                        Some(vec![TextSpan::from(label)]),
                    ))
                    .with_value(PropValue::Float(self.transfer.progress / 100.0))
                    .build();
                self.view.update(COMPONENT_PROGRESS_BAR, props)
            }
            None => None,
        }
    }

    /// ### finalize_find
    ///
    /// Finalize find process
    fn finalize_find(&mut self) {
        // Set found to none
        self.found = None;
        // Restore tab
        self.tab = match self.tab {
            FileExplorerTab::FindLocal => FileExplorerTab::Local,
            FileExplorerTab::FindRemote => FileExplorerTab::Remote,
            _ => FileExplorerTab::Local,
        };
    }

    fn update_find_list(&mut self) -> Option<(String, Msg)> {
        match self.view.get_props(COMPONENT_EXPLORER_FIND).as_mut() {
            None => None,
            Some(props) => {
                let props = props.build();
                let title: String = props.texts.title.clone().unwrap_or(String::new());
                let mut props = PropsBuilder::from(props.clone());
                // Prepare files
                let file_texts: Vec<TextSpan> = self
                    .found
                    .as_ref()
                    .unwrap()
                    .iter_files()
                    .map(|x: &FsEntry| TextSpan::from(self.found.as_ref().unwrap().fmt_file(x)))
                    .collect();
                let props = props
                    .with_texts(TextParts::new(Some(title), Some(file_texts)))
                    .build();
                self.view.update(COMPONENT_EXPLORER_FIND, props)
            }
        }
    }

    /// ### elide_wrkdir_path
    ///
    /// Elide working directory path if longer than width + host.len
    /// In this case, the path is formatted to {ANCESTOR[0]}/.../{PARENT[0]}/{BASENAME}
    fn elide_wrkdir_path(wrkdir: &Path, host: &str, width: usize) -> PathBuf {
        let fmt_path: String = format!("{}", wrkdir.display());
        // NOTE: +5 is const
        match fmt_path.len() + host.len() + 5 > width {
            false => PathBuf::from(wrkdir),
            true => {
                // Elide
                let ancestors_len: usize = wrkdir.ancestors().count();
                let mut ancestors = wrkdir.ancestors();
                let mut elided_path: PathBuf = PathBuf::new();
                // If ancestors_len's size is bigger than 2, push count - 2
                if ancestors_len > 2 {
                    elided_path.push(ancestors.nth(ancestors_len - 2).unwrap());
                }
                // If ancestors_len is bigger than 3, push '...' and parent too
                if ancestors_len > 3 {
                    elided_path.push("...");
                    if let Some(parent) = wrkdir.ancestors().nth(1) {
                        elided_path.push(parent.file_name().unwrap());
                    }
                }
                // Push file_name
                if let Some(name) = wrkdir.file_name() {
                    elided_path.push(name);
                }
                elided_path
            }
        }
    }
}
