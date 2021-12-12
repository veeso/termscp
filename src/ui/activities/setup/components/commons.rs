//! ## Config
//!
//! config tab components

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
use super::{CommonMsg, Msg, ViewLayout};

use tui_realm_stdlib::{List, Paragraph, Radio, Span};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderSides, BorderType, Borders, Color, TableBuilder, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

#[derive(MockComponent)]
pub struct ErrorPopup {
    component: Paragraph,
}

impl ErrorPopup {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(Color::Red)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::Red)
                .text(&[TextSpan::from(text.as_ref())])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for ErrorPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Common(CommonMsg::CloseErrorPopup)),
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct Footer {
    component: Span,
}

impl Default for Footer {
    fn default() -> Self {
        Self {
            component: Span::default().spans(&[
                TextSpan::new("<F1|CTRL+H>").bold().fg(Color::Cyan),
                TextSpan::new(" Help "),
                TextSpan::new("<F4|CTRL+S>").bold().fg(Color::Cyan),
                TextSpan::new(" Save parameters "),
                TextSpan::new("<F10|ESC>").bold().fg(Color::Cyan),
                TextSpan::new(" Exit "),
                TextSpan::new("<TAB>").bold().fg(Color::Cyan),
                TextSpan::new(" Change panel "),
                TextSpan::new("<UP/DOWN>").bold().fg(Color::Cyan),
                TextSpan::new(" Change field "),
            ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for Footer {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct Header {
    component: Radio,
}

impl Header {
    pub fn new(layout: ViewLayout) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(Color::Yellow)
                        .sides(BorderSides::BOTTOM),
                )
                .choices(&["Configuration parameters", "SSH Keys", "Theme"])
                .foreground(Color::Yellow)
                .value(match layout {
                    ViewLayout::SetupForm => 0,
                    ViewLayout::SshKeys => 1,
                    ViewLayout::Theme => 2,
                }),
        }
    }
}

impl Component<Msg, NoUserEvent> for Header {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct Keybindings {
    component: List,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            component: List::default()
                .borders(Borders::default().modifiers(BorderType::Rounded))
                .title("Keybindings", Alignment::Center)
                .scroll(true)
                .highlighted_str("? ")
                .rows(
                    TableBuilder::default()
                        .add_col(TextSpan::new("<ESC>").bold().fg(Color::Cyan))
                        .add_col(TextSpan::from("           Exit setup"))
                        .add_row()
                        .add_col(TextSpan::new("<TAB>").bold().fg(Color::Cyan))
                        .add_col(TextSpan::from("           Change setup page"))
                        .add_row()
                        .add_col(TextSpan::new("<RIGHT/LEFT>").bold().fg(Color::Cyan))
                        .add_col(TextSpan::from("    Change cursor"))
                        .add_row()
                        .add_col(TextSpan::new("<UP/DOWN>").bold().fg(Color::Cyan))
                        .add_col(TextSpan::from("       Change input field"))
                        .add_row()
                        .add_col(TextSpan::new("<ENTER>").bold().fg(Color::Cyan))
                        .add_col(TextSpan::from("         Select / Dismiss popup"))
                        .add_row()
                        .add_col(TextSpan::new("<DEL|E>").bold().fg(Color::Cyan))
                        .add_col(TextSpan::from("         Delete SSH key"))
                        .add_row()
                        .add_col(TextSpan::new("<CTRL+N>").bold().fg(Color::Cyan))
                        .add_col(TextSpan::from("        New SSH key"))
                        .add_row()
                        .add_col(TextSpan::new("<CTRL+R>").bold().fg(Color::Cyan))
                        .add_col(TextSpan::from("        Revert changes"))
                        .add_row()
                        .add_col(TextSpan::new("<CTRL+S>").bold().fg(Color::Cyan))
                        .add_col(TextSpan::from("        Save configuration"))
                        .build(),
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for Keybindings {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Common(CommonMsg::CloseKeybindingsPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => {
                self.perform(Cmd::Move(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => {
                self.perform(Cmd::Scroll(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => {
                self.perform(Cmd::Scroll(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct QuitPopup {
    component: Radio,
}

impl Default for QuitPopup {
    fn default() -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(Color::Red)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::Red)
                .title(
                    "There are unsaved changes! Save changes before leaving?",
                    Alignment::Center,
                )
                .rewind(true)
                .choices(&["Save", "Don't save", "Cancel"]),
        }
    }
}

impl Component<Msg, NoUserEvent> for QuitPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Common(CommonMsg::CloseQuitPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => match self.perform(Cmd::Submit) {
                CmdResult::Submit(State::One(StateValue::Usize(0))) => {
                    Some(Msg::Common(CommonMsg::SaveAndQuit))
                }
                CmdResult::Submit(State::One(StateValue::Usize(1))) => {
                    Some(Msg::Common(CommonMsg::Quit))
                }
                _ => Some(Msg::Common(CommonMsg::CloseQuitPopup)),
            },
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct SavePopup {
    component: Radio,
}

impl Default for SavePopup {
    fn default() -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(Color::Yellow)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::Yellow)
                .title("Save changes?", Alignment::Center)
                .rewind(true)
                .choices(&["Yes", "No"]),
        }
    }
}

impl Component<Msg, NoUserEvent> for SavePopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Common(CommonMsg::CloseSavePopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::One(StateValue::Usize(0)))
                ) {
                    Some(Msg::Common(CommonMsg::SaveConfig))
                } else {
                    Some(Msg::Common(CommonMsg::CloseSavePopup))
                }
            }
            _ => None,
        }
    }
}
