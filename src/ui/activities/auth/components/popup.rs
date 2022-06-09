//! ## Popup
//!
//! auth activity popups

use super::{FormMsg, Msg, UiMsg};

use tui_realm_stdlib::{List, Paragraph, Radio, Textarea};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

// -- error popup

#[derive(MockComponent)]
pub struct ErrorPopup {
    component: Paragraph,
}

impl ErrorPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
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
            }) => Some(Msg::Ui(UiMsg::CloseErrorPopup)),
            _ => None,
        }
    }
}

// -- info popup

#[derive(MockComponent)]
pub struct InfoPopup {
    component: Paragraph,
}

impl InfoPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text(&[TextSpan::from(text.as_ref())])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for InfoPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Ui(UiMsg::CloseInfoPopup)),
            _ => None,
        }
    }
}

// -- wait popup

#[derive(MockComponent)]
pub struct WaitPopup {
    component: Paragraph,
}

impl WaitPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text(&[TextSpan::from(text.as_ref())])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for WaitPopup {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

// -- window size error

#[derive(MockComponent)]
pub struct WindowSizeError {
    component: Paragraph,
}

impl WindowSizeError {
    pub fn new(color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text(&[TextSpan::from(
                    "termscp requires at least 24 lines of height to run",
                )])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for WindowSizeError {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

// -- quit popup

#[derive(MockComponent)]
pub struct QuitPopup {
    component: Radio,
}

impl QuitPopup {
    pub fn new(color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title("Quit termscp?", Alignment::Center)
                .rewind(true)
                .choices(&["Yes", "No"]),
        }
    }
}

impl Component<Msg, NoUserEvent> for QuitPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseQuitPopup))
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
                    Some(Msg::Form(FormMsg::Quit))
                } else {
                    Some(Msg::Ui(UiMsg::CloseQuitPopup))
                }
            }
            _ => None,
        }
    }
}

// -- install update popup

#[derive(MockComponent)]
pub struct InstallUpdatePopup {
    component: Radio,
}

impl InstallUpdatePopup {
    pub fn new(color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title("Install update?", Alignment::Center)
                .rewind(true)
                .choices(&["Yes", "No"]),
        }
    }
}

impl Component<Msg, NoUserEvent> for InstallUpdatePopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseInstallUpdatePopup))
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
                    Some(Msg::Form(FormMsg::InstallUpdate))
                } else {
                    Some(Msg::Ui(UiMsg::CloseInstallUpdatePopup))
                }
            }
            _ => None,
        }
    }
}

// -- release notes popup

#[derive(MockComponent)]
pub struct ReleaseNotes {
    component: Textarea,
}

impl ReleaseNotes {
    pub fn new(notes: &str, color: Color) -> Self {
        Self {
            component: Textarea::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title("Release notes", Alignment::Center)
                .text_rows(
                    notes
                        .lines()
                        .map(TextSpan::from)
                        .collect::<Vec<TextSpan>>()
                        .as_slice(),
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for ReleaseNotes {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Ui(UiMsg::CloseInstallUpdatePopup)),
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

// -- keybindings popup

#[derive(MockComponent)]
pub struct Keybindings {
    component: List,
}

impl Keybindings {
    pub fn new(color: Color) -> Self {
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .highlighted_str("? ")
                .title("Keybindings", Alignment::Center)
                .scroll(true)
                .step(4)
                .rows(
                    TableBuilder::default()
                        .add_col(TextSpan::new("<ESC>").bold().fg(color))
                        .add_col(TextSpan::from("           Quit termscp"))
                        .add_row()
                        .add_col(TextSpan::new("<TAB>").bold().fg(color))
                        .add_col(TextSpan::from("           Switch from form and bookmarks"))
                        .add_row()
                        .add_col(TextSpan::new("<RIGHT/LEFT>").bold().fg(color))
                        .add_col(TextSpan::from("    Switch bookmark tab"))
                        .add_row()
                        .add_col(TextSpan::new("<UP/DOWN>").bold().fg(color))
                        .add_col(TextSpan::from("       Move up/down in current tab"))
                        .add_row()
                        .add_col(TextSpan::new("<ENTER>").bold().fg(color))
                        .add_col(TextSpan::from("         Connect/Load bookmark"))
                        .add_row()
                        .add_col(TextSpan::new("<DEL|E>").bold().fg(color))
                        .add_col(TextSpan::from("         Delete selected bookmark"))
                        .add_row()
                        .add_col(TextSpan::new("<CTRL+C>").bold().fg(color))
                        .add_col(TextSpan::from("        Enter setup"))
                        .add_row()
                        .add_col(TextSpan::new("<CTRL+S>").bold().fg(color))
                        .add_col(TextSpan::from("        Save bookmark"))
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
            }) => Some(Msg::Ui(UiMsg::CloseKeybindingsPopup)),
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
