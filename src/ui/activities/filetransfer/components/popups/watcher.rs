use tui_realm_stdlib::components::{List, Radio};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{
    BorderType, Borders, Color, HorizontalAlignment, SpanStatic, Style, TextModifiers, Title,
};
use tuirealm::state::{State, StateValue};

use crate::ui::activities::filetransfer::{Msg, TransferMsg, UiMsg};

#[derive(Component)]
pub struct WatchedPathsList {
    component: List,
}

impl WatchedPathsList {
    pub fn new(paths: &[std::path::PathBuf], color: Color) -> Self {
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .rewind(true)
                .scroll(true)
                .step(4)
                .highlight_style(Style::default().fg(color))
                .highlight_str("\u{27a4} ")
                .title(
                    Title::from("These files are currently synched with the remote host")
                        .alignment(HorizontalAlignment::Center),
                )
                .rows(
                    paths
                        .iter()
                        .map(|x| vec![SpanStatic::from(x.to_string_lossy().to_string())])
                        .collect::<Vec<Vec<SpanStatic>>>(),
                ),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for WatchedPathsList {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseWatchedPathsList))
            }
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
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                // get state
                if let State::Single(StateValue::Usize(idx)) = self.component.state() {
                    Some(Msg::Transfer(TransferMsg::ToggleWatchFor(idx)))
                } else {
                    Some(Msg::None)
                }
            }
            _ => None,
        }
    }
}

#[derive(Component)]
pub struct WatcherPopup {
    component: Radio,
}

impl WatcherPopup {
    pub fn new(watched: bool, local: &str, remote: &str, color: Color) -> Self {
        let text = match watched {
            false => format!(r#"Synchronize changes from "{local}" to "{remote}"?"#),
            true => format!(r#"Stop synchronizing changes at "{local}"?"#),
        };
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
                .choices(["Yes", "No"])
                .title(Title::from(text).alignment(HorizontalAlignment::Center)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for WatcherPopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
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
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseWatcherPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::ToggleWatch)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::CloseWatcherPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::Single(StateValue::Usize(0)))
                ) {
                    Some(Msg::Transfer(TransferMsg::ToggleWatch))
                } else {
                    Some(Msg::Ui(UiMsg::CloseWatcherPopup))
                }
            }
            _ => None,
        }
    }
}
