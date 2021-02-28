//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

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

// Submodules
mod callbacks;
mod config;
mod input;
mod layout;
mod misc;

// Deps
extern crate crossterm;
extern crate tui;

// Locals
use super::{Activity, Context};
use crate::system::config_client::ConfigClient;
// Ext
use crossterm::event::Event as InputEvent;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::style::Color;

// Types
type OnChoiceCallback = fn(&mut SetupActivity);

/// ### UserInterfaceInputField
///
/// Input field selected in user interface
#[derive(std::cmp::PartialEq, Clone)]
enum UserInterfaceInputField {
    DefaultProtocol,
    TextEditor,
    ShowHiddenFiles,
    CheckForUpdates,
    GroupDirs,
    FileFmt,
}

/// ### SetupTab
///
/// Selected setup tab
#[derive(std::cmp::PartialEq)]
enum SetupTab {
    UserInterface(UserInterfaceInputField),
    SshConfig,
}

/// ### QuitDialogOption
///
/// Quit dialog options
#[derive(std::cmp::PartialEq, Clone)]
enum QuitDialogOption {
    Save,
    DontSave,
    Cancel,
}

/// ### YesNoDialogOption
///
/// YesNo dialog options
#[derive(std::cmp::PartialEq, Clone)]
enum YesNoDialogOption {
    Yes,
    No,
}

/// ## Popup
///
/// Popup describes the type of popup
#[derive(Clone)]
enum Popup {
    Alert(Color, String),                              // Block color; Block text
    Fatal(String),                                     // Must quit after being hidden
    Help,                                              // Show Help
    NewSshKey,                                         //
    Quit,                                              // Quit dialog
    YesNo(String, OnChoiceCallback, OnChoiceCallback), // Yes/No Dialog
}

/// ## SetupActivity
///
/// Setup activity states holder
pub struct SetupActivity {
    pub quit: bool,           // Becomes true when user requests the activity to terminate
    context: Option<Context>, // Context holder
    config_cli: Option<ConfigClient>, // Config client
    tab: SetupTab,            // Current setup tab
    popup: Option<Popup>,     // Active popup
    user_input: Vec<String>,  // User input holder
    user_input_ptr: usize,    // Selected user input
    quit_opt: QuitDialogOption, // Popup::Quit selected option
    yesno_opt: YesNoDialogOption, // Popup::YesNo selected option
    ssh_key_idx: usize,       // Index of selected ssh key in list
    redraw: bool,             // Redraw ui?
}

impl Default for SetupActivity {
    fn default() -> Self {
        // Initialize user input
        let mut user_input_buffer: Vec<String> = Vec::with_capacity(16);
        for _ in 0..16 {
            user_input_buffer.push(String::new());
        }
        SetupActivity {
            quit: false,
            context: None,
            config_cli: None,
            tab: SetupTab::UserInterface(UserInterfaceInputField::TextEditor),
            popup: None,
            user_input: user_input_buffer, // Max 16
            user_input_ptr: 0,
            quit_opt: QuitDialogOption::Save,
            yesno_opt: YesNoDialogOption::Yes,
            ssh_key_idx: 0,
            redraw: true, // Draw at first `on_draw`
        }
    }
}

impl Activity for SetupActivity {
    /// ### on_create
    ///
    /// `on_create` is the function which must be called to initialize the activity.
    /// `on_create` must initialize all the data structures used by the activity
    /// Context is taken from activity manager and will be released only when activity is destroyed
    fn on_create(&mut self, context: Context) {
        // Set context
        self.context = Some(context);
        // Clear terminal
        self.context.as_mut().unwrap().clear_screen();
        // Put raw mode on enabled
        let _ = enable_raw_mode();
        // Initialize config client
        if self.config_cli.is_none() {
            self.init_config_client();
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
        if let Ok(Some(event)) = self.context.as_ref().unwrap().input_hnd.read_event() {
            // Set redraw to true
            self.redraw = true;
            // Handle event
            self.handle_input_event(&event);
        }
        // Redraw if necessary
        if self.redraw {
            // Draw
            self.draw();
            // Redraw back to false
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
                ctx.clear_screen();
                Some(ctx)
            }
            None => None,
        }
    }
}
