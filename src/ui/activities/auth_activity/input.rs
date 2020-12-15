//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

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
    AuthActivity, DialogCallback, DialogYesNoOption, FileTransferProtocol, InputEvent, InputField, InputForm, InputMode, OnInputSubmitCallback, PopupType,
};

use crossterm::event::KeyCode;
use tui::style::Color;

impl AuthActivity {
    /// ### handle_input_event
    ///
    /// Handle input event, based on current input mode
    pub(super) fn handle_input_event(&mut self, ev: &InputEvent) {
        let popup: Option<PopupType> = match &self.input_mode {
            InputMode::Popup(ptype) => Some(ptype.clone()),
            _ => None,
        };
        match self.input_mode {
            InputMode::Form => self.handle_input_event_mode_form(ev),
            InputMode::Popup(_) => {
                if let Some(ptype) = popup {
                    self.handle_input_event_mode_popup(ev, ptype)
                }
            }
        }
    }

    /// ### handle_input_event_mode_form
    ///
    /// Handler for input event when in form mode
    pub(super) fn handle_input_event_mode_form(&mut self, ev: &InputEvent) {
        match self.input_form {
            InputForm::AuthCredentials => self.handle_input_event_mode_form_auth(ev),
        }
    }

    /// ### handle_input_event_mode_form_auth
    ///
    /// Handle input event when input mode is Form and Tab is Auth
    pub(super) fn handle_input_event_mode_form_auth(&mut self, ev: &InputEvent) {
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Esc => {
                    self.quit = true;
                }
                KeyCode::Enter => {
                    // Handle submit
                    // Check form
                    // Check address
                    if self.address.is_empty() {
                        self.input_mode = InputMode::Popup(PopupType::Alert(
                            Color::Red,
                            String::from("Invalid address"),
                        ));
                        return;
                    }
                    // Check port
                    // Convert port to number
                    match self.port.parse::<usize>() {
                        Ok(val) => {
                            if val > 65535 {
                                self.input_mode = InputMode::Popup(PopupType::Alert(
                                    Color::Red,
                                    String::from("Specified port must be in range 0-65535"),
                                ));
                                return;
                            }
                        }
                        Err(_) => {
                            self.input_mode = InputMode::Popup(PopupType::Alert(
                                Color::Red,
                                String::from("Specified port is not a number"),
                            ));
                            return;
                        }
                    }
                    // Save recent
                    self.save_recent();
                    // Everything OK, set enter
                    self.submit = true;
                }
                KeyCode::Backspace => {
                    // Pop last char
                    match self.selected_field {
                        InputField::Address => {
                            let _ = self.address.pop();
                        }
                        InputField::Password => {
                            let _ = self.password.pop();
                        }
                        InputField::Username => {
                            let _ = self.username.pop();
                        }
                        InputField::Port => {
                            let _ = self.port.pop();
                        }
                        _ => { /* Nothing to do */ }
                    };
                }
                KeyCode::Up => {
                    // Move item up
                    self.selected_field = match self.selected_field {
                        InputField::Address => InputField::Password, // End of list (wrap)
                        InputField::Port => InputField::Address,
                        InputField::Protocol => InputField::Port,
                        InputField::Username => InputField::Protocol,
                        InputField::Password => InputField::Username,
                    }
                }
                KeyCode::Down | KeyCode::Tab => {
                    // Move item down
                    self.selected_field = match self.selected_field {
                        InputField::Address => InputField::Port,
                        InputField::Port => InputField::Protocol,
                        InputField::Protocol => InputField::Username,
                        InputField::Username => InputField::Password,
                        InputField::Password => InputField::Address, // End of list (wrap)
                    }
                }
                KeyCode::Char(ch) => {
                    match self.selected_field {
                        InputField::Address => self.address.push(ch),
                        InputField::Password => self.password.push(ch),
                        InputField::Username => self.username.push(ch),
                        InputField::Port => {
                            // Value must be numeric
                            if ch.is_numeric() {
                                self.port.push(ch);
                            }
                        }
                        _ => { /* Nothing to do */ }
                    }
                }
                KeyCode::Left => {
                    // If current field is Protocol handle event... (move element left)
                    if self.selected_field == InputField::Protocol {
                        self.protocol = match self.protocol {
                            FileTransferProtocol::Sftp => FileTransferProtocol::Ftp(true), // End of list (wrap)
                            FileTransferProtocol::Scp => FileTransferProtocol::Sftp,
                            FileTransferProtocol::Ftp(ftps) => match ftps {
                                false => FileTransferProtocol::Scp,
                                true => FileTransferProtocol::Ftp(false),
                            },
                        };
                    }
                }
                KeyCode::Right => {
                    // If current field is Protocol handle event... ( move element right )
                    if self.selected_field == InputField::Protocol {
                        self.protocol = match self.protocol {
                            FileTransferProtocol::Sftp => FileTransferProtocol::Scp,
                            FileTransferProtocol::Scp => FileTransferProtocol::Ftp(false),
                            FileTransferProtocol::Ftp(ftps) => match ftps {
                                false => FileTransferProtocol::Ftp(true),
                                true => FileTransferProtocol::Sftp, // End of list (wrap)
                            },
                        };
                    }
                }
                _ => { /* Nothing to do */ }
            }
        }
    }

    /// ### handle_input_event_mode_text
    ///
    /// Handler for input event when in popup mode
    pub(super) fn handle_input_event_mode_popup(&mut self, ev: &InputEvent, ptype: PopupType) {
        match ptype {
            PopupType::Alert(_, _) => self.handle_input_event_mode_popup_alert(ev),
            PopupType::Input(_, cb) => self.handle_input_event_mode_popup_input(ev, cb),
            PopupType::YesNo(_, yes_cb, no_cb) => {
                self.handle_input_event_mode_popup_yesno(ev, yes_cb, no_cb)
            }
        }
    }

    /// ### handle_input_event_mode_popup_alert
    ///
    /// Handle input event when the input mode is popup, and popup type is alert
    pub(super) fn handle_input_event_mode_popup_alert(&mut self, ev: &InputEvent) {
        // Only enter should be allowed here
        if let InputEvent::Key(key) = ev {
            if let KeyCode::Enter = key.code {
                self.input_mode = InputMode::Form; // Hide popup
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
                    // Set mode back to form
                    self.input_mode = InputMode::Form;
                }
                KeyCode::Enter => {
                    // Submit
                    let input_text: String = self.input_txt.clone();
                    // Clear current input text
                    self.input_txt.clear();
                    // Set mode back to form BEFORE CALLBACKS!!! Callback can then overwrite this, clever uh?
                    self.input_mode = InputMode::Form;
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

    /// ### handle_input_event_mode_popup_yesno
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
                    // @! Set input mode to Form BEFORE CALLBACKS!!! Callback can then overwrite this, clever uh?
                    self.input_mode = InputMode::Form;
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
