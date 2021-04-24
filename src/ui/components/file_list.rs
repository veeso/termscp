//! ## FileList
//!
//! `FileList` component renders a file list tab

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// ext
use tuirealm::components::utils::get_block;
use tuirealm::event::{Event, KeyCode};
use tuirealm::props::{BordersProps, Props, PropsBuilder, TextParts, TextSpan};
use tuirealm::tui::{
    layout::{Corner, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{BorderType, Borders, List, ListItem, ListState},
};
use tuirealm::{Canvas, Component, Msg, Payload};

// -- props

pub struct FileListPropsBuilder {
    props: Option<Props>,
}

impl Default for FileListPropsBuilder {
    fn default() -> Self {
        FileListPropsBuilder {
            props: Some(Props::default()),
        }
    }
}

impl PropsBuilder for FileListPropsBuilder {
    fn build(&mut self) -> Props {
        self.props.take().unwrap()
    }

    fn hidden(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = false;
        }
        self
    }

    fn visible(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = true;
        }
        self
    }
}

impl From<Props> for FileListPropsBuilder {
    fn from(props: Props) -> Self {
        FileListPropsBuilder { props: Some(props) }
    }
}

impl FileListPropsBuilder {
    /// ### with_foreground
    ///
    /// Set foreground color for area
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    /// ### with_background
    ///
    /// Set background color for area
    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.background = color;
        }
        self
    }

    /// ### with_borders
    ///
    /// Set component borders style
    pub fn with_borders(
        &mut self,
        borders: Borders,
        variant: BorderType,
        color: Color,
    ) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.borders = BordersProps {
                borders,
                variant,
                color,
            }
        }
        self
    }

    pub fn with_files(&mut self, title: Option<String>, files: Vec<String>) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            let files: Vec<TextSpan> = files.into_iter().map(TextSpan::from).collect();
            props.texts = TextParts::new(title, Some(files));
        }
        self
    }
}

// -- states

/// ## OwnStates
///
/// OwnStates contains states for this component
#[derive(Clone)]
struct OwnStates {
    list_index: usize, // Index of selected element in list
    list_len: usize,   // Length of file list
    focus: bool,       // Has focus?
}

impl Default for OwnStates {
    fn default() -> Self {
        OwnStates {
            list_index: 0,
            list_len: 0,
            focus: false,
        }
    }
}

impl OwnStates {
    /// ### set_list_len
    ///
    /// Set list length
    pub fn set_list_len(&mut self, len: usize) {
        self.list_len = len;
    }

    /// ### get_list_index
    ///
    /// Return current value for list index
    pub fn get_list_index(&self) -> usize {
        self.list_index
    }

    /// ### incr_list_index
    ///
    /// Incremenet list index
    pub fn incr_list_index(&mut self) {
        // Check if index is at last element
        if self.list_index + 1 < self.list_len {
            self.list_index += 1;
        }
    }

    /// ### decr_list_index
    ///
    /// Decrement list index
    pub fn decr_list_index(&mut self) {
        // Check if index is bigger than 0
        if self.list_index > 0 {
            self.list_index -= 1;
        }
    }

    /// ### fix_list_index
    ///
    /// Keep index if possible, otherwise set to lenght - 1
    pub fn fix_list_index(&mut self) {
        if self.list_index >= self.list_len && self.list_len > 0 {
            self.list_index = self.list_len - 1;
        } else if self.list_len == 0 {
            self.list_index = 0;
        }
    }
}

// -- Component

/// ## FileList
///
/// File list component
pub struct FileList {
    props: Props,
    states: OwnStates,
}

impl FileList {
    /// ### new
    ///
    /// Instantiates a new FileList starting from Props
    /// The method also initializes the component states.
    pub fn new(props: Props) -> Self {
        // Initialize states
        let mut states: OwnStates = OwnStates::default();
        // Set list length
        states.set_list_len(match &props.texts.spans {
            Some(tokens) => tokens.len(),
            None => 0,
        });
        FileList { props, states }
    }
}

impl Component for FileList {
    #[cfg(not(tarpaulin_include))]
    fn render(&self, render: &mut Canvas, area: Rect) {
        if self.props.visible {
            // Make list
            let list_item: Vec<ListItem> = match self.props.texts.spans.as_ref() {
                None => vec![],
                Some(lines) => lines
                    .iter()
                    .map(|line| ListItem::new(Span::from(line.content.to_string())))
                    .collect(),
            };
            let (fg, bg): (Color, Color) = match self.states.focus {
                true => (Color::Black, self.props.background),
                false => (self.props.foreground, Color::Reset),
            };
            // Render
            let mut state: ListState = ListState::default();
            state.select(Some(self.states.list_index));
            render.render_stateful_widget(
                List::new(list_item)
                    .block(get_block(
                        &self.props.borders,
                        &self.props.texts.title,
                        self.states.focus,
                    ))
                    .start_corner(Corner::TopLeft)
                    .highlight_style(
                        Style::default()
                            .bg(bg)
                            .fg(fg)
                            .add_modifier(self.props.modifiers),
                    ),
                area,
                &mut state,
            );
        }
    }

    fn update(&mut self, props: Props) -> Msg {
        self.props = props;
        // re-Set list length
        self.states.set_list_len(match &self.props.texts.spans {
            Some(tokens) => tokens.len(),
            None => 0,
        });
        // Fix list index
        self.states.fix_list_index();
        Msg::None
    }

    fn get_props(&self) -> Props {
        self.props.clone()
    }

    fn on(&mut self, ev: Event) -> Msg {
        // Match event
        if let Event::Key(key) = ev {
            match key.code {
                KeyCode::Down => {
                    // Update states
                    self.states.incr_list_index();
                    Msg::None
                }
                KeyCode::Up => {
                    // Update states
                    self.states.decr_list_index();
                    Msg::None
                }
                KeyCode::PageDown => {
                    // Update states
                    for _ in 0..8 {
                        self.states.incr_list_index();
                    }
                    Msg::None
                }
                KeyCode::PageUp => {
                    // Update states
                    for _ in 0..8 {
                        self.states.decr_list_index();
                    }
                    Msg::None
                }
                KeyCode::Enter => {
                    // Report event
                    Msg::OnSubmit(self.get_state())
                }
                _ => {
                    // Return key event to activity
                    Msg::OnKey(key)
                }
            }
        } else {
            // Unhandled event
            Msg::None
        }
    }

    fn get_state(&self) -> Payload {
        Payload::Unsigned(self.states.get_list_index())
    }

    // -- events

    /// ### blur
    ///
    /// Blur component; basically remove focus
    fn blur(&mut self) {
        self.states.focus = false;
    }

    /// ### active
    ///
    /// Active component; basically give focus
    fn active(&mut self) {
        self.states.focus = true;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use tuirealm::event::KeyEvent;

    #[test]
    fn test_ui_components_file_list() {
        // Make component
        let mut component: FileList = FileList::new(
            FileListPropsBuilder::default()
                .hidden()
                .visible()
                .with_foreground(Color::Red)
                .with_background(Color::Blue)
                .with_borders(Borders::ALL, BorderType::Double, Color::Red)
                .with_files(
                    Some(String::from("files")),
                    vec![String::from("file1"), String::from("file2")],
                )
                .build(),
        );
        assert_eq!(component.props.foreground, Color::Red);
        assert_eq!(component.props.background, Color::Blue);
        assert_eq!(component.props.visible, true);
        assert_eq!(
            component.props.texts.title.as_ref().unwrap().as_str(),
            "files"
        );
        assert_eq!(component.props.texts.spans.as_ref().unwrap().len(), 2);
        // Verify states
        assert_eq!(component.states.list_index, 0);
        assert_eq!(component.states.list_len, 2);
        assert_eq!(component.states.focus, false);
        // Focus
        component.active();
        assert_eq!(component.states.focus, true);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Update
        let props = FileListPropsBuilder::from(component.get_props())
            .with_foreground(Color::Yellow)
            .hidden()
            .build();
        assert_eq!(component.update(props), Msg::None);
        assert_eq!(component.props.visible, false);
        assert_eq!(component.props.foreground, Color::Yellow);
        // Increment list index
        component.states.list_index += 1;
        assert_eq!(component.states.list_index, 1);
        // Update
        component.update(
            FileListPropsBuilder::from(component.get_props())
                .with_files(
                    Some(String::from("filelist")),
                    vec![
                        String::from("file1"),
                        String::from("file2"),
                        String::from("file3"),
                    ],
                )
                .build(),
        );
        // Verify states
        assert_eq!(component.states.list_index, 1); // Kept
        assert_eq!(component.states.list_len, 3);
        // get value
        assert_eq!(component.get_state(), Payload::Unsigned(1));
        // Render
        assert_eq!(component.states.list_index, 1);
        // Handle inputs
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Down))),
            Msg::None
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 2);
        // Index should be decremented
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Up))),
            Msg::None
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 1);
        // Index should be 2
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::PageDown))),
            Msg::None
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 2);
        // Index should be 0
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::PageUp))),
            Msg::None
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 0);
        // Enter
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Enter))),
            Msg::OnSubmit(Payload::Unsigned(0))
        );
        // On key
        assert_eq!(
            component.on(Event::Key(KeyEvent::from(KeyCode::Backspace))),
            Msg::OnKey(KeyEvent::from(KeyCode::Backspace))
        );
    }
}
