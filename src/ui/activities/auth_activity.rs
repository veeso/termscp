//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

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

// Dependencies
extern crate crossterm;
extern crate tui;
extern crate unicode_width;

// locals
use super::{Activity, Context};
use crate::filetransfer::FileTransferProtocol;
use crate::utils::align_text_center;

// Includes
use crossterm::event::Event as InputEvent;
use crossterm::event::KeyCode;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Clear, Paragraph, Tabs},
};
use unicode_width::UnicodeWidthStr;

/// ### InputField
///
/// InputField describes the current input field to edit
#[derive(std::cmp::PartialEq)]
enum InputField {
    Address,
    Port,
    Protocol,
    Username,
    Password,
}

/// ### InputMode
///
/// InputMode describes the current input mode
/// Each input mode handle the input events in a different way
#[derive(std::cmp::PartialEq)]
enum InputMode {
    Text,
    Popup,
}

/// ### AuthActivity
///
/// AuthActivity is the data holder for the authentication activity
pub struct AuthActivity {
    pub address: String,
    pub port: String,
    pub protocol: FileTransferProtocol,
    pub username: String,
    pub password: String,
    pub submit: bool, // becomes true after user has submitted fields
    pub quit: bool,   // Becomes true if user has pressed esc
    context: Option<Context>,
    selected_field: InputField,
    input_mode: InputMode,
    popup_message: Option<String>,
    password_placeholder: String,
    redraw: bool, // Should ui actually be redrawned?
}

impl Default for AuthActivity {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthActivity {
    /// ### new
    ///
    /// Instantiates a new AuthActivity
    pub fn new() -> AuthActivity {
        AuthActivity {
            address: String::new(),
            port: String::from("22"),
            protocol: FileTransferProtocol::Sftp,
            username: String::new(),
            password: String::new(),
            submit: false,
            quit: false,
            context: None,
            selected_field: InputField::Address,
            input_mode: InputMode::Text,
            popup_message: None,
            password_placeholder: String::new(),
            redraw: true, // True at startup
        }
    }

    /// ### set_input_mode
    ///
    /// Update input mode based on current parameters
    fn select_input_mode(&mut self) -> InputMode {
        if self.popup_message.is_some() {
            return InputMode::Popup;
        }
        // Default to text
        InputMode::Text
    }

    /// ### handle_input_event
    ///
    /// Handle input event, based on current input mode
    fn handle_input_event(&mut self, ev: &InputEvent) {
        match self.input_mode {
            InputMode::Text => self.handle_input_event_mode_text(ev),
            InputMode::Popup => self.handle_input_event_mode_popup(ev),
        }
    }

    /// ### handle_input_event_mode_text
    ///
    /// Handler for input event when in textmode
    fn handle_input_event_mode_text(&mut self, ev: &InputEvent) {
        if let InputEvent::Key(key) = ev {
            match key.code {
                KeyCode::Esc => {
                    self.quit = true;
                }
                KeyCode::Enter => {
                    // Handle submit
                    // Check form
                    // Check address
                    if self.address.is_empty() {
                        self.popup_message = Some(String::from("Invalid address"));
                        return;
                    }
                    // Check port
                    // Convert port to number
                    match self.port.parse::<usize>() {
                        Ok(val) => {
                            if val > 65535 {
                                self.popup_message =
                                    Some(String::from("Specified port must be in range 0-65535"));
                                return;
                            }
                        }
                        Err(_) => {
                            self.popup_message =
                                Some(String::from("Specified port is not a number"));
                            return;
                        }
                    }
                    // Check username
                    //if self.username.len() == 0 {
                    //    self.popup_message = Some(String::from("Invalid username"));
                    //    return;
                    //}
                    // Everything OK, set enter
                    self.submit = true;
                    self.popup_message =
                        Some(format!("Connecting to {}:{}...", self.address, self.port));
                }
                KeyCode::Backspace => {
                    // Pop last char
                    match self.selected_field {
                        InputField::Address => {
                            let _ = self.address.pop();
                        }
                        InputField::Password => {
                            let _ = self.password.pop();
                        }
                        InputField::Username => {
                            let _ = self.username.pop();
                        }
                        InputField::Port => {
                            let _ = self.port.pop();
                        }
                        _ => { /* Nothing to do */ }
                    };
                }
                KeyCode::Up => {
                    // Move item up
                    self.selected_field = match self.selected_field {
                        InputField::Address => InputField::Password, // End of list (wrap)
                        InputField::Port => InputField::Address,
                        InputField::Protocol => InputField::Port,
                        InputField::Username => InputField::Protocol,
                        InputField::Password => InputField::Username,
                    }
                }
                KeyCode::Down | KeyCode::Tab => {
                    // Move item down
                    self.selected_field = match self.selected_field {
                        InputField::Address => InputField::Port,
                        InputField::Port => InputField::Protocol,
                        InputField::Protocol => InputField::Username,
                        InputField::Username => InputField::Password,
                        InputField::Password => InputField::Address, // End of list (wrap)
                    }
                }
                KeyCode::Char(ch) => {
                    match self.selected_field {
                        InputField::Address => self.address.push(ch),
                        InputField::Password => self.password.push(ch),
                        InputField::Username => self.username.push(ch),
                        InputField::Port => {
                            // Value must be numeric
                            if ch.is_numeric() {
                                self.port.push(ch);
                            }
                        }
                        _ => { /* Nothing to do */ }
                    }
                }
                KeyCode::Left => {
                    // If current field is Protocol handle event... (move element left)
                    if self.selected_field == InputField::Protocol {
                        self.protocol = match self.protocol {
                            FileTransferProtocol::Sftp => FileTransferProtocol::Ftp(true), // End of list (wrap)
                            FileTransferProtocol::Scp => FileTransferProtocol::Sftp,
                            FileTransferProtocol::Ftp(ftps) => match ftps {
                                false => FileTransferProtocol::Scp,
                                true => FileTransferProtocol::Ftp(false),
                            },
                        };
                    }
                }
                KeyCode::Right => {
                    // If current field is Protocol handle event... ( move element right )
                    if self.selected_field == InputField::Protocol {
                        self.protocol = match self.protocol {
                            FileTransferProtocol::Sftp => FileTransferProtocol::Scp,
                            FileTransferProtocol::Scp => FileTransferProtocol::Ftp(false),
                            FileTransferProtocol::Ftp(ftps) => match ftps {
                                false => FileTransferProtocol::Ftp(true),
                                true => FileTransferProtocol::Sftp, // End of list (wrap)
                            },
                        };
                    }
                }
                _ => { /* Nothing to do */ }
            }
        }
    }

    /// ### handle_input_event_mode_text
    ///
    /// Handler for input event when in popup mode
    fn handle_input_event_mode_popup(&mut self, ev: &InputEvent) {
        // Only enter should be allowed here
        if let InputEvent::Key(key) = ev {
            if let KeyCode::Enter = key.code {
                self.popup_message = None; // Hide popup
            }
        }
    }

    /// ### draw_remote_address
    ///
    /// Draw remote address block
    fn draw_remote_address(&self) -> Paragraph {
        Paragraph::new(self.address.as_ref())
            .style(match self.selected_field {
                InputField::Address => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Remote address"),
            )
    }

    /// ### draw_remote_port
    ///
    /// Draw remote port block
    fn draw_remote_port(&self) -> Paragraph {
        Paragraph::new(self.port.as_ref())
            .style(match self.selected_field {
                InputField::Port => Style::default().fg(Color::Cyan),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("Remote port"))
    }

    /// ### draw_protocol_select
    ///
    /// Draw protocol select
    fn draw_protocol_select(&self) -> Tabs {
        let protocols: Vec<Spans> = vec![
            Spans::from("SFTP"),
            Spans::from("SCP"),
            Spans::from("FTP"),
            Spans::from("FTPS"),
        ];
        let index: usize = match self.protocol {
            FileTransferProtocol::Sftp => 0,
            FileTransferProtocol::Scp => 1,
            FileTransferProtocol::Ftp(ftps) => match ftps {
                false => 2,
                true => 3,
            },
        };
        Tabs::new(protocols)
            .block(Block::default().borders(Borders::ALL).title("Protocol"))
            .select(index)
            .style(match self.selected_field {
                InputField::Protocol => Style::default().fg(Color::Green),
                _ => Style::default(),
            })
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Green)
                    .fg(Color::Black),
            )
    }

    /// ### draw_protocol_username
    ///
    /// Draw username block
    fn draw_protocol_username(&self) -> Paragraph {
        Paragraph::new(self.username.as_ref())
            .style(match self.selected_field {
                InputField::Username => Style::default().fg(Color::Magenta),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("Username"))
    }

    /// ### draw_protocol_password
    ///
    /// Draw password block
    fn draw_protocol_password(&mut self) -> Paragraph {
        // Create password secret
        self.password_placeholder = (0..self.password.width()).map(|_| "*").collect::<String>();
        Paragraph::new(self.password_placeholder.as_ref())
            .style(match self.selected_field {
                InputField::Password => Style::default().fg(Color::LightBlue),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("Password"))
    }

    /// ### draw_header
    ///
    /// Draw header
    fn draw_header(&self) -> Paragraph {
        Paragraph::new(" _____                   ____   ____ ____  \n|_   _|__ _ __ _ __ ___ / ___| / ___|  _ \\ \n  | |/ _ \\ '__| '_ ` _ \\\\___ \\| |   | |_) |\n  | |  __/ |  | | | | | |___) | |___|  __/ \n  |_|\\___|_|  |_| |_| |_|____/ \\____|_|    \n")
            .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
    }

    /// ### draw_footer
    ///
    /// Draw authentication page footer
    fn draw_footer(&self) -> Paragraph {
        // Write header
        let (footer, h_style) = (
            vec![
                Span::raw("Press "),
                Span::styled("<ESC>", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("<UP,DOWN>", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to change input field, "),
                Span::styled("<ENTER>", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to submit form"),
            ],
            Style::default().add_modifier(Modifier::BOLD),
        );
        let mut footer_text = Text::from(Spans::from(footer));
        footer_text.patch_style(h_style);
        Paragraph::new(footer_text)
    }

    /// ### draw_popup
    ///
    /// Draw popup block
    fn draw_popup(&self, r: Rect) -> (Paragraph, Rect) {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(30), // Offset top
                    Constraint::Percentage(10), // Actual height
                    Constraint::Percentage(60), // Offset bottom
                ]
                .as_ref(),
            )
            .split(r);
        let area: Rect = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((80) / 2),
                    Constraint::Percentage(20),
                    Constraint::Percentage((80) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1];
        let popup: Paragraph = Paragraph::new(align_text_center(
            self.popup_message.as_ref().unwrap().as_ref(),
            area.width,
        ))
        .style(Style::default().fg(Color::Red))
        .block(Block::default().borders(Borders::ALL).title("Alert"));
        (popup, area)
    }
}

impl Activity for AuthActivity {
    /// ### on_create
    ///
    /// `on_create` is the function which must be called to initialize the activity.
    /// `on_create` must initialize all the data structures used by the activity
    /// Context is taken from activity manager and will be released only when activity is destroyed
    fn on_create(&mut self, context: Context) {
        // Set context
        self.context = Some(context);
        // Clear terminal
        let _ = self.context.as_mut().unwrap().terminal.clear();
        // Put raw mode on enabled
        let _ = enable_raw_mode();
    }

    /// ### on_draw
    ///
    /// `on_draw` is the function which draws the graphical interface.
    /// This function must be called at each tick to refresh the interface
    fn on_draw(&mut self) {
        // Context must be something
        if self.context.is_none() {
            return;
        }
        // Start catching Input Events
        if let Ok(input_events) = self.context.as_ref().unwrap().input_hnd.fetch_events() {
            if !input_events.is_empty() {
                self.redraw = true; // Set redraw to true if there is at least one event
            }
            // Iterate over input events
            for event in input_events.iter() {
                self.handle_input_event(event);
            }
        }
        // Redraw if necessary
        if self.redraw {
            // Determine input mode
            self.input_mode = self.select_input_mode();
            // draw interface
            let mut ctx: Context = self.context.take().unwrap();
            let _ = ctx.terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(5),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                // Draw header
                f.render_widget(self.draw_header(), chunks[0]);
                // Draw input fields
                f.render_widget(self.draw_remote_address(), chunks[1]);
                f.render_widget(self.draw_remote_port(), chunks[2]);
                f.render_widget(self.draw_protocol_select(), chunks[3]);
                f.render_widget(self.draw_protocol_username(), chunks[4]);
                f.render_widget(self.draw_protocol_password(), chunks[5]);
                // Draw footer
                f.render_widget(self.draw_footer(), chunks[6]);
                if self.popup_message.is_some() {
                    let (popup, popup_area): (Paragraph, Rect) = self.draw_popup(f.size());
                    f.render_widget(Clear, popup_area); //this clears out the background
                    f.render_widget(popup, popup_area);
                }
                // Set cursor
                match self.selected_field {
                    InputField::Address => f.set_cursor(
                        chunks[1].x + self.address.width() as u16 + 1,
                        chunks[1].y + 1,
                    ),
                    InputField::Port => {
                        f.set_cursor(chunks[2].x + self.port.width() as u16 + 1, chunks[2].y + 1)
                    }
                    InputField::Username => f.set_cursor(
                        chunks[4].x + self.username.width() as u16 + 1,
                        chunks[4].y + 1,
                    ),
                    InputField::Password => f.set_cursor(
                        chunks[5].x + self.password_placeholder.width() as u16 + 1,
                        chunks[5].y + 1,
                    ),
                    _ => {}
                }
            });
            // Reset ctx
            self.context = Some(ctx);
            // Set redraw to false
            self.redraw = false;
        }
    }

    /// ### on_destroy
    ///
    /// `on_destroy` is the function which cleans up runtime variables and data before terminating the activity.
    /// This function must be called once before terminating the activity.
    /// This function finally releases the context
    fn on_destroy(&mut self) -> Option<Context> {
        // Disable raw mode
        let _ = disable_raw_mode();
        self.context.as_ref()?;
        // Clear terminal and return
        match self.context.take() {
            Some(mut ctx) => {
                let _ = ctx.terminal.clear();
                Some(ctx)
            }
            None => None,
        }
    }
}
