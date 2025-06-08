//! ## FileList
//!
//! `FileList` component renders a file list tab

use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, Style, Table, TextModifiers, TextSpan,
};
use tuirealm::ratatui::text::{Line, Span};
use tuirealm::ratatui::widgets::{List as TuiList, ListDirection, ListItem, ListState};
use tuirealm::{MockComponent, Props, State, StateValue};

pub const FILE_LIST_CMD_SELECT_ALL: &str = "A";
pub const FILE_LIST_CMD_DESELECT_ALL: &str = "D";
const PROP_DOT_DOT: &str = "dot_dot";

/// OwnStates contains states for this component
#[derive(Clone, Default)]
struct OwnStates {
    list_index: usize, // Index of selected element in list
    list_len: usize,   // Length of the list
    dot_dot: bool,
}

impl OwnStates {
    /// Initialize list states
    pub fn init_list_states(&mut self, len: usize, has_dot_dot: bool) {
        self.list_len = len + if has_dot_dot { 1 } else { 0 };
        self.fix_list_index();
        self.dot_dot = has_dot_dot;
    }

    /// Incremenet list index.
    /// If `can_rewind` is `true` the index rewinds when boundary is reached
    pub fn incr_list_index(&mut self, can_rewind: bool) {
        // Check if index is at last element
        if self.list_index + 1 < self.list_len() {
            self.list_index += 1;
        } else if can_rewind {
            self.list_index = 0;
        }
    }

    pub fn real_index(&self) -> usize {
        if self.dot_dot {
            self.list_index.saturating_sub(1)
        } else {
            self.list_index
        }
    }

    /// Decrement list index
    /// If `can_rewind` is `true` the index rewinds when boundary is reached
    pub fn decr_list_index(&mut self, can_rewind: bool) {
        // Check if index is bigger than 0
        if self.list_index > 0 {
            self.list_index -= 1;
        } else if self.list_len() > 0 && can_rewind {
            self.list_index = self.list_len() - 1;
        }
    }

    pub fn list_index_at_first(&mut self) {
        self.list_index = 0;
    }

    pub fn list_index_at_last(&mut self) {
        self.list_index = match self.list_len() {
            0 => 0,
            len => len - 1,
        };
    }

    /// Returns the length of the file list, which is actually the capacity of the selection vector
    pub fn list_len(&self) -> usize {
        self.list_len
    }

    /// Keep index if possible, otherwise set to lenght - 1
    fn fix_list_index(&mut self) {
        if self.list_index >= self.list_len() && self.list_len() > 0 {
            self.list_index = self.list_len() - 1;
        } else if self.list_len() == 0 {
            self.list_index = 0;
        }
    }
}

#[derive(Default)]
pub struct FileList {
    props: Props,
    states: OwnStates,
}

impl FileList {
    pub fn foreground(mut self, fg: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self
    }

    pub fn highlighted_color(mut self, c: Color) -> Self {
        self.attr(Attribute::HighlightedColor, AttrValue::Color(c));
        self
    }

    pub fn rows(mut self, rows: Table) -> Self {
        self.attr(Attribute::Content, AttrValue::Table(rows));
        self
    }

    /// If enabled, show `..` entry at the beginning of the list
    pub fn dot_dot(mut self, show: bool) -> Self {
        self.attr(Attribute::Custom(PROP_DOT_DOT), AttrValue::Flag(show));
        self
    }

    /// Returns the value of the `dot_dot` property
    fn has_dot_dot(&self) -> bool {
        self.props
            .get(Attribute::Custom(PROP_DOT_DOT))
            .map(|x| x.unwrap_flag())
            .unwrap_or(false)
    }
}

impl MockComponent for FileList {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::ratatui::layout::Rect) {
        let title = self
            .props
            .get_or(
                Attribute::Title,
                AttrValue::Title((String::default(), Alignment::Left)),
            )
            .unwrap_title();
        let borders = self
            .props
            .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
            .unwrap_borders();
        let focus = self
            .props
            .get_or(Attribute::Focus, AttrValue::Flag(false))
            .unwrap_flag();
        let div = tui_realm_stdlib::utils::get_block(borders, Some(&title), focus, None);
        // Make list entries
        let init_table_iter = if self.has_dot_dot() {
            vec![vec![TextSpan::from("..")]]
        } else {
            vec![]
        };

        let list_items: Vec<ListItem> = match self
            .props
            .get(Attribute::Content)
            .map(|x| x.unwrap_table())
        {
            Some(table) => init_table_iter
                .iter()
                .chain(table.iter())
                .map(|row| {
                    let columns: Vec<Span> = row
                        .iter()
                        .map(|col| {
                            let (fg, bg, modifiers) =
                                tui_realm_stdlib::utils::use_or_default_styles(&self.props, col);

                            Span::styled(
                                col.content.clone(),
                                Style::default().add_modifier(modifiers).fg(fg).bg(bg),
                            )
                        })
                        .collect();
                    ListItem::new(Line::from(columns))
                })
                .collect(), // Make List item from TextSpan
            _ => Vec::new(),
        };
        let highlighted_color = self
            .props
            .get(Attribute::HighlightedColor)
            .map(|x| x.unwrap_color());
        let modifiers = match focus {
            true => TextModifiers::REVERSED,
            false => TextModifiers::empty(),
        };
        // Make list
        let mut list = TuiList::new(list_items)
            .block(div)
            .direction(ListDirection::TopToBottom);
        if let Some(highlighted_color) = highlighted_color {
            list = list.highlight_style(
                Style::default()
                    .fg(highlighted_color)
                    .add_modifier(modifiers),
            );
        }
        let mut state: ListState = ListState::default();
        state.select(Some(self.states.list_index));
        frame.render_stateful_widget(list, area, &mut state);
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
        if matches!(attr, Attribute::Content) {
            self.states.init_list_states(
                match self.props.get(Attribute::Content).map(|x| x.unwrap_table()) {
                    Some(line) => line.len(),
                    _ => 0,
                },
                self.has_dot_dot(),
            );
            self.states.fix_list_index();
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn state(&self) -> State {
        if self.has_dot_dot() && self.states.list_index == 0 {
            return State::One(StateValue::String("..".to_string()));
        }

        State::One(StateValue::Usize(if self.has_dot_dot() {
            self.states.list_index.checked_sub(1).unwrap_or_default()
        } else {
            self.states.list_index
        }))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(Direction::Down) => {
                let prev = self.states.list_index;
                self.states.incr_list_index(true);
                if prev != self.states.list_index {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::Move(Direction::Up) => {
                let prev = self.states.list_index;
                self.states.decr_list_index(true);
                if prev != self.states.list_index {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::Scroll(Direction::Down) => {
                let prev = self.states.list_index;
                (0..8).for_each(|_| self.states.incr_list_index(false));
                if prev != self.states.list_index {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::Scroll(Direction::Up) => {
                let prev = self.states.list_index;
                (0..8).for_each(|_| self.states.decr_list_index(false));
                if prev != self.states.list_index {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::GoTo(Position::Begin) => {
                let prev = self.states.list_index;
                self.states.list_index_at_first();
                if prev != self.states.list_index {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::GoTo(Position::End) => {
                let prev = self.states.list_index;
                self.states.list_index_at_last();
                if prev != self.states.list_index {
                    CmdResult::Changed(self.state())
                } else {
                    CmdResult::None
                }
            }
            Cmd::Toggle => {
                if self.states.list_index == 0 && self.has_dot_dot() {
                    return CmdResult::None;
                }

                let index = self.states.real_index();
                self.states.list_index = self
                    .states
                    .list_index
                    .saturating_add(1)
                    .min(self.states.list_len.saturating_sub(1));
                CmdResult::Changed(State::One(StateValue::Usize(index)))
            }
            _ => CmdResult::None,
        }
    }
}
