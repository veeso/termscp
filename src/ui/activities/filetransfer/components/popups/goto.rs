use std::path::PathBuf;

use tui_realm_stdlib::Input;
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::{Alignment, BorderType, Borders, Color, InputType, Style};
use tuirealm::{
    AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

use crate::ui::activities::filetransfer::{Msg, TransferMsg, UiMsg};

pub const ATTR_FILES: &str = "files";

#[derive(Default)]
struct OwnStates {
    /// Path and name of the files
    files: Vec<(String, String)>,
    search: Option<String>,
    last_suggestion: Option<String>,
}

impl OwnStates {
    pub fn set_files(&mut self, files: Vec<String>) {
        self.files = files
            .into_iter()
            .map(|f| {
                (
                    f.clone(),
                    PathBuf::from(&f)
                        .file_name()
                        .map(|x| x.to_string_lossy().to_string())
                        .unwrap_or(f),
                )
            })
            .collect();
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Suggestion {
    /// No suggestion
    None,
    /// Suggest a string
    Suggest(String),
    /// Rescan at `path` is required to satisfy the user input
    Rescan(PathBuf),
}

impl From<CmdResult> for Suggestion {
    fn from(value: CmdResult) -> Self {
        match value {
            CmdResult::Batch(v) if v.len() == 1 => {
                if let CmdResult::Submit(State::One(StateValue::String(s))) = v.first().unwrap() {
                    Suggestion::Suggest(s.clone())
                } else {
                    Suggestion::None
                }
            }
            CmdResult::Batch(v) if v.len() == 2 => {
                if let CmdResult::Submit(State::One(StateValue::String(s))) = v.get(1).unwrap() {
                    Suggestion::Rescan(PathBuf::from(s))
                } else {
                    Suggestion::None
                }
            }
            _ => Suggestion::None,
        }
    }
}

impl From<Suggestion> for CmdResult {
    fn from(value: Suggestion) -> Self {
        match value {
            Suggestion::None => CmdResult::None,
            Suggestion::Suggest(s) => {
                CmdResult::Batch(vec![CmdResult::Submit(State::One(StateValue::String(s)))])
            }
            Suggestion::Rescan(p) => CmdResult::Batch(vec![
                CmdResult::None,
                CmdResult::Submit(State::One(StateValue::String(
                    p.to_string_lossy().to_string(),
                ))),
            ]),
        }
    }
}

impl OwnStates {
    /// Return the current suggestion if any, otherwise return search
    pub fn computed_search(&self) -> String {
        match (&self.search, &self.last_suggestion) {
            (_, Some(s)) => s.clone(),
            (Some(s), _) => s.clone(),
            _ => "".to_string(),
        }
    }

    /// Suggest files based on the input
    pub fn suggest(&mut self, input: &str) -> Suggestion {
        debug!(
            "Suggesting for: {input}; files {files:?}",
            files = self.files
        );

        let is_path = PathBuf::from(input).is_absolute();

        // case 1. search if any file starts with the input; get first if suggestion is `None`, otherwise get first after suggestion
        let suggestions: Vec<&String> = self
            .files
            .iter()
            .filter(|(path, file_name)| {
                if is_path {
                    path.contains(input)
                } else {
                    file_name.contains(input)
                }
            })
            .map(|(path, _)| path)
            .collect();

        debug!("Suggestions for {input}: {:?}", suggestions);

        // case 1. if suggestions not empty; then suggest next
        if !suggestions.is_empty() {
            let suggestion;
            if let Some(last_suggestion) = self.last_suggestion.take() {
                suggestion = suggestions
                    .iter()
                    .skip_while(|f| **f != &last_suggestion)
                    .nth(1)
                    .unwrap_or_else(|| suggestions.first().unwrap())
                    .to_string();
            } else {
                suggestion = suggestions.first().map(|x| x.to_string()).unwrap();
            }

            debug!("Suggested: {suggestion}");
            self.last_suggestion = Some(suggestion.clone());

            return Suggestion::Suggest(suggestion);
        }

        self.last_suggestion = None;

        // case 2. otherwise convert suggest to a path and get the parent
        // to rescan the files
        let input_as_path = if input.starts_with('/') {
            input.to_string()
        } else {
            format!("./{}", input)
        };

        let p = PathBuf::from(input_as_path);
        let parent = p
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("/"));

        // if path is `.`, then return None
        if parent == PathBuf::from(".") {
            return Suggestion::None;
        }

        debug!("Rescan required at: {}", parent.display());

        Suggestion::Rescan(parent)
    }
}

pub struct GotoPopup {
    input: Input,
    states: OwnStates,
}

impl GotoPopup {
    pub fn new(color: Color, files: Vec<String>) -> Self {
        let mut states = OwnStates::default();
        states.set_files(files);

        Self {
            input: Input::default()
                .borders(
                    Borders::default()
                        .color(color)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(color)
                .input_type(InputType::Text)
                .placeholder(
                    "/foo/bar/buzz",
                    Style::default().fg(Color::Rgb(128, 128, 128)),
                )
                .title("Go to… (Press <TAB> for autocompletion)", Alignment::Center),
            states,
        }
    }
}

impl MockComponent for GotoPopup {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::prelude::Rect) {
        self.input.view(frame, area);
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        match attr {
            Attribute::Custom(ATTR_FILES) => {
                let files = value
                    .unwrap_payload()
                    .unwrap_vec()
                    .into_iter()
                    .map(|x| x.unwrap_str())
                    .collect();

                self.states.set_files(files);
                // call perform Change
                self.perform(Cmd::Change);
            }
            _ => self.input.attr(attr, value),
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.input.query(attr)
    }

    fn state(&self) -> State {
        State::One(StateValue::String(self.states.computed_search()))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Change => {
                let input = self
                    .states
                    .search
                    .as_ref()
                    .cloned()
                    .unwrap_or_else(|| self.input.state().unwrap_one().unwrap_string());
                let suggest = self.states.suggest(&input);
                if let Suggestion::Suggest(suggestion) = suggest.clone() {
                    self.input
                        .attr(Attribute::Value, AttrValue::String(suggestion.clone()));
                }

                suggest.into()
            }
            cmd => {
                let res = self.input.perform(cmd);
                if let CmdResult::Changed(State::One(StateValue::String(new_text))) = &res {
                    self.states.search = Some(new_text.clone());
                }
                res
            }
        }
    }
}

impl Component<Msg, NoUserEvent> for GotoPopup {
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
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => {
                if let Suggestion::Rescan(path) = Suggestion::from(self.perform(Cmd::Change)) {
                    Some(Msg::Transfer(TransferMsg::RescanGotoFiles(path)))
                } else {
                    Some(Msg::None)
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => match self.state() {
                State::One(StateValue::String(i)) => Some(Msg::Transfer(TransferMsg::GoTo(i))),
                _ => Some(Msg::None),
            },
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                Some(Msg::Ui(UiMsg::CloseGotoPopup))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_convert_from_and_back_cmd_result() {
        let s = Suggestion::Suggest("foo".to_string());
        let cmd: CmdResult = s.clone().into();
        let s2: Suggestion = cmd.into();
        assert_eq!(s, s2);

        let s = Suggestion::Rescan(PathBuf::from("/foo/bar"));
        let cmd: CmdResult = s.clone().into();
        let s2: Suggestion = cmd.into();
        assert_eq!(s, s2);
    }

    #[test]
    fn test_should_suggest_next() {
        let mut states = OwnStates {
            files: vec![
                ("/home/foo".to_string(), "foo".to_string()),
                ("/home/bar".to_string(), "bar".to_string()),
                ("/home/buzz".to_string(), "buzz".to_string()),
                ("/home/fizz".to_string(), "fizz".to_string()),
            ],
            search: None,
            last_suggestion: None,
        };

        let s = states.suggest("f");
        assert_eq!(Suggestion::Suggest("/home/foo".to_string()), s);
        let s = states.suggest("f");
        assert_eq!(Suggestion::Suggest("/home/fizz".to_string()), s);

        let s = states.suggest("f");
        assert_eq!(Suggestion::Suggest("/home/foo".to_string()), s);
    }

    #[test]
    #[cfg(unix)]
    fn test_should_suggest_absolute_path() {
        let mut states = OwnStates {
            files: vec![
                ("/home/foo".to_string(), "foo".to_string()),
                ("/home/bar".to_string(), "bar".to_string()),
                ("/home/buzz".to_string(), "buzz".to_string()),
                ("/home/fizz".to_string(), "fizz".to_string()),
            ],
            search: None,
            last_suggestion: None,
        };

        let s = states.suggest("/home/f");
        assert_eq!(Suggestion::Suggest("/home/foo".to_string()), s);
    }

    #[test]
    fn test_should_suggest_rescan() {
        let mut states = OwnStates {
            files: vec![
                ("/home/foo".to_string(), "foo".to_string()),
                ("/home/bar".to_string(), "bar".to_string()),
                ("/home/buzz".to_string(), "buzz".to_string()),
                ("/home/fizz".to_string(), "fizz".to_string()),
            ],
            search: None,
            last_suggestion: None,
        };

        let s = states.suggest("/home/user");
        assert_eq!(Suggestion::Rescan(PathBuf::from("/home")), s);
    }

    #[test]
    fn test_should_suggest_none() {
        let mut states = OwnStates {
            files: vec![
                ("/home/foo".to_string(), "foo".to_string()),
                ("/home/bar".to_string(), "bar".to_string()),
                ("/home/buzz".to_string(), "buzz".to_string()),
                ("/home/fizz".to_string(), "fizz".to_string()),
            ],
            search: None,
            last_suggestion: None,
        };

        let s = states.suggest("");
        assert_eq!(Suggestion::Suggest("/home/foo".to_string()), s);
    }

    #[test]
    fn test_should_suggest_none_if_dot() {
        let mut states = OwnStates {
            files: vec![
                ("/home/foo".to_string(), "foo".to_string()),
                ("/home/bar".to_string(), "bar".to_string()),
                ("/home/buzz".to_string(), "buzz".to_string()),
                ("/home/fizz".to_string(), "fizz".to_string()),
            ],
            search: None,
            last_suggestion: None,
        };

        let s = states.suggest("./th");
        assert_eq!(Suggestion::None, s);
    }
}
