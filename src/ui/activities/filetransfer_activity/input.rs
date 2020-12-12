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

use super::{
    DialogCallback, DialogYesNoOption, FileExplorerTab, FileTransferActivity, FsEntry, InputEvent,
    InputField, InputMode, LogLevel, OnInputSubmitCallback, PopupType,
};

use crossterm::event::KeyCode;
use std::path::PathBuf;
use tui::style::Color;

impl FileTransferActivity {
    /// ### handle_input_event
    ///
    /// Handle input event based on current input mode
    pub(super) fn handle_input_event(&mut self, ev: &InputEvent) {
        // NOTE: this is necessary due to this <https://github.com/rust-lang/rust/issues/59159>
        // NOTE: Do you want my opinion about that issue? It's a bs and doesn't make any sense.
        let popup: Option<PopupType> = match &self.input_mode {
            InputMode::Popup(ptype) => Some(ptype.clone()),
            _ => None,
        };
        match &self.input_mode {
            InputMode::Explorer => self.handle_input_event_mode_explorer(ev),
            InputMode::Popup(_) => {
                if let Some(popup) = popup {
                    self.handle_input_event_mode_popup(ev, popup);
                }
            }
        }
    }

    /// ### handle_input_event_mode_explorer
    ///
    /// Input event handler for explorer mode
    pub(super) fn handle_input_event_mode_explorer(&mut self, ev: &InputEvent) {
        // Match input field
        match self.input_field {
            InputField::Explorer => match self.tab {
                // Match current selected tab
                FileExplorerTab::Local => self.handle_input_event_mode_explorer_tab_local(ev),
                FileExplorerTab::Remote => self.handle_input_event_mode_explorer_tab_remote(ev),
            },
            InputField::Logs => self.handle_input_event_mode_explorer_log(ev),
        }
    }

    /// ### handle_input_event_mode_explorer_tab_local
    ///
    /// Input event handler for explorer mode when localhost tab is selected
    pub(super) fn handle_input_event_mode_explorer_tab_local(&mut self, ev: &InputEvent) {
        // Match events
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Esc => {
                    // Handle quit event
                    // Create quit prompt dialog
                    self.input_mode = self.create_disconnect_popup();
                }
                KeyCode::Tab => self.switch_input_field(), // <TAB> switch tab
                KeyCode::Right => self.tab = FileExplorerTab::Remote, // <RIGHT> switch to right tab
                KeyCode::Up => {
                    // Move index up
                    if self.local.index > 0 {
                        self.local.index -= 1;
                    }
                }
                KeyCode::Down => {
                    // Move index down
                    if self.local.index + 1 < self.local.files.len() {
                        self.local.index += 1;
                    }
                }
                KeyCode::PageUp => {
                    // Move index up (fast)
                    if self.local.index > 8 {
                        self.local.index -= 8; // Decrease by `8` if possible
                    } else {
                        self.local.index = 0; // Set to 0 otherwise
                    }
                }
                KeyCode::PageDown => {
                    // Move index down (fast)
                    if self.local.index + 8 >= self.local.files.len() {
                        // If overflows, set to size
                        self.local.index = self.local.files.len() - 1;
                    } else {
                        self.local.index += 8; // Increase by `8`
                    }
                }
                KeyCode::Enter => {
                    // Match selected file
                    let local_files: Vec<FsEntry> = self.local.files.clone();
                    if let Some(entry) = local_files.get(self.local.index) {
                        // If directory, enter directory, otherwise check if symlink
                        match entry {
                            FsEntry::Directory(dir) => {
                                self.local_changedir(dir.abs_path.as_path(), true)
                            }
                            FsEntry::File(file) => {
                                // Check if symlink
                                if let Some(symlink_entry) = &file.symlink {
                                    // If symlink entry is a directory, go to directory
                                    if let FsEntry::Directory(dir) = &**symlink_entry {
                                        self.local_changedir(dir.abs_path.as_path(), true)
                                    }
                                }
                            }
                        }
                    }
                }
                KeyCode::Backspace => {
                    // Go to previous directory
                    if let Some(d) = self.local.popd() {
                        self.local_changedir(d.as_path(), false);
                    }
                }
                KeyCode::Delete => {
                    // Get file at index
                    if let Some(entry) = self.local.files.get(self.local.index) {
                        // Get file name
                        let file_name: String = match entry {
                            FsEntry::Directory(dir) => dir.name.clone(),
                            FsEntry::File(file) => file.name.clone(),
                        };
                        // Show delete prompt
                        self.input_mode = InputMode::Popup(PopupType::YesNo(
                            format!("Delete file \"{}\"", file_name),
                            FileTransferActivity::callback_delete_fsentry,
                            FileTransferActivity::callback_nothing_to_do,
                        ))
                    }
                }
                KeyCode::Char(ch) => match ch {
                    'q' | 'Q' => {
                        // Create quit prompt dialog
                        self.input_mode = self.create_quit_popup();
                    }
                    'e' | 'E' => {
                        // Get file at index
                        if let Some(entry) = self.local.files.get(self.local.index) {
                            // Get file name
                            let file_name: String = match entry {
                                FsEntry::Directory(dir) => dir.name.clone(),
                                FsEntry::File(file) => file.name.clone(),
                            };
                            // Show delete prompt
                            self.input_mode = InputMode::Popup(PopupType::YesNo(
                                format!("Delete file \"{}\"", file_name),
                                FileTransferActivity::callback_delete_fsentry,
                                FileTransferActivity::callback_nothing_to_do,
                            ))
                        }
                    }
                    'g' | 'G' => {
                        // Goto
                        // Show input popup
                        self.input_mode = InputMode::Popup(PopupType::Input(
                            String::from("Change working directory"),
                            FileTransferActivity::callback_change_directory,
                        ));
                    }
                    'd' | 'D' => {
                        // Make directory
                        self.input_mode = InputMode::Popup(PopupType::Input(
                            String::from("Insert directory name"),
                            FileTransferActivity::callback_mkdir,
                        ));
                    }
                    'h' | 'H' => {
                        // Show help
                        self.input_mode = InputMode::Popup(PopupType::Help);
                    }
                    'i' | 'I' => {
                        // Show file info
                        self.input_mode = InputMode::Popup(PopupType::FileInfo);
                    }
                    'r' | 'R' => {
                        // Rename
                        self.input_mode = InputMode::Popup(PopupType::Input(
                            String::from("Insert new name"),
                            FileTransferActivity::callback_rename,
                        ));
                    }
                    's' | 'S' => {
                        // Save as...
                        // Ask for input
                        self.input_mode = InputMode::Popup(PopupType::Input(
                            String::from("Save as..."),
                            FileTransferActivity::callback_save_as,
                        ));
                    }
                    'u' | 'U' => {
                        // Go to parent directory
                        // Get pwd
                        let path: PathBuf = self.context.as_ref().unwrap().local.pwd();
                        if let Some(parent) = path.as_path().parent() {
                            self.local_changedir(parent, true);
                        }
                    }
                    ' ' => {
                        // Get pwd
                        let wrkdir: PathBuf = match self.client.pwd() {
                            Ok(p) => p,
                            Err(err) => {
                                self.log(
                                    LogLevel::Error,
                                    format!("Could not get current remote path: {}", err).as_ref(),
                                );
                                self.input_mode = InputMode::Popup(PopupType::Alert(
                                    Color::Red,
                                    format!("Could not get current remote path: {}", err),
                                ));
                                return;
                            }
                        };
                        // Get files
                        let files: Vec<FsEntry> = self.local.files.clone(); // Otherwise self is borrowed both as mutable and immutable...
                                                                            // Get file at index
                        if let Some(entry) = files.get(self.local.index) {
                            let name: String = entry.get_name();
                            // Call upload; pass realfile, keep link name
                            self.filetransfer_send(
                                &entry.get_realfile(),
                                wrkdir.as_path(),
                                Some(name),
                            );
                        }
                    }
                    _ => { /* Nothing to do */ }
                },
                _ => { /* Nothing to do */ }
            }
        }
    }

    /// ### handle_input_event_mode_explorer_tab_local
    ///
    /// Input event handler for explorer mode when remote tab is selected
    pub(super) fn handle_input_event_mode_explorer_tab_remote(&mut self, ev: &InputEvent) {
        // Match events
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Esc => {
                    // Handle quit event
                    // Create quit prompt dialog
                    self.input_mode = self.create_disconnect_popup();
                }
                KeyCode::Tab => self.switch_input_field(), // <TAB> switch tab
                KeyCode::Left => self.tab = FileExplorerTab::Local, // <LEFT> switch to local tab
                KeyCode::Up => {
                    // Move index up
                    if self.remote.index > 0 {
                        self.remote.index -= 1;
                    }
                }
                KeyCode::Down => {
                    // Move index down
                    if self.remote.index + 1 < self.remote.files.len() {
                        self.remote.index += 1;
                    }
                }
                KeyCode::PageUp => {
                    // Move index up (fast)
                    if self.remote.index > 8 {
                        self.remote.index -= 8; // Decrease by `8` if possible
                    } else {
                        self.remote.index = 0; // Set to 0 otherwise
                    }
                }
                KeyCode::PageDown => {
                    // Move index down (fast)
                    if self.remote.index + 8 >= self.remote.files.len() {
                        // If overflows, set to size
                        self.remote.index = self.remote.files.len() - 1;
                    } else {
                        self.remote.index += 8; // Increase by `8`
                    }
                }
                KeyCode::Enter => {
                    // Match selected file
                    let files: Vec<FsEntry> = self.remote.files.clone();
                    if let Some(entry) = files.get(self.remote.index) {
                        // If directory, enter directory; if file, check if is symlink
                        match entry {
                            FsEntry::Directory(dir) => {
                                self.remote_changedir(dir.abs_path.as_path(), true)
                            }
                            FsEntry::File(file) => {
                                // Check if symlink
                                if let Some(symlink_entry) = &file.symlink {
                                    // If symlink entry is a directory, go to directory
                                    if let FsEntry::Directory(dir) = &**symlink_entry {
                                        self.remote_changedir(dir.abs_path.as_path(), true)
                                    }
                                }
                            }
                        }
                    }
                }
                KeyCode::Backspace => {
                    // Go to previous directory
                    if let Some(d) = self.remote.popd() {
                        self.remote_changedir(d.as_path(), false);
                    }
                }
                KeyCode::Delete => {
                    // Get file at index
                    if let Some(entry) = self.remote.files.get(self.remote.index) {
                        // Get file name
                        let file_name: String = match entry {
                            FsEntry::Directory(dir) => dir.name.clone(),
                            FsEntry::File(file) => file.name.clone(),
                        };
                        // Show delete prompt
                        self.input_mode = InputMode::Popup(PopupType::YesNo(
                            format!("Delete file \"{}\"", file_name),
                            FileTransferActivity::callback_delete_fsentry,
                            FileTransferActivity::callback_nothing_to_do,
                        ))
                    }
                }
                KeyCode::Char(ch) => match ch {
                    'q' | 'Q' => {
                        // Create quit prompt dialog
                        self.input_mode = self.create_quit_popup();
                    }
                    'e' | 'E' => {
                        // Get file at index
                        if let Some(entry) = self.remote.files.get(self.remote.index) {
                            // Get file name
                            let file_name: String = match entry {
                                FsEntry::Directory(dir) => dir.name.clone(),
                                FsEntry::File(file) => file.name.clone(),
                            };
                            // Show delete prompt
                            self.input_mode = InputMode::Popup(PopupType::YesNo(
                                format!("Delete file \"{}\"", file_name),
                                FileTransferActivity::callback_delete_fsentry,
                                FileTransferActivity::callback_nothing_to_do,
                            ))
                        }
                    }
                    'g' | 'G' => {
                        // Goto
                        // Show input popup
                        self.input_mode = InputMode::Popup(PopupType::Input(
                            String::from("Change working directory"),
                            FileTransferActivity::callback_change_directory,
                        ));
                    }
                    'd' | 'D' => {
                        // Make directory
                        self.input_mode = InputMode::Popup(PopupType::Input(
                            String::from("Insert directory name"),
                            FileTransferActivity::callback_mkdir,
                        ));
                    }
                    'h' | 'H' => {
                        // Show help
                        self.input_mode = InputMode::Popup(PopupType::Help);
                    }
                    'i' | 'I' => {
                        // Show file info
                        self.input_mode = InputMode::Popup(PopupType::FileInfo);
                    }
                    'r' | 'R' => {
                        // Rename
                        self.input_mode = InputMode::Popup(PopupType::Input(
                            String::from("Insert new name"),
                            FileTransferActivity::callback_rename,
                        ));
                    }
                    's' | 'S' => {
                        // Save as...
                        // Ask for input
                        self.input_mode = InputMode::Popup(PopupType::Input(
                            String::from("Save as..."),
                            FileTransferActivity::callback_save_as,
                        ));
                    }
                    'u' | 'U' => {
                        // Go to parent directory
                        // Get pwd
                        match self.client.pwd() {
                            Ok(path) => {
                                if let Some(parent) = path.as_path().parent() {
                                    self.remote_changedir(parent, true);
                                }
                            }
                            Err(err) => {
                                self.input_mode = InputMode::Popup(PopupType::Alert(
                                    Color::Red,
                                    format!("Could not change working directory: {}", err),
                                ))
                            }
                        }
                    }
                    ' ' => {
                        // Get files
                        let files: Vec<FsEntry> = self.remote.files.clone(); // Otherwise self is borrowed both as mutable and immutable...
                                                                             // Get file at index
                        if let Some(entry) = files.get(self.remote.index) {
                            // Preserve name
                            let name: String = entry.get_name();
                            // Call upload (use entry realfile; pass previous name)
                            self.filetransfer_recv(
                                &entry.get_realfile(),
                                self.context.as_ref().unwrap().local.pwd().as_path(),
                                Some(name),
                            );
                        }
                    }
                    _ => { /* Nothing to do */ }
                },
                _ => { /* Nothing to do */ }
            }
        }
    }

    /// ### handle_input_event_mode_explorer_log
    ///
    /// Input even handler for explorer mode when log tab is selected
    pub(super) fn handle_input_event_mode_explorer_log(&mut self, ev: &InputEvent) {
        // Match event
        let records_block: usize = 16;
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Esc => {
                    // Handle quit event
                    // Create quit prompt dialog
                    self.input_mode = self.create_disconnect_popup();
                }
                KeyCode::Tab => self.switch_input_field(), // <TAB> switch tab
                KeyCode::Down => {
                    // NOTE: Twisted logic
                    // Decrease log index
                    if self.log_index > 0 {
                        self.log_index -= 1;
                    }
                }
                KeyCode::Up => {
                    // NOTE: Twisted logic
                    // Increase log index
                    if self.log_index + 1 < self.log_records.len() {
                        self.log_index += 1;
                    }
                }
                KeyCode::PageDown => {
                    // NOTE: Twisted logic
                    // Fast decreasing of log index
                    if self.log_index >= records_block {
                        self.log_index -= records_block; // Decrease by `records_block` if possible
                    } else {
                        self.log_index = 0; // Set to 0 otherwise
                    }
                }
                KeyCode::PageUp => {
                    // NOTE: Twisted logic
                    // Fast increasing of log index
                    if self.log_index + records_block >= self.log_records.len() {
                        // If overflows, set to size
                        self.log_index = self.log_records.len() - 1;
                    } else {
                        self.log_index += records_block; // Increase by `records_block`
                    }
                }
                KeyCode::Char(ch) => match ch {
                    'q' | 'Q' => {
                        // Create quit prompt dialog
                        self.input_mode = self.create_quit_popup();
                    }
                    _ => { /* Nothing to do */ }
                },
                _ => { /* Nothing to do */ }
            }
        }
    }

    /// ### handle_input_event_mode_explorer
    ///
    /// Input event handler for popup mode. Handler is then based on Popup type
    pub(super) fn handle_input_event_mode_popup(&mut self, ev: &InputEvent, popup: PopupType) {
        match popup {
            PopupType::Alert(_, _) => self.handle_input_event_mode_popup_alert(ev),
            PopupType::FileInfo => self.handle_input_event_mode_popup_fileinfo(ev),
            PopupType::Help => self.handle_input_event_mode_popup_help(ev),
            PopupType::Fatal(_) => self.handle_input_event_mode_popup_fatal(ev),
            PopupType::Input(_, cb) => self.handle_input_event_mode_popup_input(ev, cb),
            PopupType::Progress(_) => self.handle_input_event_mode_popup_progress(ev),
            PopupType::Wait(_) => self.handle_input_event_mode_popup_wait(ev),
            PopupType::YesNo(_, yes_cb, no_cb) => {
                self.handle_input_event_mode_popup_yesno(ev, yes_cb, no_cb)
            }
        }
    }

    /// ### handle_input_event_mode_popup_alert
    ///
    /// Input event handler for popup alert
    pub(super) fn handle_input_event_mode_popup_alert(&mut self, ev: &InputEvent) {
        // If enter, close popup
        if let InputEvent::Key(key) = ev {
            if let KeyCode::Enter = key.code {
                // Set input mode back to explorer
                self.input_mode = InputMode::Explorer;
            }
        }
    }

    /// ### handle_input_event_mode_popup_fileinfo
    ///
    /// Input event handler for popup fileinfo
    pub(super) fn handle_input_event_mode_popup_fileinfo(&mut self, ev: &InputEvent) {
        // If enter, close popup
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => {
                    // Set input mode back to explorer
                    self.input_mode = InputMode::Explorer;
                }
                _ => { /* Nothing to do */ }
            }
        }
    }

    /// ### handle_input_event_mode_popup_help
    ///
    /// Input event handler for popup help
    pub(super) fn handle_input_event_mode_popup_help(&mut self, ev: &InputEvent) {
        // If enter, close popup
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => {
                    // Set input mode back to explorer
                    self.input_mode = InputMode::Explorer;
                }
                _ => { /* Nothing to do */ }
            }
        }
    }

    /// ### handle_input_event_mode_popup_fatal
    ///
    /// Input event handler for popup alert
    pub(super) fn handle_input_event_mode_popup_fatal(&mut self, ev: &InputEvent) {
        // If enter, close popup
        if let InputEvent::Key(key) = ev {
            if let KeyCode::Enter = key.code {
                // Set quit to true; since a fatal error happened
                self.disconnect();
            }
        }
    }

    /// ### handle_input_event_mode_popup_input
    ///
    /// Input event handler for input popup
    pub(super) fn handle_input_event_mode_popup_input(
        &mut self,
        ev: &InputEvent,
        cb: OnInputSubmitCallback,
    ) {
        // If enter, close popup, otherwise push chars to input
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Esc => {
                    // Abort input
                    // Clear current input text
                    self.input_txt.clear();
                    // Set mode back to explorer
                    self.input_mode = InputMode::Explorer;
                }
                KeyCode::Enter => {
                    // Submit
                    let input_text: String = self.input_txt.clone();
                    // Clear current input text
                    self.input_txt.clear();
                    // Set mode back to explorer BEFORE CALLBACKS!!! Callback can then overwrite this, clever uh?
                    self.input_mode = InputMode::Explorer;
                    // Call cb
                    cb(self, input_text);
                }
                KeyCode::Char(ch) => self.input_txt.push(ch),
                KeyCode::Backspace => {
                    let _ = self.input_txt.pop();
                }
                _ => { /* Nothing to do */ }
            }
        }
    }

    /// ### handle_input_event_mode_explorer_alert
    ///
    /// Input event handler for popup alert
    pub(super) fn handle_input_event_mode_popup_progress(&mut self, _ev: &InputEvent) {
        // There's nothing you can do here I guess... maybe ctrl+c in the future idk
    }

    /// ### handle_input_event_mode_explorer_alert
    ///
    /// Input event handler for popup alert
    pub(super) fn handle_input_event_mode_popup_wait(&mut self, _ev: &InputEvent) {
        // There's nothing you can do here I guess... maybe ctrl+c in the future idk
    }

    /// ### handle_input_event_mode_explorer_alert
    ///
    /// Input event handler for popup alert
    pub(super) fn handle_input_event_mode_popup_yesno(
        &mut self,
        ev: &InputEvent,
        yes_cb: DialogCallback,
        no_cb: DialogCallback,
    ) {
        // If enter, close popup, otherwise move dialog option
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Enter => {
                    // @! Set input mode to Explorer BEFORE CALLBACKS!!! Callback can then overwrite this, clever uh?
                    self.input_mode = InputMode::Explorer;
                    // Check if user selected yes or not
                    match self.choice_opt {
                        DialogYesNoOption::No => no_cb(self),
                        DialogYesNoOption::Yes => yes_cb(self),
                    }
                    // Reset choice option to yes
                    self.choice_opt = DialogYesNoOption::Yes;
                }
                KeyCode::Right => self.choice_opt = DialogYesNoOption::No, // Set to NO
                KeyCode::Left => self.choice_opt = DialogYesNoOption::Yes, // Set to YES
                _ => { /* Nothing to do */ }
            }
        }
    }
}
