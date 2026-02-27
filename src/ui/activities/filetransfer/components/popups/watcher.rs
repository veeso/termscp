use tui_realm_stdlib::{List, Radio};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

use crate::ui::activities::filetransfer::{Msg, TransferMsg, UiMsg};

#[derive(MockComponent)]
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
                .highlighted_color(color)
                .highlighted_str("\u{27a4} ")
                .title(
                    "These files are currently synched with the remote host",
                    Alignment::Center,
                )
                .rows(
                    paths
                        .iter()
                        .map(|x| vec![TextSpan::from(x.to_string_lossy().to_string())])
                        .collect(),
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for WatchedPathsList {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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
                if let State::One(StateValue::Usize(idx)) = self.component.state() {
                    Some(Msg::Transfer(TransferMsg::ToggleWatchFor(idx)))
                } else {
                    Some(Msg::None)
                }
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
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
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .choices(["Yes", "No"])
                .title(text, Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for WatcherPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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
                    CmdResult::Submit(State::One(StateValue::Usize(0)))
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
