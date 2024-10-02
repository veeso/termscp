use tui_realm_stdlib::Input;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{Alignment, AttrValue, Attribute, Borders, Color, Table};
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::{MockComponent, State};

use super::file_list::FileList;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    List,
    #[default]
    Search,
}

#[derive(Default)]
struct OwnStates {
    focus: Focus,
}

impl OwnStates {
    pub fn next(&mut self) {
        self.focus = match self.focus {
            Focus::List => Focus::Search,
            Focus::Search => Focus::List,
        };
    }
}

#[derive(Default)]
pub struct FileListWithSearch {
    file_list: FileList,
    search: Input,
    states: OwnStates,
}

impl FileListWithSearch {
    pub fn focus(&self) -> Focus {
        self.states.focus
    }

    pub fn foreground(mut self, fg: Color) -> Self {
        self.file_list
            .attr(Attribute::Foreground, AttrValue::Color(fg));
        self.search
            .attr(Attribute::Foreground, AttrValue::Color(fg));
        self
    }

    pub fn background(mut self, bg: Color) -> Self {
        self.file_list
            .attr(Attribute::Background, AttrValue::Color(bg));
        self.search
            .attr(Attribute::Background, AttrValue::Color(bg));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.file_list
            .attr(Attribute::Borders, AttrValue::Borders(b.clone()));
        self.search.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }

    pub fn title<S: AsRef<str>>(mut self, t: S, a: Alignment) -> Self {
        self.file_list.attr(
            Attribute::Title,
            AttrValue::Title((t.as_ref().to_string(), a)),
        );
        self.search.attr(
            Attribute::Title,
            AttrValue::Title(("Fuzzy search".to_string(), a)),
        );
        self
    }

    pub fn highlighted_color(mut self, c: Color) -> Self {
        self.file_list
            .attr(Attribute::HighlightedColor, AttrValue::Color(c));
        self
    }

    pub fn rows(mut self, rows: Table) -> Self {
        self.file_list
            .attr(Attribute::Content, AttrValue::Table(rows));
        self
    }
}

impl MockComponent for FileListWithSearch {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        // split the area in two
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3), // Search
                    Constraint::Fill(1),   // File list
                ]
                .as_ref(),
            )
            .split(area);

        // render the search input
        self.search.view(frame, chunks[0]);
        // render the file list
        self.file_list.view(frame, chunks[1]);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.file_list.query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        if attr == Attribute::Focus {
            let value = value.unwrap_flag();
            match value {
                true => self.states.focus = Focus::Search,
                false => self.states.focus = Focus::List,
            }
            self.search.attr(
                Attribute::Focus,
                AttrValue::Flag(self.states.focus == Focus::Search),
            );
            self.file_list.attr(
                Attribute::Focus,
                AttrValue::Flag(self.states.focus == Focus::List),
            );
        } else {
            self.file_list.attr(attr, value);
        }
    }

    fn state(&self) -> State {
        match self.states.focus {
            Focus::List => self.file_list.state(),
            Focus::Search => self.search.state(),
        }
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Change => {
                self.states.next();
                self.search.attr(
                    Attribute::Focus,
                    AttrValue::Flag(self.states.focus == Focus::Search),
                );
                self.file_list.attr(
                    Attribute::Focus,
                    AttrValue::Flag(self.states.focus == Focus::List),
                );

                CmdResult::None
            }
            cmd if self.states.focus == Focus::Search => self.search.perform(cmd),
            cmd => self.file_list.perform(cmd),
        }
    }
}
