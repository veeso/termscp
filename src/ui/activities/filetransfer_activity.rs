//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

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
use crate::filetransfer::FileTransferProtocol;
use super::{Activity, Context};

// File transfer
use crate::filetransfer::sftp_transfer::SftpFileTransfer;
use crate::filetransfer::FileTransfer;

// Includes
use crossterm::event::Event as InputEvent;
use crossterm::event::KeyCode;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Clear, Paragraph, Tabs},
};
use unicode_width::UnicodeWidthStr;

// Types
type DialogCallback = fn();

/// ### FileTransferParams
///
/// Holds connection parameters for file transfers
pub struct FileTransferParams {
    pub address: String,
    pub port: u16,
    pub protocol: FileTransferProtocol,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// ### InputField
/// 
/// Input field selected
#[derive(std::cmp::PartialEq)]
enum InputField {
    Explorer,
    Logs,
}

/// ## PopupType
///
/// PopupType describes the type of popup
#[derive(std::cmp::PartialEq)]
enum PopupType {
    Alert(Color, String),
    Wait(String),
    Fatal(String), // Must quit after being hidden
    Progress(String),
    YesNo(String, DialogCallback, DialogCallback), // Yes, no callback
}

/// ## InputMode
///
/// InputMode describes the current input mode
/// Each input mode handle the input events in a different way
#[derive(std::cmp::PartialEq)]
enum InputMode {
    Explorer,
    Popup(PopupType),
}

/// ## FileExplorer
///
/// File explorer states
struct FileExplorer {
    pub index: usize,
}

impl FileExplorer {
    /// ### new
    ///
    /// Instantiates a new FileExplorer
    pub fn new() -> FileExplorer {
        FileExplorer { index: 0 }
    }
}

/// ## FileExplorerTab
///
/// File explorer tab
enum FileExplorerTab {
    Local,
    Remote,
}

/// ## LogLevel
///
/// Log level type
enum LogLevel {
    Error,
    Warn,
    Info,
}

/// ## LogRecord
///
/// Log record entry
struct LogRecord {
    pub level: LogLevel,
    pub msg: String,
}

impl LogRecord {
    /// ### new
    ///
    /// Instantiates a new LogRecord
    pub fn new(level: LogLevel, msg: &str) -> LogRecord {
        LogRecord {
            level: level,
            msg: String::from(msg),
        }
    }
}

/// ## FileTransferActivity
///
/// FileTransferActivity is the data holder for the file transfer activity
pub struct FileTransferActivity {
    pub disconnected: bool,
    pub quit: bool,
    params: FileTransferParams,
    local: FileExplorer,
    remote: FileExplorer,
    tab: FileExplorerTab,
    log_index: usize,
    log_records: Vec<LogRecord>,
    progress: usize,
    input_mode: InputMode,
    input_field: InputField,
    client: Box<dyn FileTransfer>,
}

impl FileTransferActivity {
    /// ### new
    ///
    /// Instantiates a new FileTransferActivity
    pub fn new(params: FileTransferParams) -> FileTransferActivity {
        let protocol: FileTransferProtocol = params.protocol.clone();
        FileTransferActivity {
            disconnected: false,
            quit: false,
            params: params,
            local: FileExplorer::new(),
            remote: FileExplorer::new(),
            tab: FileExplorerTab::Local,
            log_index: 0,
            log_records: Vec::new(),
            progress: 0,
            input_mode: InputMode::Explorer,
            input_field: InputField::Explorer,
            client: match protocol {
                FileTransferProtocol::Sftp => Box::new(SftpFileTransfer::new()),
                FileTransferProtocol::Ftp => Box::new(SftpFileTransfer::new()), // FIXME: FTP
            },
        }
    }

    /// ### connect
    /// 
    /// Connect to remote
    fn connect(&mut self) {
        // Connect to remote
        match self.client.connect(self.params.address.clone(), self.params.port, self.params.username.clone(), self.params.password.clone()) {
            Ok(_) => {
                // Set state to explorer
                self.input_mode = InputMode::Explorer;
            },
            Err(err) => {
                // Set popup fatal error
                self.input_mode = InputMode::Popup(PopupType::Fatal(err.msg()));
            }
        }
    }

    /// ### draw
    ///
    /// Draw UI
    fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        // TODO: implement
    }

    /// ### draw_header
    ///
    /// Draw header
    fn draw_header(&self) -> Paragraph {
        Paragraph::new(" _____                   ____   ____ ____  \n|_   _|__ _ __ _ __ ___ / ___| / ___|  _ \\ \n  | |/ _ \\ '__| '_ ` _ \\\\___ \\| |   | |_) |\n  | |  __/ |  | | | | | |___) | |___|  __/ \n  |_|\\___|_|  |_| |_| |_|____/ \\____|_|    \n")
            .style(Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD))
    }

    /// ### draw_footer
    ///
    /// Draw authentication page footer
    fn draw_footer(&self) -> Paragraph {
        // Write header
        let footer = vec![
            Span::styled(
                "<ESC>",
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("quit\t"),
            Span::styled(
                "<UP,DOWN>",
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("change input field\t"),
            Span::styled(
                "<RIGHT,LEFT>",
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("change explorer tab\t"),
            Span::styled(
                "<ENTER>",
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("to upload/download file\t"),
            Span::styled(
                "<CTRL+R>",
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("to rename file\t"),
            Span::styled(
                "<CANC>",
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("to delete file\t"),
        ];
        Paragraph::new(Text::from(Spans::from(footer)))
    }
}

impl Activity for FileTransferActivity {
    /// ### on_create
    ///
    /// `on_create` is the function which must be called to initialize the activity.
    /// `on_create` must initialize all the data structures used by the activity
    fn on_create(&mut self, context: &mut Context) {
        // Put raw mode on enabled
        let _ = enable_raw_mode();
        // Clear terminal
        let _ = context.terminal.clear();
        // Set init state to connecting popup
        self.input_mode = InputMode::Popup(PopupType::Wait(format!("Connecting to {}:{}...", self.params.address, self.params.port)));
    }

    /// ### on_draw
    ///
    /// `on_draw` is the function which draws the graphical interface.
    /// This function must be called at each tick to refresh the interface
    fn on_draw(&mut self, context: &mut Context) {
        // draw interface
        let _ = context.terminal.draw(|f| {
            self.draw(f);
        });
        // Check if connected
        if ! self.client.is_connected() {
            // Connect to remote
            self.connect();
        }
        // TODO: logic
        // TODO: handle input events
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
        // Disconnect client
        if self.client.is_connected() {
            let _ = self.client.disconnect();
        }
    }
}
