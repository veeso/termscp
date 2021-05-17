//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

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
// This module is split into files, cause it's just too big
pub(self) mod actions;
pub(self) mod browser;
pub(self) mod misc;
pub(self) mod session;
pub(self) mod update;
pub(self) mod view;

// Dependencies
extern crate chrono;
extern crate crossterm;
extern crate textwrap;
extern crate tuirealm;

// locals
use super::{Activity, Context, ExitReason};
use crate::filetransfer::ftp_transfer::FtpFileTransfer;
use crate::filetransfer::scp_transfer::ScpFileTransfer;
use crate::filetransfer::sftp_transfer::SftpFileTransfer;
use crate::filetransfer::{FileTransfer, FileTransferProtocol};
use crate::fs::explorer::FileExplorer;
use crate::fs::FsEntry;
use crate::host::Localhost;
use crate::system::config_client::ConfigClient;
use browser::Browser;

// Includes
use chrono::{DateTime, Local};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::Instant;
use tuirealm::View;

// -- Storage keys

const STORAGE_EXPLORER_WIDTH: &str = "FILETRANSFER_EXPLORER_WIDTH";

// -- components

const COMPONENT_EXPLORER_LOCAL: &str = "EXPLORER_LOCAL";
const COMPONENT_EXPLORER_REMOTE: &str = "EXPLORER_REMOTE";
const COMPONENT_EXPLORER_FIND: &str = "EXPLORER_FIND";
const COMPONENT_LOG_BOX: &str = "LOG_BOX";
const COMPONENT_PROGRESS_BAR: &str = "PROGRESS_BAR";
const COMPONENT_TEXT_ERROR: &str = "TEXT_ERROR";
const COMPONENT_TEXT_FATAL: &str = "TEXT_FATAL";
const COMPONENT_TEXT_HELP: &str = "TEXT_HELP";
const COMPONENT_TEXT_WAIT: &str = "TEXT_WAIT";
const COMPONENT_INPUT_COPY: &str = "INPUT_COPY";
const COMPONENT_INPUT_EXEC: &str = "INPUT_EXEC";
const COMPONENT_INPUT_FIND: &str = "INPUT_FIND";
const COMPONENT_INPUT_GOTO: &str = "INPUT_GOTO";
const COMPONENT_INPUT_MKDIR: &str = "INPUT_MKDIR";
const COMPONENT_INPUT_NEWFILE: &str = "INPUT_NEWFILE";
const COMPONENT_INPUT_RENAME: &str = "INPUT_RENAME";
const COMPONENT_INPUT_SAVEAS: &str = "INPUT_SAVEAS";
const COMPONENT_RADIO_DELETE: &str = "RADIO_DELETE";
const COMPONENT_RADIO_DISCONNECT: &str = "RADIO_DISCONNECT";
const COMPONENT_RADIO_QUIT: &str = "RADIO_QUIT";
const COMPONENT_RADIO_SORTING: &str = "RADIO_SORTING";
const COMPONENT_SPAN_STATUS_BAR: &str = "STATUS_BAR";
const COMPONENT_LIST_FILEINFO: &str = "LIST_FILEINFO";

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
    pub fn new(level: LogLevel, msg: String) -> LogRecord {
        LogRecord {
            time: Local::now(),
            level,
            msg,
        }
    }
}

/// ### TransferStates
///
/// TransferStates contains the states related to the transfer process
struct TransferStates {
    pub progress: f64,        // Current read/write progress (percentage)
    pub started: Instant,     // Instant the transfer process started
    pub aborted: bool,        // Describes whether the transfer process has been aborted
    pub bytes_written: usize, // Bytes written during transfer
    pub bytes_total: usize,   // Total bytes to write
}

impl TransferStates {
    /// ### new
    ///
    /// Instantiates a new transfer states
    pub fn new() -> TransferStates {
        TransferStates {
            progress: 0.0,
            started: Instant::now(),
            aborted: false,
            bytes_written: 0,
            bytes_total: 0,
        }
    }

    /// ### reset
    ///
    /// Re-intiialize transfer states
    pub fn reset(&mut self) {
        self.progress = 0.0;
        self.started = Instant::now();
        self.aborted = false;
        self.bytes_written = 0;
        self.bytes_total = 0;
    }

    /// ### set_progress
    ///
    /// Calculate progress percentage based on current progress
    pub fn set_progress(&mut self, w: usize, sz: usize) {
        self.bytes_written = w;
        self.bytes_total = sz;
        let mut prog: f64 = ((self.bytes_written as f64) * 100.0) / (self.bytes_total as f64);
        // Check value
        if prog > 100.0 {
            prog = 100.0;
        } else if prog < 0.0 {
            prog = 0.0;
        }
        self.progress = prog;
    }

    /// ### byte_per_second
    ///
    /// Calculate bytes per second
    pub fn bytes_per_second(&self) -> u64 {
        // bytes_written : elapsed_secs = x : 1
        let elapsed_secs: u64 = self.started.elapsed().as_secs();
        match elapsed_secs {
            0 => match self.bytes_written == self.bytes_total {
                // NOTE: would divide by 0 :D
                true => self.bytes_total as u64, // Download completed in less than 1 second
                false => 0,                      // 0 B/S
            },
            _ => self.bytes_written as u64 / elapsed_secs,
        }
    }
}

impl Default for TransferStates {
    fn default() -> Self {
        Self::new()
    }
}

/// ## FileTransferActivity
///
/// FileTransferActivity is the data holder for the file transfer activity
pub struct FileTransferActivity {
    exit_reason: Option<ExitReason>,  // Exit reason
    context: Option<Context>,         // Context holder
    view: View,                       // View
    host: Localhost,                  // Localhost
    client: Box<dyn FileTransfer>,    // File transfer client
    browser: Browser,                 // Browser
    log_records: VecDeque<LogRecord>, // Log records
    transfer: TransferStates,         // Transfer states
}

impl FileTransferActivity {
    /// ### new
    ///
    /// Instantiates a new FileTransferActivity
    pub fn new(host: Localhost, protocol: FileTransferProtocol) -> FileTransferActivity {
        // Get config client
        let config_client: Option<ConfigClient> = Self::init_config_client();
        FileTransferActivity {
            exit_reason: None,
            context: None,
            view: View::init(),
            host,
            client: match protocol {
                FileTransferProtocol::Sftp => Box::new(SftpFileTransfer::new(
                    Self::make_ssh_storage(config_client.as_ref()),
                )),
                FileTransferProtocol::Ftp(ftps) => Box::new(FtpFileTransfer::new(ftps)),
                FileTransferProtocol::Scp => Box::new(ScpFileTransfer::new(
                    Self::make_ssh_storage(config_client.as_ref()),
                )),
            },
            browser: Browser::new(config_client.as_ref()),
            log_records: VecDeque::with_capacity(256), // 256 events is enough I guess
            transfer: TransferStates::default(),
        }
    }

    pub(crate) fn local(&self) -> &FileExplorer {
        self.browser.local()
    }

    pub(crate) fn local_mut(&mut self) -> &mut FileExplorer {
        self.browser.local_mut()
    }

    pub(crate) fn remote(&self) -> &FileExplorer {
        self.browser.remote()
    }

    pub(crate) fn remote_mut(&mut self) -> &mut FileExplorer {
        self.browser.remote_mut()
    }

    pub(crate) fn found(&self) -> Option<&FileExplorer> {
        self.browser.found()
    }

    pub(crate) fn found_mut(&mut self) -> Option<&mut FileExplorer> {
        self.browser.found_mut()
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
        debug!("Initializing activity...");
        // Set context
        self.context = Some(context);
        // Clear terminal
        self.context.as_mut().unwrap().clear_screen();
        // Put raw mode on enabled
        if let Err(err) = enable_raw_mode() {
            error!("Failed to enter raw mode: {}", err);
        }
        // Set working directory
        let pwd: PathBuf = self.host.pwd();
        // Get files at current wd
        self.local_scan(pwd.as_path());
        self.local_mut().wrkdir = pwd;
        debug!("Read working directory");
        // Configure text editor
        self.setup_text_editor();
        debug!("Setup text editor");
        // init view
        self.init();
        debug!("Initialized view");
        // Verify error state from context
        if let Some(err) = self.context.as_mut().unwrap().get_error() {
            error!("Fatal error on create: {}", err);
            self.mount_fatal(&err);
        }
        info!("Created FileTransferActivity");
    }

    /// ### on_draw
    ///
    /// `on_draw` is the function which draws the graphical interface.
    /// This function must be called at each tick to refresh the interface
    fn on_draw(&mut self) {
        // Should ui actually be redrawned?
        let mut redraw: bool = false;
        // Context must be something
        if self.context.is_none() {
            return;
        }
        // Check if connected (popup must be None, otherwise would try reconnecting in loop in case of error)
        if !self.client.is_connected() && self.view.get_props(COMPONENT_TEXT_FATAL).is_none() {
            let params = self.context.as_ref().unwrap().ft_params.as_ref().unwrap();
            info!(
                "Client is not connected to remote; connecting to {}:{}",
                params.address, params.port
            );
            let msg: String = format!("Connecting to {}:{}...", params.address, params.port);
            // Set init state to connecting popup
            self.mount_wait(msg.as_str());
            // Force ui draw
            self.view();
            // Connect to remote
            self.connect();
            // Redraw
            redraw = true;
        }
        // Handle input events (if false, becomes true; otherwise remains true)
        redraw |= self.read_input_event();
        // @! draw interface
        if redraw {
            self.view();
        }
    }

    /// ### will_umount
    ///
    /// `will_umount` is the method which must be able to report to the activity manager, whether
    /// the activity should be terminated or not.
    /// If not, the call will return `None`, otherwise return`Some(ExitReason)`
    fn will_umount(&self) -> Option<&ExitReason> {
        self.exit_reason.as_ref()
    }

    /// ### on_destroy
    ///
    /// `on_destroy` is the function which cleans up runtime variables and data before terminating the activity.
    /// This function must be called once before terminating the activity.
    fn on_destroy(&mut self) -> Option<Context> {
        // Disable raw mode
        if let Err(err) = disable_raw_mode() {
            error!("Failed to disable raw mode: {}", err);
        }
        // Disconnect client
        if self.client.is_connected() {
            let _ = self.client.disconnect();
        }
        // Clear terminal and return
        match self.context.take() {
            Some(mut ctx) => {
                ctx.clear_screen();
                Some(ctx)
            }
            None => None,
        }
    }
}
