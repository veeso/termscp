//! ## Transfer
//!
//! file transfer components

use super::{Msg, TransferMsg, UiMsg};

mod file_list;
use file_list::FileList;

use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{Alignment, Borders, Color, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

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
                code: Key::Char('m'),
                modifiers: KeyModifiers::NONE,
            }) => {
                let _ = self.perform(Cmd::Toggle);
                Some(Msg::None)
            }
            // -- comp msg
            Event::Keyboard(KeyEvent {
                code: Key::BackTab, ..
            }) => Some(Msg::Ui(UiMsg::ExplorerBackTabbed)),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseFindExplorer))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left | Key::Right | Key::Tab,
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
                .rows(files.iter().map(|x| vec![TextSpan::from(x)]).collect()),
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
                code: Key::Char('m'),
                modifiers: KeyModifiers::NONE,
            }) => {
                let _ = self.perform(Cmd::Toggle);
                Some(Msg::None)
            }
            // -- comp msg
            Event::Keyboard(KeyEvent {
                code: Key::BackTab, ..
            }) => Some(Msg::Ui(UiMsg::ExplorerBackTabbed)),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::ShowDisconnectPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right | Key::Tab,
                ..
            }) => Some(Msg::Ui(UiMsg::ChangeTransferWindow)),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => Some(Msg::Transfer(TransferMsg::GoToPreviousDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Transfer(TransferMsg::EnterDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Char(' '),
                ..
            }) => Some(Msg::Transfer(TransferMsg::TransferFile)),
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
            }) => Some(Msg::Ui(UiMsg::ShowFindPopup)),
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
                .rows(files.iter().map(|x| vec![TextSpan::from(x)]).collect()),
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
                code: Key::Char('m'),
                modifiers: KeyModifiers::NONE,
            }) => {
                let _ = self.perform(Cmd::Toggle);
                Some(Msg::None)
            }
            // -- comp msg
            Event::Keyboard(KeyEvent {
                code: Key::BackTab, ..
            }) => Some(Msg::Ui(UiMsg::ExplorerBackTabbed)),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::ShowDisconnectPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Left | Key::Tab,
                ..
            }) => Some(Msg::Ui(UiMsg::ChangeTransferWindow)),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => Some(Msg::Transfer(TransferMsg::GoToPreviousDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => Some(Msg::Transfer(TransferMsg::EnterDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Char(' '),
                ..
            }) => Some(Msg::Transfer(TransferMsg::TransferFile)),
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
            }) => Some(Msg::Ui(UiMsg::ShowFindPopup)),
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
            _ => None,
        }
    }
}
