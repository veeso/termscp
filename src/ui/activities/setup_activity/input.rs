//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

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

// Locals
use super::{
    InputEvent, OnChoiceCallback, Popup, QuitDialogOption, SetupActivity, SetupTab,
    UserInterfaceInputField, YesNoDialogOption,
};
use crate::filetransfer::FileTransferProtocol;
// Ext
use crossterm::event::{KeyCode, KeyModifiers};
use std::path::PathBuf;
use tui::style::Color;

impl SetupActivity {
    /// ### handle_input_event
    ///
    /// Handle input event, based on current input mode
    pub(super) fn handle_input_event(&mut self, ev: &InputEvent) {
        let popup: Option<Popup> = match &self.popup {
            Some(ptype) => Some(ptype.clone()),
            None => None,
        };
        match &self.popup {
            Some(_) => self.handle_input_event_popup(ev, popup.unwrap()),
            None => self.handle_input_event_forms(ev),
        }
    }

    /// ### handle_input_event_forms
    ///
    /// Handle input event when popup is not visible.
    /// InputEvent is handled based on current tab
    fn handle_input_event_forms(&mut self, ev: &InputEvent) {
        // Match tab
        match &self.tab {
            SetupTab::SshConfig => self.handle_input_event_forms_ssh_config(ev),
            SetupTab::UserInterface(_) => self.handle_input_event_forms_ui(ev),
        }
    }

    /// ### handle_input_event_forms_ssh_config
    ///
    /// Handle input event when in ssh config tab
    fn handle_input_event_forms_ssh_config(&mut self, ev: &InputEvent) {
        // Match input event
        if let InputEvent::Key(key) = ev {
            // Match key code
            match key.code {
                KeyCode::Esc => self.popup = Some(Popup::Quit), // Prompt quit
                KeyCode::Tab => {
                    self.tab = SetupTab::UserInterface(UserInterfaceInputField::DefaultProtocol)
                } // Switch tab to user interface config
                KeyCode::Up => {
                    if let Some(config_cli) = self.config_cli.as_ref() {
                        // Move ssh key index up
                        let ssh_key_size: usize = config_cli.iter_ssh_keys().count();
                        if self.ssh_key_idx > 0 {
                            // Decrement
                            self.ssh_key_idx -= 1;
                        } else {
                            // Set ssh key index to `ssh_key_size -1`
                            self.ssh_key_idx = ssh_key_size - 1;
                        }
                    }
                }
                KeyCode::Down => {
                    if let Some(config_cli) = self.config_cli.as_ref() {
                        // Move ssh key index down
                        let ssh_key_size: usize = config_cli.iter_ssh_keys().count();
                        if self.ssh_key_idx + 1 < ssh_key_size {
                            // Increment index
                            self.ssh_key_idx += 1;
                        } else {
                            // Wrap to 0
                            self.ssh_key_idx = 0;
                        }
                    }
                }
                KeyCode::Delete => {
                    // Prompt to delete selected key
                    self.yesno_opt = YesNoDialogOption::No; // Default to no
                    self.popup = Some(Popup::YesNo(
                        String::from("Delete key?"),
                        Self::callback_delete_ssh_key,
                        Self::callback_nothing_to_do,
                    ));
                }
                KeyCode::Enter => {
                    // Edit selected key
                    if let Err(err) = self.edit_ssh_key() {
                        self.popup = Some(Popup::Alert(Color::Red, err)); // Report error
                    }
                }
                KeyCode::Char(ch) => {
                    // Check if <CTRL> is enabled
                    if key.modifiers.intersects(KeyModifiers::CONTROL) {
                        // Match char
                        match ch {
                            'h' | 'H' => {
                                // Show help
                                self.popup = Some(Popup::Help);
                            }
                            'n' | 'N' => {
                                // New ssh key
                                self.popup = Some(Popup::NewSshKey);
                            }
                            'r' | 'R' => {
                                // Show reset changes dialog
                                self.popup = Some(Popup::YesNo(
                                    String::from("Reset changes?"),
                                    Self::callback_reset_config_changes,
                                    Self::callback_nothing_to_do,
                                ));
                            }
                            's' | 'S' => {
                                // Show save dialog
                                self.popup = Some(Popup::YesNo(
                                    String::from("Save changes to configuration?"),
                                    Self::callback_save_config,
                                    Self::callback_nothing_to_do,
                                ));
                            }
                            _ => { /* Nothing to do */ }
                        }
                    }
                }
                _ => { /* Nothing to do */ }
            }
        }
    }

    /// ### handle_input_event_forms_ui
    ///
    /// Handle input event when in UserInterface config tab
    fn handle_input_event_forms_ui(&mut self, ev: &InputEvent) {
        // Get `UserInterfaceInputField`
        let field: UserInterfaceInputField = match &self.tab {
            SetupTab::UserInterface(field) => field.clone(),
            _ => return,
        };
        // Match input event
        if let InputEvent::Key(key) = ev {
            // Match key code
            match key.code {
                KeyCode::Esc => self.popup = Some(Popup::Quit), // Prompt quit
                KeyCode::Tab => self.tab = SetupTab::SshConfig, // Switch tab to ssh config
                KeyCode::Backspace => {
                    // Pop character from selected input
                    if let Some(config_cli) = self.config_cli.as_mut() {
                        // NOTE: replace with match if other text fields are added
                        if matches!(field, UserInterfaceInputField::TextEditor) {
                            // Pop from text editor
                            let mut input: String = String::from(
                                config_cli.get_text_editor().as_path().to_string_lossy(),
                            );
                            input.pop();
                            // Update text editor value
                            config_cli.set_text_editor(PathBuf::from(input.as_str()));
                        }
                    }
                }
                KeyCode::Left => {
                    // Move left on fields which are tabs
                    if let Some(config_cli) = self.config_cli.as_mut() {
                        if matches!(field, UserInterfaceInputField::DefaultProtocol) {
                            // Move left
                            config_cli.set_default_protocol(
                                match config_cli.get_default_protocol() {
                                    FileTransferProtocol::Ftp(secure) => match secure {
                                        true => FileTransferProtocol::Ftp(false),
                                        false => FileTransferProtocol::Scp,
                                    },
                                    FileTransferProtocol::Scp => FileTransferProtocol::Sftp,
                                    FileTransferProtocol::Sftp => FileTransferProtocol::Ftp(true), // Wrap
                                },
                            );
                        }
                    }
                }
                KeyCode::Right => {
                    // Move right on fields which are tabs
                    if let Some(config_cli) = self.config_cli.as_mut() {
                        if matches!(field, UserInterfaceInputField::DefaultProtocol) {
                            // Move left
                            config_cli.set_default_protocol(
                                match config_cli.get_default_protocol() {
                                    FileTransferProtocol::Sftp => FileTransferProtocol::Scp,
                                    FileTransferProtocol::Scp => FileTransferProtocol::Ftp(false),
                                    FileTransferProtocol::Ftp(secure) => match secure {
                                        false => FileTransferProtocol::Ftp(true),
                                        true => FileTransferProtocol::Sftp, // Wrap
                                    },
                                },
                            );
                        }
                    }
                }
                KeyCode::Up => {
                    // Change selected field
                    self.tab = SetupTab::UserInterface(match field {
                        UserInterfaceInputField::TextEditor => {
                            UserInterfaceInputField::DefaultProtocol
                        }
                        UserInterfaceInputField::DefaultProtocol => {
                            UserInterfaceInputField::TextEditor
                        } // Wrap
                    });
                }
                KeyCode::Down => {
                    // Change selected field
                    self.tab = SetupTab::UserInterface(match field {
                        UserInterfaceInputField::DefaultProtocol => {
                            UserInterfaceInputField::TextEditor
                        }
                        UserInterfaceInputField::TextEditor => {
                            UserInterfaceInputField::DefaultProtocol
                        } // Wrap
                    });
                }
                KeyCode::Char(ch) => {
                    // Check if <CTRL> is enabled
                    if key.modifiers.intersects(KeyModifiers::CONTROL) {
                        // Match char
                        match ch {
                            'h' | 'H' => {
                                // Show help
                                self.popup = Some(Popup::Help);
                            }
                            'r' | 'R' => {
                                // Show reset changes dialog
                                self.popup = Some(Popup::YesNo(
                                    String::from("Reset changes?"),
                                    Self::callback_reset_config_changes,
                                    Self::callback_nothing_to_do,
                                ));
                            }
                            's' | 'S' => {
                                // Show save dialog
                                self.popup = Some(Popup::YesNo(
                                    String::from("Save changes to configuration?"),
                                    Self::callback_save_config,
                                    Self::callback_nothing_to_do,
                                ));
                            }
                            _ => { /* Nothing to do */ }
                        }
                    } else {
                        // Push character to input field
                        if let Some(config_cli) = self.config_cli.as_mut() {
                            // NOTE: change to match if other fields are added
                            if matches!(field, UserInterfaceInputField::TextEditor) {
                                // Get current text editor and push character
                                let mut input: String = String::from(
                                    config_cli.get_text_editor().as_path().to_string_lossy(),
                                );
                                input.push(ch);
                                // Update text editor value
                                config_cli.set_text_editor(PathBuf::from(input.as_str()));
                            }
                        }
                    }
                }
                _ => { /* Nothing to do */ }
            }
        }
    }

    /// ### handle_input_event_popup
    ///
    /// Handler for input event when popup is visible
    fn handle_input_event_popup(&mut self, ev: &InputEvent, ptype: Popup) {
        match ptype {
            Popup::Alert(_, _) => self.handle_input_event_mode_popup_alert(ev),
            Popup::Fatal(_) => self.handle_input_event_mode_popup_fatal(ev),
            Popup::Help => self.handle_input_event_mode_popup_help(ev),
            Popup::NewSshKey => self.handle_input_event_mode_popup_newsshkey(ev),
            Popup::Quit => self.handle_input_event_mode_popup_quit(ev),
            Popup::YesNo(_, yes_cb, no_cb) => {
                self.handle_input_event_mode_popup_yesno(ev, yes_cb, no_cb)
            }
        }
    }

    /// ### handle_input_event_mode_popup_alert
    ///
    /// Handle input event when the input mode is popup, and popup type is alert
    fn handle_input_event_mode_popup_alert(&mut self, ev: &InputEvent) {
        // Only enter should be allowed here
        if let InputEvent::Key(key) = ev {
            if matches!(key.code, KeyCode::Esc | KeyCode::Enter) {
                self.popup = None; // Hide popup
            }
        }
    }

    /// ### handle_input_event_mode_popup_fatal
    ///
    /// Handle input event when the input mode is popup, and popup type is fatal
    fn handle_input_event_mode_popup_fatal(&mut self, ev: &InputEvent) {
        // Only enter should be allowed here
        if let InputEvent::Key(key) = ev {
            if matches!(key.code, KeyCode::Esc | KeyCode::Enter) {
                // Quit after acknowelding fatal error
                self.quit = true;
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
                self.popup = None; // Hide popup
            }
        }
    }

    /// ### handle_input_event_mode_popup_newsshkey
    ///
    /// Handle input events for `Popup::NewSshKey`
    fn handle_input_event_mode_popup_newsshkey(&mut self, ev: &InputEvent) {
        // If enter, close popup, otherwise push chars to input
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Esc => {
                    // Abort input
                    // Clear buffer
                    self.clear_user_input();
                    // Hide popup
                    self.popup = None;
                }
                KeyCode::Enter => {
                    // Submit
                    let address: String = self.user_input.get(0).unwrap().to_string();
                    let username: String = self.user_input.get(1).unwrap().to_string();
                    // Clear buffer
                    self.clear_user_input();
                    // Close popup BEFORE CALLBACKS!!! Callback can then overwrite this, clever uh?
                    self.popup = None;
                    // Reset user ptr
                    self.user_input_ptr = 0;
                    // Call cb
                    self.callback_new_ssh_key(address, username);
                }
                KeyCode::Up => {
                    // Move ptr up, or to maximum index (1)
                    self.user_input_ptr = match self.user_input_ptr {
                        1 => 0,
                        _ => 1, // Wrap
                    };
                }
                KeyCode::Down => {
                    // Move ptr down, or to minimum index (0)
                    self.user_input_ptr = match self.user_input_ptr {
                        0 => 1,
                        _ => 0, // Wrap
                    }
                }
                KeyCode::Char(ch) => {
                    // Get current input
                    let input: &mut String = self.user_input.get_mut(self.user_input_ptr).unwrap();
                    input.push(ch);
                }
                KeyCode::Backspace => {
                    let input: &mut String = self.user_input.get_mut(self.user_input_ptr).unwrap();
                    input.pop();
                }
                _ => { /* Nothing to do */ }
            }
        }
    }

    /// ### handle_input_event_mode_popup_quit
    ///
    /// Handle input events for `Popup::Quit`
    fn handle_input_event_mode_popup_quit(&mut self, ev: &InputEvent) {
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Esc => {
                    // Hide popup
                    self.popup = None;
                }
                KeyCode::Enter => {
                    // Perform enter, based on current choice
                    match self.quit_opt {
                        QuitDialogOption::Cancel => self.popup = None, // Hide popup
                        QuitDialogOption::DontSave => self.quit = true, // Just quit
                        QuitDialogOption::Save => self.callback_save_config_and_quit(), // Save and quit
                    }
                    // Reset choice
                    self.quit_opt = QuitDialogOption::Save;
                }
                KeyCode::Right => {
                    // Change option
                    self.quit_opt = match self.quit_opt {
                        QuitDialogOption::Save => QuitDialogOption::DontSave,
                        QuitDialogOption::DontSave => QuitDialogOption::Cancel,
                        QuitDialogOption::Cancel => QuitDialogOption::Save, // Wrap
                    }
                }
                KeyCode::Left => {
                    // Change option
                    self.quit_opt = match self.quit_opt {
                        QuitDialogOption::Cancel => QuitDialogOption::DontSave,
                        QuitDialogOption::DontSave => QuitDialogOption::Save,
                        QuitDialogOption::Save => QuitDialogOption::Cancel, // Wrap
                    }
                }
                _ => { /* Nothing to do */ }
            }
        }
    }

    /// ### handle_input_event_mode_popup_yesno
    ///
    /// Input event handler for popup alert
    pub(super) fn handle_input_event_mode_popup_yesno(
        &mut self,
        ev: &InputEvent,
        yes_cb: OnChoiceCallback,
        no_cb: OnChoiceCallback,
    ) {
        // If enter, close popup, otherwise move dialog option
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Enter => {
                    // Hide popup BEFORE CALLBACKS!!! Callback can then overwrite this, clever uh?
                    self.popup = None;
                    // Check if user selected yes or not
                    match self.yesno_opt {
                        YesNoDialogOption::No => no_cb(self),
                        YesNoDialogOption::Yes => yes_cb(self),
                    }
                    // Reset choice option to yes
                    self.yesno_opt = YesNoDialogOption::Yes;
                }
                KeyCode::Right => self.yesno_opt = YesNoDialogOption::No, // Set to NO
                KeyCode::Left => self.yesno_opt = YesNoDialogOption::Yes, // Set to YES
                _ => { /* Nothing to do */ }
            }
        }
    }
}
