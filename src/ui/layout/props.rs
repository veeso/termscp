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
    pub visible: bool,     // Is the element visible ON CREATE?
    pub focus: bool,       // Is the element focused
    pub foreground: Color, // Foreground color
    pub background: Color, // Background color
    pub bold: bool,        // Text bold
    pub italic: bool,      // Italic
    pub underlined: bool,  // Underlined
    pub texts: TextParts,  // text parts
}

impl Default for Props {
    fn default() -> Self {
        Self {
            // Values
            visible: true,
            focus: false,
            foreground: Color::Reset,
            background: Color::Reset,
            bold: false,
            italic: false,
            underlined: false,
            texts: TextParts::default(),
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
    /// ### from_props
    ///
    /// Create a props builder from existing properties
    pub fn from_props(props: &Props) -> Self {
        PropsBuilder {
            props: Some(props.clone()),
        }
    }

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

    /// ### has_focus
    ///
    /// Initialize props with focus set to True
    pub fn has_focus(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.focus = true;
        }
        self
    }

    /// ### hasnt_focus
    ///
    /// Initialize props with focus set to False
    pub fn hasnt_focus(&mut self) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.focus = false;
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
    pub body: Option<Vec<String>>,
}

impl TextParts {
    /// ### new
    ///
    /// Instantiates a new TextParts entity
    pub fn new(title: Option<String>, body: Option<Vec<String>>) -> Self {
        TextParts { title, body }
    }
}

impl Default for TextParts {
    fn default() -> Self {
        TextParts {
            title: None,
            body: None,
        }
    }
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
        assert_eq!(props.focus, false);
        assert_eq!(props.italic, false);
        assert_eq!(props.underlined, false);
        assert!(props.texts.title.is_none());
        assert!(props.texts.body.is_none());
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
            .has_focus()
            .with_background(Color::Blue)
            .with_foreground(Color::Green)
            .bold()
            .italic()
            .underlined()
            .with_texts(TextParts::new(
                Some(String::from("hello")),
                Some(vec![String::from("hey")]),
            ))
            .build();
        assert_eq!(props.background, Color::Blue);
        assert_eq!(props.bold, true);
        assert_eq!(props.focus, true);
        assert_eq!(props.foreground, Color::Green);
        assert_eq!(props.italic, true);
        assert_eq!(props.texts.title.as_ref().unwrap().as_str(), "hello");
        assert_eq!(
            props.texts.body.as_ref().unwrap().get(0).unwrap().as_str(),
            "hey"
        );
        assert_eq!(props.underlined, true);
        assert_eq!(props.visible, false);
        let props: Props = PropsBuilder::default()
            .visible()
            .hasnt_focus()
            .with_background(Color::Blue)
            .with_foreground(Color::Green)
            .bold()
            .italic()
            .underlined()
            .with_texts(TextParts::new(
                Some(String::from("hello")),
                Some(vec![String::from("hey")]),
            ))
            .build();
        assert_eq!(props.background, Color::Blue);
        assert_eq!(props.bold, true);
        assert_eq!(props.focus, false);
        assert_eq!(props.foreground, Color::Green);
        assert_eq!(props.italic, true);
        assert_eq!(props.texts.title.as_ref().unwrap().as_str(), "hello");
        assert_eq!(
            props.texts.body.as_ref().unwrap().get(0).unwrap().as_str(),
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
                Some(vec![String::from("hey")]),
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
                Some(vec![String::from("hey")]),
            ))
            .build();
        // Ok, now make a builder from properties
        let builder: PropsBuilder = PropsBuilder::from_props(&props);
        assert!(builder.props.is_some());
    }

    #[test]
    fn test_ui_layout_props_text_parts_with_values() {
        let parts: TextParts = TextParts::new(
            Some(String::from("Hello world!")),
            Some(vec![String::from("row1"), String::from("row2")]),
        );
        assert_eq!(parts.title.as_ref().unwrap().as_str(), "Hello world!");
        assert_eq!(
            parts.body.as_ref().unwrap().get(0).unwrap().as_str(),
            "row1"
        );
        assert_eq!(
            parts.body.as_ref().unwrap().get(1).unwrap().as_str(),
            "row2"
        );
    }

    #[test]
    fn test_ui_layout_props_text_parts_default() {
        let parts: TextParts = TextParts::default();
        assert!(parts.title.is_none());
        assert!(parts.body.is_none());
    }
}
