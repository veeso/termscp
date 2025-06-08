//! ## Popups
//!
//! popups components

mod chmod;
mod goto;

use std::time::UNIX_EPOCH;

use bytesize::ByteSize;
use remotefs::File;
use tui_realm_stdlib::{Input, List, Paragraph, ProgressBar, Radio, Span};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::props::{
    Alignment, BorderSides, BorderType, Borders, Color, InputType, Style, TableBuilder, TextSpan,
};
use tuirealm::{Component, Event, MockComponent, NoUserEvent, State, StateValue};
#[cfg(posix)]
use uzers::{get_group_by_gid, get_user_by_uid};

pub use self::chmod::ChmodPopup;
pub use self::goto::{ATTR_FILES, GotoPopup};
use super::super::Browser;
use super::{Msg, PendingActionMsg, TransferMsg, UiMsg};
use crate::explorer::FileSorting;
use crate::utils::fmt::fmt_time;

#[derive(MockComponent)]
pub struct CopyPopup {
    component: Input,
}

impl CopyPopup {
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
                    "destination",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("Copy file(s) to…", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for CopyPopup {
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
                    Some(Msg::Transfer(TransferMsg::CopyFileTo(i)))
                }
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseCopyPopup))
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct FilterPopup {
    component: Input,
}

impl FilterPopup {
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
                    "regex or wildmatch",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title(
                    "Filter files by regex or wildmatch in the current directory",
                    Alignment::Center,
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for FilterPopup {
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
                State::One(StateValue::String(filter)) => Some(Msg::Ui(UiMsg::FilterFiles(filter))),
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseFilterPopup))
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct DeletePopup {
    component: Radio,
}

impl DeletePopup {
    pub fn new(color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .choices(["Yes", "No"])
                .value(1)
                .title("Delete file(s)?", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for DeletePopup {
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
                Some(Msg::Ui(UiMsg::CloseDeletePopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::DeleteFile)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::CloseDeletePopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::One(StateValue::Usize(0)))
                ) {
                    Some(Msg::Transfer(TransferMsg::DeleteFile))
                } else {
                    Some(Msg::Ui(UiMsg::CloseDeletePopup))
                }
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct DisconnectPopup {
    component: Radio,
}

impl DisconnectPopup {
    pub fn new(color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .choices(["Yes", "No"])
                .title("Are you sure you want to disconnect?", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for DisconnectPopup {
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
                    CmdResult::Submit(State::One(StateValue::Usize(0)))
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

#[derive(MockComponent)]
pub struct ErrorPopup {
    component: Paragraph,
}

impl ErrorPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text([TextSpan::from(text.as_ref())])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for ErrorPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Ui(UiMsg::CloseErrorPopup)),
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct ExecPopup {
    component: Input,
}

impl ExecPopup {
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
                .placeholder("ps a", Style::default().fg(Color::Rgb(128, 128, 128)))
                .title("Execute command", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for ExecPopup {
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
                    Some(Msg::Transfer(TransferMsg::ExecuteCmd(i)))
                }
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseExecPopup))
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct FatalPopup {
    component: Paragraph,
}

impl FatalPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text([TextSpan::from(text.as_ref())])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for FatalPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Ui(UiMsg::CloseFatalPopup)),
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct FileInfoPopup {
    component: List,
}

impl FileInfoPopup {
    pub fn new(file: &File) -> Self {
        let mut texts: TableBuilder = TableBuilder::default();
        // Abs path
        let real_path = file.metadata().symlink.as_deref();
        let path: String = match real_path {
            Some(symlink) => format!("{} -> {}", file.path().display(), symlink.display()),
            None => format!("{}", file.path().display()),
        };
        // Make texts
        texts
            .add_col(TextSpan::from("Path: "))
            .add_col(TextSpan::new(path.as_str()).fg(Color::Yellow));
        if let Some(filetype) = file.extension() {
            texts
                .add_row()
                .add_col(TextSpan::from("File type: "))
                .add_col(TextSpan::new(filetype).fg(Color::LightGreen));
        }
        let (bsize, size): (ByteSize, u64) = (ByteSize(file.metadata().size), file.metadata().size);
        texts
            .add_row()
            .add_col(TextSpan::from("Size: "))
            .add_col(TextSpan::new(format!("{bsize} ({size})").as_str()).fg(Color::Cyan));
        let atime: String = fmt_time(
            file.metadata().accessed.unwrap_or(UNIX_EPOCH),
            "%b %d %Y %H:%M:%S",
        );
        let ctime: String = fmt_time(
            file.metadata().created.unwrap_or(UNIX_EPOCH),
            "%b %d %Y %H:%M:%S",
        );
        let mtime: String = fmt_time(
            file.metadata().modified.unwrap_or(UNIX_EPOCH),
            "%b %d %Y %H:%M:%S",
        );
        texts
            .add_row()
            .add_col(TextSpan::from("Creation time: "))
            .add_col(TextSpan::new(ctime.as_str()).fg(Color::LightGreen));
        texts
            .add_row()
            .add_col(TextSpan::from("Last modified time: "))
            .add_col(TextSpan::new(mtime.as_str()).fg(Color::LightBlue));
        texts
            .add_row()
            .add_col(TextSpan::from("Last access time: "))
            .add_col(TextSpan::new(atime.as_str()).fg(Color::LightRed));
        // User
        #[cfg(posix)]
        let username: String = match file.metadata().uid {
            Some(uid) => match get_user_by_uid(uid) {
                Some(user) => user.name().to_string_lossy().to_string(),
                None => uid.to_string(),
            },
            None => String::from("0"),
        };
        #[cfg(win)]
        let username: String = format!("{}", file.metadata().uid.unwrap_or(0));
        // Group
        #[cfg(posix)]
        let group: String = match file.metadata().gid {
            Some(gid) => match get_group_by_gid(gid) {
                Some(group) => group.name().to_string_lossy().to_string(),
                None => gid.to_string(),
            },
            None => String::from("0"),
        };
        #[cfg(win)]
        let group: String = format!("{}", file.metadata().gid.unwrap_or(0));
        texts
            .add_row()
            .add_col(TextSpan::from("User: "))
            .add_col(TextSpan::new(username.as_str()).fg(Color::LightYellow));
        texts
            .add_row()
            .add_col(TextSpan::from("Group: "))
            .add_col(TextSpan::new(group.as_str()).fg(Color::Blue));
        Self {
            component: List::default()
                .borders(Borders::default().modifiers(BorderType::Rounded))
                .scroll(false)
                .title(file.name(), Alignment::Left)
                .rows(texts.build()),
        }
    }
}

impl Component<Msg, NoUserEvent> for FileInfoPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Ui(UiMsg::CloseFileInfoPopup)),
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct KeybindingsPopup {
    component: List,
}

impl KeybindingsPopup {
    pub fn new(key_color: Color) -> Self {
        Self {
            component: List::default()
                .borders(Borders::default().modifiers(BorderType::Rounded))
                .scroll(true)
                .step(8)
                .highlighted_str("? ")
                .title("Keybindings", Alignment::Center)
                .rewind(true)
                .rows(
                    TableBuilder::default()
                        .add_col(TextSpan::new("<ESC>").bold().fg(key_color))
                        .add_col(TextSpan::from("             Disconnect"))
                        .add_row()
                        .add_col(TextSpan::new("<BACKSPACE>").bold().fg(key_color))
                        .add_col(TextSpan::from("       Go to previous directory"))
                        .add_row()
                        .add_col(TextSpan::new("<TAB|RIGHT|LEFT>").bold().fg(key_color))
                        .add_col(TextSpan::from("  Change explorer tab"))
                        .add_row()
                        .add_col(TextSpan::new("<UP/DOWN>").bold().fg(key_color))
                        .add_col(TextSpan::from("         Move up/down in list"))
                        .add_row()
                        .add_col(TextSpan::new("<ENTER>").bold().fg(key_color))
                        .add_col(TextSpan::from("           Enter directory"))
                        .add_row()
                        .add_col(TextSpan::new("<SPACE>").bold().fg(key_color))
                        .add_col(TextSpan::from("           Upload/Download file"))
                        .add_row()
                        .add_col(TextSpan::new("<BACKTAB>").bold().fg(key_color))
                        .add_col(TextSpan::from(
                            "         Switch between explorer and log window",
                        ))
                        .add_row()
                        .add_col(TextSpan::new("<A>").bold().fg(key_color))
                        .add_col(TextSpan::from("               Toggle hidden files"))
                        .add_row()
                        .add_col(TextSpan::new("<B>").bold().fg(key_color))
                        .add_col(TextSpan::from("               Change file sorting mode"))
                        .add_row()
                        .add_col(TextSpan::new("<C|F5>").bold().fg(key_color))
                        .add_col(TextSpan::from("            Copy"))
                        .add_row()
                        .add_col(TextSpan::new("<D|F7>").bold().fg(key_color))
                        .add_col(TextSpan::from("            Make directory"))
                        .add_row()
                        .add_col(TextSpan::new("<F>").bold().fg(key_color))
                        .add_col(TextSpan::from("               Search files"))
                        .add_row()
                        .add_col(TextSpan::new("<G>").bold().fg(key_color))
                        .add_col(TextSpan::from("               Go to path"))
                        .add_row()
                        .add_col(TextSpan::new("<H|F1>").bold().fg(key_color))
                        .add_col(TextSpan::from("            Show help"))
                        .add_row()
                        .add_col(TextSpan::new("<I>").bold().fg(key_color))
                        .add_col(TextSpan::from(
                            "               Show info about selected file",
                        ))
                        .add_row()
                        .add_col(TextSpan::new("<K>").bold().fg(key_color))
                        .add_col(TextSpan::from(
                            "               Create symlink pointing to the current selected entry",
                        ))
                        .add_row()
                        .add_col(TextSpan::new("<L>").bold().fg(key_color))
                        .add_col(TextSpan::from("               Reload directory content"))
                        .add_row()
                        .add_col(TextSpan::new("<M>").bold().fg(key_color))
                        .add_col(TextSpan::from("               Select file"))
                        .add_row()
                        .add_col(TextSpan::new("<N>").bold().fg(key_color))
                        .add_col(TextSpan::from("               Create new file"))
                        .add_row()
                        .add_col(TextSpan::new("<O|F4>").bold().fg(key_color))
                        .add_col(TextSpan::from(
                            "            Open text file with preferred editor",
                        ))
                        .add_row()
                        .add_col(TextSpan::new("<P>").bold().fg(key_color))
                        .add_col(TextSpan::from("               Toggle bottom panel"))
                        .add_row()
                        .add_col(TextSpan::new("<Q|F10>").bold().fg(key_color))
                        .add_col(TextSpan::from("           Quit termscp"))
                        .add_row()
                        .add_col(TextSpan::new("<R|F6>").bold().fg(key_color))
                        .add_col(TextSpan::from("            Rename file"))
                        .add_row()
                        .add_col(TextSpan::new("<S|F2>").bold().fg(key_color))
                        .add_col(TextSpan::from("            Save file as"))
                        .add_row()
                        .add_col(TextSpan::new("<T>").bold().fg(key_color))
                        .add_col(TextSpan::from("               Watch/unwatch file changes"))
                        .add_row()
                        .add_col(TextSpan::new("<U>").bold().fg(key_color))
                        .add_col(TextSpan::from("               Go to parent directory"))
                        .add_row()
                        .add_col(TextSpan::new("<V|F3>").bold().fg(key_color))
                        .add_col(TextSpan::from(
                            "            Open file with default application for file type",
                        ))
                        .add_row()
                        .add_col(TextSpan::new("<W>").bold().fg(key_color))
                        .add_col(TextSpan::from(
                            "               Open file with specified application",
                        ))
                        .add_row()
                        .add_col(TextSpan::new("<X>").bold().fg(key_color))
                        .add_col(TextSpan::from("               Execute shell command"))
                        .add_row()
                        .add_col(TextSpan::new("<Y>").bold().fg(key_color))
                        .add_col(TextSpan::from(
                            "               Toggle synchronized browsing",
                        ))
                        .add_row()
                        .add_col(TextSpan::new("<Z>").bold().fg(key_color))
                        .add_col(TextSpan::from("               Change file permissions"))
                        .add_row()
                        .add_col(TextSpan::new("</>").bold().fg(key_color))
                        .add_col(TextSpan::from("               Filter files"))
                        .add_row()
                        .add_col(TextSpan::new("<DEL|F8|E>").bold().fg(key_color))
                        .add_col(TextSpan::from("        Delete selected file"))
                        .add_row()
                        .add_col(TextSpan::new("<CTRL+A>").bold().fg(key_color))
                        .add_col(TextSpan::from("          Select all files"))
                        .add_row()
                        .add_col(TextSpan::new("<ALT+A>").bold().fg(key_color))
                        .add_col(TextSpan::from("          Deselect all files"))
                        .add_row()
                        .add_col(TextSpan::new("<CTRL+C>").bold().fg(key_color))
                        .add_col(TextSpan::from("          Interrupt file transfer"))
                        .add_row()
                        .add_col(TextSpan::new("<CTRL+T>").bold().fg(key_color))
                        .add_col(TextSpan::from("          Show watched paths"))
                        .build(),
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for KeybindingsPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Esc | Key::Enter,
                ..
            }) => Some(Msg::Ui(UiMsg::CloseKeybindingsPopup)),
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
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct MkdirPopup {
    component: Input,
}

impl MkdirPopup {
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
                    "New directory name",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("directory-name", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for MkdirPopup {
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
                State::One(StateValue::String(i)) => Some(Msg::Transfer(TransferMsg::Mkdir(i))),
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseMkdirPopup))
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct NewfilePopup {
    component: Input,
}

impl NewfilePopup {
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
                    "New file name",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("file.txt", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for NewfilePopup {
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
                State::One(StateValue::String(i)) => Some(Msg::Transfer(TransferMsg::NewFile(i))),
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseNewFilePopup))
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct OpenWithPopup {
    component: Input,
}

impl OpenWithPopup {
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
                    "Open file with…",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("Type the program to open the file with", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for OpenWithPopup {
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
                    Some(Msg::Transfer(TransferMsg::OpenFileWith(i)))
                }
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseOpenWithPopup))
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct ProgressBarFull {
    component: ProgressBar,
}

impl ProgressBarFull {
    pub fn new<S: Into<String>>(prog: f64, label: S, title: S, color: Color) -> Self {
        Self {
            component: ProgressBar::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .sides(BorderSides::TOP | BorderSides::LEFT | BorderSides::RIGHT),
                )
                .foreground(color)
                .label(label)
                .progress(prog)
                .title(title, Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for ProgressBarFull {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        if matches!(
            ev,
            Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers: KeyModifiers::CONTROL
            })
        ) {
            Some(Msg::Transfer(TransferMsg::AbortTransfer))
        } else {
            None
        }
    }
}

#[derive(MockComponent)]
pub struct ProgressBarPartial {
    component: ProgressBar,
}

impl ProgressBarPartial {
    pub fn new<S: Into<String>>(prog: f64, label: S, title: S, color: Color) -> Self {
        Self {
            component: ProgressBar::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .sides(BorderSides::BOTTOM | BorderSides::LEFT | BorderSides::RIGHT),
                )
                .foreground(color)
                .label(label)
                .progress(prog)
                .title(title, Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for ProgressBarPartial {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        if matches!(
            ev,
            Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers: KeyModifiers::CONTROL
            })
        ) {
            Some(Msg::Transfer(TransferMsg::AbortTransfer))
        } else {
            None
        }
    }
}

#[derive(MockComponent)]
pub struct QuitPopup {
    component: Radio,
}

impl QuitPopup {
    pub fn new(color: Color) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .choices(["Yes", "No"])
                .title("Are you sure you want to quit termscp?", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for QuitPopup {
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
                Some(Msg::Ui(UiMsg::CloseQuitPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::Quit)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::CloseQuitPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::One(StateValue::Usize(0)))
                ) {
                    Some(Msg::Ui(UiMsg::Quit))
                } else {
                    Some(Msg::Ui(UiMsg::CloseQuitPopup))
                }
            }
            _ => None,
        }
    }
}

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
                .title("Move file(s) to…", Alignment::Center),
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
                .choices(["Yes", "No"])
                .title(text, Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for ReplacePopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                Some(Msg::Ui(UiMsg::ReplacePopupTabbed))
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
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::PendingAction(PendingActionMsg::CloseReplacePopups))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::PendingAction(PendingActionMsg::TransferPendingFile)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::PendingAction(PendingActionMsg::CloseReplacePopups)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::One(StateValue::Usize(0)))
                ) {
                    Some(Msg::PendingAction(PendingActionMsg::TransferPendingFile))
                } else {
                    Some(Msg::PendingAction(PendingActionMsg::CloseReplacePopups))
                }
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct ReplacingFilesListPopup {
    component: List,
}

impl ReplacingFilesListPopup {
    pub fn new(files: &[String], color: Color) -> Self {
        Self {
            component: List::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .scroll(true)
                .step(4)
                .highlighted_color(color)
                .highlighted_str("➤ ")
                .title(
                    "The following files are going to be replaced",
                    Alignment::Center,
                )
                .rows(files.iter().map(|x| vec![TextSpan::from(x)]).collect()),
        }
    }
}

impl Component<Msg, NoUserEvent> for ReplacingFilesListPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::PendingAction(PendingActionMsg::CloseReplacePopups))
            }
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                Some(Msg::Ui(UiMsg::ReplacePopupTabbed))
            }
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
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct SaveAsPopup {
    component: Input,
}

impl SaveAsPopup {
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
                .title("Save as…", Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for SaveAsPopup {
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
                    Some(Msg::Transfer(TransferMsg::SaveFileAs(i)))
                }
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseSaveAsPopup))
            }
            _ => None,
        }
    }
}

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
                .title("Sort files by…", Alignment::Center)
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

#[derive(MockComponent)]
pub struct StatusBarLocal {
    component: Span,
}

impl StatusBarLocal {
    pub fn new(browser: &Browser, sorting_color: Color, hidden_color: Color) -> Self {
        let file_sorting = file_sorting_label(browser.host_bridge().file_sorting);
        let hidden_files = hidden_files_label(browser.host_bridge().hidden_files_visible());
        Self {
            component: Span::default().spans([
                TextSpan::new("File sorting: ").fg(sorting_color),
                TextSpan::new(file_sorting).fg(sorting_color).reversed(),
                TextSpan::new(" Hidden files: ").fg(hidden_color),
                TextSpan::new(hidden_files).fg(hidden_color).reversed(),
            ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for StatusBarLocal {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct StatusBarRemote {
    component: Span,
}

impl StatusBarRemote {
    pub fn new(
        browser: &Browser,
        sorting_color: Color,
        hidden_color: Color,
        sync_color: Color,
    ) -> Self {
        let file_sorting = file_sorting_label(browser.remote().file_sorting);
        let hidden_files = hidden_files_label(browser.remote().hidden_files_visible());
        let sync_browsing = match browser.sync_browsing {
            true => "ON ",
            false => "OFF",
        };
        Self {
            component: Span::default().spans([
                TextSpan::new("File sorting: ").fg(sorting_color),
                TextSpan::new(file_sorting).fg(sorting_color).reversed(),
                TextSpan::new(" Hidden files: ").fg(hidden_color),
                TextSpan::new(hidden_files).fg(hidden_color).reversed(),
                TextSpan::new(" Sync browsing: ").fg(sync_color),
                TextSpan::new(sync_browsing).fg(sync_color).reversed(),
            ]),
        }
    }
}

impl Component<Msg, NoUserEvent> for StatusBarRemote {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

fn file_sorting_label(sorting: FileSorting) -> &'static str {
    match sorting {
        FileSorting::CreationTime => "By creation time",
        FileSorting::ModifyTime => "By modify time",
        FileSorting::Name => "By name",
        FileSorting::Size => "By size",
        FileSorting::None => "",
    }
}

fn hidden_files_label(visible: bool) -> &'static str {
    match visible {
        true => "Show",
        false => "Hide",
    }
}

#[derive(MockComponent)]
pub struct SymlinkPopup {
    component: Input,
}

impl SymlinkPopup {
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
                    "Symlink name",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title(
                    "Create a symlink pointing to the selected entry",
                    Alignment::Center,
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for SymlinkPopup {
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
                    Some(Msg::Transfer(TransferMsg::CreateSymlink(i)))
                }
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseSymlinkPopup))
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct SyncBrowsingMkdirPopup {
    component: Radio,
}

impl SyncBrowsingMkdirPopup {
    pub fn new(color: Color, dir_name: &str) -> Self {
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .choices(["Yes", "No"])
                .title(
                    format!(
                        r#"Sync browsing: directory "{dir_name}" doesn't exist. Do you want to create it?"#
                    ),
                    Alignment::Center,
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for SyncBrowsingMkdirPopup {
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
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => Some(Msg::PendingAction(
                PendingActionMsg::CloseSyncBrowsingMkdirPopup,
            )),
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::PendingAction(PendingActionMsg::MakePendingDirectory)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::PendingAction(
                PendingActionMsg::CloseSyncBrowsingMkdirPopup,
            )),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::One(StateValue::Usize(0)))
                ) {
                    Some(Msg::PendingAction(PendingActionMsg::MakePendingDirectory))
                } else {
                    Some(Msg::PendingAction(
                        PendingActionMsg::CloseSyncBrowsingMkdirPopup,
                    ))
                }
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct WaitPopup {
    component: Paragraph,
}

impl WaitPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text([TextSpan::from(text.as_ref())])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for WaitPopup {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

#[derive(MockComponent)]
pub struct WalkdirWaitPopup {
    component: Paragraph,
}

impl WalkdirWaitPopup {
    pub fn new<S: AsRef<str>>(text: S, color: Color) -> Self {
        Self {
            component: Paragraph::default()
                .alignment(Alignment::Center)
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .text([
                    TextSpan::from(text.as_ref()),
                    TextSpan::from("Press 'CTRL+C' to abort"),
                ])
                .wrap(true),
        }
    }
}

impl Component<Msg, NoUserEvent> for WalkdirWaitPopup {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        if matches!(
            ev,
            Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers: KeyModifiers::CONTROL
            })
        ) {
            Some(Msg::Transfer(TransferMsg::AbortWalkdir))
        } else {
            None
        }
    }
}

#[derive(MockComponent)]
pub struct WatchedPathsList {
    component: List,
}

impl WatchedPathsList {
    pub fn new(paths: &[std::path::PathBuf], color: Color) -> Self {
        Self {
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
                .title(
                    "These files are currently synched with the remote host",
                    Alignment::Center,
                )
                .rows(
                    paths
                        .iter()
                        .map(|x| vec![TextSpan::from(x.to_string_lossy().to_string())])
                        .collect(),
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for WatchedPathsList {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseWatchedPathsList))
            }
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
            }) => {
                // get state
                if let State::One(StateValue::Usize(idx)) = self.component.state() {
                    Some(Msg::Transfer(TransferMsg::ToggleWatchFor(idx)))
                } else {
                    Some(Msg::None)
                }
            }
            _ => None,
        }
    }
}

#[derive(MockComponent)]
pub struct WatcherPopup {
    component: Radio,
}

impl WatcherPopup {
    pub fn new(watched: bool, local: &str, remote: &str, color: Color) -> Self {
        let text = match watched {
            false => format!(r#"Synchronize changes from "{local}" to "{remote}"?"#),
            true => format!(r#"Stop synchronizing changes at "{local}"?"#),
        };
        Self {
            component: Radio::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .choices(["Yes", "No"])
                .title(text, Alignment::Center),
        }
    }
}

impl Component<Msg, NoUserEvent> for WatcherPopup {
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
                Some(Msg::Ui(UiMsg::CloseWatcherPopup))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('y'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Transfer(TransferMsg::ToggleWatch)),
            Event::Keyboard(KeyEvent {
                code: Key::Char('n'),
                modifiers: KeyModifiers::NONE,
            }) => Some(Msg::Ui(UiMsg::CloseWatcherPopup)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if matches!(
                    self.perform(Cmd::Submit),
                    CmdResult::Submit(State::One(StateValue::Usize(0)))
                ) {
                    Some(Msg::Transfer(TransferMsg::ToggleWatch))
                } else {
                    Some(Msg::Ui(UiMsg::CloseWatcherPopup))
                }
            }
            _ => None,
        }
    }
}
