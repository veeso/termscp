use std::path::PathBuf;

use tui_realm_stdlib::List;
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

use crate::ui::activities::filetransfer::{MarkQueue, Msg, UiMsg};

#[derive(MockComponent)]
pub struct SelectedFilesList {
    component: List,
    paths: Vec<PathBuf>,
    queue: MarkQueue,
}

impl SelectedFilesList {
    pub fn new(
        paths: &[(PathBuf, PathBuf)],
        queue: MarkQueue,
        color: Color,
        title: &'static str,
    ) -> Self {
        let enqueued_paths = paths
            .iter()
            .map(|(src, _)| src.clone())
            .collect::<Vec<PathBuf>>();

        Self {
            queue,
            paths: enqueued_paths,
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
                .highlighted_str("➤ ")
                .title(title, Alignment::Center)
                .rows(
                    paths
                        .iter()
                        .map(|(src, dest)| {
                            let name = src
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            let dest = dest
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();

                            vec![
                                TextSpan::from(name),
                                TextSpan::from(" -> "),
                                TextSpan::from(dest),
                            ]
                        })
                        .collect(),
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for SelectedFilesList {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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
                code: Key::Right, ..
            }) => Some(Msg::Ui(UiMsg::BottomPanelRight)),
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => Some(Msg::Ui(UiMsg::BottomPanelLeft)),
            Event::Keyboard(KeyEvent {
                code: Key::BackTab | Key::Tab | Key::Char('p'),
                ..
            }) => Some(Msg::Ui(UiMsg::LogBackTabbed)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                // unmark the selected file
                let State::One(StateValue::Usize(idx)) = self.state() else {
                    return None;
                };

                let path = self.paths.get(idx)?;

                Some(Msg::Ui(UiMsg::MarkRemove(self.queue, path.clone())))
            }
            _ => None,
        }
    }
}
