//! ## TextList
//!
//! `TextList` component renders a radio group

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
// locals
use super::{Canvas, Component, InputEvent, Msg, Payload, Props, PropsBuilder};
// ext
use tui::{
    layout::{Corner, Rect},
    style::Style,
    text::{Span, Spans},
    widgets::{Block, BorderType, List, ListItem},
};

// -- component

/// ## Table
///
/// Table is a table component. List n rows with n text span columns
pub struct Table {
    props: Props,
}

impl Table {
    /// ### new
    ///
    /// Instantiate a new Table component
    pub fn new(props: Props) -> Self {
        Table { props }
    }
}

impl Component for Table {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    #[cfg(not(tarpaulin_include))]
    fn render(&self, render: &mut Canvas, area: Rect) {
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
            render.render_widget(
                List::new(list_items)
                    .block(
                        Block::default()
                            .borders(self.props.borders)
                            .border_style(Style::default())
                            .border_type(BorderType::Rounded)
                            .title(title),
                    )
                    .start_corner(Corner::TopLeft),
                area,
            );
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
    fn blur(&mut self) {}

    /// ### active
    ///
    /// Active component
    fn active(&mut self) {}
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
        // Get value
        assert_eq!(component.get_value(), Payload::None);
        // Event
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Delete))),
            Msg::OnKey(KeyEvent::from(KeyCode::Delete))
        );
    }
}
