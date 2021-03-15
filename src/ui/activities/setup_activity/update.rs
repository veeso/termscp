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

// locals
use super::{
    AuthActivity, COMPONENT_INPUT_FILE_FMT, COMPONENT_INPUT_TEXT_EDITOR, COMPONENT_LIST_SSH_KEYS, COMPONENT_RADIO_DEFAULT_PROTOCOL, COMPONENT_RADIO_GROUP_DIRS,
    COMPONENT_RADIO_HIDDEN_FILES, COMPONENT_RADIO_TAB, COMPONENT_RADIO_UPDATES, COMPONENT_TEXT_FOOTER, COMPONENT_TEXT_HELP,
};
use crate::ui::layout::{Msg, Payload};
// ext
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// -- keymap
const MSG_KEY_ENTER: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Enter,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_ESC: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Esc,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_TAB: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Tab,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_DOWN: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Down,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_LEFT: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Left,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_RIGHT: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Right,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_UP: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Up,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_DEL: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Delete,
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_CHAR_E: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('c'),
    modifiers: KeyModifiers::NONE,
});
const MSG_KEY_CTRL_C: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('c'),
    modifiers: KeyModifiers::CONTROL,
});
const MSG_KEY_CTRL_H: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('h'),
    modifiers: KeyModifiers::CONTROL,
});
const MSG_KEY_CTRL_S: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('s'),
    modifiers: KeyModifiers::CONTROL,
});

// -- update

impl AuthActivity {
    /// ### update
    ///
    /// Update auth activity model based on msg
    /// The function exits when returns None
    pub(super) fn update(&mut self, msg: Option<(String, Msg)>) -> Option<(String, Msg)> {
        let ref_msg: Option<(&str, &Msg)> = match msg.as_ref() {
            None => None,
            Some((s, msg)) => Some((s, msg)),
        };
        // Match msg
        match ref_msg {
        }
    }
}
