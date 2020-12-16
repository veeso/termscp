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

// Sub modules
mod bookmarks;
mod callbacks;
mod input;
mod layout;

// Dependencies
extern crate crossterm;
extern crate tui;
extern crate unicode_width;

// locals
use super::{Activity, Context};
use crate::filetransfer::FileTransferProtocol;
use crate::system::bookmarks_client::BookmarksClient;

// Includes
use crossterm::event::Event as InputEvent;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::style::Color;

// Types
type DialogCallback = fn(&mut AuthActivity);
type OnInputSubmitCallback = fn(&mut AuthActivity, String);

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

/// ### DialogYesNoOption
///
/// Current yes/no dialog option
#[derive(std::cmp::PartialEq, Clone)]
enum DialogYesNoOption {
    Yes,
    No,
}

/// ### PopupType
///
/// PopupType describes the type of the popup displayed
#[derive(Clone)]
enum PopupType {
    Alert(Color, String), // Show a message displaying text with the provided color
    Help,                 // Help page
    SaveBookmark(String, OnInputSubmitCallback), // Input description; Callback for submit
    YesNo(String, DialogCallback, DialogCallback), // Yes, no callback
}

/// ### InputMode
///
/// InputMode describes the current input mode
/// Each input mode handle the input events in a different way
enum InputMode {
    Form,
    Popup(PopupType),
}

#[derive(std::cmp::PartialEq)]
/// ### InputForm
///
/// InputForm describes the selected input form
enum InputForm {
    AuthCredentials,
    Bookmarks,
    Recents,
}

/// ### AuthActivity
///
/// AuthActivity is the data holder for the authentication activity
pub struct AuthActivity {
    pub address: String,
    pub port: String,
    pub protocol: FileTransferProtocol,
    pub username: String,
    pub password: String,
    pub submit: bool, // becomes true after user has submitted fields
    pub quit: bool,   // Becomes true if user has pressed esc
    context: Option<Context>,
    bookmarks_client: Option<BookmarksClient>,
    selected_field: InputField, // Selected field in AuthCredentials Form
    input_mode: InputMode,
    input_form: InputForm,
    password_placeholder: String,
    redraw: bool,                  // Should ui actually be redrawned?
    input_txt: String,             // Input text
    choice_opt: DialogYesNoOption, // Dialog popup selected option
    bookmarks_idx: usize,          // Index of selected bookmark
    recents_idx: usize,            // Index of selected recent
}

impl Default for AuthActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthActivity {
    /// ### new
    ///
    /// Instantiates a new AuthActivity
    pub fn new() -> AuthActivity {
        AuthActivity {
            address: String::new(),
            port: String::from("22"),
            protocol: FileTransferProtocol::Sftp,
            username: String::new(),
            password: String::new(),
            submit: false,
            quit: false,
            context: None,
            bookmarks_client: None,
            selected_field: InputField::Address,
            input_mode: InputMode::Form,
            input_form: InputForm::AuthCredentials,
            password_placeholder: String::new(),
            redraw: true, // True at startup
            input_txt: String::new(),
            choice_opt: DialogYesNoOption::Yes,
            bookmarks_idx: 0,
            recents_idx: 0,
        }
    }
}

impl Activity for AuthActivity {
    /// ### on_create
    ///
    /// `on_create` is the function which must be called to initialize the activity.
    /// `on_create` must initialize all the data structures used by the activity
    /// Context is taken from activity manager and will be released only when activity is destroyed
    fn on_create(&mut self, context: Context) {
        // Set context
        self.context = Some(context);
        // Clear terminal
        let _ = self.context.as_mut().unwrap().terminal.clear();
        // Put raw mode on enabled
        let _ = enable_raw_mode();
        self.input_mode = InputMode::Form;
        // Init bookmarks client
        if self.bookmarks_client.is_none() {
            self.init_bookmarks_client();
        }
    }

    /// ### on_draw
    ///
    /// `on_draw` is the function which draws the graphical interface.
    /// This function must be called at each tick to refresh the interface
    fn on_draw(&mut self) {
        // Context must be something
        if self.context.is_none() {
            return;
        }
        // Read one event
        if let Ok(event) = self.context.as_ref().unwrap().input_hnd.read_event() {
            if let Some(event) = event {
                // Set redraw to true
                self.redraw = true;
                // Handle event
                self.handle_input_event(&event);
            }
        }
        // Redraw if necessary
        if self.redraw {
            // Draw
            self.draw();
            // Set redraw to false
            self.redraw = false;
        }
    }

    /// ### on_destroy
    ///
    /// `on_destroy` is the function which cleans up runtime variables and data before terminating the activity.
    /// This function must be called once before terminating the activity.
    /// This function finally releases the context
    fn on_destroy(&mut self) -> Option<Context> {
        // Disable raw mode
        let _ = disable_raw_mode();
        self.context.as_ref()?;
        // Clear terminal and return
        match self.context.take() {
            Some(mut ctx) => {
                let _ = ctx.terminal.clear();
                Some(ctx)
            }
            None => None,
        }
    }
}
