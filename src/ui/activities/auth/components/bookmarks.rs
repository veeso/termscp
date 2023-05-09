//! ## Bookmarks
//!
//! auth activity bookmarks components

use tui_realm_stdlib::{Input, List, Radio};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, BorderSides, BorderType, Borders, Color, InputType, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

use super::{FormMsg, Msg, UiMsg};

// -- bookmark list

#[derive(MockComponent)]
pub struct BookmarksList {
    component: List,
}

impl BookmarksList {
    pub fn new(bookmarks: &[String], color: Color) -> Self {
        Self {
            component: List::default()
                .borders(Borders::default().color(color).modifiers(BorderType::Plain))
                .highlighted_color(color)
                .rewind(true)
                .scroll(true)
                .step(4)
                .title("Bookmarks", Alignment::Left)
                .rows(
                    bookmarks
                        .iter()
                        .map(|x| vec![TextSpan::from(x.as_str())])
                        .collect(),
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for BookmarksList {
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
                code: Key::Enter, ..
            }) => match self.state() {
                State::One(StateValue::Usize(choice)) => {
                    Some(Msg::Form(FormMsg::LoadBookmark(choice)))
                }
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => Some(Msg::Ui(UiMsg::BookmarksListBlur)),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                Some(Msg::Ui(UiMsg::BookmarksTabBlur))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => Some(Msg::Ui(UiMsg::ShowDeleteBookmarkPopup)),
            _ => None,
        }
    }
}

// -- recents list

#[derive(MockComponent)]
pub struct RecentsList {
    component: List,
}

impl RecentsList {
    pub fn new(bookmarks: &[String], color: Color) -> Self {
        Self {
            component: List::default()
                .borders(Borders::default().color(color).modifiers(BorderType::Plain))
                .highlighted_color(color)
                .rewind(true)
                .scroll(true)
                .step(4)
                .title("Recent connections", Alignment::Left)
                .rows(
                    bookmarks
                        .iter()
                        .map(|x| vec![TextSpan::from(x.as_str())])
                        .collect(),
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for RecentsList {
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
                code: Key::Enter, ..
            }) => match self.state() {
                State::One(StateValue::Usize(choice)) => {
                    Some(Msg::Form(FormMsg::LoadRecent(choice)))
                }
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => Some(Msg::Ui(UiMsg::RececentsListBlur)),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                Some(Msg::Ui(UiMsg::BookmarksTabBlur))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => Some(Msg::Ui(UiMsg::ShowDeleteRecentPopup)),
            _ => None,
        }
    }
}

// -- delete bookmark

#[derive(MockComponent)]
pub struct DeleteBookmarkPopup {
    component: Radio,
}

impl DeleteBookmarkPopup {
    pub fn new(color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .choices(&["Yes", "No"])
                .value(1)
                .rewind(true)
                .foreground(color)
                .title("Delete selected bookmark?", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for DeleteBookmarkPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseDeleteBookmark))
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
            }) => Some(Msg::Form(FormMsg::DeleteBookmark)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::CloseDeleteBookmark)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::One(StateValue::Usize(0)))
                ) {
                    Some(Msg::Form(FormMsg::DeleteBookmark))
                } else {
                    Some(Msg::Ui(UiMsg::CloseDeleteBookmark))
                }
            }
            _ => None,
        }
    }
}

// -- delete recent

#[derive(MockComponent)]
pub struct DeleteRecentPopup {
    component: Radio,
}

impl DeleteRecentPopup {
    pub fn new(color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .choices(&["Yes", "No"])
                .value(1)
                .rewind(true)
                .foreground(color)
                .title("Delete selected recent host?", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for DeleteRecentPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseDeleteRecent))
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
            }) => Some(Msg::Form(FormMsg::DeleteRecent)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::CloseDeleteRecent)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::One(StateValue::Usize(0)))
                ) {
                    Some(Msg::Form(FormMsg::DeleteRecent))
                } else {
                    Some(Msg::Ui(UiMsg::CloseDeleteRecent))
                }
            }
            _ => None,
        }
    }
}

// -- bookmark name

// -- save password

#[derive(MockComponent)]
pub struct BookmarkSavePassword {
    component: Radio,
}

impl BookmarkSavePassword {
    pub fn new(color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(Color::Reset)
                        .sides(BorderSides::BOTTOM | BorderSides::LEFT | BorderSides::RIGHT)
                        .modifiers(BorderType::Rounded),
                )
                .choices(&["Yes", "No"])
                .value(0)
                .rewind(true)
                .foreground(color)
                .title("Save secrets?", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for BookmarkSavePassword {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseSaveBookmark))
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
                code: Key::Enter, ..
            }) => Some(Msg::Form(FormMsg::SaveBookmark)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                Some(Msg::Ui(UiMsg::SaveBookmarkPasswordBlur))
            }
            _ => None,
        }
    }
}

// -- new bookmark name

#[derive(MockComponent)]
pub struct BookmarkName {
    component: Input,
}

impl BookmarkName {
    pub fn new(color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(Color::Reset)
                        .sides(BorderSides::TOP | BorderSides::LEFT | BorderSides::RIGHT)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title("Bookmark name", Alignment::Left)
                .input_type(InputType::Text),
        }
    }
}

impl Component<Msg, NoUserEvent> for BookmarkName {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseSaveBookmark))
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
            }) => Some(Msg::Form(FormMsg::SaveBookmark)),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::Ui(UiMsg::BookmarkNameBlur)),
            _ => None,
        }
    }
}
