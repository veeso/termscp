//! ## Input
//!
//! `input` is the module which provides all the functionalities related to input events in the user interface

/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
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

extern crate crossterm;

use crossterm::event::{poll, read, Event};
use std::time::Duration;

/// ## InputHandler
///
/// InputHandler is the struct which runs a thread which waits for
/// input events from the user and reports them through a receiver
pub(crate) struct InputHandler {}

impl InputHandler {
    /// ### InputHandler
    ///
    ///
    pub(crate) fn new() -> InputHandler {
        InputHandler {}
    }

    /// ### fetch_events
    ///
    /// Check if new events have been received from handler
    pub(crate) fn fetch_events(&self) -> Result<Vec<Event>, ()> {
        let mut inbox: Vec<Event> = Vec::new();
        loop {
            match self.read_event() {
                Ok(ev_opt) => match ev_opt {
                    Some(ev) => inbox.push(ev),
                    None => break
                },
                Err(_) => return Err(())
            }
        }
        Ok(inbox)
    }

    /// ### read_event
    /// 
    /// Read event from input listener
    pub(crate) fn read_event(&self) -> Result<Option<Event>, ()> {
        if let Ok(available) = poll(Duration::from_millis(10)) {
            match available {
                true => {
                    // Read event
                    if let Ok(ev) = read() {
                        Ok(Some(ev))
                    } else {
                        Err(())
                    }
                }
                false => Ok(None)
            }
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ui_input_new() {
        let _: InputHandler = InputHandler::new();
    }

    /* ERRORS ON GITHUB ACTIONS
    #[test]
    fn test_ui_input_fetch() {
        let input_hnd: InputHandler = InputHandler::new();
        // Try recv
        assert_eq!(input_hnd.fetch_messages().ok().unwrap().len(), 0);
    }*/
}
