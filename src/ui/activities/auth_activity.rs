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

// Dependencies
extern crate crossterm;
extern crate tui;

// locals
use super::{Activity, Context};

// Includes
use crossterm::event::Event as InputEvent;
use crossterm::event::{KeyCode, KeyEvent};
use tui::layout;
use tui::style;
use tui::widgets;

/// ### InputField
///
/// InputField describes the current input field to edit
#[derive(std::cmp::PartialEq)]
enum InputField {
    Address,
    Port,
    Protocol,
    Username,
    Password,
}

/// ### ScpProtocol
///
/// ScpProtocol describes the communication protocol selected by the user to communicate with the remote
pub enum ScpProtocol {
    Sftp,
    Ftp,
}

/// ### AuthActivity
///
/// AuthActivity is the data holder for the authentication activity
pub struct AuthActivity {
    pub address: String,
    pub port: String,
    pub protocol: ScpProtocol,
    pub username: String,
    pub password: String,
    pub form_submit: bool, // becomes true after user has submitted fields
    pub esc_called: bool, // Becomes true if user has pressed esc
    selected_field: InputField,
}

impl AuthActivity {
    /// ### new
    ///
    /// Instantiates a new AuthActivity
    pub fn new() -> AuthActivity {
        AuthActivity {
            address: String::new(),
            port: String::from("22"),
            protocol: ScpProtocol::Sftp,
            username: String::new(),
            password: String::new(),
            form_submit: false,
            esc_called: false,
            selected_field: InputField::Address,
        }
    }
}

impl Activity for AuthActivity {
    /// ### on_create
    ///
    /// `on_create` is the function which must be called to initialize the activity.
    /// `on_create` must initialize all the data structures used by the activity
    fn on_create(&mut self, context: &mut Context) {
        // Mhm, nothing to do here I guess...
    }

    /// ### on_draw
    ///
    /// `on_draw` is the function which draws the graphical interface.
    /// This function must be called at each tick to refresh the interface
    fn on_draw(&mut self, context: &mut Context) {
        // Start catching Input Events
        let mut popup: Option<String> = None;
        if let Ok(input_events) = context.input_hnd.fetch_events() {
            // Iterate over input events
            for event in input_events.iter() {
                match event {
                    InputEvent::Key(key) => {
                        match key.code {
                            KeyCode::Esc => {
                                self.esc_called = true;
                                break
                            },
                            KeyCode::Enter => {
                                // TODO: handle submit (check form)
                            },
                            KeyCode::Backspace => {
                                // Pop last char
                                match self.selected_field {
                                    InputField::Address => {
                                        let _ = self.address.pop();
                                    },
                                    InputField::Password => {
                                        let _ = self.password.pop();
                                    },
                                    InputField::Username => {
                                        let _ = self.username.pop();
                                    },
                                    InputField::Port => {
                                        let _ = self.port.pop();
                                    },
                                    _ => { /* Nothing to do */ }
                                };
                            },
                            KeyCode::Up => {
                                // Move item up
                                self.selected_field = match self.selected_field {
                                    InputField::Address => InputField::Address, // End of list
                                    InputField::Port => InputField::Address,
                                    InputField::Protocol => InputField::Port,
                                    InputField::Username => InputField::Protocol,
                                    InputField::Password => InputField::Username,
                                }
                            },
                            KeyCode::Down => {
                                // Move item down
                                self.selected_field = match self.selected_field {
                                    InputField::Address => InputField::Port,
                                    InputField::Port => InputField::Protocol,
                                    InputField::Protocol => InputField::Username,
                                    InputField::Username => InputField::Password,
                                    InputField::Password => InputField::Password, // End of list
                                }
                            },
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
                                    },
                                    _ => { /* Nothing to do */ }
                                }
                            },
                            KeyCode::Left => {
                                // If current field is Protocol handle event... (move element left)
                                if self.selected_field == InputField::Protocol {
                                    self.protocol = match self.protocol {
                                        ScpProtocol::Sftp => ScpProtocol::Sftp,
                                        ScpProtocol::Ftp => ScpProtocol::Sftp, // End of list
                                    }
                                }
                            },
                            KeyCode::Right => {
                                // If current field is Protocol handle event... ( move element right )
                                if self.selected_field == InputField::Protocol {
                                    self.protocol = match self.protocol {
                                        ScpProtocol::Sftp => ScpProtocol::Ftp,
                                        ScpProtocol::Ftp => ScpProtocol::Ftp, // End of list
                                    }
                                }
                            },
                            _ => { /* Nothing to do */ }
                        }
                    }
                    _ => { /* Nothing to do */ }
                }
            }
        }
        // TODO: draw interface
    }

    /// ### on_destroy
    ///
    /// `on_destroy` is the function which cleans up runtime variables and data before terminating the activity.
    /// This function must be called once before terminating the activity.
    fn on_destroy(&mut self, context: &mut Context) {
        // Mhm, nothing to do here I guess...
    }
}
