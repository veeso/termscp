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
extern crate chrono;
extern crate crossterm;
extern crate tui;
extern crate unicode_width;

// locals
use super::{Activity, Context};
use crate::filetransfer::FileTransferProtocol;

// File transfer
use crate::filetransfer::sftp_transfer::SftpFileTransfer;
use crate::filetransfer::FileTransfer;
use crate::fs::FsEntry;

// Includes
use chrono::{DateTime, Local};
use crossterm::event::Event as InputEvent;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::collections::VecDeque;
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};
use tui::{
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Tabs},
};
use unicode_width::UnicodeWidthStr;

// Types
type DialogCallback = fn(&mut FileTransferActivity);
type OnInputSubmitCallback = fn(&mut FileTransferActivity, String);

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
    Help,                                          // Show Help
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
    dirstack: VecDeque<PathBuf>,
}

impl FileExplorer {
    /// ### new
    ///
    /// Instantiates a new FileExplorer
    pub fn new() -> FileExplorer {
        FileExplorer {
            index: 0,
            files: Vec::new(),
            dirstack: VecDeque::with_capacity(16),
        }
    }

    /// ### pushd
    ///
    /// push directory to stack
    pub fn pushd(&mut self, dir: &Path) {
        // Check if stack overflows the size
        if self.dirstack.len() + 1 > 16 {
            self.dirstack.pop_back(); // Start cleaning events from back
        }
        // Eventually push front the new record
        self.dirstack.push_front(PathBuf::from(dir));
    }

    /// ### popd
    ///
    /// Pop directory from the stack and return the directory
    pub fn popd(&mut self) -> Option<PathBuf> {
        self.dirstack.pop_front()
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
    pub time: DateTime<Local>,
    pub level: LogLevel,
    pub msg: String,
}

impl LogRecord {
    /// ### new
    ///
    /// Instantiates a new LogRecord
    pub fn new(level: LogLevel, msg: &str) -> LogRecord {
        LogRecord {
            time: Local::now(),
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
    context: Option<Context>,         // Context holder
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
    transfer_progress: f64,           // Current write/read progress (percentage)
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
            context: None,
            params: params,
            client: match protocol {
                FileTransferProtocol::Sftp => Box::new(SftpFileTransfer::new()),
                FileTransferProtocol::Ftp => panic!("FTP is not supported YET!"), // FIXME: FTP
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
            transfer_progress: 0.0,
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
    fn disconnect(&mut self) {
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
            self.remote_scan(pwd.as_path());
        }
    }

    /// ### filetransfer_send
    ///
    /// Send fs entry to remote.
    /// If dst_name is Some, entry will be saved with a different name.
    /// If entry is a directory, this applies to directory only
    fn filetransfer_send(&mut self, entry: &FsEntry, curr_remote_path: &Path, dst_name: Option<String>) {
        // Write popup
        let file_name: String = match entry {
            FsEntry::Directory(dir) => dir.name.clone(),
            FsEntry::File(file) => file.name.clone(),
        };
        self.input_mode = InputMode::Popup(PopupType::Wait(format!("Uploading \"{}\"", file_name)));
        // Draw
        self.draw();
        // Get remote path
        let mut remote_path: PathBuf = PathBuf::from(curr_remote_path);
        let remote_file_name: PathBuf = match dst_name {
            Some(s) => PathBuf::from(s.as_str()),
            None => PathBuf::from(file_name.as_str()),
        };
        remote_path.push(remote_file_name);
        // Match entry
        match entry {
            FsEntry::File(file) => {
                // Upload file
                // Try to open local file
                match self
                    .context
                    .as_ref()
                    .unwrap()
                    .local
                    .open_file_read(file.abs_path.as_path())
                {
                    Ok(mut fhnd) => match self.client.send_file(remote_path.as_path()) {
                        Ok(mut rhnd) => {
                            // Write file
                            let file_size: usize =
                                fhnd.seek(std::io::SeekFrom::End(0)).unwrap_or(0) as usize;
                            // rewind
                            if let Err(err) = fhnd.seek(std::io::SeekFrom::Start(0)) {
                                self.log(
                                    LogLevel::Error,
                                    format!("Could not rewind local file: {}", err).as_ref(),
                                );
                            }
                            // Write remote file
                            let mut total_bytes_written: usize = 0;
                            // Set input state to popup progress
                            self.input_mode = InputMode::Popup(PopupType::Progress(format!(
                                "Uploading \"{}\"",
                                file_name
                            )));
                            loop {
                                // Read till you can
                                let mut buffer: [u8; 8192] = [0; 8192];
                                match fhnd.read(&mut buffer) {
                                    Ok(bytes_read) => {
                                        total_bytes_written += bytes_read;
                                        if bytes_read == 0 {
                                            break;
                                        } else {
                                            // Write bytes
                                            if let Err(err) = rhnd.write(&buffer[0..bytes_read]) {
                                                self.log(
                                                    LogLevel::Error,
                                                    format!("Could not write remote file: {}", err)
                                                        .as_ref(),
                                                );
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        self.log(
                                            LogLevel::Error,
                                            format!("Could not read local file: {}", err).as_ref(),
                                        );
                                    }
                                }
                                // Increase progress
                                self.set_progress(total_bytes_written, file_size);
                                // Draw
                                self.draw();
                            }
                            self.log(
                                LogLevel::Info,
                                format!(
                                    "Saved file \"{}\" to \"{}\"",
                                    file.abs_path.display(),
                                    remote_path.display()
                                )
                                .as_ref(),
                            );
                        }
                        Err(err) => self.log(
                            LogLevel::Error,
                            format!(
                                "Failed to upload file \"{}\": {}",
                                file.abs_path.display(),
                                err
                            )
                            .as_ref(),
                        ),
                    },
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
                match self.client.mkdir(remote_path.as_path()) {
                    Ok(_) => {
                        self.log(
                            LogLevel::Info,
                            format!("Created directory \"{}\"", remote_path.display()).as_ref(),
                        );
                        // Get files in dir
                        match self
                            .context
                            .as_ref()
                            .unwrap()
                            .local
                            .scan_dir(dir.abs_path.as_path())
                        {
                            Ok(entries) => {
                                // Iterate over files
                                for entry in entries.iter() {
                                    // Send entry; name is always None after first call
                                    self.filetransfer_send(&entry, remote_path.as_path(), None);
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
                            remote_path.display(),
                            err
                        )
                        .as_ref(),
                    ),
                }
            }
        }
        // Scan dir on remote
        if let Ok(path) = self.client.pwd() {
            self.remote_scan(path.as_path());
        }
        // Eventually, Reset input mode to explorer
        self.input_mode = InputMode::Explorer;
    }

    /// ### filetransfer_recv
    ///
    /// Recv fs entry from remote.
    /// If dst_name is Some, entry will be saved with a different name.
    /// If entry is a directory, this applies to directory only
    fn filetransfer_recv(&mut self, entry: &FsEntry, local_path: &Path, dst_name: Option<String>) {
        // Write popup
        let file_name: String = match entry {
            FsEntry::Directory(dir) => dir.name.clone(),
            FsEntry::File(file) => file.name.clone(),
        };
        self.input_mode =
            InputMode::Popup(PopupType::Wait(format!("Downloading \"{}\"...", file_name)));
        // Draw
        self.draw();
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
                match self
                    .context
                    .as_ref()
                    .unwrap()
                    .local
                    .open_file_write(local_file_path.as_path())
                {
                    Ok(mut local_file) => {
                        // Download file from remote
                        match self.client.recv_file(file.abs_path.as_path()) {
                            Ok((mut rhnd, file_size)) => {
                                // Set popup progress
                                self.input_mode = InputMode::Popup(PopupType::Progress(format!(
                                    "Downloading \"{}\"...",
                                    file_name
                                )));
                                let mut total_bytes_written: usize = 0;
                                // Write local file
                                loop {
                                    // Read till you can
                                    let mut buffer: [u8; 8192] = [0; 8192];
                                    match rhnd.read(&mut buffer) {
                                        Ok(bytes_read) => {
                                            total_bytes_written += bytes_read;
                                            if bytes_read == 0 {
                                                break;
                                            } else {
                                                // Write bytes
                                                if let Err(err) =
                                                    local_file.write(&buffer[0..bytes_read])
                                                {
                                                    self.log(
                                                        LogLevel::Error,
                                                        format!(
                                                            "Could not write local file: {}",
                                                            err
                                                        )
                                                        .as_ref(),
                                                    );
                                                }
                                            }
                                        }
                                        Err(err) => self.log(
                                            LogLevel::Error,
                                            format!("Could not read remote file: {}", err).as_ref(),
                                        ),
                                    }
                                    // Set progress
                                    self.set_progress(total_bytes_written, file_size);
                                    // Draw
                                    self.draw();
                                }
                                // Log
                                self.log(
                                    LogLevel::Info,
                                    format!(
                                        "Saved file \"{}\" to \"{}\"",
                                        file.abs_path.display(),
                                        local_file_path.display()
                                    )
                                    .as_ref(),
                                );
                            }
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
                match self
                    .context
                    .as_mut()
                    .unwrap()
                    .local
                    .mkdir_ex(local_dir_path.as_path(), true)
                {
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
                                    self.filetransfer_recv(&entry, local_dir_path.as_path(), None);
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
        // Reload directory on local
        self.local_scan(local_path);
        // Eventually, Reset input mode to explorer
        self.input_mode = InputMode::Explorer;
    }

    /// ### local_scan
    ///
    /// Scan current local directory
    fn local_scan(&mut self, path: &Path) {
        match self.context.as_ref().unwrap().local.scan_dir(path) {
            Ok(files) => {
                // Reset index
                self.local.index = 0;
                self.local.files = files;
            }
            Err(err) => {
                self.log(
                    LogLevel::Error,
                    format!("Could not scan current directory: {}", err).as_str(),
                );
            }
        }
    }

    /// ### remote_scan
    ///
    /// Scan current remote directory
    fn remote_scan(&mut self, path: &Path) {
        match self.client.list_dir(path) {
            Ok(files) => {
                // Reset index
                self.remote.index = 0;
                self.remote.files = files
            }
            Err(err) => {
                self.log(
                    LogLevel::Error,
                    format!("Could not scan current directory: {}", err).as_str(),
                );
            }
        }
    }

    /// ### local_changedir
    ///
    /// Change directory for local
    fn local_changedir(&mut self, path: &Path, push: bool) {
        // Get current directory
        let prev_dir: PathBuf = self.context.as_ref().unwrap().local.pwd();
        // Change directory
        match self
            .context
            .as_mut()
            .unwrap()
            .local
            .change_wrkdir(PathBuf::from(path))
        {
            Ok(_) => {
                self.log(
                    LogLevel::Info,
                    format!("Changed directory on local: {}", path.display()).as_str(),
                );
                // Reload files
                self.local_scan(path);
                // Push prev_dir to stack
                if push {
                    self.local.pushd(prev_dir.as_path())
                }
            }
            Err(err) => {
                // Report err
                self.input_mode = InputMode::Popup(PopupType::Alert(
                    Color::Red,
                    format!("Could not change working directory: {}", err),
                ));
            }
        }
    }

    fn remote_changedir(&mut self, path: &Path, push: bool) {
        // Get current directory
        match self.client.pwd() {
            Ok(prev_dir) => {
                // Change directory
                match self.client.change_dir(path) {
                    Ok(_) => {
                        self.log(
                            LogLevel::Info,
                            format!("Changed directory on remote: {}", path.display()).as_str(),
                        );
                        // Update files
                        self.remote_scan(path);
                        // Push prev_dir to stack
                        if push {
                            self.remote.pushd(prev_dir.as_path())
                        }
                    }
                    Err(err) => {
                        // Report err
                        self.input_mode = InputMode::Popup(PopupType::Alert(
                            Color::Red,
                            format!("Could not change working directory: {}", err),
                        ));
                    }
                }
            }
            Err(err) => {
                // Report err
                self.input_mode = InputMode::Popup(PopupType::Alert(
                    Color::Red,
                    format!("Could not change working directory: {}", err),
                ));
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
        self.log_index = 0;
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

    /// ### set_progress
    ///
    /// Calculate progress percentage based on current progress
    fn set_progress(&mut self, it: usize, sz: usize) {
        self.transfer_progress = ((it as f64) * 100.0) / (sz as f64);
    }

    // @! input listeners

    /// ### handle_input_event
    ///
    /// Handle input event based on current input mode
    fn handle_input_event(&mut self, ev: &InputEvent) {
        // NOTE: this is necessary due to this <https://github.com/rust-lang/rust/issues/59159>
        // NOTE: Do you want my opinion about that issue? It's a bs and doesn't make any sense.
        let popup: Option<PopupType> = match &self.input_mode {
            InputMode::Popup(ptype) => Some(ptype.clone()),
            _ => None
        };
        match &self.input_mode {
            InputMode::Explorer => self.handle_input_event_mode_explorer(ev),
            InputMode::Popup(_) => if let Some(popup) = popup {
                self.handle_input_event_mode_popup(ev, popup);
            }
        }
    }

    /// ### handle_input_event_mode_explorer
    ///
    /// Input event handler for explorer mode
    fn handle_input_event_mode_explorer(&mut self, ev: &InputEvent) {
        // Match input field
        match self.input_field {
            InputField::Explorer => match self.tab {
                // Match current selected tab
                FileExplorerTab::Local => self.handle_input_event_mode_explorer_tab_local(ev),
                FileExplorerTab::Remote => self.handle_input_event_mode_explorer_tab_remote(ev),
            },
            InputField::Logs => self.handle_input_event_mode_explorer_log(ev),
        }
    }

    /// ### handle_input_event_mode_explorer_tab_local
    ///
    /// Input event handler for explorer mode when localhost tab is selected
    fn handle_input_event_mode_explorer_tab_local(&mut self, ev: &InputEvent) {
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
                        let local_files: Vec<FsEntry> = self.local.files.clone();
                        if let Some(entry) = local_files.get(self.local.index) {
                            if let FsEntry::Directory(dir) = entry {
                                self.local_changedir(dir.abs_path.as_path(), true);
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        // Go to previous directory
                        if let Some(d) = self.local.popd() {
                            self.local_changedir(d.as_path(), false);
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
                        'd' | 'D' => {
                            // Make directory
                            // If ctrl is enabled...
                            if key.modifiers.intersects(KeyModifiers::CONTROL) {
                                self.input_mode = InputMode::Popup(PopupType::Input(
                                    String::from("Insert directory name"),
                                    FileTransferActivity::callback_mkdir,
                                ));
                            }
                        }
                        'h' | 'H' => {
                            // Show help
                            // If ctrl is enabled...
                            if key.modifiers.intersects(KeyModifiers::CONTROL) {
                                self.input_mode = InputMode::Popup(PopupType::Help);
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
                        'u' | 'U' => {
                            // Go to parent directory
                            // If ctrl is enabled...
                            if key.modifiers.intersects(KeyModifiers::CONTROL) {
                                // Get pwd
                                let path: PathBuf = self.context.as_ref().unwrap().local.pwd();
                                if let Some(parent) = path.as_path().parent() {
                                    self.local_changedir(parent, true);
                                }
                            }
                        }
                        ' ' => {
                            // Get pwd
                            let wrkdir: PathBuf = match self.client.pwd() {
                                Ok(p) => p,
                                Err(err) => {
                                    self.log(LogLevel::Error, format!("Could not get current remote path: {}", err).as_ref());
                                    return
                                }
                            };
                            // Get files
                            let files: Vec<FsEntry> = self.local.files.clone(); // Otherwise self is borrowed both as mutable and immutable...
                                                                                // Get file at index
                            if let Some(entry) = files.get(self.local.index) {
                                // Call upload
                                self.filetransfer_send(entry, wrkdir.as_path(), None);
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
    fn handle_input_event_mode_explorer_tab_remote(&mut self, ev: &InputEvent) {
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
                    KeyCode::Up => {
                        // Move index up
                        if self.remote.index > 0 {
                            self.remote.index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        // Move index down
                        if self.remote.index + 1 < self.remote.files.len() {
                            self.remote.index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        // Match selected file
                        let files: Vec<FsEntry> = self.remote.files.clone();
                        if let Some(entry) = files.get(self.remote.index) {
                            if let FsEntry::Directory(dir) = entry {
                                // Get current directory
                                self.remote_changedir(dir.abs_path.as_path(), true);
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        // Go to previous directory
                        if let Some(d) = self.remote.popd() {
                            self.remote_changedir(d.as_path(), false);
                        }
                    }
                    KeyCode::Delete => {
                        // Get file at index
                        if let Some(entry) = self.remote.files.get(self.remote.index) {
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
                        'd' | 'D' => {
                            // Make directory
                            // If ctrl is enabled...
                            if key.modifiers.intersects(KeyModifiers::CONTROL) {
                                self.input_mode = InputMode::Popup(PopupType::Input(
                                    String::from("Insert directory name"),
                                    FileTransferActivity::callback_mkdir,
                                ));
                            }
                        }
                        'h' | 'H' => {
                            // Show help
                            // If ctrl is enabled...
                            if key.modifiers.intersects(KeyModifiers::CONTROL) {
                                self.input_mode = InputMode::Popup(PopupType::Help);
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
                        'u' | 'U' => {
                            // Go to parent directory
                            // If ctrl is enabled...
                            if key.modifiers.intersects(KeyModifiers::CONTROL) {
                                // Get pwd
                                match self.client.pwd() {
                                    Ok(path) => {
                                        if let Some(parent) = path.as_path().parent() {
                                            self.remote_changedir(parent, true);
                                        }
                                    }
                                    Err(err) => {
                                        self.input_mode = InputMode::Popup(PopupType::Alert(
                                            Color::Red,
                                            format!("Could not change working directory: {}", err),
                                        ))
                                    }
                                }
                            }
                        }
                        ' ' => {
                            // Get files
                            let files: Vec<FsEntry> = self.remote.files.clone(); // Otherwise self is borrowed both as mutable and immutable...
                                                                                 // Get file at index
                            if let Some(entry) = files.get(self.remote.index) {
                                // Call upload
                                self.filetransfer_recv(
                                    entry,
                                    self.context.as_ref().unwrap().local.pwd().as_path(),
                                    None,
                                );
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
                    KeyCode::Down => { // NOTE: Twisted logic
                        // Decrease log index
                        if self.log_index > 0 {
                            self.log_index = self.log_index - 1;
                        }
                    }
                    KeyCode::Up => { // NOTE: Twisted logic
                        // Increase log index
                        if self.log_index + 1 < self.log_records.len() {
                            self.log_index = self.log_index + 1;
                        }
                    }
                    KeyCode::PageDown => { // NOTE: Twisted logic
                        // Fast decreasing of log index
                        if self.log_index >= records_block {
                            self.log_index = self.log_index - records_block; // Decrease by `records_block` if possible
                        } else {
                            self.log_index = 0; // Set to 0 otherwise
                        }
                    }
                    KeyCode::PageUp => { // NOTE: Twisted logic
                        // Fast increasing of log index
                        if self.log_index + records_block >= self.log_records.len() {
                            // If overflows, set to size
                            self.log_index = self.log_records.len() - 1;
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
    fn handle_input_event_mode_popup(&mut self, ev: &InputEvent, popup: PopupType) {
        match popup {
            PopupType::Alert(_, _) => self.handle_input_event_mode_popup_alert(ev),
            PopupType::Help => self.handle_input_event_mode_popup_help(ev),
            PopupType::Fatal(_) => self.handle_input_event_mode_popup_fatal(ev),
            PopupType::Input(_, cb) => self.handle_input_event_mode_popup_input(ev, cb),
            PopupType::Progress(_) => self.handle_input_event_mode_popup_progress(ev),
            PopupType::Wait(_) => self.handle_input_event_mode_popup_wait(ev),
            PopupType::YesNo(_, yes_cb, no_cb) => {
                self.handle_input_event_mode_popup_yesno(ev, yes_cb, no_cb)
            }
        }
    }

    /// ### handle_input_event_mode_popup_alert
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

    /// ### handle_input_event_mode_popup_help
    ///
    /// Input event handler for popup help
    fn handle_input_event_mode_popup_help(&mut self, ev: &InputEvent) {
        // If enter, close popup
        match ev {
            InputEvent::Key(key) => {
                match key.code {
                    KeyCode::Enter | KeyCode::Esc => {
                        // Set input mode back to explorer
                        self.input_mode = InputMode::Explorer;
                    }
                    _ => { /* Nothing to do */ }
                }
            }
            _ => { /* Nothing to do */ }
        }
    }

    /// ### handle_input_event_mode_popup_fatal
    ///
    /// Input event handler for popup alert
    fn handle_input_event_mode_popup_fatal(&mut self, ev: &InputEvent) {
        // If enter, close popup
        match ev {
            InputEvent::Key(key) => {
                match key.code {
                    KeyCode::Enter => {
                        // Set quit to true; since a fatal error happened
                        self.disconnect();
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
    fn handle_input_event_mode_popup_input(&mut self, ev: &InputEvent, cb: OnInputSubmitCallback) {
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
                        cb(self, input_text);
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
                            DialogYesNoOption::No => no_cb(self),
                            DialogYesNoOption::Yes => yes_cb(self),
                        }
                        // Reset choice option to yes
                        self.choice_opt = DialogYesNoOption::Yes;
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
    fn callback_nothing_to_do(&mut self) {}

    /// ### callback_change_directory
    ///
    /// Callback for GOTO command
    fn callback_change_directory(&mut self, input: String) {
        let dir_path: PathBuf = PathBuf::from(input);
        match self.tab {
            FileExplorerTab::Local => {
                // If path is relative, concat pwd
                let abs_dir_path: PathBuf = match dir_path.is_relative() {
                    true => {
                        let mut d: PathBuf = self.context.as_ref().unwrap().local.pwd();
                        d.push(dir_path);
                        d
                    },
                    false => dir_path
                };
                self.local_changedir(abs_dir_path.as_path(), true);
            }
            FileExplorerTab::Remote => {
                // If path is relative, concat pwd
                let abs_dir_path: PathBuf = match dir_path.is_relative() {
                    true => {
                        match self.client.pwd() {
                            Ok(mut wkrdir) => {
                                wkrdir.push(dir_path);
                                wkrdir
                            }
                            Err(err) => {
                                self.input_mode = InputMode::Popup(PopupType::Alert(Color::Red, format!("Could not retrieve current directory: {}", err)));
                                return;
                            }
                        }
                    },
                    false => dir_path
                };
                self.remote_changedir(abs_dir_path.as_path(), true);
            }
        }
    }

    /// ### callback_mkdir
    ///
    /// Callback for MKDIR command (supports both local and remote)
    fn callback_mkdir(&mut self, input: String) {
        match self.tab {
            FileExplorerTab::Local => {
                match self
                    .context
                    .as_mut()
                    .unwrap()
                    .local
                    .mkdir(PathBuf::from(input.as_str()).as_path())
                {
                    Ok(_) => {
                        // Reload files
                        self.log(
                            LogLevel::Info,
                            format!("Created directory \"{}\"", input).as_ref(),
                        );
                        self.local.files = self.context.as_ref().unwrap().local.list_dir();
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
    fn callback_rename(&mut self, input: String) {
        match self.tab {
            FileExplorerTab::Local => {
                let mut dst_path: PathBuf = PathBuf::from(input);
                // Check if path is relative
                if dst_path.as_path().is_relative() {
                    let mut wrkdir: PathBuf = self.context.as_ref().unwrap().local.pwd();
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
                    match self
                        .context
                        .as_mut()
                        .unwrap()
                        .local
                        .rename(entry, dst_path.as_path())
                    {
                        Ok(_) => {
                            // Reload files
                            self.local_scan(self.context.as_ref().unwrap().local.pwd().as_path());
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
                            if let Ok(path) = self.client.pwd() {
                                self.remote_scan(path.as_path());
                            }
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
    fn callback_delete_fsentry(&mut self) {
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
                    match self.context.as_mut().unwrap().local.remove(entry) {
                        Ok(_) => {
                            // Reload files
                            self.local_scan(self.context.as_ref().unwrap().local.pwd().as_path());
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
    fn callback_save_as(&mut self, input: String) {
        match self.tab {
            FileExplorerTab::Local => {
                // Get pwd
                let wrkdir: PathBuf = match self.client.pwd() {
                    Ok(p) => p,
                    Err(err) => {
                        self.log(LogLevel::Error, format!("Could not get current remote path: {}", err).as_ref());
                        return
                    }
                };
                let files: Vec<FsEntry> = self.local.files.clone();
                // Get file at index
                if let Some(entry) = files.get(self.local.index) {
                    // Call send (upload)
                    self.filetransfer_send(entry, wrkdir.as_path(), Some(input));
                }
            }
            FileExplorerTab::Remote => {
                let files: Vec<FsEntry> = self.remote.files.clone();
                // Get file at index
                if let Some(entry) = files.get(self.remote.index) {
                    // Call receive (download)
                    self.filetransfer_recv(
                        entry,
                        self.context.as_ref().unwrap().local.pwd().as_path(),
                        Some(input),
                    );
                }
            }
        }
    }

    // @! Gfx

    /// ### draw
    ///
    /// Draw UI
    fn draw(&mut self) {
        let mut ctx: Context = self.context.take().unwrap();
        let _ = ctx.terminal.draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(5),  // Header
                        Constraint::Length(20), // Explorer
                        Constraint::Length(16), // Log
                    ]
                    .as_ref(),
                )
                .split(f.size());
            // Create explorer chunks
            let tabs_chunks = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .direction(Direction::Horizontal)
                .split(chunks[1]);
            // Draw header
            f.render_widget(self.draw_header(), chunks[0]);
            // Set localhost state
            let mut localhost_state: ListState = ListState::default();
            localhost_state.select(Some(self.local.index));
            // Set remote state
            let mut remote_state: ListState = ListState::default();
            remote_state.select(Some(self.remote.index));
            // Draw tabs
            f.render_stateful_widget(
                self.draw_local_explorer(),
                tabs_chunks[0],
                &mut localhost_state,
            );
            f.render_stateful_widget(
                self.draw_remote_explorer(),
                tabs_chunks[1],
                &mut remote_state,
            );
            // Set log state
            let mut log_state: ListState = ListState::default();
            log_state.select(Some(self.log_index));
            // Draw log
            f.render_stateful_widget(self.draw_log_list(), chunks[2], &mut log_state);
            // Draw popup
            if let InputMode::Popup(popup) = &self.input_mode {
                // Calculate popup size
                let (width, height): (u16, u16) = match popup {
                    PopupType::Alert(_, _) => (30, 10),
                    PopupType::Fatal(_) => (30, 10),
                    PopupType::Help => (50, 70),
                    PopupType::Input(_, _) => (30, 10),
                    PopupType::Progress(_) => (40, 10),
                    PopupType::Wait(_) => (50, 10),
                    PopupType::YesNo(_, _, _) => (20, 10),
                };
                let popup_area: Rect = self.draw_popup_area(f.size(), width, height);
                f.render_widget(Clear, popup_area); //this clears out the background
                match popup {
                    PopupType::Alert(color, txt) => f.render_widget(
                        self.draw_popup_alert(color.clone(), txt.clone()),
                        popup_area,
                    ),
                    PopupType::Fatal(txt) => {
                        f.render_widget(self.draw_popup_fatal(txt.clone()), popup_area)
                    }
                    PopupType::Help => f.render_widget(self.draw_popup_help(), popup_area),
                    PopupType::Input(txt, _) => {
                        f.render_widget(self.draw_popup_input(txt.clone()), popup_area);
                        // Set cursor
                        f.set_cursor(
                            popup_area.x + self.input_txt.width() as u16 + 1,
                            popup_area.y + 1,
                        )
                    }
                    PopupType::Progress(txt) => {
                        f.render_widget(self.draw_popup_progress(txt.clone()), popup_area)
                    }
                    PopupType::Wait(txt) => {
                        f.render_widget(self.draw_popup_wait(txt.clone()), popup_area)
                    }
                    PopupType::YesNo(txt, _, _) => {
                        f.render_widget(self.draw_popup_yesno(txt.clone()), popup_area)
                    }
                }
            }
        });
        self.context = Some(ctx);
    }

    /// ### draw_header
    ///
    /// Draw header
    fn draw_header(&self) -> Paragraph {
        Paragraph::new(" _____                   ____   ____ ____  \n|_   _|__ _ __ _ __ ___ / ___| / ___|  _ \\ \n  | |/ _ \\ '__| '_ ` _ \\\\___ \\| |   | |_) |\n  | |  __/ |  | | | | | |___) | |___|  __/ \n  |_|\\___|_|  |_| |_| |_|____/ \\____|_|    \n")
            .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
    }

    /// ### draw_local_explorer
    ///
    /// Draw local explorer list
    fn draw_local_explorer(&self) -> List {
        let files: Vec<ListItem> = self
            .local
            .files
            .iter()
            .map(|entry: &FsEntry| ListItem::new(Span::from(format!("{}", entry))))
            .collect();
        List::new(files)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(match self.input_field {
                        InputField::Explorer => match self.tab {
                            FileExplorerTab::Local => Style::default().fg(Color::LightYellow),
                            _ => Style::default(),
                        },
                        _ => Style::default(),
                    })
                    .title("Localhost"),
            )
            .start_corner(Corner::TopLeft)
            .highlight_style(
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )
    }

    /// ### draw_remote_explorer
    ///
    /// Draw remote explorer list
    fn draw_remote_explorer(&self) -> List {
        let files: Vec<ListItem> = self
            .remote
            .files
            .iter()
            .map(|entry: &FsEntry| ListItem::new(Span::from(format!("{}", entry))))
            .collect();
        List::new(files)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(match self.input_field {
                        InputField::Explorer => match self.tab {
                            FileExplorerTab::Remote => Style::default().fg(Color::LightBlue),
                            _ => Style::default(),
                        },
                        _ => Style::default(),
                    })
                    .title(self.params.address.clone()),
            )
            .start_corner(Corner::TopLeft)
            .highlight_style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )
    }

    /// ### draw_log_list
    ///
    /// Draw log list
    fn draw_log_list(&self) -> List {
        let events: Vec<ListItem> = self
            .log_records
            .iter()
            .map(|record: &LogRecord| {
                let s = match record.level {
                    LogLevel::Error => Style::default().fg(Color::Red),
                    LogLevel::Warn => Style::default().fg(Color::Yellow),
                    LogLevel::Info => Style::default().fg(Color::Green),
                };
                let log = Spans::from(vec![
                    Span::from(format!("{}", record.time.format("%Y-%m-%dT%H:%M:%S%Z"))),
                    Span::raw(" ["),
                    Span::styled(
                        format!(
                            "{}",
                            match record.level {
                                LogLevel::Error => "ERROR",
                                LogLevel::Warn => "WARN",
                                LogLevel::Info => "INFO",
                            }
                        ),
                        s,
                    ),
                    Span::raw("]: "),
                    Span::from(record.msg.clone()),
                ]);
                ListItem::new(log)
            })
            .collect();
        List::new(events)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(match self.input_field {
                        InputField::Logs => Style::default().fg(Color::LightGreen),
                        _ => Style::default(),
                    })
                    .title("Log"),
            )
            .start_corner(Corner::BottomLeft)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    }

    /// ### draw_popup_area
    ///
    /// Draw popup area
    fn draw_popup_area(&self, area: Rect, width: u16, height: u16) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - height) / 2),
                    Constraint::Percentage(height),
                    Constraint::Percentage((100 - height) / 2),
                ]
                .as_ref(),
            )
            .split(area);
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - width) / 2),
                    Constraint::Percentage(width),
                    Constraint::Percentage((100 - width) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }

    /// ### draw_popup_alert
    ///
    /// Draw alert popup
    fn draw_popup_alert(&self, color: Color, text: String) -> Paragraph {
        Paragraph::new(text)
            .style(Style::default().fg(color))
            .block(Block::default().borders(Borders::ALL).title("Alert"))
    }

    /// ### draw_popup_fatal
    ///
    /// Draw fatal error popup
    fn draw_popup_fatal(&self, text: String) -> Paragraph {
        Paragraph::new(text)
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Fatal error"))
    }
    /// ### draw_popup_input
    ///
    /// Draw input popup
    fn draw_popup_input(&self, text: String) -> Paragraph {
        Paragraph::new(self.input_txt.as_ref())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title(text))
    }

    /// ### draw_popup_progress
    ///
    /// Draw progress popup
    fn draw_popup_progress(&self, text: String) -> Gauge {
        let label = format!("{:.2}%", self.transfer_progress);
        Gauge::default()
            .block(Block::default().borders(Borders::ALL).title(text))
            .gauge_style(
                Style::default()
                    .fg(Color::Magenta)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .label(label)
            .ratio(self.transfer_progress / 100.0)
    }

    /// ### draw_popup_wait
    ///
    /// Draw wait popup
    fn draw_popup_wait(&self, text: String) -> Paragraph {
        Paragraph::new(text)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title("Please wait"))
    }

    /// ### draw_popup_yesno
    ///
    /// Draw yes/no select popup
    fn draw_popup_yesno(&self, text: String) -> Tabs {
        let choices: Vec<Spans> = vec![Spans::from("Yes"), Spans::from("No")];
        let index: usize = match self.choice_opt {
            DialogYesNoOption::Yes => 0,
            DialogYesNoOption::No => 1,
        };
        Tabs::new(choices)
            .block(Block::default().borders(Borders::ALL).title(text))
            .select(index)
            .style(Style::default())
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            )
    }

    /// ### draw_footer
    ///
    /// Draw authentication page footer
    fn draw_popup_help(&self) -> List {
        // Write header
        let cmds: Vec<ListItem> = vec![
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<ESC>         ",
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("quit"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<TAB>         ",
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("change input field"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<RIGHT/LEFT>  ",
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("change explorer tab"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<ENTER>       ",
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("enter directory"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<SPACE>       ",
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("upload/download file"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<CTRL+D>      ",
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("make directory"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<CTRL+G>      ",
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("goto path"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<CTRL+R>      ",
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("rename file"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<CTRL+U>      ",
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("go to parent directory"),
            ])),
            ListItem::new(Spans::from(vec![
                Span::styled(
                    "<CANC>        ",
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("delete file"),
            ])),
        ];
        List::new(cmds)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default())
                    .title("Help"),
            )
            .start_corner(Corner::TopLeft)
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
    fn on_create(&mut self, context: Context) {
        // Set context
        self.context = Some(context);
        // Clear terminal
        let _ = self.context.as_mut().unwrap().terminal.clear();
        // Put raw mode on enabled
        let _ = enable_raw_mode();
        // Get files at current wd
        self.local_scan(self.context.as_ref().unwrap().local.pwd().as_path());
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
        let is_explorer_mode: bool = match self.input_mode {
            InputMode::Explorer => true,
            _ => false,
        };
        // Check if connected
        if !self.client.is_connected() && is_explorer_mode {
            // Set init state to connecting popup
            self.input_mode = InputMode::Popup(PopupType::Wait(format!(
                "Connecting to {}:{}...",
                self.params.address, self.params.port
            )));
            // Force ui draw
            self.draw();
            // Connect to remote
            self.connect();
        }
        // Handle input events
        if let Ok(event) = self.context.as_ref().unwrap().input_hnd.read_event() {
            // Iterate over input events
            if let Some(event) = event {
                self.handle_input_event(&event);
            }
        }
        // @! draw interface
        self.draw();
    }

    /// ### on_destroy
    ///
    /// `on_destroy` is the function which cleans up runtime variables and data before terminating the activity.
    /// This function must be called once before terminating the activity.
    fn on_destroy(&mut self) -> Option<Context> {
        // Disable raw mode
        let _ = disable_raw_mode();
        // Disconnect client
        if self.client.is_connected() {
            let _ = self.client.disconnect();
        }
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
