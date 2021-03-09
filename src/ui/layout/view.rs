//! ## View
//!
//! `View` is the module which handles layout components

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

// imports
use super::{Component, Msg, Props};
// ext
use std::collections::HashMap;

/// ## View
///
/// View is the wrapper and manager for all the components.
/// A View is a container for all the components in a certain layout.
/// It is possible to have ligatures between elements, which causes for example a component a to lose focus in favour of b if a certain key is pressed.
/// Each View can have only one focused component.
pub struct View {
    components: HashMap<String, Box<dyn Component>>, // all the components in the view
    focus: Option<String>,                           // Current active component
    focus_stack: Vec<String>,                        // Focus stack; used to give focus in case the current element loses focus
}

/// ## Ligature
///
/// A ligature describes how a certain event must be handled by the view when a certain element 'a has focus.
/// The purpose of the ligature is to make a certain event to trigger another component.
/// This means a ligature is an event that if triggered on 'a causes 'b to something else.
/// E.g. 'a -> onKey(KeyCode::Char('e')) -> focus -> 'b
struct Ligature {
    origin: String, // Element which causes the Ligature to trigger
    event: Msg,     // The event the ligature summons
    action: Action, // The action which must be performed in case the ligature triggers
    target: String, // The element the action is performed for
}

/// ## Action
///
/// Action describes an action to perform in case a ligature triggers
pub enum Action {
    Active, // Give focus to component
    Blur,   // Remove focus to component
    Show,   // Set component to visible
    Hide,   // Hide component
    Umount, // Umount element
}

// -- view

impl View {
    /// ### new
    ///
    /// Instantiates a new `View`
    pub fn new() -> Self {
        View {
            components: HashMap::new(),
            focus: None,
            focus_stack: Vec::new(),
        }
    }

    // -- private

    /// ### blur
    ///
    /// Blur selected element and push it into the stack;
    /// Last element in stack becomes active
    fn blur(&mut self) {
        if let Some(component) = self.focus.take() {
            // Set last element as active
            let mut new: Option<String> = None;
            if let Some(last) = self.focus_stack.last() {
                // Set focus to last element
                new = Some(last.clone());
                self.focus = Some(last.clone());
            }
            // Pop element from stack
            if let Some(new) = new {
                self.pop_from_stack(new.as_str());
            }
            // Finally previous active component to stack
            self.push_to_stack(&component);
        }
    }

    /// ### active
    /// 
    /// Active provided element
    fn active(&mut self, component: &str) {
        // If there is an element active; call blur
        if self.focus.is_some() {
            self.blur();
        }
        // Set focus
        self.focus = Some(component.to_string());
    }

    /// ### push_to_stack
    ///
    /// Push component to stack; first remove it from the stack if any
    fn push_to_stack(&mut self, name: &str) {
        self.pop_from_stack(name);
        self.focus_stack.push(name.to_string());
    }

    /// ### pop_from_stack
    ///
    /// Pop element from focus stack
    fn pop_from_stack(&mut self, name: &str) {
        self.focus_stack.retain(|c| c.as_str() != name);
    }
}
