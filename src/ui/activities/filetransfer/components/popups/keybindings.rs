use tui_realm_stdlib::components::List;
use tuirealm::command::{Cmd, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{
    BorderType, Borders, Color, HorizontalAlignment, SpanStatic, TableBuilder, Title,
};
use tuirealm::ratatui::style::Stylize;

use crate::ui::activities::filetransfer::{Msg, UiMsg};

#[derive(Component)]
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
                .highlight_str("? ")
                .title(Title::from("Keybindings").alignment(HorizontalAlignment::Center))
                .rewind(true)
                .rows(
                    TableBuilder::default()
                        .add_col(SpanStatic::raw("<ESC>").bold().fg(key_color))
                        .add_col(SpanStatic::from("             Disconnect"))
                        .add_row()
                        .add_col(SpanStatic::raw("<BACKSPACE>").bold().fg(key_color))
                        .add_col(SpanStatic::from("       Go to previous directory"))
                        .add_row()
                        .add_col(SpanStatic::raw("<TAB|RIGHT|LEFT>").bold().fg(key_color))
                        .add_col(SpanStatic::from("  Change explorer tab"))
                        .add_row()
                        .add_col(SpanStatic::raw("<UP/DOWN>").bold().fg(key_color))
                        .add_col(SpanStatic::from("         Move up/down in list"))
                        .add_row()
                        .add_col(SpanStatic::raw("<ENTER>").bold().fg(key_color))
                        .add_col(SpanStatic::from("           Enter directory"))
                        .add_row()
                        .add_col(SpanStatic::raw("<SPACE>").bold().fg(key_color))
                        .add_col(SpanStatic::from("           Upload/Download file"))
                        .add_row()
                        .add_col(SpanStatic::raw("<BACKTAB>").bold().fg(key_color))
                        .add_col(SpanStatic::from(
                            "         Switch between explorer and log window",
                        ))
                        .add_row()
                        .add_col(SpanStatic::raw("<A>").bold().fg(key_color))
                        .add_col(SpanStatic::from("               Toggle hidden files"))
                        .add_row()
                        .add_col(SpanStatic::raw("<B>").bold().fg(key_color))
                        .add_col(SpanStatic::from("               Change file sorting mode"))
                        .add_row()
                        .add_col(SpanStatic::raw("<C|F5>").bold().fg(key_color))
                        .add_col(SpanStatic::from("            Copy"))
                        .add_row()
                        .add_col(SpanStatic::raw("<D|F7>").bold().fg(key_color))
                        .add_col(SpanStatic::from("            Make directory"))
                        .add_row()
                        .add_col(SpanStatic::raw("<F>").bold().fg(key_color))
                        .add_col(SpanStatic::from("               Search files"))
                        .add_row()
                        .add_col(SpanStatic::raw("<G>").bold().fg(key_color))
                        .add_col(SpanStatic::from("               Go to path"))
                        .add_row()
                        .add_col(SpanStatic::raw("<H|F1>").bold().fg(key_color))
                        .add_col(SpanStatic::from("            Show help"))
                        .add_row()
                        .add_col(SpanStatic::raw("<I>").bold().fg(key_color))
                        .add_col(SpanStatic::from(
                            "               Show info about selected file",
                        ))
                        .add_row()
                        .add_col(SpanStatic::raw("<K>").bold().fg(key_color))
                        .add_col(SpanStatic::from(
                            "               Create symlink pointing to the current selected entry",
                        ))
                        .add_row()
                        .add_col(SpanStatic::raw("<L>").bold().fg(key_color))
                        .add_col(SpanStatic::from("               Reload directory content"))
                        .add_row()
                        .add_col(SpanStatic::raw("<M>").bold().fg(key_color))
                        .add_col(SpanStatic::from("               Select file"))
                        .add_row()
                        .add_col(SpanStatic::raw("<N>").bold().fg(key_color))
                        .add_col(SpanStatic::from("               Create new file"))
                        .add_row()
                        .add_col(SpanStatic::raw("<O|F4>").bold().fg(key_color))
                        .add_col(SpanStatic::from(
                            "            Open text file with preferred editor",
                        ))
                        .add_row()
                        .add_col(SpanStatic::raw("<P>").bold().fg(key_color))
                        .add_col(SpanStatic::from("               Toggle bottom panel"))
                        .add_row()
                        .add_col(SpanStatic::raw("<Q|F10>").bold().fg(key_color))
                        .add_col(SpanStatic::from("           Quit termscp"))
                        .add_row()
                        .add_col(SpanStatic::raw("<R|F6>").bold().fg(key_color))
                        .add_col(SpanStatic::from("            Rename file"))
                        .add_row()
                        .add_col(SpanStatic::raw("<S|F2>").bold().fg(key_color))
                        .add_col(SpanStatic::from("            Save file as"))
                        .add_row()
                        .add_col(SpanStatic::raw("<T>").bold().fg(key_color))
                        .add_col(SpanStatic::from(
                            "               Watch/unwatch file changes",
                        ))
                        .add_row()
                        .add_col(SpanStatic::raw("<U>").bold().fg(key_color))
                        .add_col(SpanStatic::from("               Go to parent directory"))
                        .add_row()
                        .add_col(SpanStatic::raw("<V|F3>").bold().fg(key_color))
                        .add_col(SpanStatic::from(
                            "            Open file with default application for file type",
                        ))
                        .add_row()
                        .add_col(SpanStatic::raw("<W>").bold().fg(key_color))
                        .add_col(SpanStatic::from(
                            "               Open file with specified application",
                        ))
                        .add_row()
                        .add_col(SpanStatic::raw("<X>").bold().fg(key_color))
                        .add_col(SpanStatic::from("               Execute shell command"))
                        .add_row()
                        .add_col(SpanStatic::raw("<Y>").bold().fg(key_color))
                        .add_col(SpanStatic::from(
                            "               Toggle synchronized browsing",
                        ))
                        .add_row()
                        .add_col(SpanStatic::raw("<Z>").bold().fg(key_color))
                        .add_col(SpanStatic::from("               Change file permissions"))
                        .add_row()
                        .add_col(SpanStatic::raw("</>").bold().fg(key_color))
                        .add_col(SpanStatic::from("               Filter files"))
                        .add_row()
                        .add_col(SpanStatic::raw("<DEL|F8|E>").bold().fg(key_color))
                        .add_col(SpanStatic::from("        Delete selected file"))
                        .add_row()
                        .add_col(SpanStatic::raw("<CTRL+A>").bold().fg(key_color))
                        .add_col(SpanStatic::from("          Select all files"))
                        .add_row()
                        .add_col(SpanStatic::raw("<ALT+A>").bold().fg(key_color))
                        .add_col(SpanStatic::from("          Deselect all files"))
                        .add_row()
                        .add_col(SpanStatic::raw("<CTRL+C>").bold().fg(key_color))
                        .add_col(SpanStatic::from("          Interrupt file transfer"))
                        .add_row()
                        .add_col(SpanStatic::raw("<CTRL+S>").bold().fg(key_color))
                        .add_col(SpanStatic::from(
                            "          Get total path size of selected files",
                        ))
                        .add_row()
                        .add_col(SpanStatic::raw("<CTRL+T>").bold().fg(key_color))
                        .add_col(SpanStatic::from("          Show watched paths"))
                        .build()
                        .into_iter()
                        .map(|row| row.into_iter().flat_map(|l| l.spans).collect::<Vec<_>>()),
                ),
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for KeybindingsPopup {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
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
