//! ## Input
//!
//! `Input` component renders an input box

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
use super::super::props::InputType;
use super::{Component, InputEvent, Msg, Payload, PropValue, Props, PropsBuilder, Render};
// ext
use crossterm::event::{KeyCode, KeyModifiers};
use tui::{
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
};

// -- states

/// ## OwnStates
///
/// OwnStates contains states for this component
#[derive(Clone)]
struct OwnStates {
    input: Vec<char>, // Current input
    cursor: usize,    // Input position
    focus: bool,      // Focus
}

impl Default for OwnStates {
    fn default() -> Self {
        OwnStates {
            input: Vec::new(),
            cursor: 0,
            focus: false,
        }
    }
}

impl OwnStates {
    /// ### append
    ///
    /// Append, if possible according to input type, the character to the input vec
    pub fn append(&mut self, ch: char, itype: InputType, max_len: Option<usize>) {
        // Check if max length has been reached
        if self.input.len() < max_len.unwrap_or(usize::MAX) {
            match itype {
                InputType::Number => {
                    if ch.is_digit(10) {
                        // Must be digit
                        self.input.insert(self.cursor, ch);
                        // Increment cursor
                        self.cursor += 1;
                    }
                }
                _ => {
                    // No rule
                    self.input.insert(self.cursor, ch);
                    // Increment cursor
                    self.cursor += 1;
                }
            }
        }
    }

    /// ### backspace
    ///
    /// Delete element at cursor -1; then decrement cursor by 1
    pub fn backspace(&mut self) {
        if self.cursor > 0 && self.input.len() > 0 {
            self.input.remove(self.cursor - 1);
            // Decrement cursor
            self.cursor -= 1;
        }
    }

    /// ### delete
    ///
    /// Delete element at cursor
    pub fn delete(&mut self) {
        if self.cursor < self.input.len() {
            self.input.remove(self.cursor);
        }
    }

    /// ### incr_cursor
    ///
    /// Increment cursor value by one if possible
    pub fn incr_cursor(&mut self) {
        if self.cursor + 1 <= self.input.len() {
            self.cursor += 1;
        }
    }

    /// ### decr_cursor
    ///
    /// Decrement cursor value by one if possible
    pub fn decr_cursor(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// ### render_value
    ///
    /// Get value as string to render
    pub fn render_value(&self, itype: InputType) -> String {
        match itype {
            InputType::Password => (0..self.input.len()).map(|_| '*').collect(),
            _ => self.get_value(),
        }
    }

    /// ### get_value
    ///
    /// Get value as string
    pub fn get_value(&self) -> String {
        self.input.iter().collect()
    }
}

// -- Component

/// ## FileList
///
/// File list component
pub struct Input {
    props: Props,
    states: OwnStates,
}

impl Input {
    /// ### new
    ///
    /// Instantiates a new Input starting from Props
    /// The method also initializes the component states.
    pub fn new(props: Props) -> Self {
        // Initialize states
        let mut states: OwnStates = OwnStates::default();
        // Set state value from props
        if let PropValue::Str(val) = props.value.clone() {
            for ch in val.chars() {
                states.append(ch, props.input_type, props.input_len);
            }
        }
        Input { props, states }
    }
}

impl Component for Input {
    /// ### render
    ///
    /// Based on the current properties and states, return a Widget instance for the Component
    /// Returns None if the component is hidden
    fn render(&self) -> Option<Render> {
        if self.props.visible {
            let title: String = match self.props.texts.title.as_ref() {
                Some(t) => t.clone(),
                None => String::new(),
            };
            let p: Paragraph = Paragraph::new(self.states.render_value(self.props.input_type))
                .style(match self.states.focus {
                    true => Style::default().fg(self.props.foreground),
                    false => Style::default(),
                })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title(title),
                );
            Some(Render {
                widget: Box::new(p),
                cursor: self.states.cursor,
            })
        } else {
            None
        }
    }

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update.
    /// Returns a Msg to the view
    fn update(&mut self, props: Props) -> Msg {
        self.props = props;
        // Set value from props
        if let PropValue::Str(val) = self.props.value.clone() {
            self.states.input = Vec::new();
            self.states.cursor = 0;
            for ch in val.chars() {
                self.states
                    .append(ch, self.props.input_type, self.props.input_len);
            }
        }
        Msg::None
    }

    /// ### get_props
    ///
    /// Returns a props builder starting from component properties.
    /// This returns a prop builder in order to make easier to create
    /// new properties for the element.
    fn get_props(&self) -> PropsBuilder {
        // Make properties with value from states
        let mut props: Props = self.props.clone();
        props.value = PropValue::Str(self.states.get_value());
        PropsBuilder::from(props)
    }

    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view
    fn on(&mut self, ev: InputEvent) -> Msg {
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Backspace => {
                    // Backspace and None
                    self.states.backspace();
                    Msg::None
                }
                KeyCode::Delete => {
                    // Delete and None
                    self.states.delete();
                    Msg::None
                }
                KeyCode::Enter => Msg::OnSubmit(self.get_value()),
                KeyCode::Left => {
                    // Move cursor left; msg None
                    self.states.decr_cursor();
                    Msg::None
                }
                KeyCode::Right => {
                    // Move cursor right; Msg None
                    self.states.incr_cursor();
                    Msg::None
                }
                KeyCode::Char(ch) => {
                    // Check if modifiers is NOT CTRL OR ALT
                    if !key.modifiers.intersects(KeyModifiers::CONTROL)
                        && !key.modifiers.intersects(KeyModifiers::ALT)
                    {
                        // Push char to input
                        self.states
                            .append(ch, self.props.input_type, self.props.input_len);
                        // Message none
                        Msg::None
                    } else {
                        // Return key
                        Msg::OnKey(key)
                    }
                }
                _ => Msg::OnKey(key),
            }
        } else {
            Msg::None
        }
    }

    /// ### get_value
    ///
    /// Get current value from component
    /// Returns the value as string or as a number based on the input value
    fn get_value(&self) -> Payload {
        match self.props.input_type {
            InputType::Number => {
                Payload::Unsigned(self.states.get_value().parse::<usize>().ok().unwrap_or(0))
            }
            _ => Payload::Text(self.states.get_value()),
        }
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

    use crossterm::event::KeyEvent;

    #[test]
    fn test_ui_layout_components_input_text() {
        // Instantiate Input with value
        let mut component: Input = Input::new(
            PropsBuilder::default()
                .with_input(InputType::Text)
                .with_input_len(5)
                .with_value(PropValue::Str(String::from("home")))
                .build(),
        );
        // Verify initial state
        assert_eq!(component.states.cursor, 4);
        assert_eq!(component.states.input.len(), 4);
        // Focus
        assert_eq!(component.states.focus, false);
        component.active();
        assert_eq!(component.states.focus, true);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Get value
        assert_eq!(component.get_value(), Payload::Text(String::from("home")));
        // Render
        assert_eq!(component.render().unwrap().cursor, 4);
        // Handle events
        // Try key with ctrl
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::new(
                KeyCode::Char('a'),
                KeyModifiers::CONTROL
            ))),
            Msg::OnKey(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL)),
        );
        // String shouldn't have changed
        assert_eq!(component.get_value(), Payload::Text(String::from("home")));
        assert_eq!(component.render().unwrap().cursor, 4);
        // Character
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Char('/')))),
            Msg::None
        );
        assert_eq!(component.get_value(), Payload::Text(String::from("home/")));
        assert_eq!(component.render().unwrap().cursor, 5);
        // Verify max length (shouldn't push any character)
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::None
        );
        assert_eq!(component.get_value(), Payload::Text(String::from("home/")));
        assert_eq!(component.render().unwrap().cursor, 5);
        // Enter
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Enter))),
            Msg::OnSubmit(Payload::Text(String::from("home/")))
        );
        // Backspace
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Backspace))),
            Msg::None
        );
        assert_eq!(component.get_value(), Payload::Text(String::from("home")));
        assert_eq!(component.render().unwrap().cursor, 4);
        // Check backspace at 0
        component.states.input = vec!['h'];
        component.states.cursor = 1;
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Backspace))),
            Msg::None
        );
        assert_eq!(component.get_value(), Payload::Text(String::from("")));
        assert_eq!(component.render().unwrap().cursor, 0);
        // Another one...
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Backspace))),
            Msg::None
        );
        assert_eq!(component.get_value(), Payload::Text(String::from("")));
        assert_eq!(component.render().unwrap().cursor, 0);
        // See del behaviour here
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Delete))),
            Msg::None
        );
        assert_eq!(component.get_value(), Payload::Text(String::from("")));
        assert_eq!(component.render().unwrap().cursor, 0);
        // Check del behaviour
        component.states.input = vec!['h', 'e'];
        component.states.cursor = 1;
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Delete))),
            Msg::None
        );
        assert_eq!(component.get_value(), Payload::Text(String::from("h")));
        assert_eq!(component.render().unwrap().cursor, 1); // Shouldn't move
                                                           // Another one (should do nothing)
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Delete))),
            Msg::None
        );
        assert_eq!(component.get_value(), Payload::Text(String::from("h")));
        assert_eq!(component.render().unwrap().cursor, 1); // Shouldn't move
                                                           // Move cursor right
        component.states.input = vec!['h', 'e', 'l', 'l', 'o'];
        component.states.cursor = 1;
        component.props.input_len = Some(16); // Let's change length
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Right))), // between 'e' and 'l'
            Msg::None
        );
        assert_eq!(component.render().unwrap().cursor, 2); // Should increment
                                                           // Put a character here
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::None
        );
        assert_eq!(component.get_value(), Payload::Text(String::from("heallo")));
        assert_eq!(component.render().unwrap().cursor, 3);
        // Move left
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Left))),
            Msg::None
        );
        assert_eq!(component.render().unwrap().cursor, 2); // Should decrement
                                                           // Go at the end
        component.states.cursor = 6;
        // Move right
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Right))),
            Msg::None
        );
        assert_eq!(component.render().unwrap().cursor, 6); // Should stay
                                                           // Move left
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Left))),
            Msg::None
        );
        assert_eq!(component.render().unwrap().cursor, 5); // Should decrement
                                                           // Go at the beginning
        component.states.cursor = 0;
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Left))),
            Msg::None
        );
        assert_eq!(component.render().unwrap().cursor, 0); // Should stay
        // Update value
        component.update(component.get_props().with_value(PropValue::Str("new-value".to_string())).build());
        assert_eq!(component.get_value(), Payload::Text(String::from("new-value")));
    }

    #[test]
    fn test_ui_layout_components_input_number() {
        // Instantiate Input with value
        let mut component: Input = Input::new(
            PropsBuilder::default()
                .with_input(InputType::Number)
                .with_input_len(5)
                .with_value(PropValue::Str(String::from("3000")))
                .build(),
        );
        // Verify initial state
        assert_eq!(component.states.cursor, 4);
        assert_eq!(component.states.input.len(), 4);
        // Push a non numeric value
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::None
        );
        assert_eq!(component.get_value(), Payload::Unsigned(3000));
        assert_eq!(component.render().unwrap().cursor, 4);
        // Push a number
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Char('1')))),
            Msg::None
        );
        assert_eq!(component.get_value(), Payload::Unsigned(30001));
        assert_eq!(component.render().unwrap().cursor, 5);
    }
}
