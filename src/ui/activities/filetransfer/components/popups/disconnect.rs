use tui_realm_stdlib::components::Radio;
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{
    BorderType, Borders, Color, HorizontalAlignment, Style, TextModifiers, Title,
};
use tuirealm::state::{State, StateValue};

use crate::ui::activities::filetransfer::{Msg, UiMsg};

#[derive(Component)]
pub struct DisconnectPopup {
    component: Radio,
}

impl DisconnectPopup {
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
                .choices(["Yes", "No"])
                .title(
                    Title::from("Are you sure you want to disconnect?")
                        .alignment(HorizontalAlignment::Center),
                ),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for DisconnectPopup {
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
                Some(Msg::Ui(UiMsg::CloseDisconnectPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::Disconnect)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::CloseDisconnectPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::Single(StateValue::Usize(0)))
                ) {
                    Some(Msg::Ui(UiMsg::Disconnect))
                } else {
                    Some(Msg::Ui(UiMsg::CloseDisconnectPopup))
                }
            }
            _ => None,
        }
    }
}
