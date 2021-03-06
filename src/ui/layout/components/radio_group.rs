//! ## RadioGroup
//!
//! `RadioGroup` component renders a radio group

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
use super::{Component, InputEvent, Msg, Payload, PropValue, Props, PropsBuilder, Render};
// ext
use crossterm::event::KeyCode;
use tui::{
    style::{Color, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, Tabs},
};

// -- states

/// ## OwnStates
///
/// OwnStates contains states for this component
#[derive(Clone)]
struct OwnStates {
    choice: usize,        // Selected option
    choices: Vec<String>, // Available choices
    focus: bool,          // has focus?
}

impl Default for OwnStates {
    fn default() -> Self {
        OwnStates {
            choice: 0,
            choices: Vec::new(),
            focus: false,
        }
    }
}

impl OwnStates {
    /// ### next_choice
    ///
    /// Move choice index to next choice
    pub fn next_choice(&mut self) {
        if self.choice + 1 < self.choices.len() {
            self.choice += 1;
        }
    }

    /// ### prev_choice
    ///
    /// Move choice index to previous choice
    pub fn prev_choice(&mut self) {
        if self.choice > 0 {
            self.choice -= 1;
        }
    }
}

// -- component

/// ## RadioGroup
///
/// RadioGroup component represents a group of tabs to select from
pub struct RadioGroup {
    props: Props,
    states: OwnStates,
}

impl RadioGroup {
    /// ### new
    ///
    /// Instantiate a new Radio Group component
    pub fn new(props: Props) -> Self {
        // Make states
        let mut states: OwnStates = OwnStates::default();
        // Update choices
        states.choices = props.texts.body.clone().unwrap_or(Vec::new());
        // Get value
        if let PropValue::Unsigned(choice) = props.value {
            states.choice = choice;
        }
        RadioGroup { props, states }
    }
}

impl Component for RadioGroup {
    /// ### render
    ///
    /// Based on the current properties and states, return a Widget instance for the Component
    /// Returns None if the component is hidden
    fn render(&self) -> Option<Render> {
        match self.props.visible {
            false => None,
            true => {
                // Make choices
                let choices: Vec<Spans> = self
                    .states
                    .choices
                    .iter()
                    .map(|x| Spans::from(x.clone()))
                    .collect();
                // Make colors
                let (bg, fg, block_fg): (Color, Color, Color) = match &self.states.focus {
                    true => (
                        self.props.foreground,
                        self.props.background,
                        self.props.foreground,
                    ),
                    false => (Color::Reset, Color::Reset, Color::Reset),
                };
                let title: String = match &self.props.texts.title {
                    Some(t) => t.clone(),
                    None => String::new(),
                };
                Some(Render {
                    cursor: 0,
                    widget: Box::new(
                        Tabs::new(choices)
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .border_type(BorderType::Rounded)
                                    .style(Style::default().fg(block_fg))
                                    .title(title),
                            )
                            .select(self.states.choice)
                            .style(Style::default())
                            .highlight_style(
                                Style::default()
                                    .add_modifier(self.props.get_modifiers())
                                    .fg(fg)
                                    .bg(bg),
                            ),
                    ),
                })
            }
        }
    }

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update.
    /// Returns a Msg to the view
    fn update(&mut self, props: Props) -> Msg {
        // Reset choices
        self.states.choices = props.texts.body.clone().unwrap_or(Vec::new());
        // Get value
        if let PropValue::Unsigned(choice) = props.value {
            self.states.choice = choice;
        }
        self.props = props;
        // Msg none
        Msg::None
    }

    /// ### get_props
    ///
    /// Returns a props builder starting from component properties.
    /// This returns a prop builder in order to make easier to create
    /// new properties for the element.
    fn get_props(&self) -> PropsBuilder {
        PropsBuilder::from_props(&self.props)
    }

    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view
    fn on(&mut self, ev: InputEvent) -> Msg {
        // Match event
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Right => {
                    // Increment choice
                    self.states.next_choice();
                    // Return Msg On Change
                    Msg::OnChange(self.get_value())
                }
                KeyCode::Left => {
                    // Decrement choice
                    self.states.prev_choice();
                    // Return Msg On Change
                    Msg::OnChange(self.get_value())
                }
                KeyCode::Enter => {
                    // Return Submit
                    Msg::OnSubmit(self.get_value())
                }
                _ => {
                    // Return key event to activity
                    Msg::OnKey(key)
                }
            }
        } else {
            // Ignore event
            Msg::None
        }
    }

    /// ### get_value
    ///
    /// Get current value from component
    /// Returns the selected option
    fn get_value(&self) -> Payload {
        Payload::Unsigned(self.states.choice)
    }

    // -- events

    /// ### should_umount
    ///
    /// The component must provide to the supervisor whether it should be umounted (destroyed)
    /// This makes sense to be called after an `on` or after an `update`, where the states changes.
    /// Always false for this component
    fn should_umount(&self) -> bool {
        false
    }

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
    use crate::ui::layout::props::TextParts;

    use crossterm::event::KeyEvent;

    #[test]
    fn test_ui_layout_components_radio() {
        // Make component
        let mut component: RadioGroup = RadioGroup::new(
            PropsBuilder::default()
                .with_texts(TextParts::new(
                    Some(String::from("yes or no?")),
                    Some(vec![
                        String::from("Yes!"),
                        String::from("No"),
                        String::from("Maybe"),
                    ]),
                ))
                .with_value(PropValue::Unsigned(1))
                .build(),
        );
        // Verify states
        assert_eq!(component.states.choice, 1);
        assert_eq!(component.states.choices.len(), 3);
        // Focus
        assert_eq!(component.states.focus, false);
        component.active();
        assert_eq!(component.states.focus, true);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Should umount
        assert_eq!(component.should_umount(), false);
        // Get value
        assert_eq!(component.get_value(), Payload::Unsigned(1));
        // Render
        assert_eq!(component.render().unwrap().cursor, 0);
        // Handle events
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Left))),
            Msg::OnChange(Payload::Unsigned(0)),
        );
        assert_eq!(component.get_value(), Payload::Unsigned(0));
        // Left again
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Left))),
            Msg::OnChange(Payload::Unsigned(0)),
        );
        assert_eq!(component.get_value(), Payload::Unsigned(0));
        // Right
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Right))),
            Msg::OnChange(Payload::Unsigned(1)),
        );
        assert_eq!(component.get_value(), Payload::Unsigned(1));
        // Right again
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Right))),
            Msg::OnChange(Payload::Unsigned(2)),
        );
        assert_eq!(component.get_value(), Payload::Unsigned(2));
        // Right again
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Right))),
            Msg::OnChange(Payload::Unsigned(2)),
        );
        assert_eq!(component.get_value(), Payload::Unsigned(2));
        // Submit
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Enter))),
            Msg::OnSubmit(Payload::Unsigned(2)),
        );
        // Any key
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Char('a')))),
            Msg::OnKey(KeyEvent::from(KeyCode::Char('a'))),
        );
    }
}
