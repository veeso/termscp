use tui_term::vt100::Parser;
use tui_term::widget::PseudoTerminal;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{BorderSides, BorderType, Style};
use tuirealm::ratatui::layout::Rect;
use tuirealm::ratatui::widgets::Block;
use tuirealm::{AttrValue, Attribute, MockComponent, Props, State, StateValue};

use super::Line;
use super::history::History;

const DEFAULT_HISTORY_SIZE: usize = 128;

pub struct TerminalComponent {
    pub parser: Parser,
    history: History,
    line: Line,
    props: Props,
    scroll: usize,
    size: (u16, u16),
}

impl Default for TerminalComponent {
    fn default() -> Self {
        let props = Props::default();
        let parser = Parser::new(40, 220, 2048);

        TerminalComponent {
            parser,
            history: History::new(DEFAULT_HISTORY_SIZE),
            line: Line::default(),
            props,
            scroll: 0,
            size: (40, 220),
        }
    }
}

impl TerminalComponent {
    /// Set prompt line for the terminal
    pub fn prompt(mut self, prompt: impl ToString) -> Self {
        self.attr(Attribute::Content, AttrValue::String(prompt.to_string()));
        self.write_prompt();
        self
    }

    pub fn write_prompt(&mut self) {
        if let Some(value) = self.query(Attribute::Content) {
            let prompt = value.unwrap_string();
            self.parser.process(prompt.as_bytes());
        }
    }

    /// Set current line to the previous command in the [`History`]
    fn history_prev(&mut self) {
        if let Some(cmd) = self.history.previous() {
            self.write_line(cmd.as_bytes());
            self.line.set(cmd);
        }
    }

    /// Set current line to the next command in the [`History`]
    fn history_next(&mut self) {
        if let Some(cmd) = self.history.next() {
            self.write_line(cmd.as_bytes());
            self.line.set(cmd);
        } else {
            // If there is no next command, clear the line
            self.line.set(String::new());
            self.write_line(&[]);
        }
    }

    /// Write a line to the terminal, processing it through the parser
    fn write_line(&mut self, data: &[u8]) {
        self.parser.process(b"\r");
        // blank the line
        self.write_prompt();
        self.parser.process(&[b' '; 15]);
        self.parser.process(b"\r");
        self.write_prompt();
        self.parser.process(data);
    }
}

impl MockComponent for TerminalComponent {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: Rect) {
        let width = area.width.saturating_sub(2);
        let height = area.height.saturating_sub(2);

        // update the terminal size if it has changed
        if self.size != (width, height) {
            self.size = (width, height);
            self.parser.set_size(height, width);
        }

        let title = self
            .query(Attribute::Title)
            .map(|value| value.unwrap_string())
            .unwrap_or_else(|| "Terminal".to_string());

        let fg = self
            .query(Attribute::Foreground)
            .map(|value| value.unwrap_color())
            .unwrap_or(tuirealm::ratatui::style::Color::Reset);

        let bg = self
            .query(Attribute::Background)
            .map(|value| value.unwrap_color())
            .unwrap_or(tuirealm::ratatui::style::Color::Reset);

        let border_color = self
            .query(Attribute::Borders)
            .map(|value| value.unwrap_color())
            .unwrap_or(tuirealm::ratatui::style::Color::Reset);

        let terminal = PseudoTerminal::new(self.parser.screen())
            .block(
                Block::default()
                    .title(title)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(border_color))
                    .borders(BorderSides::ALL)
                    .style(Style::default().fg(fg).bg(bg)),
            )
            .style(Style::default().fg(fg).bg(bg));

        frame.render_widget(terminal, area);
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: AttrValue) {
        if attr == Attribute::Text {
            if let tuirealm::AttrValue::String(s) = value {
                self.parser.process(b"\r");
                self.parser.process(s.as_bytes());
                self.parser.process(b"\r");
                self.write_prompt();
            }
        } else {
            self.props.set(attr, value);
        }
    }

    fn state(&self) -> State {
        State::One(StateValue::String(self.line.content().to_string()))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Type(s) => {
                if !s.is_ascii() || self.scroll > 0 {
                    return CmdResult::None; // Ignore non-ASCII characters or if scrolled
                }
                self.parser.process(&[s as u8]);
                self.line.push(s);
                CmdResult::Changed(self.state())
            }
            Cmd::Move(Direction::Down) => {
                if self.scroll > 0 {
                    return CmdResult::None; // Cannot move down if not scrolled
                }

                self.history_next();

                CmdResult::None
            }
            Cmd::Move(Direction::Left) => {
                if self.scroll > 0 {
                    return CmdResult::None; // Cannot move up if not scrolled
                }

                if self.line.left() {
                    self.parser.process(&[27, 91, 68]);
                }

                CmdResult::None
            }
            Cmd::Move(Direction::Right) => {
                if self.scroll > 0 {
                    return CmdResult::None; // Cannot move up if not scrolled
                }

                if self.line.right() {
                    self.parser.process(&[27, 91, 67]);
                }

                CmdResult::None
            }
            Cmd::Move(Direction::Up) => {
                if self.scroll > 0 {
                    return CmdResult::None; // Cannot move up if not scrolled
                }

                self.history_prev();
                CmdResult::None
            }
            Cmd::Cancel => {
                if self.scroll > 0 {
                    return CmdResult::None; // Cannot move to the beginning if scrolled
                }

                if !self.line.is_empty() {
                    self.line.backspace();
                    self.parser.process(&[8]); // Backspace character
                    // delete the last character from the line
                    // write one empty character to the terminal
                    self.parser.process(&[32]); // Space character
                    self.parser.process(&[8]); // Backspace character
                }
                CmdResult::Changed(self.state())
            }
            Cmd::Delete => {
                if self.scroll > 0 {
                    return CmdResult::None; // Cannot move to the beginning if scrolled
                }

                if !self.line.is_empty() {
                    self.line.delete();
                    self.parser.process(&[27, 91, 51, 126]); // Delete character
                    // write one empty character to the terminal
                    self.parser.process(&[32]); // Space character
                    self.parser.process(&[8]); // Backspace character
                }
                CmdResult::Changed(self.state())
            }
            Cmd::Scroll(Direction::Down) => {
                self.scroll = self.scroll.saturating_sub(8);
                self.parser.set_scrollback(self.scroll);

                CmdResult::None
            }
            Cmd::Scroll(Direction::Up) => {
                self.parser.set_scrollback(self.scroll.saturating_add(8));
                let scrollback = self.parser.screen().scrollback();
                self.scroll = scrollback;

                CmdResult::None
            }
            Cmd::Toggle => {
                // insert
                self.parser.process(&[27, 91, 50, 126]); // Toggle insert mode
                CmdResult::None
            }
            Cmd::GoTo(Position::Begin) => {
                if self.scroll > 0 {
                    return CmdResult::None; // Cannot move to the beginning if scrolled
                }

                for _ in 0..self.line.begin() {
                    self.parser.process(&[27, 91, 68]); // Move cursor to the left
                }

                CmdResult::None
            }
            Cmd::GoTo(Position::End) => {
                if self.scroll > 0 {
                    return CmdResult::None; // Cannot move to the beginning if scrolled
                }

                for _ in 0..self.line.end() {
                    self.parser.process(&[27, 91, 67]); // Move cursor to the right
                }
                CmdResult::None
            }
            Cmd::Submit => {
                self.scroll = 0; // Reset scroll on submit
                self.parser.set_scrollback(self.scroll);

                if cfg!(target_family = "unix") {
                    self.parser.process(b"\n");
                } else {
                    self.parser.process(b"\r\n\r");
                }

                let line = self.line.take();
                if !line.is_empty() {
                    self.history.push(&line);
                }

                CmdResult::Submit(State::One(StateValue::String(line)))
            }
            _ => CmdResult::None,
        }
    }
}
