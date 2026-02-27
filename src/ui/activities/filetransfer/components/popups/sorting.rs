use tui_realm_stdlib::Radio;
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

use crate::explorer::FileSorting;
use crate::ui::activities::filetransfer::{Msg, UiMsg};

#[derive(MockComponent)]
pub struct SortingPopup {
    component: Radio,
}

impl SortingPopup {
    pub fn new(value: FileSorting, color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .choices(["Name", "Modify time", "Creation time", "Size"])
                .title("Sort files by\u{2026}", Alignment::Center)
                .value(match value {
                    FileSorting::CreationTime => 2,
                    FileSorting::ModifyTime => 1,
                    FileSorting::Name => 0,
                    FileSorting::Size => 3,
                    FileSorting::None => 0,
                }),
        }
    }
}

impl Component<Msg, NoUserEvent> for SortingPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let result = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => return Some(Msg::Ui(UiMsg::CloseFileSortingPopup)),
            _ => return None,
        };
        if let CmdResult::Changed(State::One(StateValue::Usize(i))) = result {
            Some(Msg::Ui(UiMsg::ChangeFileSorting(match i {
                0 => FileSorting::Name,
                1 => FileSorting::ModifyTime,
                2 => FileSorting::CreationTime,
                3 => FileSorting::Size,
                _ => FileSorting::Name,
            })))
        } else {
            Some(Msg::None)
        }
    }
}
