//! ## Transfer
//!
//! file transfer components

mod file_list;
mod file_list_with_search;

use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, Borders, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};

use self::file_list::FileList;
use self::file_list_with_search::FileListWithSearch;
use super::{Msg, TransferMsg, UiMsg};

#[derive(MockComponent)]
pub struct ExplorerFuzzy {
    component: FileListWithSearch,
}

impl ExplorerFuzzy {
    pub fn new<S: AsRef<str>>(title: S, files: &[&str], bg: Color, fg: Color, hg: Color) -> Self {
        Self {
            component: FileListWithSearch::default()
                .background(bg)
                .borders(Borders::default().color(hg))
                .foreground(fg)
                .highlighted_color(hg)
                .title(title, Alignment::Left)
                .rows(files.iter().map(|x| vec![TextSpan::from(x)]).collect()),
        }
    }

    fn on_search(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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
                code: Key::Tab | Key::Up | Key::Down,
                ..
            }) => {
                self.perform(Cmd::Change);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                ..
            }) => match self.perform(Cmd::Type(ch)) {
                CmdResult::Changed(State::One(StateValue::String(search))) => {
                    Some(Msg::Ui(UiMsg::FuzzySearch(search)))
                }
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseFindExplorer))
            }
            _ => None,
        }
    }

    fn on_file_list(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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
                code: Key::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_SELECT_ALL));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::ALT,
            }) => {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_DESELECT_ALL));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('m'),
                modifiers: KeyModifiers::NONE,
            }) => {
                let _ = self.perform(Cmd::Toggle);
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                self.perform(Cmd::Change);
                Some(Msg::None)
            }
            // -- comp msg
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseFindExplorer))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left | Key::Right,
                ..
            }) => Some(Msg::Ui(UiMsg::ChangeTransferWindow)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Transfer(TransferMsg::EnterDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Char(' '),
                ..
            }) => Some(Msg::Transfer(TransferMsg::TransferFile)),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => Some(Msg::Transfer(TransferMsg::GoToPreviousDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ToggleHiddenFiles)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('b'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowFileSortingPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('e') | Key::Delete | Key::Function(8),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowDeletePopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('i'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowFileInfoPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('s') | Key::Function(2),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowSaveAsPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('v') | Key::Function(3),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::OpenFile)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('w'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowOpenWithPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('z'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowChmodPopup)),
            _ => None,
        }
    }
}

impl Component<Msg, NoUserEvent> for ExplorerFuzzy {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match self.component.focus() {
            file_list_with_search::Focus::List => self.on_file_list(ev),
            file_list_with_search::Focus::Search => self.on_search(ev),
        }
    }
}

#[derive(MockComponent)]
pub struct ExplorerFind {
    component: FileList,
}

impl ExplorerFind {
    pub fn new<S: AsRef<str>>(title: S, files: &[&str], bg: Color, fg: Color, hg: Color) -> Self {
        Self {
            component: FileList::default()
                .background(bg)
                .borders(Borders::default().color(hg))
                .foreground(fg)
                .highlighted_color(hg)
                .title(title, Alignment::Left)
                .rows(files.iter().map(|x| vec![TextSpan::from(x)]).collect()),
        }
    }
}

impl Component<Msg, NoUserEvent> for ExplorerFind {
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
                code: Key::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_SELECT_ALL));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::ALT,
            }) => {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_DESELECT_ALL));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('m'),
                modifiers: KeyModifiers::NONE,
            }) => {
                let _ = self.perform(Cmd::Toggle);
                Some(Msg::None)
            }
            // -- comp msg
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseFindExplorer))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left | Key::Right | Key::Tab | Key::BackTab,
                ..
            }) => Some(Msg::Ui(UiMsg::ChangeTransferWindow)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Transfer(TransferMsg::EnterDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Char(' '),
                ..
            }) => Some(Msg::Transfer(TransferMsg::TransferFile)),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => Some(Msg::Transfer(TransferMsg::GoToPreviousDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ToggleHiddenFiles)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('b'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowFileSortingPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('e') | Key::Delete | Key::Function(8),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowDeletePopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('i'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowFileInfoPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('s') | Key::Function(2),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowSaveAsPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('v') | Key::Function(3),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::OpenFile)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('w'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowOpenWithPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('z'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowChmodPopup)),
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct ExplorerLocal {
    component: FileList,
}

impl ExplorerLocal {
    pub fn new<S: AsRef<str>>(title: S, files: &[&str], bg: Color, fg: Color, hg: Color) -> Self {
        Self {
            component: FileList::default()
                .background(bg)
                .borders(Borders::default().color(hg))
                .foreground(fg)
                .highlighted_color(hg)
                .title(title, Alignment::Left)
                .rows(files.iter().map(|x| vec![TextSpan::from(x)]).collect())
                .dot_dot(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for ExplorerLocal {
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
                code: Key::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_SELECT_ALL));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::ALT,
            }) => {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_DESELECT_ALL));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('m'),
                modifiers: KeyModifiers::NONE,
            }) => {
                let _ = self.perform(Cmd::Toggle);
                Some(Msg::None)
            }
            // -- comp msg
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::ShowDisconnectPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right | Key::Tab | Key::BackTab,
                ..
            }) => Some(Msg::Ui(UiMsg::ChangeTransferWindow)),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => Some(Msg::Transfer(TransferMsg::GoToPreviousDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(self.component.state(), State::One(StateValue::String(_))) {
                    Some(Msg::Transfer(TransferMsg::GoToParentDirectory))
                } else {
                    Some(Msg::Transfer(TransferMsg::EnterDirectory))
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(' '),
                ..
            }) => {
                if matches!(self.component.state(), State::One(StateValue::String(_))) {
                    Some(Msg::None)
                } else {
                    Some(Msg::Transfer(TransferMsg::TransferFile))
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ToggleHiddenFiles)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('b'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowFileSortingPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('c') | Key::Function(5),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowCopyPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('d') | Key::Function(7),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowMkdirPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('e') | Key::Delete | Key::Function(8),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowDeletePopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('f'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::InitFuzzySearch)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('g'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowGotoPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('i'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowFileInfoPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('k'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowSymlinkPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('l'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::ReloadDir)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowNewFilePopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('o') | Key::Function(4),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::OpenTextFile)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('p'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowLogPanel)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('r') | Key::Function(6),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowRenamePopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('s') | Key::Function(2),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowSaveAsPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('t'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowWatcherPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('t'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::Ui(UiMsg::ShowWatchedPathsList)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('u'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::GoToParentDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('x'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowExecPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ToggleSyncBrowsing)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('v') | Key::Function(3),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::OpenFile)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('w'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowOpenWithPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('z'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowChmodPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('/'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowFilterPopup)),
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct ExplorerRemote {
    component: FileList,
}

impl ExplorerRemote {
    pub fn new<S: AsRef<str>>(title: S, files: &[&str], bg: Color, fg: Color, hg: Color) -> Self {
        Self {
            component: FileList::default()
                .background(bg)
                .borders(Borders::default().color(hg))
                .foreground(fg)
                .highlighted_color(hg)
                .title(title, Alignment::Left)
                .rows(files.iter().map(|x| vec![TextSpan::from(x)]).collect())
                .dot_dot(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for ExplorerRemote {
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
                code: Key::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            }) => {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_SELECT_ALL));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::ALT,
            }) => {
                let _ = self.perform(Cmd::Custom(file_list::FILE_LIST_CMD_DESELECT_ALL));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('m'),
                modifiers: KeyModifiers::NONE,
            }) => {
                let _ = self.perform(Cmd::Toggle);
                Some(Msg::None)
            }
            // -- comp msg
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::ShowDisconnectPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left | Key::Tab | Key::BackTab,
                ..
            }) => Some(Msg::Ui(UiMsg::ChangeTransferWindow)),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => Some(Msg::Transfer(TransferMsg::GoToPreviousDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(self.component.state(), State::One(StateValue::String(_))) {
                    Some(Msg::Transfer(TransferMsg::GoToParentDirectory))
                } else {
                    Some(Msg::Transfer(TransferMsg::EnterDirectory))
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(' '),
                ..
            }) => {
                if matches!(self.component.state(), State::One(StateValue::String(_))) {
                    Some(Msg::None)
                } else {
                    Some(Msg::Transfer(TransferMsg::TransferFile))
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('a'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ToggleHiddenFiles)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('b'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowFileSortingPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('c') | Key::Function(5),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowCopyPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('d') | Key::Function(7),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowMkdirPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('e') | Key::Delete | Key::Function(8),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowDeletePopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('f'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::InitFuzzySearch)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('g'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowGotoPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('i'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowFileInfoPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('k'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowSymlinkPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('l'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::ReloadDir)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowNewFilePopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('o') | Key::Function(4),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::OpenTextFile)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('p'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowLogPanel)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('r') | Key::Function(6),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowRenamePopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('s') | Key::Function(2),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowSaveAsPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('t'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowWatcherPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('t'),
                modifiers: KeyModifiers::CONTROL,
            }) => Some(Msg::Ui(UiMsg::ShowWatchedPathsList)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('u'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::GoToParentDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('x'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowExecPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ToggleSyncBrowsing)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('v') | Key::Function(3),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::OpenFile)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('w'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowOpenWithPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('z'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowChmodPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('/'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::ShowFilterPopup)),
            _ => None,
        }
    }
}
