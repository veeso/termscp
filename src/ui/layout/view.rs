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
use super::{Canvas, Component, InputEvent, Msg, Payload, Props, PropsBuilder, Rect};
// ext
use std::collections::HashMap;

/// ## View
///
/// View is the wrapper and manager for all the components.
/// A View is a container for all the components in a certain layout.
/// Each View can have only one focused component.
pub struct View {
    components: HashMap<String, Box<dyn Component>>, // all the components in the view
    focus: Option<String>,                           // Current active component
    focus_stack: Vec<String>, // Focus stack; used to give focus in case the current element loses focus
}

// -- view

impl View {
    /// ### init
    ///
    /// Initialize a new `View`
    pub fn init() -> Self {
        View {
            components: HashMap::new(),
            focus: None,
            focus_stack: Vec::new(),
        }
    }

    // -- mount / umount

    /// ### mount
    ///
    /// Mount a new component in the view
    pub fn mount(&mut self, id: &str, component: Box<dyn Component>) {
        self.components.insert(id.to_string(), component);
    }

    /// ### umount
    ///
    /// Umount a component from the view.
    /// If component has focus, blur component and remove it from the stack
    pub fn umount(&mut self, id: &str) {
        // Check if component has focus
        if let Some(focus) = self.focus.as_ref() {
            // If has focus, blur component
            if focus == id {
                self.blur();
            }
        }
        // Remove component from focus stack
        self.pop_from_stack(id);
        self.components.remove(id);
    }

    // -- render

    /// ### render
    ///
    /// RenderData component with the provided id
    #[cfg(not(tarpaulin_include))]
    pub fn render(&self, id: &str, frame: &mut Canvas, area: Rect) {
        if let Some(component) = self.components.get(id) {
            component.render(frame, area);
        }
    }

    // -- props

    /// ### get_props
    ///
    /// Get component properties
    pub fn get_props(&self, id: &str) -> Option<PropsBuilder> {
        match self.components.get(id) {
            None => None,
            Some(cmp) => Some(cmp.get_props()),
        }
    }

    /// update
    ///
    /// Update component properties
    /// Returns `None` if component doesn't exist
    pub fn update(&mut self, id: &str, props: Props) -> Option<(String, Msg)> {
        match self.components.get_mut(id) {
            None => None,
            Some(cmp) => Some((id.to_string(), cmp.update(props))),
        }
    }

    // -- state

    /// ### get_value
    ///
    /// Get component value
    pub fn get_value(&self, id: &str) -> Option<Payload> {
        match self.components.get(id) {
            None => None,
            Some(cmp) => Some(cmp.get_value()),
        }
    }

    // -- events

    /// ### on
    ///
    /// Handle event for the focused component (if any)
    /// Returns `None` if no component is focused
    pub fn on(&mut self, ev: InputEvent) -> Option<(String, Msg)> {
        match self.focus.as_ref() {
            None => None,
            Some(id) => match self.components.get_mut(id) {
                None => None,
                Some(cmp) => Some((id.to_string(), cmp.on(ev))),
            },
        }
    }

    // -- focus

    /// ### blur
    ///
    /// Blur selected element AND DON'T PUSH CURRENT ACTIVE ELEMENT INTO THE STACK
    /// Last element in stack becomes active and is removed from the stack
    pub fn blur(&mut self) {
        if let Some(component) = self.focus.take() {
            // Blur component
            if let Some(cmp) = self.components.get_mut(component.as_str()) {
                cmp.blur();
            }
            // Set last element in the stack as active
            let mut new: Option<String> = None;
            if let Some(last) = self.focus_stack.last() {
                // Set focus to last element
                new = Some(last.clone());
                self.focus = Some(last.clone());
                // Active
                if let Some(new) = self.components.get_mut(last) {
                    new.active();
                }
            }
            // Pop element from stack
            if let Some(new) = new {
                self.pop_from_stack(new.as_str());
            }
        }
    }

    /// ### active
    ///
    /// Active provided element
    /// Current active component, if any, GETS PUSHED to the STACK
    pub fn active(&mut self, component: &str) {
        // Active component if exists
        if let Some(cmp) = self.components.get_mut(component) {
            // Active component
            cmp.active();
            // Put current focus if any, into the stack
            if let Some(active_component) = self.focus.take() {
                // Blur active component
                if let Some(active_component) = self.components.get_mut(active_component.as_str()) {
                    active_component.blur();
                }
                self.push_to_stack(active_component.as_str());
            }
            // Give focus to component
            self.focus = Some(component.to_string());
            // Remove new component from the stack
            self.pop_from_stack(component);
        }
    }

    // -- private

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

#[cfg(test)]
mod tests {

    use super::*;

    use super::super::components::input::Input;
    use super::super::components::text::Text;
    use super::super::props::{PropValue, TextParts, TextSpan};

    use crossterm::event::{KeyCode, KeyEvent};

    #[test]
    fn test_ui_layout_view_init() {
        let view: View = View::init();
        // Verify view
        assert_eq!(view.components.len(), 0);
        assert!(view.focus.is_none());
        assert_eq!(view.focus_stack.len(), 0);
    }

    #[test]
    fn test_ui_layout_view_mount_umount() {
        let mut view: View = View::init();
        // Mount component
        let input: &str = "INPUT";
        view.mount(input, make_component_input());
        // Verify is mounted
        assert!(view.components.get(input).is_some());
        // Mount another
        let text: &str = "TEXT";
        view.mount(text, make_component_text());
        assert!(view.components.get(text).is_some());
        assert_eq!(view.components.len(), 2);
        // Verify you cannot have duplicates
        view.mount(input, make_component_input());
        assert_eq!(view.components.len(), 2); // length should still be 2
                                              // Umount
        view.umount(text);
        assert_eq!(view.components.len(), 1);
        assert!(view.components.get(text).is_none());
    }

    /*
    #[test]
    fn test_ui_layout_view_mount_render() {
        let mut view: View = View::init();
        // Mount component
        let input: &str = "INPUT";
        view.mount(input, make_component_input());
        assert!(view.render(input).is_some());
        assert!(view.render("unexisting").is_none());
    }
    */

    #[test]
    fn test_ui_layout_view_focus() {
        let mut view: View = View::init();
        // Prepare ids
        let input1: &str = "INPUT_1";
        let input2: &str = "INPUT_2";
        let input3: &str = "INPUT_3";
        let text1: &str = "TEXT_1";
        let text2: &str = "TEXT_2";
        // Mount components
        view.mount(input1, make_component_input());
        view.mount(input2, make_component_input());
        view.mount(input3, make_component_input());
        view.mount(text1, make_component_text());
        view.mount(text2, make_component_text());
        // Verify focus
        assert!(view.focus.is_none());
        assert_eq!(view.focus_stack.len(), 0);
        // Blur when nothing is selected
        view.blur();
        assert!(view.focus.is_none());
        assert_eq!(view.focus_stack.len(), 0);
        // Active unexisting component
        view.active("UNEXISTING-COMPONENT");
        assert!(view.focus.is_none());
        assert_eq!(view.focus_stack.len(), 0);
        // Give focus to a component
        view.active(input1);
        // Check focus
        assert_eq!(view.focus.as_ref().unwrap().as_str(), input1);
        assert_eq!(view.focus_stack.len(), 0); // NOTE: stack is empty until a focus gets blurred
                                               // Active a new component
        view.active(input2);
        // Now focus should be on input2, but input 1 should be in the focus stack
        assert_eq!(view.focus.as_ref().unwrap().as_str(), input2);
        assert_eq!(view.focus_stack.len(), 1);
        assert_eq!(view.focus_stack[0].as_str(), input1);
        // Active input 3
        view.active(input3);
        // now focus should be hold by input3, and stack should have len 2
        assert_eq!(view.focus.as_ref().unwrap().as_str(), input3);
        assert_eq!(view.focus_stack.len(), 2);
        assert_eq!(view.focus_stack[0].as_str(), input1);
        assert_eq!(view.focus_stack[1].as_str(), input2);
        // blur
        view.blur();
        // Focus should now be hold by input2; input 3 should NOT be in the stack
        assert_eq!(view.focus.as_ref().unwrap().as_str(), input2);
        assert_eq!(view.focus_stack.len(), 1);
        assert_eq!(view.focus_stack[0].as_str(), input1);
        // Active twice
        view.active(input2);
        // Nothing should have changed
        assert_eq!(view.focus.as_ref().unwrap().as_str(), input2);
        assert_eq!(view.focus_stack.len(), 1);
        assert_eq!(view.focus_stack[0].as_str(), input1);
        // Blur again; stack should become empty, whether focus should then be hold by input 1
        view.blur();
        assert_eq!(view.focus.as_ref().unwrap().as_str(), input1);
        assert_eq!(view.focus_stack.len(), 0);
        // Blur again; now everything should be none
        view.blur();
        assert!(view.focus.is_none());
        assert_eq!(view.focus_stack.len(), 0);
    }

    #[test]
    fn test_ui_layout_view_focus_umount() {
        let mut view: View = View::init();
        // Mount component
        let input: &str = "INPUT";
        let text: &str = "TEXT";
        let text2: &str = "TEXT2";
        view.mount(input, make_component_input());
        view.mount(text, make_component_text());
        view.mount(text2, make_component_text());
        // Give focus to input
        view.active(input);
        // Give focus to text
        view.active(text);
        view.active(text2);
        // Stack should have 1 element
        assert_eq!(view.focus_stack.len(), 2);
        // Focus should be some
        assert!(view.focus.is_some());
        // Umount text
        view.umount(text2);
        // Focus should now be hold by 'text'; stack should now have size 1
        assert_eq!(view.focus.as_ref().unwrap(), text);
        assert_eq!(view.focus_stack.len(), 1);
        // Umount input
        view.umount(input);
        assert_eq!(view.focus.as_ref().unwrap(), text);
        assert_eq!(view.focus_stack.len(), 0);
        // Umount text
        view.umount(text);
        assert!(view.focus.is_none());
        assert_eq!(view.focus_stack.len(), 0);
    }

    #[test]
    fn test_ui_layout_view_update() {
        let mut view: View = View::init();
        // Prepare ids
        let text: &str = "TEXT";
        // Mount text
        view.mount(text, make_component_text());
        // Get properties and update
        let props: Props = view.get_props(text).unwrap().build();
        // Verify bold is false
        assert_eq!(props.bold, false);
        // Update properties and set bold to true
        let mut builder = view.get_props(text).unwrap();
        let (id, msg) = view.update(text, builder.bold().build()).unwrap();
        // Verify return values
        assert_eq!(id, text);
        assert_eq!(msg, Msg::None);
        // Verify bold is now true
        let props: Props = view.get_props(text).unwrap().build();
        // Verify bold is false
        assert_eq!(props.bold, true);
        // Get properties for unexisting component
        assert!(view.update("foobar", props).is_none());
    }

    #[test]
    fn test_ui_layout_view_on() {
        let mut view: View = View::init();
        // Prepare ids
        let text: &str = "TEXT";
        let input: &str = "INPUT";
        // Mount
        view.mount(text, make_component_text());
        view.mount(input, make_component_input());
        // Verify current value
        assert_eq!(view.get_value(text).unwrap(), Payload::None); // Text value is Nothing
        assert_eq!(
            view.get_value(input).unwrap(),
            Payload::Text(String::from("text"))
        ); // Defined in `make_component_input`
           // Handle events WITHOUT ANY ACTIVE ELEMENT
        assert!(view
            .on(InputEvent::Key(KeyEvent::from(KeyCode::Enter)))
            .is_none());
        // Active input
        view.active(input);
        // Now handle events on input
        // Try char
        assert_eq!(
            view.on(InputEvent::Key(KeyEvent::from(KeyCode::Char('1'))))
                .unwrap(),
            (input.to_string(), Msg::None)
        );
        // Verify new value
        assert_eq!(
            view.get_value(input).unwrap(),
            Payload::Text(String::from("text1"))
        );
        // Verify enter
        assert_eq!(
            view.on(InputEvent::Key(KeyEvent::from(KeyCode::Enter)))
                .unwrap(),
            (
                input.to_string(),
                Msg::OnSubmit(Payload::Text(String::from("text1")))
            )
        );
    }

    /// ### make_component
    ///
    /// Make a new component; we'll use Input, which uses a rich set of events
    fn make_component_input() -> Box<dyn Component> {
        Box::new(Input::new(
            PropsBuilder::default()
                .with_texts(TextParts::new(Some(String::from("mytext")), None))
                .with_value(PropValue::Str(String::from("text")))
                .build(),
        ))
    }

    fn make_component_text() -> Box<dyn Component> {
        Box::new(Text::new(
            PropsBuilder::default()
                .with_texts(TextParts::new(
                    None,
                    Some(vec![TextSpan::from("Sample text")]),
                ))
                .build(),
        ))
    }
}
