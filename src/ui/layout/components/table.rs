//! ## TextList
//!
//! `TextList` component renders a radio group

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
    layout::Corner,
    style::Style,
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
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

/// ## Table
///
/// Table is a table component. List n rows with n text span columns
pub struct Table {
    props: Props,
    states: OwnStates,
}

impl Table {
    /// ### new
    ///
    /// Instantiate a new Table component
    pub fn new(props: Props) -> Self {
        Table {
            props,
            states: OwnStates::default(),
        }
    }
}

impl Component for Table {
    /// ### render
    ///
    /// Based on the current properties and states, return a Widget instance for the Component
    /// Returns None if the component is hidden
    fn render(&self) -> Option<Render> {
        // Make a Span
        if self.props.visible {
            let title: String = match self.props.texts.title.as_ref() {
                Some(t) => t.clone(),
                None => String::new(),
            };
            // Make list entries
            let list_items: Vec<ListItem> = match self.props.texts.table.as_ref() {
                None => Vec::new(),
                Some(table) => table
                    .iter()
                    .map(|row| {
                        let columns: Vec<Span> = row
                            .iter()
                            .map(|col| {
                                Span::styled(
                                    col.content.clone(),
                                    Style::default()
                                        .add_modifier(col.get_modifiers())
                                        .fg(col.fg)
                                        .bg(col.bg),
                                )
                            })
                            .collect();
                        ListItem::new(Spans::from(columns))
                    })
                    .collect(), // Make List item from TextSpan
            };
            // Make list
            Some(Render {
                cursor: 0,
                widget: Box::new(
                    List::new(list_items)
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(Style::default())
                                .border_type(BorderType::Rounded)
                                .title(title),
                        )
                        .start_corner(Corner::TopLeft),
                ),
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
    use crate::ui::layout::props::{TableBuilder, TextParts, TextSpan};

    use crossterm::event::{KeyCode, KeyEvent};

    #[test]
    fn test_ui_layout_components_table() {
        let mut component: Table = Table::new(
            PropsBuilder::default()
                .with_texts(TextParts::table(
                    Some(String::from("My data")),
                    TableBuilder::default()
                        .add_col(TextSpan::from("name"))
                        .add_col(TextSpan::from("age"))
                        .add_row()
                        .add_col(TextSpan::from("omar"))
                        .add_col(TextSpan::from("24"))
                        .build(),
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
