mod component;
mod history;
mod line;

use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::Color;
use tuirealm::{AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent};

use self::component::TerminalComponent;
use self::line::Line;
use super::Msg;
use crate::ui::activities::filetransfer::{TransferMsg, UiMsg};

#[derive(MockComponent, Default)]
pub struct Terminal {
    component: TerminalComponent,
}

impl Terminal {
    /// Construct a new [`Terminal`] component with the given prompt line.
    pub fn prompt(mut self, prompt: impl ToString) -> Self {
        self.component = self.component.prompt(prompt);
        self
    }

    /// Construct a new [`Terminal`] component with the given title.
    pub fn title(mut self, title: impl ToString) -> Self {
        self.component
            .attr(Attribute::Title, AttrValue::String(title.to_string()));
        self
    }

    /// Construct a new [`Terminal`] component with the foreground color
    pub fn foreground(mut self, color: Color) -> Self {
        self.component
            .attr(Attribute::Foreground, AttrValue::Color(color));
        self
    }
}

impl Component<Msg, NoUserEvent> for Terminal {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseExecPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => match self.component.perform(Cmd::Submit) {
                CmdResult::Submit(state) => {
                    let cmd = state.unwrap_one().unwrap_string();
                    Some(Msg::Transfer(TransferMsg::ExecuteCmd(cmd)))
                }
                _ => None,
            },
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.component.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.component.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => {
                self.component.perform(Cmd::Cancel);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => {
                self.component.perform(Cmd::Delete);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.component.perform(Cmd::Move(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => {
                self.component.perform(Cmd::Move(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => {
                self.component.perform(Cmd::Move(Direction::Left));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => {
                self.component.perform(Cmd::Move(Direction::Right));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Insert, ..
            }) => {
                self.component.perform(Cmd::Toggle);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => {
                self.component.perform(Cmd::Scroll(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => {
                self.component.perform(Cmd::Scroll(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(c), ..
            }) => {
                self.component.perform(Cmd::Type(c));
                Some(Msg::None)
            }
            _ => None,
        }
    }
}
