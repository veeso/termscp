//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

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

// locals
use super::{
    AuthActivity, FileTransferProtocol, InputEvent,
    COMPONENT_TEXT_HELP
};
use crate::ui::layout::{Msg, Payload};
// ext
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// -- update

impl AuthActivity {

    /// ### handle_input_event
    ///
    /// Handle input event, based on current input mode
    pub(super) fn handle_input_event(&mut self, ev: InputEvent) {
        // Call update passing the return value from on
        self.update(self.view.on(ev));
    }

    /// ### update
    /// 
    /// Update auth activity model based on msg
    /// The function exits when returns None
    pub(super) fn update(&mut self, msg: Option<(&str, Msg)>) -> Option<(&str, Msg)> {
        let key_enter = KeyEvent::from(KeyCode::Enter);
        // Match msg
        match msg {
            None => None, // Exit after None
            Some(msg) => match msg {
                (COMPONENT_TEXT_HELP, Msg::OnKey(key_enter) | (COMPONENT_TEXT_HELP, Msg::OnKey(KeyEvent::from(KeyCode::Esc))) => {
                    // Hide text help
                    match self.view.get_props(COMPONENT_TEXT_HELP) {
                        None => None,
                        Some(props) => self.update(self.view.update(COMPONENT_TEXT_HELP, props.hidden().build())),
                    }
                }
                (_, Msg::OnSubmit(_)) | (_, Msg::OnKey(KeyEvent::from(KeyCode::Enter))) => {
                    // Match <ENTER> key for all other components
                }
                (_, _) => None, // Ignore other events
            }
        }
    }

}