//! ## MsgBox
//!
//! `MsgBox` component renders a simple readonly no event associated centered text

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
// deps
extern crate textwrap;
// locals
use super::{Canvas, Component, InputEvent, Msg, Payload, Props, PropsBuilder};
use crate::utils::fmt::align_text_center;
// ext
use tui::{
    layout::{Corner, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, List, ListItem},
};

// -- state

struct OwnStates {
    focus: bool,
}

impl Default for OwnStates {
    fn default() -> Self {
        Self { focus: false }
    }
}

// -- component

pub struct MsgBox {
    props: Props,
    states: OwnStates,
}

impl MsgBox {
    /// ### new
    ///
    /// Instantiate a new Text component
    pub fn new(props: Props) -> Self {
        MsgBox {
            props,
            states: OwnStates::default(),
        }
    }
}

impl Component for MsgBox {
    /// ### render
    ///
    /// Based on the current properties and states, renders a widget using the provided render engine in the provided Area
    /// If focused, cursor is also set (if supported by widget)
    #[cfg(not(tarpaulin_include))]
    fn render(&self, render: &mut Canvas, area: Rect) {
        // Make a Span
        if self.props.visible {
            let lines: Vec<ListItem> = match self.props.texts.rows.as_ref() {
                None => Vec::new(),
                Some(rows) => {
                    let mut lines: Vec<ListItem> = Vec::new();
                    for line in rows.iter() {
                        // Keep line color, or use default
                        let line_fg: Color = match line.fg {
                            Color::Reset => self.props.foreground,
                            _ => line.fg,
                        };
                        let line_bg: Color = match line.bg {
                            Color::Reset => self.props.background,
                            _ => line.bg,
                        };
                        let message_row =
                            textwrap::wrap(line.content.as_str(), area.width as usize);
                        for msg in message_row.iter() {
                            lines.push(ListItem::new(Spans::from(vec![Span::styled(
                                align_text_center(msg, area.width),
                                Style::default()
                                    .add_modifier(line.get_modifiers())
                                    .fg(line_fg)
                                    .bg(line_bg),
                            )])));
                        }
                    }
                    lines
                }
            };
            let title: String = match self.props.texts.title.as_ref() {
                Some(t) => t.clone(),
                None => String::new(),
            };
            render.render_widget(
                List::new(lines)
                    .block(
                        Block::default()
                            .borders(self.props.borders)
                            .border_style(Style::default().fg(self.props.foreground))
                            .border_type(BorderType::Rounded)
                            .title(title),
                    )
                    .start_corner(Corner::TopLeft)
                    .style(
                        Style::default()
                            .fg(self.props.foreground)
                            .bg(self.props.background),
                    ),
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
    fn test_ui_layout_components_msgbox() {
        let mut component: MsgBox = MsgBox::new(
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
        // Event
        assert_eq!(
            component.on(InputEvent::Key(KeyEvent::from(KeyCode::Delete))),
            Msg::OnKey(KeyEvent::from(KeyCode::Delete))
        );
    }
}
