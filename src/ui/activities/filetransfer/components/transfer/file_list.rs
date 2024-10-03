//! ## FileList
//!
//! `FileList` component renders a file list tab

use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, AttrValue, Attribute, Borders, Color, Style, Table, TextModifiers,
};
use tuirealm::tui::text::{Line, Span};
use tuirealm::tui::widgets::{List as TuiList, ListDirection, ListItem, ListState};
use tuirealm::{MockComponent, Props, State, StateValue};

pub const FILE_LIST_CMD_SELECT_ALL: &str = "A";
pub const FILE_LIST_CMD_DESELECT_ALL: &str = "D";

/// OwnStates contains states for this component
#[derive(Clone, Default)]
struct OwnStates {
    list_index: usize,    // Index of selected element in list
    selected: Vec<usize>, // Selected files
}

impl OwnStates {
    /// Initialize list states
    pub fn init_list_states(&mut self, len: usize) {
        self.selected = Vec::with_capacity(len);
        self.fix_list_index();
    }

    /// Return current value for list index
    pub fn list_index(&self) -> usize {
        self.list_index
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
        self.selected.capacity()
    }

    /// Returns whether the file with index `entry` is selected
    pub fn is_selected(&self, entry: usize) -> bool {
        self.selected.contains(&entry)
    }

    /// Returns whether the selection is currently empty
    pub fn is_selection_empty(&self) -> bool {
        self.selected.is_empty()
    }

    /// Returns current file selection
    pub fn get_selection(&self) -> Vec<usize> {
        self.selected.clone()
    }

    /// Keep index if possible, otherwise set to lenght - 1
    fn fix_list_index(&mut self) {
        if self.list_index >= self.list_len() && self.list_len() > 0 {
            self.list_index = self.list_len() - 1;
        } else if self.list_len() == 0 {
            self.list_index = 0;
        }
    }

    // -- select manipulation

    /// Select or deselect file with provided entry index
    pub fn toggle_file(&mut self, entry: usize) {
        match self.is_selected(entry) {
            true => self.deselect(entry),
            false => self.select(entry),
        }
        // increment index
        self.incr_list_index(false);
    }

    /// Select all files
    pub fn select_all(&mut self) {
        for i in 0..self.list_len() {
            self.select(i);
        }
    }

    /// Select all files
    pub fn deselect_all(&mut self) {
        self.selected.clear();
    }

    /// Select provided index if not selected yet
    fn select(&mut self, entry: usize) {
        if !self.is_selected(entry) {
            self.selected.push(entry);
        }
    }

    /// Remove element file with associated index
    fn deselect(&mut self, entry: usize) {
        if self.is_selected(entry) {
            self.selected.retain(|&x| x != entry);
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
}

impl MockComponent for FileList {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
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
        let div = tui_realm_stdlib::utils::get_block(borders, Some(title), focus, None);
        // Make list entries
        let list_items: Vec<ListItem> = match self
            .props
            .get(Attribute::Content)
            .map(|x| x.unwrap_table())
        {
            Some(table) => table
                .iter()
                .enumerate()
                .map(|(num, row)| {
                    let columns: Vec<Span> = row
                        .iter()
                        .map(|col| {
                            let (fg, bg, mut modifiers) =
                                tui_realm_stdlib::utils::use_or_default_styles(&self.props, col);
                            if self.states.is_selected(num) {
                                modifiers |= TextModifiers::REVERSED
                                    | TextModifiers::UNDERLINED
                                    | TextModifiers::ITALIC;
                            }
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
            );
            self.states.fix_list_index();
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn state(&self) -> State {
        match self.states.is_selection_empty() {
            true => State::One(StateValue::Usize(self.states.list_index())),
            false => State::Vec(
                self.states
                    .get_selection()
                    .into_iter()
                    .map(StateValue::Usize)
                    .collect(),
            ),
        }
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
            Cmd::Custom(FILE_LIST_CMD_SELECT_ALL) => {
                self.states.select_all();
                CmdResult::None
            }
            Cmd::Custom(FILE_LIST_CMD_DESELECT_ALL) => {
                self.states.deselect_all();
                CmdResult::None
            }
            Cmd::Toggle => {
                self.states.toggle_file(self.states.list_index());
                CmdResult::None
            }
            _ => CmdResult::None,
        }
    }
}
