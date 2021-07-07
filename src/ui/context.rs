//! ## Context
//!
//! `Context` is the module which provides all the functionalities related to the UI data holder, called Context

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
// Locals
use super::input::InputHandler;
use super::store::Store;
use crate::filetransfer::FileTransferProtocol;
use crate::system::config_client::ConfigClient;
use crate::system::theme_provider::ThemeProvider;

// Includes
use crossterm::event::DisableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use std::io::{stdout, Stdout};
use std::path::PathBuf;
use tuirealm::tui::backend::CrosstermBackend;
use tuirealm::tui::Terminal;

/// ## Context
///
/// Context holds data structures used by the ui
pub struct Context {
    pub ft_params: Option<FileTransferParams>,
    pub(crate) config_client: Option<ConfigClient>,
    pub(crate) store: Store,
    pub(crate) input_hnd: InputHandler,
    pub(crate) terminal: Terminal<CrosstermBackend<Stdout>>,
    pub(crate) theme_provider: ThemeProvider,
    error: Option<String>,
}

/// ### FileTransferParams
///
/// Holds connection parameters for file transfers
pub struct FileTransferParams {
    pub address: String,
    pub port: u16,
    pub protocol: FileTransferProtocol,
    pub username: Option<String>,
    pub password: Option<String>,
    pub entry_directory: Option<PathBuf>,
}

impl Context {
    /// ### new
    ///
    /// Instantiates a new Context
    pub fn new(
        config_client: Option<ConfigClient>,
        theme_provider: ThemeProvider,
        error: Option<String>,
    ) -> Context {
        // Create terminal
        let mut stdout = stdout();
        assert!(execute!(stdout, EnterAlternateScreen).is_ok());
        Context {
            ft_params: None,
            config_client,
            store: Store::init(),
            input_hnd: InputHandler::new(),
            terminal: Terminal::new(CrosstermBackend::new(stdout)).unwrap(),
            theme_provider,
            error,
        }
    }

    /// ### set_error
    ///
    /// Set context error
    pub fn set_error(&mut self, err: String) {
        self.error = Some(err);
    }

    /// ### get_error
    ///
    /// Get error message and remove it from the context
    pub fn get_error(&mut self) -> Option<String> {
        self.error.take()
    }

    /// ### enter_alternate_screen
    ///
    /// Enter alternate screen (gui window)
    #[cfg(not(target_os = "windows"))]
    pub fn enter_alternate_screen(&mut self) {
        match execute!(
            self.terminal.backend_mut(),
            EnterAlternateScreen,
            DisableMouseCapture
        ) {
            Err(err) => error!("Failed to enter alternate screen: {}", err),
            Ok(_) => info!("Entered alternate screen"),
        }
    }

    /// ### leave_alternate_screen
    ///
    /// Go back to normal screen (gui window)
    pub fn leave_alternate_screen(&mut self) {
        match execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ) {
            Err(err) => error!("Failed to leave alternate screen: {}", err),
            Ok(_) => info!("Left alternate screen"),
        }
    }

    /// ### clear_screen
    ///
    /// Clear terminal screen
    pub fn clear_screen(&mut self) {
        match self.terminal.clear() {
            Err(err) => error!("Failed to clear screen: {}", err),
            Ok(_) => info!("Cleared screen"),
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        // Re-enable terminal stuff
        self.leave_alternate_screen();
    }
}

impl Default for FileTransferParams {
    fn default() -> Self {
        Self {
            address: String::new(),
            port: 22,
            protocol: FileTransferProtocol::Sftp,
            username: None,
            password: None,
            entry_directory: None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_ui_context_ft_params() {
        let params: FileTransferParams = FileTransferParams::default();
        assert_eq!(params.address.as_str(), "");
        assert_eq!(params.port, 22);
        assert_eq!(params.protocol, FileTransferProtocol::Sftp);
        assert!(params.username.is_none());
        assert!(params.password.is_none());
    }
}
