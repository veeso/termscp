//! ## Bookmark list
//!
//! `BookmarkList` component renders a bookmark list tab

/*
*
*   Copyright (C) 2020-2021 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "TermSCP"
*
*   TermSCP is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   TermSCP is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with TermSCP.  If not, see <http://www.gnu.org/licenses/>.
*
*/

// locals
use super::{Canvas, Component, InputEvent, Msg, Payload, Props, PropsBuilder};
// ext
use crossterm::event::KeyCode;
use tui::{
    layout::{Corner, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, List, ListItem, ListState},
};

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

    /// ### reset_list_index
    ///
    /// Reset list index to 0
    pub fn reset_list_index(&mut self) {
        self.list_index = 0;
    }
}

// -- Component

/// ## BookmarkList
///
/// Bookmark list component
pub struct BookmarkList {
    props: Props,
    states: OwnStates,
}

impl BookmarkList {
    /// ### new
    ///
    /// Instantiates a new FileList starting from Props
    /// The method also initializes the component states.
    pub fn new(props: Props) -> Self {
        // Initialize states
        let mut states: OwnStates = OwnStates::default();
        // Set list length
        states.set_list_len(match &props.texts.rows {
            Some(tokens) => tokens.len(),
            None => 0,
        });
        BookmarkList { props, states }
    }
}

impl Component for BookmarkList {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    #[cfg(not(tarpaulin_include))]
    fn render(&self, render: &mut Canvas, area: Rect) {
        if self.props.visible {
            // Make list
            let list_item: Vec<ListItem> = match self.props.texts.rows.as_ref() {
                None => vec![],
                Some(lines) => lines
                    .iter()
                    .map(|line| ListItem::new(Span::from(line.content.to_string())))
                    .collect(),
            };
            let (fg, bg): (Color, Color) = match self.states.focus {
                true => (self.props.foreground, self.props.background),
                false => (Color::Reset, Color::Reset),
            };
            let title: String = match self.props.texts.title.as_ref() {
                Some(t) => t.clone(),
                None => String::new(),
            };
            // Render
            let mut state: ListState = ListState::default();
            state.select(Some(self.states.list_index));
            render.render_stateful_widget(
                List::new(list_item)
                    .block(
                        Block::default()
                            .borders(self.props.borders)
                            .border_style(match self.states.focus {
                                true => Style::default().fg(self.props.background),
                                false => Style::default(),
                            })
                            .title(title),
                    )
                    .start_corner(Corner::TopLeft)
                    .highlight_style(
                        Style::default()
                            .bg(bg)
                            .fg(fg)
                            .add_modifier(self.props.get_modifiers()),
                    ),
                area,
                &mut state,
            );
        }
    }

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update
    fn update(&mut self, props: Props) -> Msg {
        self.props = props;
        // re-Set list length
        self.states.set_list_len(match &self.props.texts.rows {
            Some(tokens) => tokens.len(),
            None => 0,
        });
        // Reset list index
        self.states.reset_list_index();
        Msg::None
    }

    /// ### get_props
    ///
    /// Returns a props builder starting from component properties.
    /// This returns a prop builder in order to make easier to create
    /// new properties for the element.
    fn get_props(&self) -> PropsBuilder {
        PropsBuilder::from(self.props.clone())
    }

    /// ### on
    ///
    /// Handle input event and update internal states
    fn on(&mut self, ev: InputEvent) -> Msg {
        // Match event
        if let InputEvent::Key(key) = ev {
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
                    Msg::OnSubmit(self.get_value())
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

    /// ### get_value
    ///
    /// Return component value. File list return index
    fn get_value(&self) -> Payload {
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
    use crate::ui::layout::props::{TextParts, TextSpan};

    use crossterm::event::KeyEvent;

    #[test]
    fn test_ui_layout_components_bookmarks_list() {
        // Make component
        let mut component: BookmarkList = BookmarkList::new(
            PropsBuilder::default()
                .with_texts(TextParts::new(
                    Some(String::from("filelist")),
                    Some(vec![TextSpan::from("file1"), TextSpan::from("file2")]),
                ))
                .build(),
        );
        // Verify states
        assert_eq!(component.states.list_index, 0);
        assert_eq!(component.states.list_len, 2);
        assert_eq!(component.states.focus, false);
        // Focus
        component.active();
        assert_eq!(component.states.focus, true);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Increment list index
        component.states.list_index += 1;
        assert_eq!(component.states.list_index, 1);
        // Update
        component.update(
            component
                .get_props()
                .with_texts(TextParts::new(
                    Some(String::from("filelist")),
                    Some(vec![
                        TextSpan::from("file1"),
                        TextSpan::from("file2"),
                        TextSpan::from("file3"),
                    ]),
                ))
                .build(),
        );
        // Verify states
        assert_eq!(component.states.list_index, 0);
        assert_eq!(component.states.list_len, 3);
        // get value
        assert_eq!(component.get_value(), Payload::Unsigned(0));
        // Render
        assert_eq!(component.states.list_index, 0);
        // Handle inputs
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Down))),
            Msg::None
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 1);
        // Index should be decremented
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Up))),
            Msg::None
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 0);
        // Index should be 2
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::PageDown))),
            Msg::None
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 2);
        // Index should be 0
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::PageUp))),
            Msg::None
        );
        // Index should be incremented
        assert_eq!(component.states.list_index, 0);
        // Enter
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Enter))),
            Msg::OnSubmit(Payload::Unsigned(0))
        );
        // On key
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Backspace))),
            Msg::OnKey(KeyEvent::from(KeyCode::Backspace))
        );
    }
}
