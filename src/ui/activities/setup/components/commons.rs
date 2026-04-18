//! ## Config
//!
//! config tab components

use tui_realm_stdlib::components::{List, Paragraph, Radio, Span};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{
    BorderSides, BorderType, Borders, Color, HorizontalAlignment, SpanStatic, Style, TableBuilder,
    TextModifiers, Title,
};
use tuirealm::ratatui::style::Stylize;
use tuirealm::ratatui::text::Text;
use tuirealm::state::{State, StateValue};

use super::{CommonMsg, Msg, ViewLayout};

#[derive(Component)]
pub struct ErrorPopup {
    component: Paragraph,
}

impl ErrorPopup {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            component: Paragraph::default()
                .alignment_horizontal(HorizontalAlignment::Center)
                .borders(
                    Borders::default()
                        .color(Color::Red)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::Red)
                .text(Text::from_iter([SpanStatic::from(
                    text.as_ref().to_string(),
                )]))
                .wrap_trim(true),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for ErrorPopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Common(CommonMsg::CloseErrorPopup)),
            _ => None,
        }
    }
}

#[derive(Component)]
pub struct Footer {
    component: Span,
}

impl Default for Footer {
    fn default() -> Self {
        Self {
            component: Span::default().spans([
                SpanStatic::raw("<F1|CTRL+H>").bold().fg(Color::Cyan),
                SpanStatic::raw(" Help "),
                SpanStatic::raw("<F4|CTRL+S>").bold().fg(Color::Cyan),
                SpanStatic::raw(" Save parameters "),
                SpanStatic::raw("<F10|ESC>").bold().fg(Color::Cyan),
                SpanStatic::raw(" Exit "),
                SpanStatic::raw("<TAB>").bold().fg(Color::Cyan),
                SpanStatic::raw(" Change panel "),
                SpanStatic::raw("<UP/DOWN>").bold().fg(Color::Cyan),
                SpanStatic::raw(" Change field "),
            ]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for Footer {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(Component)]
pub struct Header {
    component: Radio,
}

impl Header {
    pub fn new(layout: ViewLayout) -> Self {
        Self {
            component: Radio::default()
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(TextModifiers::REVERSED),
                )
                .borders(
                    Borders::default()
                        .color(Color::Yellow)
                        .sides(BorderSides::BOTTOM),
                )
                .choices(["Configuration parameters", "SSH Keys", "Theme"])
                .value(match layout {
                    ViewLayout::SetupForm => 0,
                    ViewLayout::SshKeys => 1,
                    ViewLayout::Theme => 2,
                }),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for Header {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(Component)]
pub struct Keybindings {
    component: List,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            component: List::default()
                .borders(Borders::default().modifiers(BorderType::Rounded))
                .title(Title::from("Keybindings").alignment(HorizontalAlignment::Center))
                .scroll(true)
                .highlight_str("? ")
                .rows(
                    TableBuilder::default()
                        .add_col(SpanStatic::raw("<ESC>").bold().fg(Color::Cyan))
                        .add_col(SpanStatic::from("           Exit setup"))
                        .add_row()
                        .add_col(SpanStatic::raw("<TAB>").bold().fg(Color::Cyan))
                        .add_col(SpanStatic::from("           Change setup page"))
                        .add_row()
                        .add_col(SpanStatic::raw("<RIGHT/LEFT>").bold().fg(Color::Cyan))
                        .add_col(SpanStatic::from("    Change cursor"))
                        .add_row()
                        .add_col(SpanStatic::raw("<UP/DOWN>").bold().fg(Color::Cyan))
                        .add_col(SpanStatic::from("       Change input field"))
                        .add_row()
                        .add_col(SpanStatic::raw("<ENTER>").bold().fg(Color::Cyan))
                        .add_col(SpanStatic::from("         Select / Dismiss popup"))
                        .add_row()
                        .add_col(SpanStatic::raw("<DEL|E>").bold().fg(Color::Cyan))
                        .add_col(SpanStatic::from("         Delete SSH key"))
                        .add_row()
                        .add_col(SpanStatic::raw("<CTRL+N>").bold().fg(Color::Cyan))
                        .add_col(SpanStatic::from("        New SSH key"))
                        .add_row()
                        .add_col(SpanStatic::raw("<CTRL+R>").bold().fg(Color::Cyan))
                        .add_col(SpanStatic::from("        Revert changes"))
                        .add_row()
                        .add_col(SpanStatic::raw("<CTRL+S>").bold().fg(Color::Cyan))
                        .add_col(SpanStatic::from("        Save configuration"))
                        .build()
                        .into_iter()
                        .map(|row| row.into_iter().flat_map(|l| l.spans).collect::<Vec<_>>()),
                ),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for Keybindings {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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

#[derive(Component)]
pub struct QuitPopup {
    component: Radio,
}

impl Default for QuitPopup {
    fn default() -> Self {
        Self {
            component: Radio::default()
                .highlight_style(
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(TextModifiers::REVERSED),
                )
                .borders(
                    Borders::default()
                        .color(Color::Red)
                        .modifiers(BorderType::Rounded),
                )
                .title(
                    Title::from("There are unsaved changes! Save changes before leaving?")
                        .alignment(HorizontalAlignment::Center),
                )
                .rewind(true)
                .choices(["Save", "Don't save", "Cancel"]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for QuitPopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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
                CmdResult::Submit(State::Single(StateValue::Usize(0))) => {
                    Some(Msg::Common(CommonMsg::SaveAndQuit))
                }
                CmdResult::Submit(State::Single(StateValue::Usize(1))) => {
                    Some(Msg::Common(CommonMsg::Quit))
                }
                _ => Some(Msg::Common(CommonMsg::CloseQuitPopup)),
            },
            _ => None,
        }
    }
}

#[derive(Component)]
pub struct SavePopup {
    component: Radio,
}

impl Default for SavePopup {
    fn default() -> Self {
        Self {
            component: Radio::default()
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(TextModifiers::REVERSED),
                )
                .borders(
                    Borders::default()
                        .color(Color::Yellow)
                        .modifiers(BorderType::Rounded),
                )
                .title(Title::from("Save changes?").alignment(HorizontalAlignment::Center))
                .rewind(true)
                .choices(["Yes", "No"]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for SavePopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Common(CommonMsg::SaveConfig)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Common(CommonMsg::CloseSavePopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::Single(StateValue::Usize(0)))
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
