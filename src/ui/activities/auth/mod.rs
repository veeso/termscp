//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

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
// Sub modules
mod bookmarks;
mod misc;
mod update;
mod view;

// Dependencies
extern crate crossterm;
extern crate tuirealm;

// locals
use super::{Activity, Context, ExitReason};
use crate::filetransfer::FileTransferProtocol;
use crate::system::bookmarks_client::BookmarksClient;
use crate::ui::context::FileTransferParams;
use crate::utils::git;

// Includes
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tuirealm::View;

// -- components
const COMPONENT_TEXT_H1: &str = "TEXT_H1";
const COMPONENT_TEXT_H2: &str = "TEXT_H2";
const COMPONENT_TEXT_NEW_VERSION: &str = "TEXT_NEW_VERSION";
const COMPONENT_TEXT_FOOTER: &str = "TEXT_FOOTER";
const COMPONENT_TEXT_HELP: &str = "TEXT_HELP";
const COMPONENT_TEXT_ERROR: &str = "TEXT_ERROR";
const COMPONENT_INPUT_ADDR: &str = "INPUT_ADDRESS";
const COMPONENT_INPUT_PORT: &str = "INPUT_PORT";
const COMPONENT_INPUT_USERNAME: &str = "INPUT_USERNAME";
const COMPONENT_INPUT_PASSWORD: &str = "INPUT_PASSWORD";
const COMPONENT_INPUT_BOOKMARK_NAME: &str = "INPUT_BOOKMARK_NAME";
const COMPONENT_RADIO_PROTOCOL: &str = "RADIO_PROTOCOL";
const COMPONENT_RADIO_QUIT: &str = "RADIO_QUIT";
const COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK: &str = "RADIO_DELETE_BOOKMARK";
const COMPONENT_RADIO_BOOKMARK_DEL_RECENT: &str = "RADIO_DELETE_RECENT";
const COMPONENT_RADIO_BOOKMARK_SAVE_PWD: &str = "RADIO_SAVE_PASSWORD";
const COMPONENT_BOOKMARKS_LIST: &str = "BOOKMARKS_LIST";
const COMPONENT_RECENTS_LIST: &str = "RECENTS_LIST";

// Store keys
const STORE_KEY_LATEST_VERSION: &str = "AUTH_LATEST_VERSION";

/// ### AuthActivity
///
/// AuthActivity is the data holder for the authentication activity
pub struct AuthActivity {
    exit_reason: Option<ExitReason>,
    context: Option<Context>,
    view: View,
    bookmarks_client: Option<BookmarksClient>,
    redraw: bool,                // Should ui actually be redrawned?
    bookmarks_list: Vec<String>, // List of bookmarks
    recents_list: Vec<String>,   // list of recents
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
            exit_reason: None,
            context: None,
            view: View::init(),
            bookmarks_client: None,
            redraw: true, // True at startup
            bookmarks_list: Vec::new(),
            recents_list: Vec::new(),
        }
    }

    /// ### on_create
    ///
    /// If enabled in configuration, check for updates from Github
    fn check_for_updates(&mut self) {
        debug!("Check for updates...");
        // Check version only if unset in the store
        let ctx: &Context = self.context.as_ref().unwrap();
        if !ctx.store.isset(STORE_KEY_LATEST_VERSION) {
            debug!("Version is not set in storage");
            let mut new_version: Option<String> = match ctx.config_client.as_ref() {
                Some(client) => {
                    if client.get_check_for_updates() {
                        debug!("Check for updates is enabled");
                        // Send request
                        match git::check_for_updates(env!("CARGO_PKG_VERSION")) {
                            Ok(version) => {
                                info!("Latest version is: {:?}", version);
                                version
                            }
                            Err(err) => {
                                // Report error
                                error!("Failed to get latest version: {}", err);
                                self.mount_error(
                                    format!("Could not check for new updates: {}", err).as_str(),
                                );
                                // None
                                None
                            }
                        }
                    } else {
                        info!("Check for updates is disabled");
                        None
                    }
                }
                None => None,
            };
            let ctx: &mut Context = self.context.as_mut().unwrap();
            // Set version into the store (or just a flag)
            match new_version.take() {
                Some(new_version) => ctx.store.set_string(STORE_KEY_LATEST_VERSION, new_version), // If Some, set String
                None => ctx.store.set(STORE_KEY_LATEST_VERSION), // If None, just set flag
            }
        }
    }
}

impl Activity for AuthActivity {
    /// ### on_create
    ///
    /// `on_create` is the function which must be called to initialize the activity.
    /// `on_create` must initialize all the data structures used by the activity
    /// Context is taken from activity manager and will be released only when activity is destroyed
    fn on_create(&mut self, mut context: Context) {
        debug!("Initializing activity");
        // Initialize file transfer params
        context.ft_params = Some(FileTransferParams::default());
        // Set context
        self.context = Some(context);
        // Clear terminal
        self.context.as_mut().unwrap().clear_screen();
        // Put raw mode on enabled
        if let Err(err) = enable_raw_mode() {
            error!("Failed to enter raw mode: {}", err);
        }
        // Initialize view
        self.init();
        // Init bookmarks client
        if self.bookmarks_client.is_none() {
            self.init_bookmarks_client();
        }
        // If check for updates is enabled, check for updates
        self.check_for_updates();
        // Verify error state from context
        if let Some(err) = self.context.as_mut().unwrap().get_error() {
            self.mount_error(err.as_str());
        }
        info!("Activity initialized");
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
            // Handle event on view and update
            let msg = self.view.on(event);
            self.update(msg);
        }
        // Redraw if necessary
        if self.redraw {
            // View
            self.view();
            // Set redraw to false
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
        if let Err(err) = disable_raw_mode() {
            error!("Failed to disable raw mode: {}", err);
        }
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
