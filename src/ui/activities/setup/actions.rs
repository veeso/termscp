//! ## SetupActivity
//!
//! `setup_activity` is the module which implements the Setup activity, which is the activity to
//! work on termscp configuration

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
// Locals
use super::{Id, IdSsh, IdTheme, SetupActivity, ViewLayout};
// Ext
use crate::config::themes::Theme;
use std::env;
use tuirealm::tui::style::Color;
use tuirealm::{State, StateValue};

impl SetupActivity {
    /// On <ESC>, if there are changes in the configuration, the quit dialog must be shown, otherwise
    /// we can exit  without any problem
    pub(super) fn action_on_esc(&mut self) {
        if self.config_changed() {
            self.mount_quit();
        } else {
            self.exit_reason = Some(super::ExitReason::Quit);
        }
    }

    /// Save all configurations. If current tab can load values, they will be loaded, otherwise they'll just be saved.
    /// Once all the configuration has been changed, set config_changed to false
    pub(super) fn action_save_all(&mut self) -> Result<(), String> {
        self.action_save_config()?;
        self.action_save_theme()?;
        // Set config changed to false
        self.set_config_changed(false);
        Ok(())
    }

    /// Save configuration
    fn action_save_config(&mut self) -> Result<(), String> {
        // Collect input values if in setup form
        if self.layout == ViewLayout::SetupForm {
            self.collect_input_values();
        }
        self.save_config()
    }

    /// Save configuration
    fn action_save_theme(&mut self) -> Result<(), String> {
        // Collect input values if in theme form
        if self.layout == ViewLayout::Theme {
            self.collect_styles()
                .map_err(|e| format!("'{:?}' has an invalid color", e))?;
        }
        // save theme
        self.save_theme()
    }

    /// Change view tab and load input values in order not to lose them
    pub(super) fn action_change_tab(&mut self, new_tab: ViewLayout) -> Result<(), String> {
        // load values for current tab first
        match self.layout {
            ViewLayout::SetupForm => self.collect_input_values(),
            ViewLayout::Theme => self
                .collect_styles()
                .map_err(|e| format!("'{:?}' has an invalid color", e))?,
            _ => {}
        }
        // Update view
        self.init(new_tab);
        Ok(())
    }

    /// Reset configuration input fields
    pub(super) fn action_reset_config(&mut self) -> Result<(), String> {
        match self.reset_config_changes() {
            Err(err) => Err(err),
            Ok(_) => {
                self.load_input_values();
                Ok(())
            }
        }
    }

    /// Reset configuration input fields
    pub(super) fn action_reset_theme(&mut self) -> Result<(), String> {
        match self.reset_theme_changes() {
            Err(err) => Err(err),
            Ok(_) => {
                self.load_styles();
                Ok(())
            }
        }
    }

    /// delete of a ssh key
    pub(super) fn action_delete_ssh_key(&mut self) {
        // Get key
        // get index
        let idx: Option<usize> = match self.app.state(&Id::Ssh(IdSsh::SshKeys)) {
            Ok(State::One(StateValue::Usize(idx))) => Some(idx),
            _ => None,
        };
        if let Some(idx) = idx {
            let key: Option<String> = self.config().iter_ssh_keys().nth(idx).cloned();
            if let Some(key) = key {
                match self.config().get_ssh_key(&key) {
                    Ok(opt) => {
                        if let Some((host, username, _)) = opt {
                            if let Err(err) = self.delete_ssh_key(host.as_str(), username.as_str())
                            {
                                // Report error
                                self.mount_error(err.as_str());
                            }
                        }
                    }
                    Err(err) => {
                        // Report error
                        self.mount_error(
                            format!("Could not get ssh key \"{}\": {}", key, err).as_str(),
                        );
                    }
                }
            }
        }
    }

    /// Create a new ssh key
    pub(super) fn action_new_ssh_key(&mut self) {
        // get parameters
        let host: String = match self.app.state(&Id::Ssh(IdSsh::SshHost)) {
            Ok(State::One(StateValue::String(host))) => host,
            _ => String::new(),
        };
        let username: String = match self.app.state(&Id::Ssh(IdSsh::SshUsername)) {
            Ok(State::One(StateValue::String(user))) => user,
            _ => String::new(),
        };
        // Prepare text editor
        env::set_var("EDITOR", self.config().get_text_editor());
        let placeholder: String = format!("# Type private SSH key for {}@{}\n", username, host);
        // Put input mode back to normal
        if let Err(err) = self.context_mut().terminal().disable_raw_mode() {
            error!("Could not disable raw mode: {}", err);
        }
        // Leave alternate mode
        if let Err(err) = self.context_mut().terminal().leave_alternate_screen() {
            error!("Could not leave alternate screen: {}", err);
        }
        // Lock ports
        assert!(self.app.lock_ports().is_ok());
        // Write key to file
        match edit::edit(placeholder.as_bytes()) {
            Ok(rsa_key) => {
                // Remove placeholder from `rsa_key`
                let rsa_key: String = rsa_key.as_str().replace(placeholder.as_str(), "");
                if rsa_key.is_empty() {
                    // Report error: empty key
                    self.mount_error("SSH key is empty!");
                } else {
                    // Add key
                    if let Err(err) =
                        self.add_ssh_key(host.as_str(), username.as_str(), rsa_key.as_str())
                    {
                        self.mount_error(
                            format!("Could not create new private key: {}", err).as_str(),
                        );
                    }
                }
            }
            Err(err) => {
                // Report error
                self.mount_error(format!("Could not write private key to file: {}", err).as_str());
            }
        }
        // Restore terminal
        if let Some(ctx) = self.context.as_mut() {
            // Enter alternate mode
            if let Err(err) = ctx.terminal().enter_alternate_screen() {
                error!("Could not enter alternate screen: {}", err);
            }
            // Re-enable raw mode
            if let Err(err) = ctx.terminal().enable_raw_mode() {
                error!("Failed to enter raw mode: {}", err);
            }
            if let Err(err) = ctx.terminal().clear_screen() {
                error!("Could not clear screen screen: {}", err);
            }
            // Unlock ports
            assert!(self.app.unlock_ports().is_ok());
        }
    }

    /// Given a component and a color, save the color into the theme
    pub(super) fn action_save_color(&mut self, component: IdTheme, color: Color) {
        let theme: &mut Theme = self.theme_mut();
        match component {
            IdTheme::AuthAddress => {
                theme.auth_address = color;
            }
            IdTheme::AuthBookmarks => {
                theme.auth_bookmarks = color;
            }
            IdTheme::AuthPassword => {
                theme.auth_password = color;
            }
            IdTheme::AuthPort => {
                theme.auth_port = color;
            }
            IdTheme::AuthProtocol => {
                theme.auth_protocol = color;
            }
            IdTheme::AuthRecentHosts => {
                theme.auth_recents = color;
            }
            IdTheme::AuthUsername => {
                theme.auth_username = color;
            }
            IdTheme::MiscError => {
                theme.misc_error_dialog = color;
            }
            IdTheme::MiscInfo => {
                theme.misc_info_dialog = color;
            }
            IdTheme::MiscInput => {
                theme.misc_input_dialog = color;
            }
            IdTheme::MiscKeys => {
                theme.misc_keys = color;
            }
            IdTheme::MiscQuit => {
                theme.misc_quit_dialog = color;
            }
            IdTheme::MiscSave => {
                theme.misc_save_dialog = color;
            }
            IdTheme::MiscWarn => {
                theme.misc_warn_dialog = color;
            }
            IdTheme::ExplorerLocalBg => {
                theme.transfer_local_explorer_background = color;
            }
            IdTheme::ExplorerLocalFg => {
                theme.transfer_local_explorer_foreground = color;
            }
            IdTheme::ExplorerLocalHg => {
                theme.transfer_local_explorer_highlighted = color;
            }
            IdTheme::ExplorerRemoteBg => {
                theme.transfer_remote_explorer_background = color;
            }
            IdTheme::ExplorerRemoteFg => {
                theme.transfer_remote_explorer_foreground = color;
            }
            IdTheme::ExplorerRemoteHg => {
                theme.transfer_remote_explorer_highlighted = color;
            }
            IdTheme::LogBg => {
                theme.transfer_log_background = color;
            }
            IdTheme::LogWindow => {
                theme.transfer_log_window = color;
            }
            IdTheme::ProgBarFull => {
                theme.transfer_progress_bar_full = color;
            }
            IdTheme::ProgBarPartial => {
                theme.transfer_progress_bar_partial = color;
            }
            IdTheme::StatusHidden => {
                theme.transfer_status_hidden = color;
            }
            IdTheme::StatusSorting => {
                theme.transfer_status_sorting = color;
            }
            IdTheme::StatusSync => {
                theme.transfer_status_sync_browsing = color;
            }
            _ => {}
        }
    }

    /// Collect values from input and put them into the theme.
    /// If a component has an invalid color, returns Err(component_id)
    fn collect_styles(&mut self) -> Result<(), Id> {
        // auth
        let auth_address = self
            .get_color(&Id::Theme(IdTheme::AuthAddress))
            .map_err(|_| Id::Theme(IdTheme::AuthAddress))?;
        let auth_bookmarks = self
            .get_color(&Id::Theme(IdTheme::AuthBookmarks))
            .map_err(|_| Id::Theme(IdTheme::AuthBookmarks))?;
        let auth_password = self
            .get_color(&Id::Theme(IdTheme::AuthPassword))
            .map_err(|_| Id::Theme(IdTheme::AuthPassword))?;
        let auth_port = self
            .get_color(&Id::Theme(IdTheme::AuthPort))
            .map_err(|_| Id::Theme(IdTheme::AuthPort))?;
        let auth_protocol = self
            .get_color(&Id::Theme(IdTheme::AuthProtocol))
            .map_err(|_| Id::Theme(IdTheme::AuthProtocol))?;
        let auth_recents = self
            .get_color(&Id::Theme(IdTheme::AuthRecentHosts))
            .map_err(|_| Id::Theme(IdTheme::AuthRecentHosts))?;
        let auth_username = self
            .get_color(&Id::Theme(IdTheme::AuthUsername))
            .map_err(|_| Id::Theme(IdTheme::AuthUsername))?;
        // misc
        let misc_error_dialog = self
            .get_color(&Id::Theme(IdTheme::MiscError))
            .map_err(|_| Id::Theme(IdTheme::MiscError))?;
        let misc_info_dialog = self
            .get_color(&Id::Theme(IdTheme::MiscInfo))
            .map_err(|_| Id::Theme(IdTheme::MiscInfo))?;
        let misc_input_dialog = self
            .get_color(&Id::Theme(IdTheme::MiscInput))
            .map_err(|_| Id::Theme(IdTheme::MiscInput))?;
        let misc_keys = self
            .get_color(&Id::Theme(IdTheme::MiscKeys))
            .map_err(|_| Id::Theme(IdTheme::MiscKeys))?;
        let misc_quit_dialog = self
            .get_color(&Id::Theme(IdTheme::MiscQuit))
            .map_err(|_| Id::Theme(IdTheme::MiscQuit))?;
        let misc_save_dialog = self
            .get_color(&Id::Theme(IdTheme::MiscSave))
            .map_err(|_| Id::Theme(IdTheme::MiscSave))?;
        let misc_warn_dialog = self
            .get_color(&Id::Theme(IdTheme::MiscWarn))
            .map_err(|_| Id::Theme(IdTheme::MiscWarn))?;
        // transfer
        let transfer_local_explorer_background = self
            .get_color(&Id::Theme(IdTheme::ExplorerLocalBg))
            .map_err(|_| Id::Theme(IdTheme::ExplorerLocalBg))?;
        let transfer_local_explorer_foreground = self
            .get_color(&Id::Theme(IdTheme::ExplorerLocalFg))
            .map_err(|_| Id::Theme(IdTheme::ExplorerLocalFg))?;
        let transfer_local_explorer_highlighted = self
            .get_color(&Id::Theme(IdTheme::ExplorerLocalHg))
            .map_err(|_| Id::Theme(IdTheme::ExplorerLocalHg))?;
        let transfer_remote_explorer_background = self
            .get_color(&Id::Theme(IdTheme::ExplorerRemoteBg))
            .map_err(|_| Id::Theme(IdTheme::ExplorerRemoteBg))?;
        let transfer_remote_explorer_foreground = self
            .get_color(&Id::Theme(IdTheme::ExplorerRemoteFg))
            .map_err(|_| Id::Theme(IdTheme::ExplorerRemoteFg))?;
        let transfer_remote_explorer_highlighted = self
            .get_color(&Id::Theme(IdTheme::ExplorerRemoteHg))
            .map_err(|_| Id::Theme(IdTheme::ExplorerRemoteHg))?;
        let transfer_log_background = self
            .get_color(&Id::Theme(IdTheme::LogBg))
            .map_err(|_| Id::Theme(IdTheme::LogBg))?;
        let transfer_log_window = self
            .get_color(&Id::Theme(IdTheme::LogWindow))
            .map_err(|_| Id::Theme(IdTheme::LogWindow))?;
        let transfer_progress_bar_full = self
            .get_color(&Id::Theme(IdTheme::ProgBarFull))
            .map_err(|_| Id::Theme(IdTheme::ProgBarFull))?;
        let transfer_progress_bar_partial = self
            .get_color(&Id::Theme(IdTheme::ProgBarPartial))
            .map_err(|_| Id::Theme(IdTheme::ProgBarPartial))?;
        let transfer_status_hidden = self
            .get_color(&Id::Theme(IdTheme::StatusHidden))
            .map_err(|_| Id::Theme(IdTheme::StatusHidden))?;
        let transfer_status_sorting = self
            .get_color(&Id::Theme(IdTheme::StatusSorting))
            .map_err(|_| Id::Theme(IdTheme::StatusSorting))?;
        let transfer_status_sync_browsing = self
            .get_color(&Id::Theme(IdTheme::StatusSync))
            .map_err(|_| Id::Theme(IdTheme::StatusSync))?;
        // Update theme
        let mut theme: &mut Theme = self.theme_mut();
        theme.auth_address = auth_address;
        theme.auth_bookmarks = auth_bookmarks;
        theme.auth_password = auth_password;
        theme.auth_port = auth_port;
        theme.auth_protocol = auth_protocol;
        theme.auth_recents = auth_recents;
        theme.auth_username = auth_username;
        theme.misc_error_dialog = misc_error_dialog;
        theme.misc_info_dialog = misc_info_dialog;
        theme.misc_input_dialog = misc_input_dialog;
        theme.misc_keys = misc_keys;
        theme.misc_quit_dialog = misc_quit_dialog;
        theme.misc_save_dialog = misc_save_dialog;
        theme.misc_warn_dialog = misc_warn_dialog;
        theme.transfer_local_explorer_background = transfer_local_explorer_background;
        theme.transfer_local_explorer_foreground = transfer_local_explorer_foreground;
        theme.transfer_local_explorer_highlighted = transfer_local_explorer_highlighted;
        theme.transfer_remote_explorer_background = transfer_remote_explorer_background;
        theme.transfer_remote_explorer_foreground = transfer_remote_explorer_foreground;
        theme.transfer_remote_explorer_highlighted = transfer_remote_explorer_highlighted;
        theme.transfer_log_background = transfer_log_background;
        theme.transfer_log_window = transfer_log_window;
        theme.transfer_progress_bar_full = transfer_progress_bar_full;
        theme.transfer_progress_bar_partial = transfer_progress_bar_partial;
        theme.transfer_status_hidden = transfer_status_hidden;
        theme.transfer_status_sorting = transfer_status_sorting;
        theme.transfer_status_sync_browsing = transfer_status_sync_browsing;
        Ok(())
    }

    /// Get color from component
    fn get_color(&self, component: &Id) -> Result<Color, ()> {
        match self.app.state(component) {
            Ok(State::One(StateValue::String(color))) => {
                match crate::utils::parser::parse_color(color.as_str()) {
                    Some(c) => Ok(c),
                    None => Err(()),
                }
            }
            _ => Err(()),
        }
    }
}
