use tui_realm_stdlib::List;
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan};
use tuirealm::{Component, Event, MockComponent, NoUserEvent};

use crate::ui::activities::filetransfer::{Msg, UiMsg};

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
                        .add_col(TextSpan::new("<CTRL+S>").bold().fg(key_color))
                        .add_col(TextSpan::from(
                            "          Get total path size of selected files",
                        ))
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
