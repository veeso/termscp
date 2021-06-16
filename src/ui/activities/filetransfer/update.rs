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
// deps
extern crate bytesize;
// locals
use super::{
    actions::SelectedEntry, browser::FileExplorerTab, FileTransferActivity, LogLevel,
    COMPONENT_EXPLORER_FIND, COMPONENT_EXPLORER_LOCAL, COMPONENT_EXPLORER_REMOTE,
    COMPONENT_INPUT_COPY, COMPONENT_INPUT_EXEC, COMPONENT_INPUT_FIND, COMPONENT_INPUT_GOTO,
    COMPONENT_INPUT_MKDIR, COMPONENT_INPUT_NEWFILE, COMPONENT_INPUT_RENAME, COMPONENT_INPUT_SAVEAS,
    COMPONENT_LIST_FILEINFO, COMPONENT_LOG_BOX, COMPONENT_PROGRESS_BAR_FULL,
    COMPONENT_PROGRESS_BAR_PARTIAL, COMPONENT_RADIO_DELETE, COMPONENT_RADIO_DISCONNECT,
    COMPONENT_RADIO_QUIT, COMPONENT_RADIO_SORTING, COMPONENT_TEXT_ERROR, COMPONENT_TEXT_FATAL,
    COMPONENT_TEXT_HELP,
};
use crate::fs::explorer::FileSorting;
use crate::fs::FsEntry;
use crate::ui::components::{file_list::FileListPropsBuilder, logbox::LogboxPropsBuilder};
use crate::ui::keymap::*;
// externals
use std::path::{Path, PathBuf};
use tuirealm::{
    components::progress_bar::ProgressBarPropsBuilder,
    props::{PropsBuilder, TableBuilder, TextSpan, TextSpanBuilder},
    tui::style::Color,
    Msg, Payload, Update, Value,
};

impl Update for FileTransferActivity {
    // -- update

    /// ### update
    ///
    /// Update auth activity model based on msg
    /// The function exits when returns None
    fn update(&mut self, msg: Option<(String, Msg)>) -> Option<(String, Msg)> {
        let ref_msg: Option<(&str, &Msg)> = msg.as_ref().map(|(s, msg)| (s.as_str(), msg));
        // Match msg
        match ref_msg {
            None => None, // Exit after None
            Some(msg) => match msg {
                // -- local tab
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_RIGHT) => {
                    // Change tab
                    self.view.active(COMPONENT_EXPLORER_REMOTE);
                    self.browser.change_tab(FileExplorerTab::Remote);
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_BACKSPACE) => {
                    // Go to previous directory
                    self.action_go_to_previous_local_dir(false);
                    if self.browser.sync_browsing {
                        let _ = self.update_remote_filelist();
                    }
                    // Reload file list component
                    self.update_local_filelist()
                }
                (COMPONENT_EXPLORER_LOCAL, Msg::OnSubmit(Payload::One(Value::Usize(idx)))) => {
                    // Match selected file
                    let mut entry: Option<FsEntry> = None;
                    if let Some(e) = self.local().get(*idx) {
                        entry = Some(e.clone());
                    }
                    if let Some(entry) = entry {
                        if self.action_enter_local_dir(entry, false) {
                            // Update file list if sync
                            if self.browser.sync_browsing {
                                let _ = self.update_remote_filelist();
                            }
                            self.update_local_filelist()
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_SPACE) => {
                    self.action_local_send();
                    self.update_remote_filelist()
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_A) => {
                    // Toggle hidden files
                    self.local_mut().toggle_hidden_files();
                    // Update status bar
                    self.refresh_local_status_bar();
                    // Reload file list component
                    self.update_local_filelist()
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_I) => {
                    if let SelectedEntry::One(file) = self.get_local_selected_entries() {
                        self.mount_file_info(&file);
                    }
                    None
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_L) => {
                    // Reload directory
                    let pwd: PathBuf = self.local().wrkdir.clone();
                    self.local_scan(pwd.as_path());
                    // Reload file list component
                    self.update_local_filelist()
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_O) => {
                    self.action_edit_local_file();
                    // Reload file list component
                    self.update_local_filelist()
                }
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_U) => {
                    self.action_go_to_local_upper_dir(false);
                    if self.browser.sync_browsing {
                        let _ = self.update_remote_filelist();
                    }
                    // Reload file list component
                    self.update_local_filelist()
                }
                // -- remote tab
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_LEFT) => {
                    // Change tab
                    self.view.active(COMPONENT_EXPLORER_LOCAL);
                    self.browser.change_tab(FileExplorerTab::Local);
                    None
                }
                (COMPONENT_EXPLORER_REMOTE, Msg::OnSubmit(Payload::One(Value::Usize(idx)))) => {
                    // Match selected file
                    let mut entry: Option<FsEntry> = None;
                    if let Some(e) = self.remote().get(*idx) {
                        entry = Some(e.clone());
                    }
                    if let Some(entry) = entry {
                        if self.action_enter_remote_dir(entry, false) {
                            // Update file list if sync
                            if self.browser.sync_browsing {
                                let _ = self.update_local_filelist();
                            }
                            self.update_remote_filelist()
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_SPACE) => {
                    self.action_remote_recv();
                    self.update_local_filelist()
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_BACKSPACE) => {
                    // Go to previous directory
                    self.action_go_to_previous_remote_dir(false);
                    // If sync is enabled update local too
                    if self.browser.sync_browsing {
                        let _ = self.update_local_filelist();
                    }
                    // Reload file list component
                    self.update_remote_filelist()
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_A) => {
                    // Toggle hidden files
                    self.remote_mut().toggle_hidden_files();
                    // Update status bar
                    self.refresh_remote_status_bar();
                    // Reload file list component
                    self.update_remote_filelist()
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_I) => {
                    if let SelectedEntry::One(file) = self.get_remote_selected_entries() {
                        self.mount_file_info(&file);
                    }
                    None
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_L) => {
                    // Reload directory
                    let pwd: PathBuf = self.remote().wrkdir.clone();
                    self.remote_scan(pwd.as_path());
                    // Reload file list component
                    self.update_remote_filelist()
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_O) => {
                    // Edit file
                    self.action_edit_remote_file();
                    // Reload file list component
                    self.update_remote_filelist()
                }
                (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_U) => {
                    self.action_go_to_remote_upper_dir(false);
                    if self.browser.sync_browsing {
                        let _ = self.update_local_filelist();
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
                (COMPONENT_EXPLORER_LOCAL, &MSG_KEY_CHAR_Y)
                | (COMPONENT_EXPLORER_REMOTE, &MSG_KEY_CHAR_Y) => {
                    // Toggle browser sync
                    self.browser.toggle_sync_browsing();
                    // Update status bar
                    self.refresh_remote_status_bar();
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
                (COMPONENT_EXPLORER_FIND, Msg::OnSubmit(_)) => {
                    // Find changedir
                    self.action_find_changedir();
                    // Umount find
                    self.umount_find();
                    // Finalize find
                    self.finalize_find();
                    // Reload files
                    match self.browser.tab() {
                        FileExplorerTab::Local => self.update_local_filelist(),
                        FileExplorerTab::Remote => self.update_remote_filelist(),
                        _ => None,
                    }
                }
                (COMPONENT_EXPLORER_FIND, &MSG_KEY_SPACE) => {
                    // Get entry
                    self.action_find_transfer(None);
                    // Reload files
                    match self.browser.tab() {
                        // NOTE: swapped by purpose
                        FileExplorerTab::FindLocal => self.update_remote_filelist(),
                        FileExplorerTab::FindRemote => self.update_local_filelist(),
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
                (COMPONENT_INPUT_COPY, Msg::OnSubmit(Payload::One(Value::Str(input)))) => {
                    // Copy file
                    match self.browser.tab() {
                        FileExplorerTab::Local => self.action_local_copy(input.to_string()),
                        FileExplorerTab::Remote => self.action_remote_copy(input.to_string()),
                        _ => panic!("Found tab doesn't support COPY"),
                    }
                    self.umount_copy();
                    // Reload files
                    match self.browser.tab() {
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
                (COMPONENT_INPUT_EXEC, Msg::OnSubmit(Payload::One(Value::Str(input)))) => {
                    // Exex command
                    match self.browser.tab() {
                        FileExplorerTab::Local => self.action_local_exec(input.to_string()),
                        FileExplorerTab::Remote => self.action_remote_exec(input.to_string()),
                        _ => panic!("Found tab doesn't support EXEC"),
                    }
                    self.umount_exec();
                    // Reload files
                    match self.browser.tab() {
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
                (COMPONENT_INPUT_FIND, Msg::OnSubmit(Payload::One(Value::Str(input)))) => {
                    self.umount_find_input();
                    // Find
                    let res: Result<Vec<FsEntry>, String> = match self.browser.tab() {
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
                            self.browser.set_found(files);
                            // Mount result widget
                            self.mount_find(input);
                            self.update_find_list();
                            // Initialize tab
                            self.browser.change_tab(match self.browser.tab() {
                                FileExplorerTab::Local => FileExplorerTab::FindLocal,
                                FileExplorerTab::Remote => FileExplorerTab::FindRemote,
                                _ => FileExplorerTab::FindLocal,
                            });
                        }
                    }
                    None
                }
                // -- goto popup
                (COMPONENT_INPUT_GOTO, &MSG_KEY_ESC) => {
                    self.umount_goto();
                    None
                }
                (COMPONENT_INPUT_GOTO, Msg::OnSubmit(Payload::One(Value::Str(input)))) => {
                    match self.browser.tab() {
                        FileExplorerTab::Local => {
                            self.action_change_local_dir(input.to_string(), false)
                        }
                        FileExplorerTab::Remote => {
                            self.action_change_remote_dir(input.to_string(), false)
                        }
                        _ => panic!("Found tab doesn't support GOTO"),
                    }
                    // Umount
                    self.umount_goto();
                    // Reload files if sync
                    if self.browser.sync_browsing {
                        match self.browser.tab() {
                            FileExplorerTab::Remote => self.update_local_filelist(),
                            FileExplorerTab::Local => self.update_remote_filelist(),
                            _ => None,
                        };
                    }
                    // Reload files
                    match self.browser.tab() {
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
                (COMPONENT_INPUT_MKDIR, Msg::OnSubmit(Payload::One(Value::Str(input)))) => {
                    match self.browser.tab() {
                        FileExplorerTab::Local => self.action_local_mkdir(input.to_string()),
                        FileExplorerTab::Remote => self.action_remote_mkdir(input.to_string()),
                        _ => panic!("Found tab doesn't support MKDIR"),
                    }
                    self.umount_mkdir();
                    // Reload files
                    match self.browser.tab() {
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
                (COMPONENT_INPUT_NEWFILE, Msg::OnSubmit(Payload::One(Value::Str(input)))) => {
                    match self.browser.tab() {
                        FileExplorerTab::Local => self.action_local_newfile(input.to_string()),
                        FileExplorerTab::Remote => self.action_remote_newfile(input.to_string()),
                        _ => panic!("Found tab doesn't support NEWFILE"),
                    }
                    self.umount_newfile();
                    // Reload files
                    match self.browser.tab() {
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
                (COMPONENT_INPUT_RENAME, Msg::OnSubmit(Payload::One(Value::Str(input)))) => {
                    match self.browser.tab() {
                        FileExplorerTab::Local => self.action_local_rename(input.to_string()),
                        FileExplorerTab::Remote => self.action_remote_rename(input.to_string()),
                        _ => panic!("Found tab doesn't support RENAME"),
                    }
                    self.umount_rename();
                    // Reload files
                    match self.browser.tab() {
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
                (COMPONENT_INPUT_SAVEAS, Msg::OnSubmit(Payload::One(Value::Str(input)))) => {
                    match self.browser.tab() {
                        FileExplorerTab::Local => self.action_local_saveas(input.to_string()),
                        FileExplorerTab::Remote => self.action_remote_saveas(input.to_string()),
                        FileExplorerTab::FindLocal | FileExplorerTab::FindRemote => {
                            // Get entry
                            self.action_find_transfer(Some(input.to_string()));
                        }
                    }
                    self.umount_saveas();
                    // Reload files
                    match self.browser.tab() {
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
                | (COMPONENT_RADIO_DELETE, Msg::OnSubmit(Payload::One(Value::Usize(1)))) => {
                    self.umount_radio_delete();
                    None
                }
                (COMPONENT_RADIO_DELETE, Msg::OnSubmit(Payload::One(Value::Usize(0)))) => {
                    // Choice is 'YES'
                    match self.browser.tab() {
                        FileExplorerTab::Local => self.action_local_delete(),
                        FileExplorerTab::Remote => self.action_remote_delete(),
                        FileExplorerTab::FindLocal | FileExplorerTab::FindRemote => {
                            // Get entry
                            self.action_find_delete();
                            // Delete entries
                            match self.view.get_state(COMPONENT_EXPLORER_FIND) {
                                Some(Payload::One(Value::Usize(idx))) => {
                                    // Reload entries
                                    self.found_mut().unwrap().del_entry(idx);
                                }
                                Some(Payload::Vec(values)) => {
                                    values
                                        .iter()
                                        .map(|x| match x {
                                            Value::Usize(v) => *v,
                                            _ => 0,
                                        })
                                        .for_each(|x| self.found_mut().unwrap().del_entry(x));
                                }
                                _ => {}
                            }
                            self.update_find_list();
                        }
                    }
                    self.umount_radio_delete();
                    // Reload files
                    match self.browser.tab() {
                        FileExplorerTab::Local => self.update_local_filelist(),
                        FileExplorerTab::Remote => self.update_remote_filelist(),
                        FileExplorerTab::FindLocal => self.update_local_filelist(),
                        FileExplorerTab::FindRemote => self.update_remote_filelist(),
                    }
                }
                // -- disconnect
                (COMPONENT_RADIO_DISCONNECT, &MSG_KEY_ESC)
                | (COMPONENT_RADIO_DISCONNECT, Msg::OnSubmit(Payload::One(Value::Usize(1)))) => {
                    self.umount_disconnect();
                    None
                }
                (COMPONENT_RADIO_DISCONNECT, Msg::OnSubmit(Payload::One(Value::Usize(0)))) => {
                    self.disconnect();
                    self.umount_disconnect();
                    None
                }
                // -- quit
                (COMPONENT_RADIO_QUIT, &MSG_KEY_ESC)
                | (COMPONENT_RADIO_QUIT, Msg::OnSubmit(Payload::One(Value::Usize(1)))) => {
                    self.umount_quit();
                    None
                }
                (COMPONENT_RADIO_QUIT, Msg::OnSubmit(Payload::One(Value::Usize(0)))) => {
                    self.disconnect_and_quit();
                    self.umount_quit();
                    None
                }
                // -- sorting
                (COMPONENT_RADIO_SORTING, &MSG_KEY_ESC)
                | (COMPONENT_RADIO_SORTING, Msg::OnSubmit(_)) => {
                    self.umount_file_sorting();
                    None
                }
                (COMPONENT_RADIO_SORTING, Msg::OnChange(Payload::One(Value::Usize(mode)))) => {
                    // Get sorting mode
                    let sorting: FileSorting = match mode {
                        1 => FileSorting::ByModifyTime,
                        2 => FileSorting::ByCreationTime,
                        3 => FileSorting::BySize,
                        _ => FileSorting::ByName,
                    };
                    match self.browser.tab() {
                        FileExplorerTab::Local => self.local_mut().sort_by(sorting),
                        FileExplorerTab::Remote => self.remote_mut().sort_by(sorting),
                        _ => panic!("Found result doesn't support SORTING"),
                    }
                    // Update status bar
                    match self.browser.tab() {
                        FileExplorerTab::Local => self.refresh_local_status_bar(),
                        FileExplorerTab::Remote => self.refresh_remote_status_bar(),
                        _ => panic!("Found result doesn't support SORTING"),
                    };
                    // Reload files
                    match self.browser.tab() {
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
                (COMPONENT_PROGRESS_BAR_PARTIAL, &MSG_KEY_CTRL_C) => {
                    // Set transfer aborted to True
                    self.transfer.abort();
                    None
                }
                // -- fallback
                (_, _) => None, // Nothing to do
            },
        }
    }
}

impl FileTransferActivity {
    /// ### update_local_filelist
    ///
    /// Update local file list
    pub(super) fn update_local_filelist(&mut self) -> Option<(String, Msg)> {
        match self.view.get_props(super::COMPONENT_EXPLORER_LOCAL) {
            Some(props) => {
                // Get width
                let width: usize = self
                    .context
                    .as_ref()
                    .unwrap()
                    .store
                    .get_unsigned(super::STORAGE_EXPLORER_WIDTH)
                    .unwrap_or(256);
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
                        self.local().wrkdir.as_path(),
                        hostname.as_str(),
                        width
                    )
                    .display()
                );
                let files: Vec<String> = self
                    .local()
                    .iter_files()
                    .map(|x: &FsEntry| self.local().fmt_file(x))
                    .collect();
                // Update
                let props = FileListPropsBuilder::from(props)
                    .with_files(Some(hostname), files)
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
        match self.view.get_props(super::COMPONENT_EXPLORER_REMOTE) {
            Some(props) => {
                // Get width
                let width: usize = self
                    .context
                    .as_ref()
                    .unwrap()
                    .store
                    .get_unsigned(super::STORAGE_EXPLORER_WIDTH)
                    .unwrap_or(256);
                let params = self.context.as_ref().unwrap().ft_params.as_ref().unwrap();
                let hostname: String = format!(
                    "{}:{} ",
                    params.address,
                    FileTransferActivity::elide_wrkdir_path(
                        self.remote().wrkdir.as_path(),
                        params.address.as_str(),
                        width
                    )
                    .display()
                );
                let files: Vec<String> = self
                    .remote()
                    .iter_files()
                    .map(|x: &FsEntry| self.remote().fmt_file(x))
                    .collect();
                // Update
                let props = FileListPropsBuilder::from(props)
                    .with_files(Some(hostname), files)
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
        match self.view.get_props(super::COMPONENT_LOG_BOX) {
            Some(props) => {
                // Make log entries
                let mut table: TableBuilder = TableBuilder::default();
                for (idx, record) in self.log_records.iter().enumerate() {
                    // Add row if not first row
                    if idx > 0 {
                        table.add_row();
                    }
                    let fg = match record.level {
                        LogLevel::Error => Color::Red,
                        LogLevel::Warn => Color::Yellow,
                        LogLevel::Info => Color::Green,
                    };
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
                        .add_col(TextSpan::from(record.msg.as_ref()));
                }
                let table = table.build();
                let props = LogboxPropsBuilder::from(props)
                    .with_log(Some(String::from("Log")), table)
                    .build();
                self.view.update(super::COMPONENT_LOG_BOX, props)
            }
            None => None,
        }
    }

    pub(super) fn update_progress_bar(&mut self, filename: String) -> Option<(String, Msg)> {
        if let Some(props) = self.view.get_props(COMPONENT_PROGRESS_BAR_FULL) {
            let root_name: String = props.texts.title.as_deref().unwrap_or("").to_string();
            let props = ProgressBarPropsBuilder::from(props)
                .with_texts(Some(root_name), self.transfer.full.to_string())
                .with_progress(self.transfer.full.calc_progress())
                .build();
            let _ = self.view.update(COMPONENT_PROGRESS_BAR_FULL, props);
        }
        match self.view.get_props(COMPONENT_PROGRESS_BAR_PARTIAL) {
            Some(props) => {
                let props = ProgressBarPropsBuilder::from(props)
                    .with_texts(Some(filename), self.transfer.partial.to_string())
                    .with_progress(self.transfer.partial.calc_progress())
                    .build();
                self.view.update(COMPONENT_PROGRESS_BAR_PARTIAL, props)
            }
            None => None,
        }
    }

    /// ### finalize_find
    ///
    /// Finalize find process
    fn finalize_find(&mut self) {
        // Set found to none
        self.browser.del_found();
        // Restore tab
        self.browser.change_tab(match self.browser.tab() {
            FileExplorerTab::FindLocal => FileExplorerTab::Local,
            FileExplorerTab::FindRemote => FileExplorerTab::Remote,
            _ => FileExplorerTab::Local,
        });
    }

    fn update_find_list(&mut self) -> Option<(String, Msg)> {
        match self.view.get_props(COMPONENT_EXPLORER_FIND) {
            None => None,
            Some(props) => {
                let title: String = props.texts.title.clone().unwrap_or_default();
                // Prepare files
                let files: Vec<String> = self
                    .found()
                    .unwrap()
                    .iter_files()
                    .map(|x: &FsEntry| self.found().unwrap().fmt_file(x))
                    .collect();
                let props = FileListPropsBuilder::from(props)
                    .with_files(Some(title), files)
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
