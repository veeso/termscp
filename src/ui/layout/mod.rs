//! ## Layout
//!
//! `Layout` is the module which provides components, view, state and properties to create layouts

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

// Modules
pub mod props;

// locals
use props::{Props, PropsBuilder};
// ext
use crossterm::event::Event as InputEvent;
use tui::widgets::Widget;

// -- States

/// ## States
///
/// States is a trait which defines the behaviours for the states model for the different component.
/// A state contains internal values for each component.
pub(crate) trait States {
    /// ### update
    ///
    /// Create a new state from current one and new
    fn update(&self, new: dyn States) -> dyn States;
}

// -- Component

/// ## Component
///
/// Component is a trait which defines the behaviours for a Layout component.
/// All layout components must implement a method to render and one to update
pub trait Component {
    /// ### render
    ///
    /// Based on the current properties and states, return a Widget instance for the Component
    /// Returns None if the component is hidden
    fn render(&self) -> Option<Box<dyn Widget>>;

    /// ### update
    ///
    /// Update component properties
    /// Properties should first be retrieved through `get_props` which creates a builder from
    /// existing properties and then edited before calling update
    fn update(&mut self, props: Option<Props>);

    /// ### get_props
    ///
    /// Returns a props builder starting from component properties.
    /// This returns a prop builder in order to make easier to create
    /// new properties for the element.
    fn get_props(&self) -> PropsBuilder;

    /// ### on
    ///
    /// Handle input event and update internal states
    fn on(&mut self, ev: InputEvent);

    // -- events

    /// ### should_umount
    ///
    /// The component must provide to the supervisor whether it should be umounted (destroyed)
    /// This makes sense to be called after an `on` or after an `update`, where the states changes.
    fn should_umount(&self) -> bool;
}
