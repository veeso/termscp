//! ## Bookmarks
//!
//! auth activity bookmarks components

use tui_realm_stdlib::components::{Input, List, Radio};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, KeyModifiers, NoUserEvent};
use tuirealm::props::{
    BorderSides, BorderType, Borders, Color, HorizontalAlignment, InputType, SpanStatic, Style,
    TextModifiers, Title,
};
use tuirealm::state::{State, StateValue};

use super::{FormMsg, Msg, UiMsg};
use crate::ui::activities::auth::FormTab;

// -- bookmark list

#[derive(Component)]
pub struct BookmarksList {
    component: List,
}

impl BookmarksList {
    pub fn new(bookmarks: &[String], color: Color) -> Self {
        Self {
            component: List::default()
                .borders(Borders::default().color(color).modifiers(BorderType::Plain))
                .highlight_style(Style::default().fg(color))
                .rewind(true)
                .scroll(true)
                .step(4)
                .title(Title::from("Bookmarks").alignment(HorizontalAlignment::Left))
                .rows(
                    bookmarks
                        .iter()
                        .map(|x| vec![SpanStatic::from(x.clone())])
                        .collect::<Vec<Vec<SpanStatic>>>(),
                ),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for BookmarksList {
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

#[derive(Component)]
pub struct RecentsList {
    component: List,
}

impl RecentsList {
    pub fn new(bookmarks: &[String], color: Color) -> Self {
        Self {
            component: List::default()
                .borders(Borders::default().color(color).modifiers(BorderType::Plain))
                .highlight_style(Style::default().fg(color))
                .rewind(true)
                .scroll(true)
                .step(4)
                .title(Title::from("Recent connections").alignment(HorizontalAlignment::Left))
                .rows(
                    bookmarks
                        .iter()
                        .map(|x| vec![SpanStatic::from(x.clone())])
                        .collect::<Vec<Vec<SpanStatic>>>(),
                ),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for RecentsList {
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

#[derive(Component)]
pub struct DeleteBookmarkPopup {
    component: Radio,
}

impl DeleteBookmarkPopup {
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
                .value(1)
                .rewind(true)
                .title(
                    Title::from("Delete selected bookmark?").alignment(HorizontalAlignment::Center),
                ),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for DeleteBookmarkPopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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
                    CmdResult::Submit(State::Single(StateValue::Usize(0)))
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

#[derive(Component)]
pub struct DeleteRecentPopup {
    component: Radio,
}

impl DeleteRecentPopup {
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
                .value(1)
                .rewind(true)
                .title(
                    Title::from("Delete selected recent host?")
                        .alignment(HorizontalAlignment::Center),
                ),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for DeleteRecentPopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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
                    CmdResult::Submit(State::Single(StateValue::Usize(0)))
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

#[derive(Component)]
pub struct BookmarkSavePassword {
    component: Radio,
    form_tab: FormTab,
}

impl BookmarkSavePassword {
    pub fn new(form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Radio::default()
                .highlight_style(
                    Style::default()
                        .fg(color)
                        .add_modifier(TextModifiers::REVERSED),
                )
                .borders(
                    Borders::default()
                        .color(Color::Reset)
                        .sides(BorderSides::BOTTOM | BorderSides::LEFT | BorderSides::RIGHT)
                        .modifiers(BorderType::Rounded),
                )
                .choices(["Yes", "No"])
                .value(0)
                .rewind(true)
                .title(Title::from("Save secrets?").alignment(HorizontalAlignment::Center)),
            form_tab,
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for BookmarkSavePassword {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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
            }) => Some(Msg::Form(FormMsg::SaveBookmark(self.form_tab))),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                Some(Msg::Ui(UiMsg::SaveBookmarkPasswordBlur))
            }
            _ => None,
        }
    }
}

// -- new bookmark name

#[derive(Component)]
pub struct BookmarkName {
    component: Input,
    form_tab: FormTab,
}

impl BookmarkName {
    pub fn new(form_tab: FormTab, color: Color) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .color(Color::Reset)
                        .sides(BorderSides::TOP | BorderSides::LEFT | BorderSides::RIGHT)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .title(Title::from("Bookmark name").alignment(HorizontalAlignment::Left))
                .input_type(InputType::Text),
            form_tab,
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for BookmarkName {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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
                self.perform(Cmd::Type(*ch));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Form(FormMsg::SaveBookmark(self.form_tab))),
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => Some(Msg::Ui(UiMsg::BookmarkNameBlur)),
            _ => None,
        }
    }
}
