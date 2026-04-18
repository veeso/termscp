//! ## Ssh
//!
//! ssh components

use tui_realm_stdlib::components::{Input, List, Radio};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{
    BorderSides, BorderType, Borders, Color, HorizontalAlignment, InputType, SpanStatic, Style,
    TextModifiers, Title,
};
use tuirealm::state::{State, StateValue};

use super::{Msg, SshMsg};

/* DelSshKeyPopup,
SshHost,
SshKeys,
SshUsername, */

#[derive(Component)]
pub struct DelSshKeyPopup {
    component: Radio,
}

impl Default for DelSshKeyPopup {
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
                .choices(["Yes", "No"])
                .rewind(true)
                .title(Title::from("Delete key?").alignment(HorizontalAlignment::Center))
                .value(1),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for DelSshKeyPopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ssh(SshMsg::CloseDelSshKeyPopup))
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
            }) => Some(Msg::Ssh(SshMsg::DeleteSshKey)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ssh(SshMsg::CloseDelSshKeyPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::Single(StateValue::Usize(0)))
                ) {
                    Some(Msg::Ssh(SshMsg::DeleteSshKey))
                } else {
                    Some(Msg::Ssh(SshMsg::CloseDelSshKeyPopup))
                }
            }
            _ => None,
        }
    }
}

#[derive(Component)]
pub struct SshKeys {
    component: List,
}

impl SshKeys {
    pub fn new(keys: &[String]) -> Self {
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .color(Color::LightGreen)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::LightGreen)
                .highlight_style(Style::default().fg(Color::LightGreen))
                .rewind(true)
                .rows(
                    keys.iter()
                        .map(|x| vec![SpanStatic::from(x.clone())])
                        .collect::<Vec<Vec<SpanStatic>>>(),
                )
                .step(4)
                .scroll(true)
                .title(Title::from("SSH Keys").alignment(HorizontalAlignment::Left)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for SshKeys {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
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
            }) => match self.state() {
                State::Single(StateValue::Usize(choice)) => {
                    Some(Msg::Ssh(SshMsg::EditSshKey(choice)))
                }
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent {
                code: Key::Delete | Key::Char('e'),
                ..
            }) => Some(Msg::Ssh(SshMsg::ShowDelSshKeyPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::Ssh(SshMsg::ShowNewSshKeyPopup)),
            _ => None,
        }
    }
}

#[derive(Component)]
pub struct SshHost {
    component: Input,
}

impl Default for SshHost {
    fn default() -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .sides(BorderSides::TOP | BorderSides::RIGHT | BorderSides::LEFT),
                )
                .input_type(InputType::Text)
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "192.168.1.2",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(Title::from("Hostname or address").alignment(HorizontalAlignment::Center)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for SshHost {
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
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                // NOTE: escaped control sequence
                code: Key::Char('h' | 'r' | 's'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::None),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                ..
            }) => {
                self.perform(Cmd::Type(*ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Ssh(SshMsg::SaveSshKey)),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::Ssh(SshMsg::SshHostBlur)),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ssh(SshMsg::CloseNewSshKeyPopup))
            }
            _ => None,
        }
    }
}

#[derive(Component)]
pub struct SshUsername {
    component: Input,
}

impl Default for SshUsername {
    fn default() -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .sides(BorderSides::BOTTOM | BorderSides::RIGHT | BorderSides::LEFT),
                )
                .input_type(InputType::Text)
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "root",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(Title::from("Username").alignment(HorizontalAlignment::Center)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for SshUsername {
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
                code: Key::Delete, ..
            }) => {
                self.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            }) => {
                self.perform(Cmd::Type(*ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Ssh(SshMsg::SaveSshKey)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                Some(Msg::Ssh(SshMsg::SshUsernameBlur))
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ssh(SshMsg::CloseNewSshKeyPopup))
            }
            _ => None,
        }
    }
}
