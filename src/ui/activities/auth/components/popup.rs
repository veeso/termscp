//! ## Popup
//!
//! auth activity popups

use tui_realm_stdlib::components::{List, Paragraph, Radio, Textarea};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{
    BorderType, Borders, Color, HorizontalAlignment, SpanStatic, Style, TableBuilder,
    TextModifiers, Title,
};
use tuirealm::ratatui::style::Stylize;
use tuirealm::ratatui::text::Text;
use tuirealm::state::{State, StateValue};

use super::{FormMsg, Msg, UiMsg};

// -- error popup

#[derive(Component)]
pub struct ErrorPopup {
    component: Paragraph,
}

impl ErrorPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment_horizontal(HorizontalAlignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
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
            }) => Some(Msg::Ui(UiMsg::CloseErrorPopup)),
            _ => None,
        }
    }
}

// -- info popup

#[derive(Component)]
pub struct InfoPopup {
    component: Paragraph,
}

impl InfoPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment_horizontal(HorizontalAlignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text(Text::from_iter([SpanStatic::from(
                    text.as_ref().to_string(),
                )]))
                .wrap_trim(true),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InfoPopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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

#[derive(Component)]
pub struct WaitPopup {
    component: Paragraph,
}

impl WaitPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment_horizontal(HorizontalAlignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text(Text::from_iter([SpanStatic::from(
                    text.as_ref().to_string(),
                )]))
                .wrap_trim(true),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for WaitPopup {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

// -- window size error

#[derive(Component)]
pub struct WindowSizeError {
    component: Paragraph,
}

impl WindowSizeError {
    pub fn new(color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment_horizontal(HorizontalAlignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text(Text::from_iter([SpanStatic::from(
                    "termscp requires at least 24 lines of height to run",
                )]))
                .wrap_trim(true),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for WindowSizeError {
    fn on(&mut self, _ev: &Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

// -- quit popup

#[derive(Component)]
pub struct QuitPopup {
    component: Radio,
}

impl QuitPopup {
    pub fn new(color: Color) -> Self {
        Self {
            component: Radio::default()
                .highlight_style(
                    Style::default()
                        .fg(color)
                        .add_modifier(TextModifiers::REVERSED),
                )
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .title(Title::from("Quit termscp?").alignment(HorizontalAlignment::Center))
                .rewind(true)
                .choices(["Yes", "No"]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for QuitPopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Form(FormMsg::Quit)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::CloseQuitPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::Single(StateValue::Usize(0)))
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

#[derive(Component)]
pub struct InstallUpdatePopup {
    component: Radio,
}

impl InstallUpdatePopup {
    pub fn new(color: Color) -> Self {
        Self {
            component: Radio::default()
                .highlight_style(
                    Style::default()
                        .fg(color)
                        .add_modifier(TextModifiers::REVERSED),
                )
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .title(Title::from("Install update?").alignment(HorizontalAlignment::Center))
                .rewind(true)
                .choices(["Yes", "No"]),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for InstallUpdatePopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Form(FormMsg::InstallUpdate)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::CloseInstallUpdatePopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::Single(StateValue::Usize(0)))
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

#[derive(Component)]
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
                .title(Title::from("Release notes").alignment(HorizontalAlignment::Center))
                .text_rows(notes.lines().map(|l| SpanStatic::from(l.to_string()))),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for ReleaseNotes {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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

#[derive(Component)]
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
                .highlight_str("? ")
                .title(Title::from("Keybindings").alignment(HorizontalAlignment::Center))
                .scroll(true)
                .step(4)
                .rows(
                    TableBuilder::default()
                        .add_col(SpanStatic::raw("<ESC>").bold().fg(color))
                        .add_col(SpanStatic::from("           Quit termscp"))
                        .add_row()
                        .add_col(SpanStatic::raw("<TAB>").bold().fg(color))
                        .add_col(SpanStatic::from(
                            "           Switch from form and bookmarks",
                        ))
                        .add_row()
                        .add_col(SpanStatic::raw("<RIGHT/LEFT>").bold().fg(color))
                        .add_col(SpanStatic::from("    Switch bookmark tab"))
                        .add_row()
                        .add_col(SpanStatic::raw("<UP/DOWN>").bold().fg(color))
                        .add_col(SpanStatic::from("       Move up/down in current tab"))
                        .add_row()
                        .add_col(SpanStatic::raw("<ENTER>").bold().fg(color))
                        .add_col(SpanStatic::from("         Connect/Load bookmark"))
                        .add_row()
                        .add_col(SpanStatic::raw("<DEL|E>").bold().fg(color))
                        .add_col(SpanStatic::from("         Delete selected bookmark"))
                        .add_row()
                        .add_col(SpanStatic::raw("<CTRL+C>").bold().fg(color))
                        .add_col(SpanStatic::from("        Enter setup"))
                        .add_row()
                        .add_col(SpanStatic::raw("<CTRL+S>").bold().fg(color))
                        .add_col(SpanStatic::from("        Save bookmark"))
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
