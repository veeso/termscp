use tui_realm_stdlib::Input;
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, InputType, Style};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

use crate::ui::activities::filetransfer::{Msg, TransferMsg, UiMsg};

#[derive(MockComponent)]
pub struct RenamePopup {
    component: Input,
}

impl RenamePopup {
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
                .placeholder(
                    "/foo/bar/buzz.txt",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("Move file(s) to\u{2026}", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for RenamePopup {
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
                self.perform(Cmd::Type(ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => match self.state() {
                State::One(StateValue::String(i)) => {
                    Some(Msg::Transfer(TransferMsg::RenameFile(i)))
                }
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseRenamePopup))
            }
            _ => None,
        }
    }
}
