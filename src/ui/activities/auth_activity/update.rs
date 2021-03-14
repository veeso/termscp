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
    AuthActivity, COMPONENT_BOOKMARKS_LIST, COMPONENT_INPUT_ADDR, COMPONENT_INPUT_BOOKMARK_NAME,
    COMPONENT_INPUT_PASSWORD, COMPONENT_INPUT_PORT, COMPONENT_INPUT_USERNAME,
    COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK, COMPONENT_RADIO_BOOKMARK_DEL_RECENT,
    COMPONENT_RADIO_BOOKMARK_SAVE_PWD, COMPONENT_RADIO_PROTOCOL, COMPONENT_RADIO_QUIT,
    COMPONENT_RECENTS_LIST, COMPONENT_TEXT_ERROR, COMPONENT_TEXT_HELP,
};
use crate::ui::layout::{props::TextParts, Msg, Payload};
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
            None => None, // Exit after None
            Some(msg) => match msg {
                // Focus ( DOWN )
                (COMPONENT_INPUT_ADDR, &MSG_KEY_DOWN) => {
                    // Give focus to port
                    self.view.active(COMPONENT_INPUT_PORT);
                    None
                }
                (COMPONENT_INPUT_PORT, &MSG_KEY_DOWN) => {
                    // Give focus to port
                    self.view.active(COMPONENT_RADIO_PROTOCOL);
                    None
                }
                (COMPONENT_RADIO_PROTOCOL, &MSG_KEY_DOWN) => {
                    // Give focus to port
                    self.view.active(COMPONENT_INPUT_USERNAME);
                    None
                }
                (COMPONENT_INPUT_USERNAME, &MSG_KEY_DOWN) => {
                    // Give focus to port
                    self.view.active(COMPONENT_INPUT_PASSWORD);
                    None
                }
                (COMPONENT_INPUT_PASSWORD, &MSG_KEY_DOWN) => {
                    // Give focus to port
                    self.view.active(COMPONENT_INPUT_ADDR);
                    None
                }
                // Focus ( UP )
                (COMPONENT_INPUT_PASSWORD, &MSG_KEY_UP) => {
                    // Give focus to port
                    self.view.active(COMPONENT_INPUT_USERNAME);
                    None
                }
                (COMPONENT_INPUT_USERNAME, &MSG_KEY_UP) => {
                    // Give focus to port
                    self.view.active(COMPONENT_RADIO_PROTOCOL);
                    None
                }
                (COMPONENT_RADIO_PROTOCOL, &MSG_KEY_UP) => {
                    // Give focus to port
                    self.view.active(COMPONENT_INPUT_PORT);
                    None
                }
                (COMPONENT_INPUT_PORT, &MSG_KEY_UP) => {
                    // Give focus to port
                    self.view.active(COMPONENT_INPUT_ADDR);
                    None
                }
                (COMPONENT_INPUT_ADDR, &MSG_KEY_UP) => {
                    // Give focus to port
                    self.view.active(COMPONENT_INPUT_PASSWORD);
                    None
                }
                // <TAB> bookmarks
                (COMPONENT_BOOKMARKS_LIST, &MSG_KEY_TAB)
                | (COMPONENT_RECENTS_LIST, &MSG_KEY_TAB) => {
                    // Give focus to address
                    self.view.active(COMPONENT_INPUT_ADDR);
                    None
                }
                // Any <TAB>, go to bookmarks
                (_, &MSG_KEY_TAB) => {
                    self.view.active(COMPONENT_BOOKMARKS_LIST);
                    None
                }
                // Bookmarks commands
                // <RIGHT> / <LEFT>
                (COMPONENT_BOOKMARKS_LIST, &MSG_KEY_RIGHT) => {
                    // Give focus to recents
                    self.view.active(COMPONENT_RECENTS_LIST);
                    None
                }
                (COMPONENT_RECENTS_LIST, &MSG_KEY_LEFT) => {
                    // Give focus to bookmarks
                    self.view.active(COMPONENT_BOOKMARKS_LIST);
                    None
                }
                // <DEL | 'E'>
                (COMPONENT_BOOKMARKS_LIST, &MSG_KEY_DEL)
                | (COMPONENT_BOOKMARKS_LIST, &MSG_KEY_CHAR_E) => {
                    // Show delete popup
                    self.mount_bookmark_del_dialog();
                    None
                }
                (COMPONENT_RECENTS_LIST, &MSG_KEY_DEL)
                | (COMPONENT_RECENTS_LIST, &MSG_KEY_CHAR_E) => {
                    // Show delete popup
                    self.mount_recent_del_dialog();
                    None
                }
                // Enter
                (COMPONENT_BOOKMARKS_LIST, Msg::OnSubmit(Payload::Unsigned(idx))) => {
                    self.load_bookmark(*idx);
                    None
                }
                (COMPONENT_RECENTS_LIST, Msg::OnSubmit(Payload::Unsigned(idx))) => {
                    self.load_recent(*idx);
                    None
                }
                // Bookmark radio
                // Del bookmarks
                (
                    COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK,
                    Msg::OnSubmit(Payload::Unsigned(index)),
                ) => {
                    // hide bookmark delete
                    self.umount_bookmark_del_dialog();
                    // Index must be 0 => YES
                    match *index {
                        0 => {
                            // Get selected bookmark
                            match self.view.get_value(COMPONENT_BOOKMARKS_LIST) {
                                Some(Payload::Unsigned(index)) => {
                                    // Delete bookmark
                                    self.del_bookmark(index);
                                    // Update bookmarks
                                    match self.view.get_props(COMPONENT_BOOKMARKS_LIST).as_mut() {
                                        None => None,
                                        Some(props) => {
                                            let msg = self.view.update(
                                                COMPONENT_BOOKMARKS_LIST,
                                                props
                                                    .with_texts(TextParts::new(
                                                        Some(String::from("Bookmarks")),
                                                        Some(self.view_bookmarks()),
                                                    ))
                                                    .build(),
                                            );
                                            self.update(msg)
                                        }
                                    }
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                (COMPONENT_RADIO_BOOKMARK_DEL_RECENT, Msg::OnSubmit(Payload::Unsigned(index))) => {
                    // hide bookmark delete
                    self.umount_recent_del_dialog();
                    // Index must be 0 => YES
                    match *index {
                        0 => {
                            // Get selected bookmark
                            match self.view.get_value(COMPONENT_RECENTS_LIST) {
                                Some(Payload::Unsigned(index)) => {
                                    // Delete recent
                                    self.del_recent(index);
                                    // Update bookmarks
                                    match self.view.get_props(COMPONENT_RECENTS_LIST).as_mut() {
                                        None => None,
                                        Some(props) => {
                                            let msg = self.view.update(
                                                COMPONENT_RECENTS_LIST,
                                                props
                                                    .with_texts(TextParts::new(
                                                        Some(String::from("Recent connections")),
                                                        Some(self.view_recent_connections()),
                                                    ))
                                                    .build(),
                                            );
                                            self.update(msg)
                                        }
                                    }
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                // <ESC> hide tab
                (COMPONENT_RADIO_BOOKMARK_DEL_RECENT, &MSG_KEY_ESC) => {
                    match self
                        .view
                        .get_props(COMPONENT_RADIO_BOOKMARK_DEL_RECENT)
                        .as_mut()
                    {
                        Some(props) => {
                            let msg = self.view.update(
                                COMPONENT_RADIO_BOOKMARK_DEL_RECENT,
                                props.hidden().build(),
                            );
                            self.update(msg)
                        }
                        None => None,
                    }
                }
                (COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK, &MSG_KEY_ESC) => {
                    match self
                        .view
                        .get_props(COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK)
                        .as_mut()
                    {
                        Some(props) => {
                            let msg = self.view.update(
                                COMPONENT_RADIO_BOOKMARK_DEL_BOOKMARK,
                                props.hidden().build(),
                            );
                            self.update(msg)
                        }
                        None => None,
                    }
                }
                // Help
                (_, &MSG_KEY_CTRL_H) => {
                    // Show help
                    self.mount_help();
                    None
                }
                (COMPONENT_TEXT_HELP, &MSG_KEY_ENTER) | (COMPONENT_TEXT_HELP, &MSG_KEY_ESC) => {
                    // Hide text help
                    self.umount_help();
                    None
                }
                // Enter setup
                (_, &MSG_KEY_CTRL_C) => {
                    self.setup = true;
                    None
                }
                // Save bookmark; show popup
                (_, &MSG_KEY_CTRL_S) => {
                    // Show popup
                    self.mount_bookmark_save_dialog();
                    // Give focus to bookmark name
                    self.view.active(COMPONENT_INPUT_BOOKMARK_NAME);
                    None
                }
                (COMPONENT_INPUT_BOOKMARK_NAME, &MSG_KEY_DOWN) => {
                    // Give focus to pwd
                    self.view.active(COMPONENT_RADIO_BOOKMARK_SAVE_PWD);
                    None
                }
                (COMPONENT_RADIO_BOOKMARK_SAVE_PWD, &MSG_KEY_UP) => {
                    // Give focus to pwd
                    self.view.active(COMPONENT_INPUT_BOOKMARK_NAME);
                    None
                }
                // Save bookmark
                (COMPONENT_INPUT_BOOKMARK_NAME, Msg::OnSubmit(_))
                | (COMPONENT_RADIO_BOOKMARK_SAVE_PWD, Msg::OnSubmit(_)) => {
                    // Get values
                    let bookmark_name: String =
                        match self.view.get_value(COMPONENT_INPUT_BOOKMARK_NAME) {
                            Some(Payload::Text(s)) => s,
                            _ => String::new(),
                        };
                    let save_pwd: bool =
                        match self.view.get_value(COMPONENT_RADIO_BOOKMARK_SAVE_PWD) {
                            Some(Payload::Unsigned(idx)) => match idx {
                                0 => true,
                                _ => false,
                            },
                            _ => false,
                        };
                    // Save bookmark
                    self.save_bookmark(bookmark_name, save_pwd);
                    // Umount popup
                    self.umount_bookmark_save_dialog();
                    None
                }
                // Hide save bookmark
                (COMPONENT_INPUT_BOOKMARK_NAME, &MSG_KEY_ESC)
                | (COMPONENT_RADIO_BOOKMARK_SAVE_PWD, &MSG_KEY_ESC) => {
                    // Umount popup
                    self.umount_bookmark_save_dialog();
                    None
                }
                // Error message
                (COMPONENT_TEXT_ERROR, &MSG_KEY_ENTER) => {
                    // Umount text error
                    self.umount_error();
                    None
                }
                // Quit dialog
                (COMPONENT_RADIO_QUIT, Msg::OnSubmit(Payload::Unsigned(choice))) => {
                    // If choice is 0, quit termscp
                    if *choice == 0 {
                        self.quit = true;
                    }
                    self.umount_quit();
                    None
                }
                // On submit on any unhandled (connect)
                (_, Msg::OnSubmit(_)) | (_, &MSG_KEY_ENTER) => {
                    // Match <ENTER> key for all other components
                    self.save_recent();
                    let (address, port, protocol, username, password) = self.get_input();
                    // TOREM: remove this after removing states
                    self.address = address;
                    self.port = port.to_string();
                    self.protocol = protocol;
                    self.username = username;
                    self.password = password;
                    // Submit true
                    self.submit = true;
                    // Return None
                    None
                }
                // <ESC> => Quit
                (_, &MSG_KEY_ESC) => {
                    self.mount_quit();
                    None
                }
                (_, _) => None, // Ignore other events
            },
        }
    }
}
