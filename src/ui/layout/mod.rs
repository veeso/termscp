//! ## Layout
//!
//! `Layout` is the module which provides components, view, state and properties to create layouts

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
// Modules
pub mod components;
pub mod props;
pub mod utils;
pub mod view;

// locals
use props::{PropValue, Props, PropsBuilder};
// ext
use crossterm::event::Event as InputEvent;
use crossterm::event::KeyEvent;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::Frame;

type Backend = CrosstermBackend<Stdout>;
pub(crate) type Canvas<'a> = Frame<'a, Backend>;

// -- Msg

/// ## Msg
///
/// Msg is an enum returned after an event is raised for a certain component
/// Yep, I took inspiration from Elm.
#[derive(std::fmt::Debug, PartialEq, Eq)]
pub enum Msg {
    OnSubmit(Payload),
    OnChange(Payload),
    OnKey(KeyEvent),
    None,
}

/// ## Payload
///
/// Payload describes a component value
#[derive(std::fmt::Debug, PartialEq, Eq)]
pub enum Payload {
    Text(String),
    //Signed(isize),
    Unsigned(usize),
    None,
}

// -- Component

/// ## Component
///
/// Component is a trait which defines the behaviours for a Layout component.
/// All layout components must implement a method to render and one to update
pub trait Component {
    /// ### render
    ///
    /// Based on the current properties and states, renders the component in the provided area frame
    #[cfg(not(tarpaulin_include))]
    fn render(&self, frame: &mut Canvas, area: Rect);

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update.
    /// Returns a Msg to the view
    fn update(&mut self, props: Props) -> Msg;

    /// ### get_props
    ///
    /// Returns a props builder starting from component properties.
    /// This returns a prop builder in order to make easier to create
    /// new properties for the element.
    fn get_props(&self) -> PropsBuilder;

    /// ### on
    ///
    /// Handle input event and update internal states.
    /// Returns a Msg to the view
    fn on(&mut self, ev: InputEvent) -> Msg;

    /// ### get_value
    ///
    /// Get current value from component
    fn get_value(&self) -> Payload;

    // -- events

    /// ### blur
    ///
    /// Blur component; basically remove focus
    fn blur(&mut self);

    /// ### active
    ///
    /// Active component; basically give focus
    fn active(&mut self);
}
