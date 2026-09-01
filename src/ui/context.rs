//! ## Context
//!
//! `Context` is the module which provides all the functionalities related to the UI data holder, called Context

use ssh2_config::SshConfig;
// Locals
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalAdapter};

use super::store::Store;
use crate::filetransfer::{FileTransferParams, HostBridgeParams};
use crate::system::bookmarks_client::BookmarksClient;
use crate::system::config_client::ConfigClient;
use crate::system::theme_provider::ThemeProvider;

/// Context holds data structures shared by the activities
pub struct Context {
    /// Parameters used to build the host bridge.
    host_bridge_params: Option<HostBridgeParams>,
    /// Parameters used to build the remote file transfer client.
    remote_params: Option<FileTransferParams>,
    /// Client for persistent bookmarks, when initialization succeeded.
    bookmarks_client: Option<BookmarksClient>,
    /// Client for persisted application configuration.
    config_client: ConfigClient,
    /// SSH configuration parsed once during application startup.
    ssh_config: Option<SshConfig>,
    /// Shared state managed by UI activities.
    pub(crate) store: Store,
    /// Terminal adapter used to render the user interface.
    pub(crate) terminal: CrosstermTerminalAdapter,
    /// Provider for the active user interface theme.
    theme_provider: ThemeProvider,
    /// Error pending display to the user.
    error: Option<String>,
}

impl Context {
    /// Instantiates a new Context
    pub fn new(
        bookmarks_client: Option<BookmarksClient>,
        config_client: ConfigClient,
        theme_provider: ThemeProvider,
        ssh_config: Option<SshConfig>,
        error: Option<String>,
    ) -> Context {
        let mut terminal = CrosstermTerminalAdapter::new().expect("Could not initialize terminal");
        terminal
            .enable_raw_mode()
            .expect("Could not enable terminal raw mode");
        terminal
            .enter_alternate_screen()
            .expect("Could not enter alternate screen");
        let _ = terminal.disable_mouse_capture();

        Context {
            bookmarks_client,
            config_client,
            host_bridge_params: None,
            remote_params: None,
            ssh_config,
            store: Store::init(),
            terminal,
            theme_provider,
            error,
        }
    }

    // -- getters

    pub fn remote_params(&self) -> Option<&FileTransferParams> {
        self.remote_params.as_ref()
    }

    pub fn host_bridge_params(&self) -> Option<&HostBridgeParams> {
        self.host_bridge_params.as_ref()
    }

    pub fn bookmarks_client(&self) -> Option<&BookmarksClient> {
        self.bookmarks_client.as_ref()
    }

    pub fn bookmarks_client_mut(&mut self) -> Option<&mut BookmarksClient> {
        self.bookmarks_client.as_mut()
    }

    pub fn config(&self) -> &ConfigClient {
        &self.config_client
    }

    pub fn config_mut(&mut self) -> &mut ConfigClient {
        &mut self.config_client
    }

    /// Returns the SSH configuration parsed during application startup.
    pub fn ssh_config(&self) -> Option<&SshConfig> {
        self.ssh_config.as_ref()
    }

    pub(crate) fn store(&self) -> &Store {
        &self.store
    }

    pub(crate) fn store_mut(&mut self) -> &mut Store {
        &mut self.store
    }

    pub fn theme_provider(&self) -> &ThemeProvider {
        &self.theme_provider
    }

    pub fn theme_provider_mut(&mut self) -> &mut ThemeProvider {
        &mut self.theme_provider
    }

    pub fn terminal(&mut self) -> &mut CrosstermTerminalAdapter {
        &mut self.terminal
    }

    // -- setter

    pub fn set_remote_params(&mut self, params: FileTransferParams) {
        self.remote_params = Some(params);
    }

    pub fn set_host_bridge_params(&mut self, params: HostBridgeParams) {
        self.host_bridge_params = Some(params);
    }

    // -- error

    /// Get error message and remove it from the context
    pub fn error(&mut self) -> Option<String> {
        self.error.take()
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if let Err(err) = self.terminal.restore() {
            error!("Could not restore terminal: {err}");
        }
    }
}
