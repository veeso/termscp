use tui_realm_stdlib::Radio;
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderType, Borders, Color};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

use crate::ui::activities::filetransfer::{Msg, PendingActionMsg};

#[derive(MockComponent)]
pub struct ReplacePopup {
    component: Radio,
}

impl ReplacePopup {
    pub fn new(filename: Option<&str>, color: Color) -> Self {
        let text = match filename {
            Some(f) => format!(r#"File "{f}" already exists. Overwrite file?"#),
            None => "Overwrite files?".to_string(),
        };
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .choices(["Replace", "Skip", "Replace All", "Skip All", "Cancel"])
                .title(text, Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for ReplacePopup {
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
                Some(Msg::PendingAction(PendingActionMsg::ReplaceCancel))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::PendingAction(PendingActionMsg::ReplaceOverwrite)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::PendingAction(PendingActionMsg::ReplaceSkip)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => match self.perform(Cmd::Submit) {
                CmdResult::Submit(State::One(StateValue::Usize(0))) => {
                    Some(Msg::PendingAction(PendingActionMsg::ReplaceOverwrite))
                }
                CmdResult::Submit(State::One(StateValue::Usize(1))) => {
                    Some(Msg::PendingAction(PendingActionMsg::ReplaceSkip))
                }
                CmdResult::Submit(State::One(StateValue::Usize(2))) => {
                    Some(Msg::PendingAction(PendingActionMsg::ReplaceOverwriteAll))
                }
                CmdResult::Submit(State::One(StateValue::Usize(3))) => {
                    Some(Msg::PendingAction(PendingActionMsg::ReplaceSkipAll))
                }
                CmdResult::Submit(State::One(StateValue::Usize(4))) => {
                    Some(Msg::PendingAction(PendingActionMsg::ReplaceCancel))
                }
                _ => Some(Msg::None),
            },
            _ => None,
        }
    }
}
