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

// Deps
use crossterm::event::{read, Event};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

/// ## InputHandler
///
/// InputHandler is the struct which runs a thread which waits for
/// input events from the user and reports them through a receiver
pub(crate) struct InputHandler {
    running: Arc<Mutex<bool>>,
    join_hnd: Option<thread::JoinHandle<()>>,
    receiver: mpsc::Receiver<Event>,
}

impl InputHandler {
    /// ### InputHandler
    ///
    ///
    pub(crate) fn new() -> InputHandler {
        let (thread_sender, client_receiver) = mpsc::channel();
        let client_running = Arc::new(Mutex::new(false));
        let thread_running = Arc::clone(&client_running);
        let join_hnd = thread::spawn(move || {
            InputHandler::thread_run(thread_sender, thread_running);
        });
        InputHandler {
            running: client_running,
            join_hnd: Some(join_hnd),
            receiver: client_receiver,
        }
    }

    /// ### stop
    ///
    /// Stop InputHandler
    pub(crate) fn stop(&mut self) {
        if self.join_hnd.is_some() {
            //Set join to true
            {
                // Set running to false
                {
                    let mut running = self.running.lock().unwrap();
                    *running = false;
                }
                // Join
                self.join_hnd
                    .take()
                    .map(thread::JoinHandle::join)
                    .unwrap()
                    .unwrap();
            }
        }
    }

    /// ### fetch_messages
    ///
    /// Check if new events have been received from handler
    pub(crate) fn fetch_messages(&self) -> Result<Vec<Event>, ()> {
        let mut inbox: Vec<Event> = Vec::new();
        loop {
            match self.receiver.try_recv() {
                Ok(message) => inbox.push(message),
                Err(err) => match err {
                    mpsc::TryRecvError::Empty => break,
                    _ => return Err(()),
                },
            }
        }
        Ok(inbox)
    }

    // ### run
    ///
    /// Run method for thread
    fn thread_run(sender: mpsc::Sender<Event>, running: Arc<Mutex<bool>>) {
        {
            let mut running = running.lock().unwrap();
            *running = true;
        }
        loop {
            // Check if running is false
            {
                let running = running.lock().unwrap();
                if *running == false {
                    break;
                }
            }
            // Fetch events
            if let Ok(ev) = read() {
                // Send event
                if let Err(_) = sender.send(ev) {
                    // The counterpart has died
                    break;
                }
            }
            // Sleep
            thread::sleep(Duration::from_millis(50));
        }
    }
}
