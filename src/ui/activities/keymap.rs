//! ## Keymap
//!
//! Keymap contains pub constants which can be used in the `update` function to match messages

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

use crate::ui::layout::Msg;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

// -- Special keys

pub const MSG_KEY_ENTER: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Enter,
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_ESC: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Esc,
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_TAB: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Tab,
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_DEL: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Delete,
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_DOWN: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Down,
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_LEFT: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Left,
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_RIGHT: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Right,
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_UP: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Up,
    modifiers: KeyModifiers::NONE,
});

// -- char keys

pub const MSG_KEY_CHAR_A: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('a'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_B: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('b'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_C: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('c'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_D: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('d'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_E: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('e'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_F: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('f'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_G: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('g'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_H: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('h'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_I: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('i'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_J: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('j'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_K: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('k'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_L: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('l'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_M: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('m'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_N: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('n'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_O: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('o'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_P: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('p'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_Q: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('q'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_R: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('r'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_S: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('s'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_T: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('t'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_U: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('u'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_V: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('v'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_W: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('w'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_X: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('x'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_Y: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('y'),
    modifiers: KeyModifiers::NONE,
});
pub const MSG_KEY_CHAR_Z: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('z'),
    modifiers: KeyModifiers::NONE,
});

// -- control
pub const MSG_KEY_CTRL_C: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('c'),
    modifiers: KeyModifiers::CONTROL,
});
pub const MSG_KEY_CTRL_E: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('e'),
    modifiers: KeyModifiers::CONTROL,
});
pub const MSG_KEY_CTRL_H: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('h'),
    modifiers: KeyModifiers::CONTROL,
});
const MSG_KEY_CTRL_N: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('n'),
    modifiers: KeyModifiers::CONTROL,
});
const MSG_KEY_CTRL_R: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('r'),
    modifiers: KeyModifiers::CONTROL,
});
pub const MSG_KEY_CTRL_S: Msg = Msg::OnKey(KeyEvent {
    code: KeyCode::Char('s'),
    modifiers: KeyModifiers::CONTROL,
});
