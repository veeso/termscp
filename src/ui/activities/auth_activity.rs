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

// Includes
use crossterm::event::Event as InputEvent;
use crossterm::event::KeyCode;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
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

/// ### ScpProtocol
///
/// ScpProtocol describes the communication protocol selected by the user to communicate with the remote
#[derive(std::cmp::PartialEq, std::fmt::Debug)]
pub enum ScpProtocol {
    Sftp,
    Ftp,
}

/// ### AuthActivity
///
/// AuthActivity is the data holder for the authentication activity
pub struct AuthActivity {
    pub address: String,
    pub port: String,
    pub protocol: ScpProtocol,
    pub username: String,
    pub password: String,
    pub submit: bool, // becomes true after user has submitted fields
    pub quit: bool,   // Becomes true if user has pressed esc
    selected_field: InputField,
    popup_message: Option<String>,
    password_placeholder: String,
}

impl AuthActivity {
    /// ### new
    ///
    /// Instantiates a new AuthActivity
    pub fn new() -> AuthActivity {
        AuthActivity {
            address: String::new(),
            port: String::from("22"),
            protocol: ScpProtocol::Sftp,
            username: String::new(),
            password: String::new(),
            submit: false,
            quit: false,
            selected_field: InputField::Address,
            popup_message: None,
            password_placeholder: String::new(),
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
        let protocols: Vec<Spans> = vec![Spans::from("SFTP"), Spans::from("FTP")];
        let index: usize = match self.protocol {
            ScpProtocol::Sftp => 0,
            ScpProtocol::Ftp => 1,
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
                    .fg(Color::Green),
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
                Span::raw(" to change input field,"),
                Span::styled("<ENTER>", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to submit form"),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        );
        let mut footer_text = Text::from(Spans::from(footer));
        footer_text.patch_style(h_style);
        Paragraph::new(footer_text)
    }
}

impl Activity for AuthActivity {
    /// ### on_create
    ///
    /// `on_create` is the function which must be called to initialize the activity.
    /// `on_create` must initialize all the data structures used by the activity
    fn on_create(&mut self, context: &mut Context) {
        // Put raw mode on enabled
        let _ = enable_raw_mode();
        // Clear terminal
        let _ = context.terminal.clear();
    }

    /// ### on_draw
    ///
    /// `on_draw` is the function which draws the graphical interface.
    /// This function must be called at each tick to refresh the interface
    fn on_draw(&mut self, context: &mut Context) {
        // Start catching Input Events
        if let Ok(input_events) = context.input_hnd.fetch_events() {
            // Iterate over input events
            for event in input_events.iter() {
                match event {
                    InputEvent::Key(key) => {
                        match key.code {
                            KeyCode::Esc => {
                                self.quit = true;
                                break;
                            }
                            KeyCode::Enter => {
                                // Handle submit
                                // Check form
                                // Check address
                                if self.address.len() == 0 {
                                    self.popup_message = Some(String::from("Invalid address"));
                                    break;
                                }
                                // Check port
                                // Convert port to number
                                match self.port.parse::<usize>() {
                                    Ok(val) => {
                                        if val > 65535 {
                                            self.popup_message = Some(String::from(
                                                "Specified port must be in range 0-65535",
                                            ));
                                            break;
                                        }
                                    }
                                    Err(_) => {
                                        self.popup_message =
                                            Some(String::from("Specified port is not a number"));
                                        break;
                                    }
                                }
                                // Check username
                                if self.username.len() == 0 {
                                    self.popup_message = Some(String::from("Invalid username"));
                                    break;
                                }
                                // Everything OK, set enter
                                self.submit = true;
                                self.popup_message = Some(format!(
                                    "Connecting to {}:{}...",
                                    self.address, self.port
                                ));
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
                                    InputField::Address => InputField::Address, // End of list
                                    InputField::Port => InputField::Address,
                                    InputField::Protocol => InputField::Port,
                                    InputField::Username => InputField::Protocol,
                                    InputField::Password => InputField::Username,
                                }
                            }
                            KeyCode::Down => {
                                // Move item down
                                self.selected_field = match self.selected_field {
                                    InputField::Address => InputField::Port,
                                    InputField::Port => InputField::Protocol,
                                    InputField::Protocol => InputField::Username,
                                    InputField::Username => InputField::Password,
                                    InputField::Password => InputField::Password, // End of list
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
                                        ScpProtocol::Sftp => ScpProtocol::Sftp,
                                        ScpProtocol::Ftp => ScpProtocol::Sftp, // End of list
                                    }
                                }
                            }
                            KeyCode::Right => {
                                // If current field is Protocol handle event... ( move element right )
                                if self.selected_field == InputField::Protocol {
                                    self.protocol = match self.protocol {
                                        ScpProtocol::Sftp => ScpProtocol::Ftp,
                                        ScpProtocol::Ftp => ScpProtocol::Ftp, // End of list
                                    }
                                }
                            }
                            _ => { /* Nothing to do */ }
                        }
                    }
                    _ => { /* Nothing to do */ }
                }
            }
        }
        // draw interface
        let _ = context.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(5)
                .constraints(
                    [
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
            // Draw input fields
            f.render_widget(self.draw_remote_address(), chunks[0]);
            f.render_widget(self.draw_remote_port(), chunks[1]);
            f.render_widget(self.draw_protocol_select(), chunks[2]);
            f.render_widget(self.draw_protocol_username(), chunks[3]);
            f.render_widget(self.draw_protocol_password(), chunks[4]);
            // Draw footer
            f.render_widget(self.draw_footer(), chunks[5]);
            // TODO: popup
            // Set cursor
            match self.selected_field {
                InputField::Address => f.set_cursor(
                    chunks[0].x + self.address.width() as u16 + 1,
                    chunks[0].y + 1,
                ),
                InputField::Port => {
                    f.set_cursor(chunks[1].x + self.port.width() as u16 + 1, chunks[1].y + 1)
                }
                InputField::Username => f.set_cursor(
                    chunks[3].x + self.username.width() as u16 + 1,
                    chunks[3].y + 1,
                ),
                InputField::Password => f.set_cursor(
                    chunks[4].x + self.password_placeholder.width() as u16 + 1,
                    chunks[4].y + 1,
                ),
                _ => {}
            }
        });
    }

    /// ### on_destroy
    ///
    /// `on_destroy` is the function which cleans up runtime variables and data before terminating the activity.
    /// This function must be called once before terminating the activity.
    fn on_destroy(&mut self, context: &mut Context) {
        // Disable raw mode
        let _ = disable_raw_mode();
        // Clear terminal
        let _ = context.terminal.clear();
    }
}
