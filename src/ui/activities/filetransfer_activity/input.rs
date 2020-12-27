//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

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

// Deps
extern crate tempfile;
// Local
use super::{
    DialogCallback, DialogYesNoOption, FileExplorerTab, FileTransferActivity, FsEntry, InputEvent,
    InputField, LogLevel, OnInputSubmitCallback, Popup,
};
use crate::fs::explorer::{FileExplorer, FileSorting};
// Ext
use crossterm::event::{KeyCode, KeyModifiers};
use std::path::PathBuf;

impl FileTransferActivity {
    /// ### read_input_event
    ///
    /// Read one event.
    /// Returns whether at least one event has been handled
    pub(super) fn read_input_event(&mut self) -> bool {
        if let Ok(event) = self.context.as_ref().unwrap().input_hnd.read_event() {
            // Iterate over input events
            if let Some(event) = event {
                // Handle event
                self.handle_input_event(&event);
                // Return true
                true
            } else {
                // No event
                false
            }
        } else {
            // Error
            false
        }
    }

    /// ### handle_input_event
    ///
    /// Handle input event based on current input mode
    fn handle_input_event(&mut self, ev: &InputEvent) {
        // NOTE: this is necessary due to this <https://github.com/rust-lang/rust/issues/59159>
        // NOTE: Do you want my opinion about that issue? It's a bs and doesn't make any sense.
        let popup: Option<Popup> = match &self.popup {
            Some(ptype) => Some(ptype.clone()),
            _ => None,
        };
        match &self.popup {
            None => self.handle_input_event_mode_explorer(ev),
            Some(_) => {
                if let Some(popup) = popup {
                    self.handle_input_event_mode_popup(ev, popup);
                }
            }
        }
    }

    /// ### handle_input_event_mode_explorer
    ///
    /// Input event handler for explorer mode
    fn handle_input_event_mode_explorer(&mut self, ev: &InputEvent) {
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
    fn handle_input_event_mode_explorer_tab_local(&mut self, ev: &InputEvent) {
        // Match events
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Esc => {
                    // Handle quit event
                    // Create quit prompt dialog
                    self.popup = self.create_disconnect_popup();
                }
                KeyCode::Tab => self.switch_input_field(), // <TAB> switch tab
                KeyCode::Right => self.tab = FileExplorerTab::Remote, // <RIGHT> switch to right tab
                KeyCode::Up => {
                    // Decrement index
                    self.local.decr_index();
                }
                KeyCode::Down => {
                    // Increment index
                    self.local.incr_index();
                }
                KeyCode::PageUp => {
                    // Decrement index by 8
                    self.local.decr_index_by(8);
                }
                KeyCode::PageDown => {
                    // Increment index by 8
                    self.local.incr_index_by(8);
                }
                KeyCode::Enter => {
                    // Match selected file
                    let mut entry: Option<FsEntry> = None;
                    if let Some(e) = self.local.get_current_file() {
                        entry = Some(e.clone());
                    }
                    if let Some(entry) = entry {
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
                    if let Some(entry) = self.local.get_current_file() {
                        // Get file name
                        let file_name: String = match entry {
                            FsEntry::Directory(dir) => dir.name.clone(),
                            FsEntry::File(file) => file.name.clone(),
                        };
                        // Show delete prompt
                        self.popup = Some(Popup::YesNo(
                            format!("Delete file \"{}\"", file_name),
                            FileTransferActivity::callback_delete_fsentry,
                            FileTransferActivity::callback_nothing_to_do,
                        ))
                    }
                }
                KeyCode::Char(ch) => match ch {
                    'a' | 'A' => {
                        // Toggle hidden files
                        self.local.toggle_hidden_files();
                    }
                    'b' | 'B' => {
                        // Choose file sorting type
                        self.popup = Some(Popup::FileSortingDialog);
                    }
                    'c' | 'C' => {
                        // Copy
                        self.popup = Some(Popup::Input(
                            String::from("Insert destination name"),
                            FileTransferActivity::callback_copy,
                        ));
                    }
                    'd' | 'D' => {
                        // Make directory
                        self.popup = Some(Popup::Input(
                            String::from("Insert directory name"),
                            FileTransferActivity::callback_mkdir,
                        ));
                    }
                    'e' | 'E' => {
                        // Get file at index
                        if let Some(entry) = self.local.get_current_file() {
                            // Get file name
                            let file_name: String = match entry {
                                FsEntry::Directory(dir) => dir.name.clone(),
                                FsEntry::File(file) => file.name.clone(),
                            };
                            // Show delete prompt
                            self.popup = Some(Popup::YesNo(
                                format!("Delete file \"{}\"", file_name),
                                FileTransferActivity::callback_delete_fsentry,
                                FileTransferActivity::callback_nothing_to_do,
                            ))
                        }
                    }
                    'g' | 'G' => {
                        // Goto
                        // Show input popup
                        self.popup = Some(Popup::Input(
                            String::from("Change working directory"),
                            FileTransferActivity::callback_change_directory,
                        ));
                    }
                    'h' | 'H' => {
                        // Show help
                        self.popup = Some(Popup::Help);
                    }
                    'i' | 'I' => {
                        // Show file info
                        self.popup = Some(Popup::FileInfo);
                    }
                    'l' | 'L' => {
                        // Reload file entries
                        let pwd: PathBuf = self.local.wrkdir.clone();
                        self.local_scan(pwd.as_path());
                    }
                    'n' | 'N' => {
                        // New file
                        self.popup = Some(Popup::Input(
                            String::from("New file"),
                            Self::callback_new_file,
                        ));
                    }
                    'o' | 'O' => {
                        // Edit local file
                        if self.local.get_current_file().is_some() {
                            // Clone entry due to mutable stuff...
                            let fsentry: FsEntry = self.local.get_current_file().unwrap().clone();
                            // Check if file
                            if fsentry.is_file() {
                                self.log(
                                    LogLevel::Info,
                                    format!(
                                        "Opening file \"{}\"...",
                                        fsentry.get_abs_path().display()
                                    )
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
                    }
                    'q' | 'Q' => {
                        // Create quit prompt dialog
                        self.popup = self.create_quit_popup();
                    }
                    'r' | 'R' => {
                        // Rename
                        self.popup = Some(Popup::Input(
                            String::from("Insert new name"),
                            FileTransferActivity::callback_rename,
                        ));
                    }
                    's' | 'S' => {
                        // Save as...
                        // Ask for input
                        self.popup = Some(Popup::Input(
                            String::from("Save as..."),
                            FileTransferActivity::callback_save_as,
                        ));
                    }
                    'u' | 'U' => {
                        // Go to parent directory
                        // Get pwd
                        let path: PathBuf = self.local.wrkdir.clone();
                        if let Some(parent) = path.as_path().parent() {
                            self.local_changedir(parent, true);
                        }
                    }
                    ' ' => {
                        // Get pwd
                        let wrkdir: PathBuf = self.remote.wrkdir.clone();
                        // Get file and clone (due to mutable / immutable stuff...)
                        if self.local.get_current_file().is_some() {
                            let file: FsEntry = self.local.get_current_file().unwrap().clone();
                            let name: String = file.get_name().to_string();
                            // Call upload; pass realfile, keep link name
                            self.filetransfer_send(
                                &file.get_realfile(),
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
    fn handle_input_event_mode_explorer_tab_remote(&mut self, ev: &InputEvent) {
        // Match events
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Esc => {
                    // Handle quit event
                    // Create quit prompt dialog
                    self.popup = self.create_disconnect_popup();
                }
                KeyCode::Tab => self.switch_input_field(), // <TAB> switch tab
                KeyCode::Left => self.tab = FileExplorerTab::Local, // <LEFT> switch to local tab
                KeyCode::Up => {
                    // Decrement index
                    self.remote.decr_index();
                }
                KeyCode::Down => {
                    // Increment index
                    self.remote.incr_index();
                }
                KeyCode::PageUp => {
                    // Decrement index by 8
                    self.remote.decr_index_by(8);
                }
                KeyCode::PageDown => {
                    // Increment index by 8
                    self.remote.incr_index_by(8);
                }
                KeyCode::Enter => {
                    // Match selected file
                    let mut entry: Option<FsEntry> = None;
                    if let Some(e) = self.remote.get_current_file() {
                        entry = Some(e.clone());
                    }
                    if let Some(entry) = entry {
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
                    if let Some(entry) = self.remote.get_current_file() {
                        // Get file name
                        let file_name: String = match entry {
                            FsEntry::Directory(dir) => dir.name.clone(),
                            FsEntry::File(file) => file.name.clone(),
                        };
                        // Show delete prompt
                        self.popup = Some(Popup::YesNo(
                            format!("Delete file \"{}\"", file_name),
                            FileTransferActivity::callback_delete_fsentry,
                            FileTransferActivity::callback_nothing_to_do,
                        ))
                    }
                }
                KeyCode::Char(ch) => match ch {
                    'a' | 'A' => {
                        // Toggle hidden files
                        self.remote.toggle_hidden_files();
                    }
                    'b' | 'B' => {
                        // Choose file sorting type
                        self.popup = Some(Popup::FileSortingDialog);
                    }
                    'c' | 'C' => {
                        // Copy
                        self.popup = Some(Popup::Input(
                            String::from("Insert destination name"),
                            FileTransferActivity::callback_copy,
                        ));
                    }
                    'd' | 'D' => {
                        // Make directory
                        self.popup = Some(Popup::Input(
                            String::from("Insert directory name"),
                            FileTransferActivity::callback_mkdir,
                        ));
                    }
                    'e' | 'E' => {
                        // Get file at index
                        if let Some(entry) = self.remote.get_current_file() {
                            // Get file name
                            let file_name: String = match entry {
                                FsEntry::Directory(dir) => dir.name.clone(),
                                FsEntry::File(file) => file.name.clone(),
                            };
                            // Show delete prompt
                            self.popup = Some(Popup::YesNo(
                                format!("Delete file \"{}\"", file_name),
                                FileTransferActivity::callback_delete_fsentry,
                                FileTransferActivity::callback_nothing_to_do,
                            ))
                        }
                    }
                    'g' | 'G' => {
                        // Goto
                        // Show input popup
                        self.popup = Some(Popup::Input(
                            String::from("Change working directory"),
                            FileTransferActivity::callback_change_directory,
                        ));
                    }
                    'h' | 'H' => {
                        // Show help
                        self.popup = Some(Popup::Help);
                    }
                    'i' | 'I' => {
                        // Show file info
                        self.popup = Some(Popup::FileInfo);
                    }
                    'l' | 'L' => {
                        // Reload file entries
                        self.reload_remote_dir();
                    }
                    'n' | 'N' => {
                        // New file
                        self.popup = Some(Popup::Input(
                            String::from("New file"),
                            Self::callback_new_file,
                        ));
                    }
                    'o' | 'O' => {
                        // Edit remote file
                        if self.remote.get_current_file().is_some() {
                            // Clone entry due to mutable stuff...
                            let fsentry: FsEntry = self.remote.get_current_file().unwrap().clone();
                            // Check if file
                            if let FsEntry::File(file) = fsentry {
                                self.log(
                                    LogLevel::Info,
                                    format!("Opening file \"{}\"...", file.abs_path.display())
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
                                // Put input mode back to normal
                                self.popup = None;
                            }
                        }
                    }
                    'q' | 'Q' => {
                        // Create quit prompt dialog
                        self.popup = self.create_quit_popup();
                    }
                    'r' | 'R' => {
                        // Rename
                        self.popup = Some(Popup::Input(
                            String::from("Insert new name"),
                            FileTransferActivity::callback_rename,
                        ));
                    }
                    's' | 'S' => {
                        // Save as...
                        // Ask for input
                        self.popup = Some(Popup::Input(
                            String::from("Save as..."),
                            FileTransferActivity::callback_save_as,
                        ));
                    }
                    'u' | 'U' => {
                        // Get pwd
                        let path: PathBuf = self.remote.wrkdir.clone();
                        // Go to parent directory
                        if let Some(parent) = path.as_path().parent() {
                            self.remote_changedir(parent, true);
                        }
                    }
                    ' ' => {
                        // Get file and clone (due to mutable / immutable stuff...)
                        if self.remote.get_current_file().is_some() {
                            let file: FsEntry = self.remote.get_current_file().unwrap().clone();
                            let name: String = file.get_name().to_string();
                            // Call upload; pass realfile, keep link name
                            let wrkdir: PathBuf = self.local.wrkdir.clone();
                            self.filetransfer_recv(
                                &file.get_realfile(),
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

    /// ### handle_input_event_mode_explorer_log
    ///
    /// Input even handler for explorer mode when log tab is selected
    fn handle_input_event_mode_explorer_log(&mut self, ev: &InputEvent) {
        // Match event
        let records_block: usize = 16;
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Esc => {
                    // Handle quit event
                    // Create quit prompt dialog
                    self.popup = self.create_disconnect_popup();
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
                        self.popup = self.create_quit_popup();
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
    fn handle_input_event_mode_popup(&mut self, ev: &InputEvent, popup: Popup) {
        match popup {
            Popup::Alert(_, _) => self.handle_input_event_mode_popup_alert(ev),
            Popup::FileInfo => self.handle_input_event_mode_popup_fileinfo(ev),
            Popup::Fatal(_) => self.handle_input_event_mode_popup_fatal(ev),
            Popup::FileSortingDialog => self.handle_input_event_mode_popup_file_sorting(ev),
            Popup::Help => self.handle_input_event_mode_popup_help(ev),
            Popup::Input(_, cb) => self.handle_input_event_mode_popup_input(ev, cb),
            Popup::Progress(_) => self.handle_input_event_mode_popup_progress(ev),
            Popup::Wait(_) => self.handle_input_event_mode_popup_wait(ev),
            Popup::YesNo(_, yes_cb, no_cb) => {
                self.handle_input_event_mode_popup_yesno(ev, yes_cb, no_cb)
            }
        }
    }

    /// ### handle_input_event_mode_popup_alert
    ///
    /// Input event handler for popup alert
    fn handle_input_event_mode_popup_alert(&mut self, ev: &InputEvent) {
        // If enter, close popup
        if let InputEvent::Key(key) = ev {
            if matches!(key.code, KeyCode::Esc | KeyCode::Enter) {
                // Set input mode back to explorer
                self.popup = None;
            }
        }
    }

    /// ### handle_input_event_mode_popup_fileinfo
    ///
    /// Input event handler for popup fileinfo
    fn handle_input_event_mode_popup_fileinfo(&mut self, ev: &InputEvent) {
        // If enter, close popup
        if let InputEvent::Key(key) = ev {
            if matches!(key.code, KeyCode::Esc | KeyCode::Enter) {
                // Set input mode back to explorer
                self.popup = None;
            }
        }
    }

    /// ### handle_input_event_mode_popup_fatal
    ///
    /// Input event handler for popup alert
    fn handle_input_event_mode_popup_fatal(&mut self, ev: &InputEvent) {
        // If enter, close popup
        if let InputEvent::Key(key) = ev {
            if matches!(key.code, KeyCode::Esc | KeyCode::Enter) {
                // Set quit to true; since a fatal error happened
                self.disconnect();
            }
        }
    }

    /// ### handle_input_event_mode_popup_file_sorting
    ///
    /// Handle input event for file sorting dialog popup
    fn handle_input_event_mode_popup_file_sorting(&mut self, ev: &InputEvent) {
        // Match key code
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Esc | KeyCode::Enter => {
                    // Exit
                    self.popup = None;
                }
                KeyCode::Right => {
                    // Update sorting mode
                    match self.tab {
                        FileExplorerTab::Local => {
                            Self::move_sorting_mode_opt_right(&mut self.local);
                        }
                        FileExplorerTab::Remote => {
                            Self::move_sorting_mode_opt_right(&mut self.remote);
                        }
                    }
                }
                KeyCode::Left => {
                    // Update sorting mode
                    match self.tab {
                        FileExplorerTab::Local => {
                            Self::move_sorting_mode_opt_left(&mut self.local);
                        }
                        FileExplorerTab::Remote => {
                            Self::move_sorting_mode_opt_left(&mut self.remote);
                        }
                    }
                }
                _ => { /* Nothing to do */ }
            }
        }
    }

    /// ### handle_input_event_mode_popup_help
    ///
    /// Input event handler for popup help
    fn handle_input_event_mode_popup_help(&mut self, ev: &InputEvent) {
        // If enter, close popup
        if let InputEvent::Key(key) = ev {
            if matches!(key.code, KeyCode::Esc | KeyCode::Enter) {
                // Set input mode back to explorer
                self.popup = None;
            }
        }
    }

    /// ### handle_input_event_mode_popup_input
    ///
    /// Input event handler for input popup
    fn handle_input_event_mode_popup_input(&mut self, ev: &InputEvent, cb: OnInputSubmitCallback) {
        // If enter, close popup, otherwise push chars to input
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Esc => {
                    // Abort input
                    // Clear current input text
                    self.input_txt.clear();
                    // Set mode back to explorer
                    self.popup = None;
                }
                KeyCode::Enter => {
                    // Submit
                    let input_text: String = self.input_txt.clone();
                    // Clear current input text
                    self.input_txt.clear();
                    // Set mode back to explorer BEFORE CALLBACKS!!! Callback can then overwrite this, clever uh?
                    self.popup = None;
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

    /// ### handle_input_event_mode_popup_progress
    ///
    /// Input event handler for popup alert
    fn handle_input_event_mode_popup_progress(&mut self, ev: &InputEvent) {
        if let InputEvent::Key(key) = ev {
            if let KeyCode::Char(ch) = key.code {
                // If is 'C' and CTRL
                if matches!(ch, 'c' | 'C') && key.modifiers.intersects(KeyModifiers::CONTROL) {
                    // Abort transfer
                    self.transfer.aborted = true;
                }
            }
        }
    }

    /// ### handle_input_event_mode_popup_wait
    ///
    /// Input event handler for popup alert
    fn handle_input_event_mode_popup_wait(&mut self, _ev: &InputEvent) {
        // There's nothing you can do here I guess... maybe ctrl+c in the future idk
    }

    /// ### handle_input_event_mode_popup_yesno
    ///
    /// Input event handler for popup alert
    fn handle_input_event_mode_popup_yesno(
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
                    self.popup = None;
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

    /// ### move_sorting_mode_opt_left
    ///
    /// Perform <LEFT> on file sorting dialog
    fn move_sorting_mode_opt_left(explorer: &mut FileExplorer) {
        let curr_sorting: FileSorting = explorer.get_file_sorting();
        explorer.sort_by(match curr_sorting {
            FileSorting::BySize => FileSorting::ByCreationTime,
            FileSorting::ByCreationTime => FileSorting::ByModifyTime,
            FileSorting::ByModifyTime => FileSorting::ByName,
            FileSorting::ByName => FileSorting::BySize, // Wrap
        });
    }

    /// ### move_sorting_mode_opt_left
    ///
    /// Perform <RIGHT> on file sorting dialog
    fn move_sorting_mode_opt_right(explorer: &mut FileExplorer) {
        let curr_sorting: FileSorting = explorer.get_file_sorting();
        explorer.sort_by(match curr_sorting {
            FileSorting::ByName => FileSorting::ByModifyTime,
            FileSorting::ByModifyTime => FileSorting::ByCreationTime,
            FileSorting::ByCreationTime => FileSorting::BySize,
            FileSorting::BySize => FileSorting::ByName, // Wrap
        });
    }
}
