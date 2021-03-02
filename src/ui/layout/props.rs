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

// locals
use super::super::activities::Activity;
// ext
use tui::style::{Color};

// Callback types
pub type OnSubmitCb = fn(&mut dyn Activity, Option<String>); // Activity, Value

// --- Props

/// ## Props
///
/// Props holds all the possible properties for a layout component
pub struct Props {
    // Values
    pub visible: bool,     // Is the element visible ON CREATE?
    pub foreground: Color, // Foreground color
    pub background: Color, // Background color
    pub bold: bool,        // Text bold
    pub italic: bool,      // Italic
    pub underlined: bool,  // Underlined
    pub texts: TextParts,  // text parts
    // Callbacks
    pub on_submit: Option<OnSubmitCb>,
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
            texts: TextParts::default(),
            // Callbacks
            on_submit: None,
        }
    }
}

// --- Props builder

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

    /// ### with_bold
    ///
    /// Set bold property for component
    pub fn with_bold(&mut self, bold: bool) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.bold = bold;
        }
        self
    }

    /// ### with_italic
    ///
    /// Set italic property for component
    pub fn with_italic(&mut self, italic: bool) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.italic = italic;
        }
        self
    }

    /// ### with_underlined
    ///
    /// Set underlined property for component
    pub fn with_underlined(&mut self, underlined: bool) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.underlined = underlined;
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

    /// ### on_submit
    ///
    /// Set on_submit callback for component
    pub fn on_submit(&mut self, on_submit: OnSubmitCb) -> &mut Self {
        if let Some(props) = self.props.as_mut() {
            props.on_submit = Some(on_submit);
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

// --- Text parts

/// ## TextParts
///
/// TextParts holds optional component for the text displayed by a component
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
