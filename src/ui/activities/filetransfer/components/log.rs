//! ## Log
//!
//! log tab component

use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::component::{AppComponent, Component};
use tuirealm::event::{Event, Key, KeyEvent, NoUserEvent};
use tuirealm::props::{
    AttrValue, Attribute, Borders, Color, HorizontalAlignment, Props, QueryResult, Style, Table,
    Title,
};
use tuirealm::ratatui::widgets::{List as TuiList, ListDirection, ListItem, ListState};
use tuirealm::state::{State, StateValue};

use super::{Msg, UiMsg};

pub struct Log {
    props: Props,
    states: OwnStates,
}

impl Log {
    pub fn new(lines: Table, fg: Color, bg: Color) -> Self {
        let mut props = Props::default();
        props.set(
            Attribute::Borders,
            AttrValue::Borders(Borders::default().color(fg)),
        );
        props.set(Attribute::Background, AttrValue::Color(bg));
        props.set(Attribute::Content, AttrValue::Table(lines));
        Self {
            props,
            states: OwnStates::default(),
        }
    }
}

impl Component for Log {
    fn view(
        &mut self,
        frame: &mut tuirealm::ratatui::Frame,
        area: tuirealm::ratatui::layout::Rect,
    ) {
        let _width: usize = area.width as usize - 4;
        let focus = self
            .props
            .get(Attribute::Focus)
            .and_then(AttrValue::as_flag)
            .unwrap_or(false);
        let borders = self
            .props
            .get(Attribute::Borders)
            .and_then(AttrValue::as_borders)
            .unwrap_or_default();
        let bg = self
            .props
            .get(Attribute::Background)
            .and_then(AttrValue::as_color)
            .unwrap_or(Color::Reset);
        // Make list
        let list_items: Vec<ListItem> = self
            .props
            .get(Attribute::Content)
            .and_then(AttrValue::as_table)
            .map(|table| {
                table
                    .iter()
                    .map(|row| {
                        let spans = row
                            .iter()
                            .flat_map(|line| line.spans.iter().cloned())
                            .collect::<Vec<_>>();
                        ListItem::new(tuirealm::ratatui::text::Line::from(spans))
                    })
                    .collect()
            })
            .unwrap_or_default();
        let title = Title::from("Log".to_string()).alignment(HorizontalAlignment::Left);
        let w = TuiList::new(list_items)
            .block(tui_realm_stdlib::utils::get_block(
                borders,
                Some(&title),
                focus,
                None,
            ))
            .direction(ListDirection::BottomToTop)
            .highlight_symbol(">> ")
            .style(Style::default().bg(bg))
            .highlight_style(Style::default());
        let mut state: ListState = ListState::default();
        state.select(Some(self.states.get_list_index()));
        frame.render_stateful_widget(w, area, &mut state);
    }

    fn query<'a>(&'a self, attr: Attribute) -> Option<QueryResult<'a>> {
        self.props.get_for_query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
        if matches!(attr, Attribute::Content) {
            let len = self
                .props
                .get(Attribute::Content)
                .and_then(AttrValue::as_table)
                .map(std::vec::Vec::len)
                .unwrap_or(0);
            self.states.set_list_len(len);
            self.states.reset_list_index();
        }
    }

    fn state(&self) -> State {
        State::Single(StateValue::Usize(self.states.get_list_index()))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Down) => {
                let prev = self.states.get_list_index();
                self.states.incr_list_index();
                if prev != self.states.get_list_index() {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::NoChange
                }
            }
            Cmd::Move(Direction::Up) => {
                let prev = self.states.get_list_index();
                self.states.decr_list_index();
                if prev != self.states.get_list_index() {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::NoChange
                }
            }
            Cmd::Scroll(Direction::Down) => {
                let prev = self.states.get_list_index();
                (0..8).for_each(|_| self.states.incr_list_index());
                if prev != self.states.get_list_index() {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::NoChange
                }
            }
            Cmd::Scroll(Direction::Up) => {
                let prev = self.states.get_list_index();
                (0..8).for_each(|_| self.states.decr_list_index());
                if prev != self.states.get_list_index() {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::NoChange
                }
            }
            Cmd::GoTo(Position::Begin) => {
                let prev = self.states.get_list_index();
                self.states.reset_list_index();
                if prev != self.states.get_list_index() {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::NoChange
                }
            }
            Cmd::GoTo(Position::End) => {
                let prev = self.states.get_list_index();
                self.states.list_index_at_last();
                if prev != self.states.get_list_index() {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::NoChange
                }
            }
            _ => CmdResult::NoChange,
        }
    }
}

impl AppComponent<Msg, NoUserEvent> for Log {
    fn on(&mut self, ev: &Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => {
                self.perform(Cmd::Move(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => Some(Msg::Ui(UiMsg::BottomPanelRight)),
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => Some(Msg::Ui(UiMsg::BottomPanelLeft)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp, ..
            }) => {
                self.perform(Cmd::Scroll(Direction::Down));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                ..
            }) => {
                self.perform(Cmd::Scroll(Direction::Up));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::Begin));
                Some(Msg::None)
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => {
                self.perform(Cmd::GoTo(Position::End));
                Some(Msg::None)
            }
            // -- comp msg
            Event::Keyboard(KeyEvent {
                code: Key::BackTab | Key::Tab | Key::Char('p'),
                ..
            }) => Some(Msg::Ui(UiMsg::LogBackTabbed)),
            _ => None,
        }
    }
}

// -- states

/// OwnStates contains states for this component
#[derive(Clone, Default)]
struct OwnStates {
    list_index: usize, // Index of selected element in list
    list_len: usize,   // Length of file list
}

impl OwnStates {
    /// Set list length
    pub fn set_list_len(&mut self, len: usize) {
        self.list_len = len;
    }

    /// Return current value for list index
    pub fn get_list_index(&self) -> usize {
        self.list_index
    }

    /// Incremenet list index
    pub fn incr_list_index(&mut self) {
        // Check if index is at last element
        if self.list_index + 1 < self.list_len {
            self.list_index += 1;
        }
    }

    /// Decrement list index
    pub fn decr_list_index(&mut self) {
        // Check if index is bigger than 0
        if self.list_index > 0 {
            self.list_index -= 1;
        }
    }

    /// Set list index at last item
    pub fn list_index_at_last(&mut self) {
        self.list_index = match self.list_len {
            0 => 0,
            len => len - 1,
        };
    }

    /// Reset list index to last element
    pub fn reset_list_index(&mut self) {
        self.list_index = 0; // Last element is always 0
    }
}
