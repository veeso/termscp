//! ## Text
//!
//! `Text` component renders a simple readonly no event associated text

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
use super::{Component, InputEvent, Msg, Payload, Props, PropsBuilder, Render};
// ext
use tui::{
    style::Style,
    text::{Span, Spans, Text as TuiText},
    widgets::Paragraph,
};

// -- state

struct OwnStates {
    focus: bool,
}

impl Default for OwnStates {
    fn default() -> Self {
        OwnStates { focus: false }
    }
}

// -- component

pub struct Text {
    props: Props,
    states: OwnStates,
}

impl Text {
    /// ### new
    ///
    /// Instantiate a new Text component
    pub fn new(props: Props) -> Self {
        Text {
            props,
            states: OwnStates::default(),
        }
    }
}

impl Component for Text {
    /// ### render
    ///
    /// Based on the current properties and states, return a Widget instance for the Component
    /// Returns None if the component is hidden
    fn render(&self) -> Option<Render> {
        // Make a Span
        if self.props.visible {
            let spans: Vec<Span> = match self.props.texts.rows.as_ref() {
                None => Vec::new(),
                Some(rows) => rows
                    .iter()
                    .map(|x| {
                        Span::styled(
                            x.content.clone(),
                            Style::default()
                                .add_modifier(x.get_modifiers())
                                .fg(x.fg)
                                .bg(x.bg),
                        )
                    })
                    .collect(),
            };
            // Make text
            let mut text: TuiText = TuiText::from(Spans::from(spans));
            // Apply style
            text.patch_style(
                Style::default()
                    .add_modifier(self.props.get_modifiers())
                    .fg(self.props.foreground)
                    .bg(self.props.background),
            );
            Some(Render {
                widget: Box::new(Paragraph::new(text)),
                cursor: 0,
            })
        } else {
            // Invisible
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
        // Return None
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
    /// Handle input event and update internal states.
    /// Returns a Msg to the view.
    /// Returns always None, since cannot have any focus
    fn on(&mut self, ev: InputEvent) -> Msg {
        // Return key
        if let InputEvent::Key(key) = ev {
            Msg::OnKey(key)
        } else {
            Msg::None
        }
    }

    /// ### get_value
    ///
    /// Get current value from component
    /// For this component returns always None
    fn get_value(&self) -> Payload {
        Payload::None
    }

    // -- events

    /// ### blur
    ///
    /// Blur component
    fn blur(&mut self) {
        self.states.focus = false;
    }

    /// ### active
    ///
    /// Active component
    fn active(&mut self) {
        self.states.focus = true;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ui::layout::props::{TextParts, TextSpan, TextSpanBuilder};

    use crossterm::event::{KeyCode, KeyEvent};
    use tui::style::Color;

    #[test]
    fn test_ui_layout_components_text() {
        let mut component: Text = Text::new(
            PropsBuilder::default()
                .with_texts(TextParts::new(
                    None,
                    Some(vec![
                        TextSpan::from("Press "),
                        TextSpanBuilder::new("<ESC>")
                            .with_foreground(Color::Cyan)
                            .bold()
                            .build(),
                        TextSpan::from(" to quit"),
                    ]),
                ))
                .build(),
        );
        // Focus
        assert_eq!(component.states.focus, false);
        component.active();
        assert_eq!(component.states.focus, true);
        component.blur();
        assert_eq!(component.states.focus, false);
        // Get value
        assert_eq!(component.get_value(), Payload::None);
        // Render
        assert_eq!(component.render().unwrap().cursor, 0);
        // Event
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Delete))),
            Msg::OnKey(KeyEvent::from(KeyCode::Delete))
        );
    }
}
