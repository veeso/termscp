//! ## Props
//!
//! `Props` is the module which defines properties for layout components

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

// ext
use tui::style::{Color, Modifier};

// -- Props

/// ## Props
///
/// Props holds all the possible properties for a layout component
#[derive(Clone)]
pub struct Props {
    // Values
    pub visible: bool,            // Is the element visible ON CREATE?
    pub foreground: Color,        // Foreground color
    pub background: Color,        // Background color
    pub bold: bool,               // Text bold
    pub italic: bool,             // Italic
    pub underlined: bool,         // Underlined
    pub input_type: InputType,    // Input type
    pub input_len: Option<usize>, // max input len
    pub texts: TextParts,         // text parts
    pub value: PropValue,         // Initial value
}

impl Default for Props {
    fn default() -> Self {
        Self {
            // Values
            visible: true,
            foreground: Color::Reset,
            background: Color::Reset,
            bold: false,
            italic: false,
            underlined: false,
            input_type: InputType::Text,
            input_len: None,
            texts: TextParts::default(),
            value: PropValue::None,
        }
    }
}

impl Props {
    /// ### get_modifiers
    ///
    /// Get text modifiers from properties
    pub fn get_modifiers(&self) -> Modifier {
        Modifier::empty()
            | (match self.bold {
                true => Modifier::BOLD,
                false => Modifier::empty(),
            })
            | (match self.italic {
                true => Modifier::ITALIC,
                false => Modifier::empty(),
            })
            | (match self.underlined {
                true => Modifier::UNDERLINED,
                false => Modifier::empty(),
            })
    }
}

// -- Props builder

/// ## PropsBuilder
///
/// Chain constructor for `Props`
pub struct PropsBuilder {
    props: Option<Props>,
}

impl PropsBuilder {
    /// ### build
    ///
    /// Build Props from builder
    pub fn build(&mut self) -> Props {
        self.props.take().unwrap()
    }

    /// ### hidden
    ///
    /// Initialize props with visible set to False
    pub fn hidden(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = false;
        }
        self
    }

    /// ### visible
    ///
    /// Initialize props with visible set to True
    pub fn visible(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.visible = true;
        }
        self
    }

    /// ### with_foreground
    ///
    /// Set foreground color for component
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.foreground = color;
        }
        self
    }

    /// ### with_background
    ///
    /// Set background color for component
    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.background = color;
        }
        self
    }

    /// ### bold
    ///
    /// Set bold property for component
    pub fn bold(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.bold = true;
        }
        self
    }

    /// ### italic
    ///
    /// Set italic property for component
    pub fn italic(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.italic = true;
        }
        self
    }

    /// ### underlined
    ///
    /// Set underlined property for component
    pub fn underlined(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.underlined = true;
        }
        self
    }

    /// ### with_texts
    ///
    /// Set texts for component
    pub fn with_texts(&mut self, texts: TextParts) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.texts = texts;
        }
        self
    }

    /// ### with_input
    ///
    /// Set input type for component
    pub fn with_input(&mut self, input_type: InputType) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.input_type = input_type;
        }
        self
    }

    /// ### with_input_len
    ///
    /// Set max input len
    pub fn with_input_len(&mut self, len: usize) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.input_len = Some(len);
        }
        self
    }

    /// ### with_value
    ///
    /// Set initial value for component
    pub fn with_value(&mut self, value: PropValue) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.value = value;
        }
        self
    }
}

impl From<Props> for PropsBuilder {
    fn from(props: Props) -> Self {
        PropsBuilder { props: Some(props) }
    }
}

impl Default for PropsBuilder {
    fn default() -> Self {
        PropsBuilder {
            props: Some(Props::default()),
        }
    }
}

// -- Text parts

/// ## TextParts
///
/// TextParts holds optional component for the text displayed by a component
#[derive(Clone)]
pub struct TextParts {
    pub title: Option<String>,
    pub rows: Option<Vec<TextSpan>>,
}

impl TextParts {
    /// ### new
    ///
    /// Instantiates a new TextParts entity
    pub fn new(title: Option<String>, rows: Option<Vec<TextSpan>>) -> Self {
        TextParts { title, rows }
    }
}

impl Default for TextParts {
    fn default() -> Self {
        TextParts {
            title: None,
            rows: None,
        }
    }
}

/// ### TextSpan
///
/// TextSpan is a "cell" of text with its attributes
#[derive(Clone, std::fmt::Debug)]
pub struct TextSpan {
    pub content: String,
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
}

impl From<&str> for TextSpan {
    fn from(txt: &str) -> Self {
        TextSpan {
            content: txt.to_string(),
            fg: Color::Reset,
            bg: Color::Reset,
            bold: false,
            italic: false,
            underlined: false,
        }
    }
}

// -- TextSpan builder

/// ## TextSpanBuilder
///
/// TextSpanBuilder is a struct which helps building quickly a TextSpan
pub struct TextSpanBuilder {
    text: Option<TextSpan>,
}

impl TextSpanBuilder {
    /// ### new
    ///
    /// Instantiate a new TextSpanBuilder
    pub fn new(text: &str) -> Self {
        TextSpanBuilder {
            text: Some(TextSpan::from(text)),
        }
    }

    /// ### with_foreground
    ///
    /// Set foreground for text span
    pub fn with_foreground(&mut self, color: Color) -> &mut Self {
        if let Some(text) = self.text.as_mut() {
            text.fg = color;
        }
        self
    }

    /// ### with_background
    ///
    /// Set background for text span
    pub fn with_background(&mut self, color: Color) -> &mut Self {
        if let Some(text) = self.text.as_mut() {
            text.bg = color;
        }
        self
    }

    /// ### italic
    ///
    /// Set italic for text span
    pub fn italic(&mut self) -> &mut Self {
        if let Some(text) = self.text.as_mut() {
            text.italic = true;
        }
        self
    }
    /// ### bold
    ///
    /// Set bold for text span
    pub fn bold(&mut self) -> &mut Self {
        if let Some(text) = self.text.as_mut() {
            text.bold = true;
        }
        self
    }

    /// ### underlined
    ///
    /// Set underlined for text span
    pub fn underlined(&mut self) -> &mut Self {
        if let Some(text) = self.text.as_mut() {
            text.underlined = true;
        }
        self
    }

    /// ### build
    ///
    /// Make TextSpan out of builder
    pub fn build(&mut self) -> TextSpan {
        self.text.take().unwrap()
    }
}

// -- Prop value

/// ### PropValue
///
/// PropValue describes a property initial value
#[derive(Clone, PartialEq, std::fmt::Debug)]
pub enum PropValue {
    Str(String),
    Unsigned(usize),
    Signed(isize),
    Float(f64),
    Boolean(bool),
    None,
}

// -- Input Type

/// ## InputType
///
/// Input type for text inputs
#[derive(Clone, Copy, PartialEq, std::fmt::Debug)]
pub enum InputType {
    Text,
    Number,
    Password,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ui_layout_props_default() {
        let props: Props = Props::default();
        assert_eq!(props.visible, true);
        assert_eq!(props.background, Color::Reset);
        assert_eq!(props.foreground, Color::Reset);
        assert_eq!(props.bold, false);
        assert_eq!(props.italic, false);
        assert_eq!(props.underlined, false);
        assert!(props.texts.title.is_none());
        assert_eq!(props.input_type, InputType::Text);
        assert!(props.input_len.is_none());
        assert_eq!(props.value, PropValue::None);
        assert!(props.texts.rows.is_none());
    }

    #[test]
    fn test_ui_layout_props_modifiers() {
        // Make properties
        let props: Props = PropsBuilder::default().bold().italic().underlined().build();
        // Get modifiers
        let modifiers: Modifier = props.get_modifiers();
        assert!(modifiers.intersects(Modifier::BOLD));
        assert!(modifiers.intersects(Modifier::ITALIC));
        assert!(modifiers.intersects(Modifier::UNDERLINED));
    }

    #[test]
    fn test_ui_layout_props_builder() {
        let props: Props = PropsBuilder::default()
            .hidden()
            .with_background(Color::Blue)
            .with_foreground(Color::Green)
            .bold()
            .italic()
            .underlined()
            .with_texts(TextParts::new(
                Some(String::from("hello")),
                Some(vec![TextSpan::from("hey")]),
            ))
            .with_input(InputType::Password)
            .with_input_len(16)
            .with_value(PropValue::Str(String::from("Hello")))
            .build();
        assert_eq!(props.background, Color::Blue);
        assert_eq!(props.bold, true);
        assert_eq!(props.foreground, Color::Green);
        assert_eq!(props.italic, true);
        assert_eq!(props.texts.title.as_ref().unwrap().as_str(), "hello");
        assert_eq!(props.input_type, InputType::Password);
        assert_eq!(*props.input_len.as_ref().unwrap(), 16);
        if let PropValue::Str(s) = props.value {
            assert_eq!(s.as_str(), "Hello");
        } else {
            panic!("Expected value to be a string");
        }
        assert_eq!(
            props
                .texts
                .rows
                .as_ref()
                .unwrap()
                .get(0)
                .unwrap()
                .content
                .as_str(),
            "hey"
        );
        assert_eq!(props.underlined, true);
        assert_eq!(props.visible, false);
        let props: Props = PropsBuilder::default()
            .visible()
            .with_background(Color::Blue)
            .with_foreground(Color::Green)
            .bold()
            .italic()
            .underlined()
            .with_texts(TextParts::new(
                Some(String::from("hello")),
                Some(vec![TextSpan::from("hey")]),
            ))
            .build();
        assert_eq!(props.background, Color::Blue);
        assert_eq!(props.bold, true);
        assert_eq!(props.foreground, Color::Green);
        assert_eq!(props.italic, true);
        assert_eq!(props.texts.title.as_ref().unwrap().as_str(), "hello");
        assert_eq!(
            props
                .texts
                .rows
                .as_ref()
                .unwrap()
                .get(0)
                .unwrap()
                .content
                .as_str(),
            "hey"
        );
        assert_eq!(props.underlined, true);
        assert_eq!(props.visible, true);
    }

    #[test]
    #[should_panic]
    fn test_ui_layout_props_build_twice() {
        let mut builder: PropsBuilder = PropsBuilder::default();
        let _ = builder.build();
        builder
            .hidden()
            .with_background(Color::Blue)
            .with_foreground(Color::Green)
            .bold()
            .italic()
            .underlined()
            .with_texts(TextParts::new(
                Some(String::from("hello")),
                Some(vec![TextSpan::from("hey")]),
            ));
        // Rebuild
        let _ = builder.build();
    }

    #[test]
    fn test_ui_layout_props_builder_from_props() {
        let props: Props = PropsBuilder::default()
            .hidden()
            .with_background(Color::Blue)
            .with_foreground(Color::Green)
            .bold()
            .italic()
            .underlined()
            .with_texts(TextParts::new(
                Some(String::from("hello")),
                Some(vec![TextSpan::from("hey")]),
            ))
            .build();
        // Ok, now make a builder from properties
        let builder: PropsBuilder = PropsBuilder::from(props);
        assert!(builder.props.is_some());
    }

    #[test]
    fn test_ui_layout_props_text_parts_with_values() {
        let parts: TextParts = TextParts::new(
            Some(String::from("Hello world!")),
            Some(vec![TextSpan::from("row1"), TextSpan::from("row2")]),
        );
        assert_eq!(parts.title.as_ref().unwrap().as_str(), "Hello world!");
        assert_eq!(
            parts
                .rows
                .as_ref()
                .unwrap()
                .get(0)
                .unwrap()
                .content
                .as_str(),
            "row1"
        );
        assert_eq!(
            parts
                .rows
                .as_ref()
                .unwrap()
                .get(1)
                .unwrap()
                .content
                .as_str(),
            "row2"
        );
    }

    #[test]
    fn test_ui_layout_props_text_parts_default() {
        let parts: TextParts = TextParts::default();
        assert!(parts.title.is_none());
        assert!(parts.rows.is_none());
    }

    #[test]
    fn test_ui_layout_props_text_span() {
        let span: TextSpan = TextSpan::from("Hello!");
        assert_eq!(span.content.as_str(), "Hello!");
        assert_eq!(span.bold, false);
        assert_eq!(span.fg, Color::Reset);
        assert_eq!(span.bg, Color::Reset);
        assert_eq!(span.italic, false);
        assert_eq!(span.underlined, false);
        // With attributes
        let span: TextSpan = TextSpanBuilder::new("Error")
            .with_background(Color::Red)
            .with_foreground(Color::Black)
            .bold()
            .italic()
            .underlined()
            .build();
        assert_eq!(span.content.as_str(), "Error");
        assert_eq!(span.bold, true);
        assert_eq!(span.fg, Color::Black);
        assert_eq!(span.bg, Color::Red);
        assert_eq!(span.italic, true);
        assert_eq!(span.underlined, true);
    }
}
