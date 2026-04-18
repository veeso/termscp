use tui_realm_stdlib::components::{Input, Radio};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{
    BorderType, Borders, Color, HorizontalAlignment, InputType, Style, TextModifiers, Title,
};
use tuirealm::state::{State, StateValue};

use crate::ui::activities::filetransfer::{Msg, PendingActionMsg, TransferMsg, UiMsg};

#[derive(Component)]
pub struct MkdirPopup {
    component: Input,
}

impl MkdirPopup {
    pub fn new(color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .input_type(InputType::Text)
                .placeholder(tuirealm::props::SpanStatic::styled(
                    "New directory name",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                ))
                .title(Title::from("directory-name").alignment(HorizontalAlignment::Center)),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for MkdirPopup {
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
                ..
            }) => {
                self.perform(Cmd::Type(*ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => match self.state() {
                State::Single(StateValue::String(i)) => Some(Msg::Transfer(TransferMsg::Mkdir(i))),
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseMkdirPopup))
            }
            _ => None,
        }
    }
}

#[derive(Component)]
pub struct SyncBrowsingMkdirPopup {
    component: Radio,
}

impl SyncBrowsingMkdirPopup {
    pub fn new(color: Color, dir_name: &str) -> Self {
        Self {
            component: Radio::default()
                .highlight_style(Style::default().fg(color).add_modifier(TextModifiers::REVERSED))
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                                .choices(["Yes", "No"])
                .title(
                    Title::from(format!(
                        r#"Sync browsing: directory "{dir_name}" doesn't exist. Do you want to create it?"#
                    ))
                    .alignment(HorizontalAlignment::Center),
                ),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for SyncBrowsingMkdirPopup {
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
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => Some(Msg::PendingAction(
                PendingActionMsg::CloseSyncBrowsingMkdirPopup,
            )),
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::PendingAction(PendingActionMsg::MakePendingDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::PendingAction(
                PendingActionMsg::CloseSyncBrowsingMkdirPopup,
            )),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::Single(StateValue::Usize(0)))
                ) {
                    Some(Msg::PendingAction(PendingActionMsg::MakePendingDirectory))
                } else {
                    Some(Msg::PendingAction(
                        PendingActionMsg::CloseSyncBrowsingMkdirPopup,
                    ))
                }
            }
            _ => None,
        }
    }
}
