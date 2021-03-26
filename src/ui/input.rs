//! ## Input
//!
//! `input` is the module which provides all the functionalities related to input events in the user interface

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

extern crate crossterm;

use crossterm::event::{poll, read, Event};
use std::time::Duration;

/// ## InputHandler
///
/// InputHandler is the struct which runs a thread which waits for
/// input events from the user and reports them through a receiver
pub(crate) struct InputHandler;

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
    #[allow(dead_code)]
    pub(crate) fn fetch_events(&self) -> Result<Vec<Event>, ()> {
        let mut inbox: Vec<Event> = Vec::new();
        loop {
            match self.read_event() {
                Ok(ev_opt) => match ev_opt {
                    Some(ev) => inbox.push(ev),
                    None => break,
                },
                Err(_) => return Err(()),
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
                false => Ok(None),
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
