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
mod actions;
mod config;
mod update;
mod view;

// Deps
extern crate crossterm;
extern crate tui;

// Locals
use super::{Activity, Context, ExitReason};
use crate::ui::layout::view::View;
// Ext
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

// -- components
const COMPONENT_TEXT_HELP: &str = "TEXT_HELP";
const COMPONENT_TEXT_FOOTER: &str = "TEXT_FOOTER";
const COMPONENT_TEXT_ERROR: &str = "TEXT_ERROR";
const COMPONENT_RADIO_QUIT: &str = "RADIO_QUIT";
const COMPONENT_RADIO_SAVE: &str = "RADIO_SAVE";
const COMPONENT_INPUT_TEXT_EDITOR: &str = "INPUT_TEXT_EDITOR";
const COMPONENT_RADIO_DEFAULT_PROTOCOL: &str = "RADIO_DEFAULT_PROTOCOL";
const COMPONENT_RADIO_HIDDEN_FILES: &str = "RADIO_HIDDEN_FILES";
const COMPONENT_RADIO_UPDATES: &str = "RADIO_CHECK_UPDATES";
const COMPONENT_RADIO_GROUP_DIRS: &str = "RADIO_GROUP_DIRS";
const COMPONENT_INPUT_FILE_FMT: &str = "INPUT_FILE_FMT";
const COMPONENT_RADIO_TAB: &str = "RADIO_TAB";
const COMPONENT_LIST_SSH_KEYS: &str = "LIST_SSH_KEYS";
const COMPONENT_INPUT_SSH_HOST: &str = "INPUT_SSH_HOST";
const COMPONENT_INPUT_SSH_USERNAME: &str = "INPUT_SSH_USERNAME";
const COMPONENT_RADIO_DEL_SSH_KEY: &str = "RADIO_DEL_SSH_KEY";

/// ### ViewLayout
///
/// Current view layout
#[derive(std::cmp::PartialEq)]
enum ViewLayout {
    SetupForm,
    SshKeys,
}

/// ## SetupActivity
///
/// Setup activity states holder
pub struct SetupActivity {
    exit_reason: Option<ExitReason>,
    context: Option<Context>, // Context holder
    view: View,               // View
    layout: ViewLayout,       // View layout
    redraw: bool,
}

impl Default for SetupActivity {
    fn default() -> Self {
        // Initialize user input
        let mut user_input_buffer: Vec<String> = Vec::with_capacity(16);
        for _ in 0..16 {
            user_input_buffer.push(String::new());
        }
        SetupActivity {
            exit_reason: None,
            context: None,
            view: View::init(),
            layout: ViewLayout::SetupForm,
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
        // Init view
        self.init_setup();
        // Verify error state from context
        if let Some(err) = self.context.as_mut().unwrap().get_error() {
            self.mount_error(err.as_str());
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
            let msg = self.view.on(event);
            self.update(msg);
        }
        // Redraw if necessary
        if self.redraw {
            // View
            self.view();
            // Redraw back to false
            self.redraw = false;
        }
    }

    /// ### will_umount
    ///
    /// `will_umount` is the method which must be able to report to the activity manager, whether
    /// the activity should be terminated or not.
    /// If not, the call will return `None`, otherwise return`Some(ExitReason)`
    fn will_umount(&self) -> Option<&ExitReason> {
        self.exit_reason.as_ref()
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
