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
use super::{Activity, Context};
use crate::filetransfer::FileTransferProtocol;

// File transfer
use crate::filetransfer::sftp_transfer::SftpFileTransfer;
use crate::filetransfer::{FileTransfer, ProgressCallback};
use crate::fs::FsEntry;

// Includes
use crossterm::event::Event as InputEvent;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::collections::VecDeque;
use std::io::Stdout;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Clear, Paragraph, Tabs},
};
use unicode_width::UnicodeWidthStr;

// Holder for current upload progress
static mut UPLOAD_PROGRESS: f64 = 0.0; // VERY VERY BAD CODING

// Types
type DialogCallback = fn(&mut FileTransferActivity, &mut Context);
type OnInputSubmitCallback = fn(&mut FileTransferActivity, &mut Context, String);

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

/// ### DialogYesNoOption
///
/// Current yes/no dialog option
#[derive(std::cmp::PartialEq, Clone)]
enum DialogYesNoOption {
    Yes,
    No,
}

/// ## PopupType
///
/// PopupType describes the type of popup
#[derive(Clone)]
enum PopupType {
    Alert(Color, String),                          // Block color; Block text
    Fatal(String),                                 // Must quit after being hidden
    Input(String, OnInputSubmitCallback),          // Input description; Callback for submit
    Progress(String),                              // Progress block text
    Wait(String),                                  // Wait block text
    YesNo(String, DialogCallback, DialogCallback), // Yes, no callback
}

/// ## InputMode
///
/// InputMode describes the current input mode
/// Each input mode handle the input events in a different way
#[derive(Clone)]
enum InputMode {
    Explorer,
    Popup(PopupType),
}

/// ## FileExplorer
///
/// File explorer states
struct FileExplorer {
    pub index: usize,
    pub files: Vec<FsEntry>,
}

impl FileExplorer {
    /// ### new
    ///
    /// Instantiates a new FileExplorer
    pub fn new() -> FileExplorer {
        FileExplorer {
            index: 0,
            files: Vec::new(),
        }
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
    pub time: Instant,
    pub level: LogLevel,
    pub msg: String,
}

impl LogRecord {
    /// ### new
    ///
    /// Instantiates a new LogRecord
    pub fn new(level: LogLevel, msg: &str) -> LogRecord {
        LogRecord {
            time: Instant::now(),
            level: level,
            msg: String::from(msg),
        }
    }
}

/// ## FileTransferActivity
///
/// FileTransferActivity is the data holder for the file transfer activity
pub struct FileTransferActivity {
    pub disconnected: bool,           // Has disconnected from remote?
    pub quit: bool,                   // Has quit term scp?
    params: FileTransferParams,       // FT connection params
    client: Box<dyn FileTransfer>,    // File transfer client
    local: FileExplorer,              // Local File explorer state
    remote: FileExplorer,             // Remote File explorer state
    tab: FileExplorerTab,             // Current selected tab
    log_index: usize,                 // Current log index entry selected
    log_records: VecDeque<LogRecord>, // Log records
    log_size: usize,                  // Log records size (max)
    input_mode: InputMode,            // Current input mode
    input_field: InputField,          // Current selected input mode
    input_txt: String,                // Input text
    choice_opt: DialogYesNoOption,    // Dialog popup selected option
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
            client: match protocol {
                FileTransferProtocol::Sftp => Box::new(SftpFileTransfer::new()),
                FileTransferProtocol::Ftp => Box::new(SftpFileTransfer::new()), // FIXME: FTP
            },
            local: FileExplorer::new(),
            remote: FileExplorer::new(),
            tab: FileExplorerTab::Local,
            log_index: 0,
            log_records: VecDeque::with_capacity(256), // 256 events is enough I guess
            log_size: 256,                             // Must match with capacity
            input_mode: InputMode::Explorer,
            input_field: InputField::Explorer,
            input_txt: String::new(),
            choice_opt: DialogYesNoOption::Yes,
        }
    }

    // @! Session

    /// ### connect
    ///
    /// Connect to remote
    fn connect(&mut self) {
        // Connect to remote
        match self.client.connect(
            self.params.address.clone(),
            self.params.port,
            self.params.username.clone(),
            self.params.password.clone(),
        ) {
            Ok(_) => {
                // Set state to explorer
                self.input_mode = InputMode::Explorer;
                self.reload_remote_dir();
            }
            Err(err) => {
                // Set popup fatal error
                self.input_mode = InputMode::Popup(PopupType::Fatal(format!("{}", err)));
            }
        }
    }

    /// ### disconnect
    ///
    /// disconnect from remote
    fn disconnect(&mut self, context: &mut Context) {
        // Show popup disconnecting
        self.input_mode = InputMode::Popup(PopupType::Alert(
            Color::Red,
            String::from("Disconnecting from remote..."),
        ));
        // Disconnect
        let _ = self.client.disconnect();
        // Quit
        self.disconnected = true;
    }

    /// ### reload_remote_dir
    ///
    /// Reload remote directory entries
    fn reload_remote_dir(&mut self) {
        // Get current entries
        if let Ok(pwd) = self.client.pwd() {
            match self.client.list_dir(pwd.as_path()) {
                Ok(entries) => self.remote.files = entries,
                Err(err) => self.log(
                    LogLevel::Error,
                    format!(
                        "Unable to get files from remote at '{}': {}",
                        pwd.display(),
                        err
                    )
                    .as_ref(),
                ),
            }
        }
    }

    /// ### filetransfer_send
    ///
    /// Send fs entry to remote.
    /// If dst_name is Some, entry will be saved with a different name.
    /// If entry is a directory, this applies to directory only
    fn filetransfer_send(&mut self, ctx: &mut Context, entry: &FsEntry, dst_name: Option<String>) {
        // Write popup
        let file_name: String = match entry {
            FsEntry::Directory(dir) => dir.name.clone(),
            FsEntry::File(file) => file.name.clone(),
        };
        self.input_mode = InputMode::Popup(PopupType::Progress(format!(
            "Uploading \"{}\"...",
            file_name
        )));
        let remote_path: PathBuf = match dst_name {
            Some(s) => PathBuf::from(s.as_str()),
            None => PathBuf::from(file_name.as_str()),
        };
        let prog_cb: Box<ProgressCallback> = Box::new(|c, sz| {
            // Progress callback
            let percentage: f64 = ((c as f64) * 100.0) / (sz as f64);
            unsafe {
                UPLOAD_PROGRESS = percentage;
            }
            // FIXME: can't draw here...
        });
        // Match entry
        match entry {
            FsEntry::File(file) => {
                // Upload file
                // Try to open local file
                match ctx.local.open_file_read(file.abs_path.as_path()) {
                    Ok(mut f) => {
                        match self
                            .client
                            .send_file(remote_path.as_path(), &mut f, Some(prog_cb))
                        {
                            Ok(_) => self.log(
                                LogLevel::Info,
                                format!(
                                    "Saved file \"{}\" to \"{}\"",
                                    file.abs_path.display(),
                                    remote_path.display()
                                )
                                .as_ref(),
                            ),
                            Err(err) => self.log(
                                LogLevel::Error,
                                format!(
                                    "Failed to upload file \"{}\": {}",
                                    file.abs_path.display(),
                                    err
                                )
                                .as_ref(),
                            ),
                        }
                    }
                    Err(err) => {
                        // Report error
                        self.log(
                            LogLevel::Error,
                            format!(
                                "Failed to open file \"{}\": {}",
                                file.abs_path.display(),
                                err
                            )
                            .as_ref(),
                        );
                    }
                }
            }
            FsEntry::Directory(dir) => {
                // Create directory on remote
                match self.client.mkdir(dir.abs_path.as_path()) {
                    Ok(_) => {
                        self.log(
                            LogLevel::Info,
                            format!("Created directory \"{}\"", dir.abs_path.display()).as_ref(),
                        );
                        // Get files in dir
                        match ctx.local.scan_dir(dir.abs_path.as_path()) {
                            Ok(entries) => {
                                // Iterate over files
                                for entry in entries.iter() {
                                    // Send entry; name is always None after first call
                                    self.filetransfer_send(ctx, &entry, None);
                                }
                            }
                            Err(err) => self.log(
                                LogLevel::Error,
                                format!(
                                    "Could not scan directory \"{}\": {}",
                                    dir.abs_path.display(),
                                    err
                                )
                                .as_ref(),
                            ),
                        }
                    }
                    Err(err) => self.log(
                        LogLevel::Error,
                        format!(
                            "Failed to create directory \"{}\": {}",
                            dir.abs_path.display(),
                            err
                        )
                        .as_ref(),
                    ),
                }
            }
        }
    }

    /// ### filetransfer_recv
    ///
    /// Recv fs entry from remote.
    /// If dst_name is Some, entry will be saved with a different name.
    /// If entry is a directory, this applies to directory only
    fn filetransfer_recv(
        &mut self,
        ctx: &mut Context,
        entry: &FsEntry,
        local_path: &Path,
        dst_name: Option<String>,
    ) {
        // Write popup
        let file_name: String = match entry {
            FsEntry::Directory(dir) => dir.name.clone(),
            FsEntry::File(file) => file.name.clone(),
        };
        self.input_mode = InputMode::Popup(PopupType::Progress(format!(
            "Downloading \"{}\"...",
            file_name
        )));
        let prog_cb: Box<ProgressCallback> = Box::new(|c, sz| {
            // Progress callback
            let percentage: f64 = ((c as f64) * 100.0) / (sz as f64);
            unsafe {
                UPLOAD_PROGRESS = percentage;
            }
            // FIXME: can't draw here...
        });
        // Match entry
        match entry {
            FsEntry::File(file) => {
                // Get local file
                let mut local_file_path: PathBuf = PathBuf::from(local_path);
                let local_file_name: String = match dst_name {
                    Some(n) => n.clone(),
                    None => file.name.clone(),
                };
                local_file_path.push(local_file_name.as_str());
                // Try to open local file
                match ctx.local.open_file_write(local_file_path.as_path()) {
                    Ok(mut local_file) => {
                        // Download file from remote
                        match self.client.recv_file(
                            file.abs_path.as_path(),
                            &mut local_file,
                            Some(prog_cb),
                        ) {
                            Ok(_) => self.log(
                                LogLevel::Info,
                                format!(
                                    "Saved file \"{}\" to \"{}\"",
                                    file.abs_path.display(),
                                    local_file_path.display()
                                )
                                .as_ref(),
                            ),
                            Err(err) => self.log(
                                LogLevel::Error,
                                format!(
                                    "Failed to download file \"{}\": {}",
                                    file.abs_path.display(),
                                    err
                                )
                                .as_ref(),
                            ),
                        }
                    }
                    Err(err) => {
                        // Report error
                        self.log(
                            LogLevel::Error,
                            format!(
                                "Failed to open local file for write \"{}\": {}",
                                local_file_path.display(),
                                err
                            )
                            .as_ref(),
                        );
                    }
                }
            }
            FsEntry::Directory(dir) => {
                // Get dir name
                let mut local_dir_path: PathBuf = PathBuf::from(local_path);
                match dst_name {
                    Some(name) => local_dir_path.push(name),
                    None => local_dir_path.push(dir.name.as_str()),
                }
                // Create directory on local
                match ctx.local.mkdir_ex(local_dir_path.as_path(), true) {
                    Ok(_) => {
                        self.log(
                            LogLevel::Info,
                            format!("Created directory \"{}\"", local_dir_path.display()).as_ref(),
                        );
                        // Get files in dir
                        match self.client.list_dir(dir.abs_path.as_path()) {
                            Ok(entries) => {
                                // Iterate over files
                                for entry in entries.iter() {
                                    // Receive entry; name is always None after first call
                                    // Local path becomes local_dir_path
                                    self.filetransfer_recv(
                                        ctx,
                                        &entry,
                                        local_dir_path.as_path(),
                                        None,
                                    );
                                }
                            }
                            Err(err) => self.log(
                                LogLevel::Error,
                                format!(
                                    "Could not scan directory \"{}\": {}",
                                    dir.abs_path.display(),
                                    err
                                )
                                .as_ref(),
                            ),
                        }
                    }
                    Err(err) => self.log(
                        LogLevel::Error,
                        format!(
                            "Failed to create directory \"{}\": {}",
                            local_dir_path.display(),
                            err
                        )
                        .as_ref(),
                    ),
                }
            }
        }
    }

    /// ### log
    ///
    /// Add message to log events
    fn log(&mut self, level: LogLevel, msg: &str) {
        // Create log record
        let record: LogRecord = LogRecord::new(level, msg);
        //Check if history overflows the size
        if self.log_records.len() + 1 > self.log_size {
            self.log_records.pop_back(); // Start cleaning events from back
        }
        // Eventually push front the new record
        self.log_records.push_front(record);
        // Set log index
        self.log_index = self.log_records.len();
    }

    /// ### create_quit_popup
    ///
    /// Create quit popup input mode (since must be shared between different input handlers)
    fn create_quit_popup(&mut self) -> InputMode {
        InputMode::Popup(PopupType::YesNo(
            String::from("Are you sure you want to quit?"),
            FileTransferActivity::disconnect,
            FileTransferActivity::callback_nothing_to_do,
        ))
    }

    /// ### switch_input_field
    ///
    /// Switch input field based on current input field
    fn switch_input_field(&mut self) {
        self.input_field = match self.input_field {
            InputField::Explorer => InputField::Logs,
            InputField::Logs => InputField::Explorer,
        }
    }

    // @! input listeners

    /// ### handle_input_event
    ///
    /// Handle input event based on current input mode
    fn handle_input_event(&mut self, context: &mut Context, ev: &InputEvent) {
        match &self.input_mode {
            InputMode::Explorer => self.handle_input_event_mode_explorer(context, ev),
            InputMode::Popup(ptype) => {
                self.handle_input_event_mode_popup(ev, ptype.clone(), context)
            }
        }
    }

    /// ### handle_input_event_mode_explorer
    ///
    /// Input event handler for explorer mode
    fn handle_input_event_mode_explorer(&mut self, context: &mut Context, ev: &InputEvent) {
        // Match input field
        match self.input_field {
            InputField::Explorer => match self.tab {
                // Match current selected tab
                FileExplorerTab::Local => {
                    self.handle_input_event_mode_explorer_tab_local(context, ev)
                }
                FileExplorerTab::Remote => {
                    self.handle_input_event_mode_explorer_tab_remote(context, ev)
                }
            },
            InputField::Logs => self.handle_input_event_mode_explorer_log(ev),
        }
    }

    /// ### handle_input_event_mode_explorer_tab_local
    ///
    /// Input event handler for explorer mode when localhost tab is selected
    fn handle_input_event_mode_explorer_tab_local(
        &mut self,
        context: &mut Context,
        ev: &InputEvent,
    ) {
        // Match events
        match ev {
            InputEvent::Key(key) => {
                match key.code {
                    KeyCode::Esc => {
                        // Handle quit event
                        // Create quit prompt dialog
                        self.input_mode = self.create_quit_popup();
                    }
                    KeyCode::Tab => self.switch_input_field(), // <TAB> switch tab
                    KeyCode::Right => self.tab = FileExplorerTab::Remote, // <RIGHT> switch to right tab
                    KeyCode::Up => {
                        // Move index up
                        if self.local.index > 0 {
                            self.local.index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        // Move index down
                        if self.local.index + 1 < self.local.files.len() {
                            self.local.index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        // Match selected file
                        if let Some(entry) = self.local.files.get(self.local.index) {
                            if let FsEntry::Directory(dir) = entry {
                                // Change directory
                                if let Err(err) = context.local.change_wrkdir(dir.abs_path.clone())
                                {
                                    // Report err
                                    self.input_mode = InputMode::Popup(PopupType::Alert(
                                        Color::Red,
                                        format!("Could not change working directory: {}", err),
                                    ));
                                }
                                // Update files
                                self.local.files = context.local.list_dir();
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        // TODO: directory stack
                        // Go previous directory
                        let wrkdir: PathBuf = context.local.pwd();
                        if let Some(parent) = wrkdir.as_path().parent() {
                            // Change directory
                            if let Err(err) = context.local.change_wrkdir(PathBuf::from(parent)) {
                                // Report err
                                self.input_mode = InputMode::Popup(PopupType::Alert(
                                    Color::Red,
                                    format!("Could not change working directory: {}", err),
                                ));
                            }
                            // Update files
                            self.local.files = context.local.list_dir();
                        }
                    }
                    KeyCode::Delete => {
                        // Get file at index
                        if let Some(entry) = self.local.files.get(self.local.index) {
                            // Get file name
                            let file_name: String = match entry {
                                FsEntry::Directory(dir) => dir.name.clone(),
                                FsEntry::File(file) => file.name.clone(),
                            };
                            // Show delete prompt
                            self.input_mode = InputMode::Popup(PopupType::YesNo(
                                format!("Delete file \"{}\"", file_name),
                                FileTransferActivity::callback_delete_fsentry,
                                FileTransferActivity::callback_nothing_to_do,
                            ))
                        }
                    }
                    KeyCode::Char(ch) => match ch {
                        'g' | 'G' => {
                            // Goto
                            // If ctrl is enabled...
                            if key.modifiers.intersects(KeyModifiers::CONTROL) {
                                // Show input popup
                                self.input_mode = InputMode::Popup(PopupType::Input(
                                    String::from("Change working directory"),
                                    FileTransferActivity::callback_change_directory,
                                ));
                            }
                        }
                        'm' | 'M' => {
                            // Make directory
                            // If ctrl is enabled...
                            if key.modifiers.intersects(KeyModifiers::CONTROL) {
                                self.input_mode = InputMode::Popup(PopupType::Input(
                                    String::from("Insert directory name"),
                                    FileTransferActivity::callback_mkdir,
                                ));
                            }
                        }
                        'r' | 'R' => {
                            // Rename
                            // If ctrl is enabled...
                            if key.modifiers.intersects(KeyModifiers::CONTROL) {
                                self.input_mode = InputMode::Popup(PopupType::Input(
                                    String::from("Insert new name"),
                                    FileTransferActivity::callback_rename,
                                ));
                            }
                        }
                        's' | 'S' => {
                            // Save as...
                            // If ctrl is enabled...
                            if key.modifiers.intersects(KeyModifiers::CONTROL) {
                                // Ask for input
                                self.input_mode = InputMode::Popup(PopupType::Input(
                                    String::from("Save as..."),
                                    FileTransferActivity::callback_save_as,
                                ));
                            }
                        }
                        ' ' => {
                            // Get files
                            let files: Vec<FsEntry> = self.local.files.clone(); // Otherwise self is borrowed both as mutable and immutable...
                                                                                // Get file at index
                            if let Some(entry) = files.get(self.local.index) {
                                // Call upload
                                self.filetransfer_send(context, entry, None);
                            }
                        }
                        _ => { /* Nothing to do */ }
                    },
                    _ => { /* Nothing to do */ }
                }
            }
            _ => { /* Nothing to do */ }
        }
    }

    /// ### handle_input_event_mode_explorer_tab_local
    ///
    /// Input event handler for explorer mode when remote tab is selected
    fn handle_input_event_mode_explorer_tab_remote(
        &mut self,
        context: &mut Context,
        ev: &InputEvent,
    ) {
        // TODO: implement
        // Match events
        match ev {
            InputEvent::Key(key) => {
                match key.code {
                    KeyCode::Esc => {
                        // Handle quit event
                        // Create quit prompt dialog
                        self.input_mode = self.create_quit_popup();
                    }
                    KeyCode::Tab => self.switch_input_field(), // <TAB> switch tab
                    KeyCode::Left => self.tab = FileExplorerTab::Local, // <LEFT> switch to local tab
                    _ => { /* Nothing to do */ }
                }
            }
            _ => { /* Nothing to do */ }
        }
    }

    /// ### handle_input_event_mode_explorer_log
    ///
    /// Input even handler for explorer mode when log tab is selected
    fn handle_input_event_mode_explorer_log(&mut self, ev: &InputEvent) {
        // Match event
        let records_block: usize = 16;
        match ev {
            InputEvent::Key(key) => {
                match key.code {
                    KeyCode::Esc => {
                        // Handle quit event
                        // Create quit prompt dialog
                        self.input_mode = self.create_quit_popup();
                    }
                    KeyCode::Tab => self.switch_input_field(), // <TAB> switch tab
                    KeyCode::Up => {
                        // Decrease log index
                        if self.log_index > 0 {
                            self.log_index = self.log_index - 1;
                        }
                    }
                    KeyCode::Down => {
                        // Increase log index
                        if self.log_index + 1 >= self.log_size {
                            self.log_index = self.log_index + 1;
                        }
                    }
                    KeyCode::PageUp => {
                        // Fast decreasing of log index
                        if self.log_index >= records_block {
                            self.log_index = self.log_index - records_block; // Decrease by `records_block` if possible
                        } else {
                            self.log_index = 0; // Set to 0 otherwise
                        }
                    }
                    KeyCode::PageDown => {
                        // Fast increasing of log index
                        if self.log_index + records_block >= self.log_size {
                            // If overflows, set to size
                            self.log_index = self.log_size - 1;
                        } else {
                            self.log_index = self.log_index + records_block; // Increase by `records_block`
                        }
                    }
                    _ => { /* Nothing to do */ }
                }
            }
            _ => { /* Nothing to do */ }
        }
    }

    /// ### handle_input_event_mode_explorer
    ///
    /// Input event handler for popup mode. Handler is then based on Popup type
    fn handle_input_event_mode_popup(
        &mut self,
        ev: &InputEvent,
        popup: PopupType,
        ctx: &mut Context,
    ) {
        match popup {
            PopupType::Alert(_, _) => self.handle_input_event_mode_popup_alert(ev),
            PopupType::Fatal(_) => self.handle_input_event_mode_popup_fatal(ev),
            PopupType::Input(_, cb) => self.handle_input_event_mode_popup_input(ev, ctx, cb),
            PopupType::Progress(_) => self.handle_input_event_mode_popup_progress(ev),
            PopupType::Wait(_) => self.handle_input_event_mode_popup_wait(ev),
            PopupType::YesNo(_, yes_cb, no_cb) => {
                self.handle_input_event_mode_popup_yesno(ev, ctx, yes_cb, no_cb)
            }
        }
    }

    /// ### handle_input_event_mode_explorer_alert
    ///
    /// Input event handler for popup alert
    fn handle_input_event_mode_popup_alert(&mut self, ev: &InputEvent) {
        // If enter, close popup
        match ev {
            InputEvent::Key(key) => {
                match key.code {
                    KeyCode::Enter => {
                        // Set input mode back to explorer
                        self.input_mode = InputMode::Explorer;
                    }
                    _ => { /* Nothing to do */ }
                }
            }
            _ => { /* Nothing to do */ }
        }
    }

    /// ### handle_input_event_mode_explorer_alert
    ///
    /// Input event handler for popup alert
    fn handle_input_event_mode_popup_fatal(&mut self, ev: &InputEvent) {
        // If enter, close popup
        match ev {
            InputEvent::Key(key) => {
                match key.code {
                    KeyCode::Enter => {
                        // Set quit to true; since a fatal error happened
                        self.quit = true;
                    }
                    _ => { /* Nothing to do */ }
                }
            }
            _ => { /* Nothing to do */ }
        }
    }

    /// ### handle_input_event_mode_popup_input
    ///
    /// Input event handler for input popup
    fn handle_input_event_mode_popup_input(
        &mut self,
        ev: &InputEvent,
        ctx: &mut Context,
        cb: OnInputSubmitCallback,
    ) {
        // If enter, close popup, otherwise push chars to input
        match ev {
            InputEvent::Key(key) => {
                match key.code {
                    KeyCode::Esc => {
                        // Abort input
                        // Clear current input text
                        self.input_txt.clear();
                        // Set mode back to explorer
                        self.input_mode = InputMode::Explorer;
                    }
                    KeyCode::Enter => {
                        // Submit
                        let input_text: String = self.input_txt.clone();
                        // Clear current input text
                        self.input_txt.clear();
                        // Set mode back to explorer BEFORE CALLBACKS!!! Callback can then overwrite this, clever uh?
                        self.input_mode = InputMode::Explorer;
                        // Call cb
                        cb(self, ctx, input_text);
                    }
                    KeyCode::Char(ch) => self.input_txt.push(ch),
                    KeyCode::Backspace => {
                        let _ = self.input_txt.pop();
                    }
                    _ => { /* Nothing to do */ }
                }
            }
            _ => { /* Nothing to do */ }
        }
    }

    /// ### handle_input_event_mode_explorer_alert
    ///
    /// Input event handler for popup alert
    fn handle_input_event_mode_popup_progress(&mut self, ev: &InputEvent) {
        // There's nothing you can do here I guess... maybe ctrl+c in the future idk
        match ev {
            _ => { /* Nothing to do */ }
        }
    }

    /// ### handle_input_event_mode_explorer_alert
    ///
    /// Input event handler for popup alert
    fn handle_input_event_mode_popup_wait(&mut self, ev: &InputEvent) {
        // There's nothing you can do here I guess... maybe ctrl+c in the future idk
        match ev {
            _ => { /* Nothing to do */ }
        }
    }

    /// ### handle_input_event_mode_explorer_alert
    ///
    /// Input event handler for popup alert
    fn handle_input_event_mode_popup_yesno(
        &mut self,
        ev: &InputEvent,
        ctx: &mut Context,
        yes_cb: DialogCallback,
        no_cb: DialogCallback,
    ) {
        // If enter, close popup, otherwise move dialog option
        match ev {
            InputEvent::Key(key) => {
                match key.code {
                    KeyCode::Enter => {
                        // @! Set input mode to Explorer BEFORE CALLBACKS!!! Callback can then overwrite this, clever uh?
                        self.input_mode = InputMode::Explorer;
                        // Check if user selected yes or not
                        match self.choice_opt {
                            DialogYesNoOption::No => no_cb(self, ctx),
                            DialogYesNoOption::Yes => yes_cb(self, ctx),
                        }
                    }
                    KeyCode::Right => self.choice_opt = DialogYesNoOption::No, // Set to NO
                    KeyCode::Left => self.choice_opt = DialogYesNoOption::Yes, // Set to YES
                    _ => { /* Nothing to do */ }
                }
            }
            _ => { /* Nothing to do */ }
        }
    }

    // @! Callbacks

    /// ### callback_nothing_to_do
    ///
    /// Self titled
    fn callback_nothing_to_do(&mut self, _context: &mut Context) {}

    /// ### callback_force_input_mode_to_explorer
    ///
    /// force input mode to explorer
    fn callback_force_input_mode_to_explorer(&mut self, _context: &mut Context) {
        self.input_mode = InputMode::Explorer;
    }

    /// ### callback_change_directory
    ///
    /// Callback for GOTO command
    fn callback_change_directory(&mut self, context: &mut Context, input: String) {
        match context.local.change_wrkdir(PathBuf::from(input.as_str())) {
            Err(err) => {
                // Report err
                self.input_mode = InputMode::Popup(PopupType::Alert(
                    Color::Red,
                    format!("Could not change working directory: {}", err),
                ));
            }
            Ok(_) => self.local.files = context.local.list_dir(), // Update files
        }
    }

    /// ### callback_mkdir
    ///
    /// Callback for MKDIR command (supports both local and remote)
    fn callback_mkdir(&mut self, context: &mut Context, input: String) {
        match self.tab {
            FileExplorerTab::Local => {
                match context.local.mkdir(PathBuf::from(input.as_str()).as_path()) {
                    Ok(_) => {
                        // Reload files
                        self.log(
                            LogLevel::Info,
                            format!("Created directory \"{}\"", input).as_ref(),
                        );
                        self.local.files = context.local.list_dir();
                    }
                    Err(err) => {
                        // Report err
                        self.log(
                            LogLevel::Error,
                            format!("Could not create directory \"{}\": {}", input, err).as_ref(),
                        );
                        self.input_mode = InputMode::Popup(PopupType::Alert(
                            Color::Red,
                            format!("Could not create directory \"{}\": {}", input, err),
                        ));
                    }
                }
            }
            FileExplorerTab::Remote => {
                match self.client.mkdir(PathBuf::from(input.as_str()).as_path()) {
                    Ok(_) => {
                        // Reload files
                        self.log(
                            LogLevel::Info,
                            format!("Created directory \"{}\"", input).as_ref(),
                        );
                        self.reload_remote_dir();
                    }
                    Err(err) => {
                        // Report err
                        self.log(
                            LogLevel::Error,
                            format!("Could not create directory \"{}\": {}", input, err).as_ref(),
                        );
                        self.input_mode = InputMode::Popup(PopupType::Alert(
                            Color::Red,
                            format!("Could not create directory \"{}\": {}", input, err),
                        ));
                    }
                }
            }
        }
    }

    /// ### callback_rename
    ///
    /// Callback for RENAME command (supports borth local and remote)
    fn callback_rename(&mut self, context: &mut Context, input: String) {
        match self.tab {
            FileExplorerTab::Local => {
                let mut dst_path: PathBuf = PathBuf::from(input);
                // Check if path is relative
                if dst_path.as_path().is_relative() {
                    let mut wrkdir: PathBuf = context.local.pwd();
                    wrkdir.push(dst_path);
                    dst_path = wrkdir;
                }
                // Check if file entry exists
                if let Some(entry) = self.local.files.get(self.local.index) {
                    let full_path: PathBuf = match entry {
                        FsEntry::Directory(dir) => dir.abs_path.clone(),
                        FsEntry::File(file) => file.abs_path.clone(),
                    };
                    // Rename file or directory and report status as popup
                    match context.local.rename(entry, dst_path.as_path()) {
                        Ok(_) => {
                            // Reload files
                            self.local.files = context.local.list_dir();
                            // Log
                            self.log(
                                LogLevel::Info,
                                format!(
                                    "Renamed file \"{}\" to \"{}\"",
                                    full_path.display(),
                                    dst_path.display()
                                )
                                .as_ref(),
                            );
                        }
                        Err(err) => {
                            self.log(
                                LogLevel::Error,
                                format!(
                                    "Could not rename file \"{}\": {}",
                                    full_path.display(),
                                    err
                                )
                                .as_ref(),
                            );
                            self.input_mode = InputMode::Popup(PopupType::Alert(
                                Color::Red,
                                format!("Could not rename file: {}", err),
                            ))
                        }
                    }
                }
            }
            FileExplorerTab::Remote => {
                // Check if file entry exists
                if let Some(entry) = self.remote.files.get(self.remote.index) {
                    let full_path: PathBuf = match entry {
                        FsEntry::Directory(dir) => dir.abs_path.clone(),
                        FsEntry::File(file) => file.abs_path.clone(),
                    };
                    // Rename file or directory and report status as popup
                    let dst_path: PathBuf = PathBuf::from(input);
                    match self.client.rename(entry, dst_path.as_path()) {
                        Ok(_) => {
                            // Reload files
                            self.local.files = context.local.list_dir();
                            // Log
                            self.log(
                                LogLevel::Info,
                                format!(
                                    "Renamed file \"{}\" to \"{}\"",
                                    full_path.display(),
                                    dst_path.display()
                                )
                                .as_ref(),
                            );
                        }
                        Err(err) => {
                            self.log(
                                LogLevel::Error,
                                format!(
                                    "Could not rename file \"{}\": {}",
                                    full_path.display(),
                                    err
                                )
                                .as_ref(),
                            );
                            self.input_mode = InputMode::Popup(PopupType::Alert(
                                Color::Red,
                                format!("Could not rename file: {}", err),
                            ))
                        }
                    }
                }
            }
        }
    }

    /// ### callback_delete_fsentry
    ///
    /// Delete current selected fsentry in the currently selected TAB
    fn callback_delete_fsentry(&mut self, context: &mut Context) {
        // Match current selected tab
        match self.tab {
            FileExplorerTab::Local => {
                // Check if file entry exists
                if let Some(entry) = self.local.files.get(self.local.index) {
                    let full_path: PathBuf = match entry {
                        FsEntry::Directory(dir) => dir.abs_path.clone(),
                        FsEntry::File(file) => file.abs_path.clone(),
                    };
                    // Delete file or directory and report status as popup
                    match context.local.remove(entry) {
                        Ok(_) => {
                            // Reload files
                            self.local.files = context.local.list_dir();
                            // Log
                            self.log(
                                LogLevel::Info,
                                format!("Removed file \"{}\"", full_path.display()).as_ref(),
                            );
                        }
                        Err(err) => {
                            self.log(
                                LogLevel::Error,
                                format!(
                                    "Could not delete file \"{}\": {}",
                                    full_path.display(),
                                    err
                                )
                                .as_ref(),
                            );
                            self.input_mode = InputMode::Popup(PopupType::Alert(
                                Color::Red,
                                format!("Could not delete file: {}", err),
                            ))
                        }
                    }
                }
            }
            FileExplorerTab::Remote => {
                // Check if file entry exists
                if let Some(entry) = self.remote.files.get(self.remote.index) {
                    let full_path: PathBuf = match entry {
                        FsEntry::Directory(dir) => dir.abs_path.clone(),
                        FsEntry::File(file) => file.abs_path.clone(),
                    };
                    // Delete file
                    match self.client.remove(entry) {
                        Ok(_) => {
                            self.reload_remote_dir();
                            self.log(
                                LogLevel::Info,
                                format!("Removed file \"{}\"", full_path.display()).as_ref(),
                            );
                        }
                        Err(err) => {
                            self.log(
                                LogLevel::Error,
                                format!(
                                    "Could not delete file \"{}\": {}",
                                    full_path.display(),
                                    err
                                )
                                .as_ref(),
                            );
                            self.input_mode = InputMode::Popup(PopupType::Alert(
                                Color::Red,
                                format!("Could not delete file: {}", err),
                            ))
                        }
                    }
                }
            }
        }
    }

    /// ### callback_save_as
    ///
    /// Call file upload, but save with input as name
    /// Handled both local and remote tab
    fn callback_save_as(&mut self, ctx: &mut Context, input: String) {
        match self.tab {
            FileExplorerTab::Local => {
                let files: Vec<FsEntry> = self.local.files.clone();
                // Get file at index
                if let Some(entry) = files.get(self.local.index) {
                    // Call send (upload)
                    self.filetransfer_send(ctx, entry, Some(input));
                }
            }
            FileExplorerTab::Remote => {
                let files: Vec<FsEntry> = self.remote.files.clone();
                // Get file at index
                if let Some(entry) = files.get(self.remote.index) {
                    // Call receive (download)
                    self.filetransfer_recv(ctx, entry, ctx.local.pwd().as_path(), Some(input));
                }
            }
        }
    }

    // @! Gfx

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
                "<TAB>",
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
            Span::raw("change directory\t"),
            Span::styled(
                "<SPACE>",
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("upload/download file\t"),
            Span::styled(
                "<CTRL+G>",
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("goto path\t"),
            Span::styled(
                "<CTRL+M>",
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("make dir\t"),
            Span::styled(
                "<CTRL+R>",
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("rename file\t"),
            Span::styled(
                "<CANC>",
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("delete file\t"),
        ];
        Paragraph::new(Text::from(Spans::from(footer)))
    }
}

/**
 * Activity Trait
 * Keep it clean :)
 * Use methods instead!
 */

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
        // Get files at current wd
        self.local.files = context.local.list_dir();
    }

    /// ### on_draw
    ///
    /// `on_draw` is the function which draws the graphical interface.
    /// This function must be called at each tick to refresh the interface
    fn on_draw(&mut self, context: &mut Context) {
        // Check if connected
        if !self.client.is_connected() {
            // Set init state to connecting popup
            self.input_mode = InputMode::Popup(PopupType::Wait(format!(
                "Connecting to {}:{}...",
                self.params.address, self.params.port
            )));
            // Force ui draw
            let _ = context.terminal.draw(|f| {
                self.draw(f);
            });
            // Connect to remote
            self.connect();
        }
        // Handle input events FIXME: read one or multiple?
        if let Ok(event) = context.input_hnd.read_event() {
            // Iterate over input events
            if let Some(event) = event {
                self.handle_input_event(context, &event);
            }
        }
        // @! draw interface
        let _ = context.terminal.draw(|f| {
            self.draw(f);
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
        // Disconnect client
        if self.client.is_connected() {
            let _ = self.client.disconnect();
        }
    }
}
