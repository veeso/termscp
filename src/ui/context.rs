//! ## Context
//!
//! `Context` is the module which provides all the functionalities related to the UI data holder, called Context

// Locals
use tuirealm::terminal::TerminalBridge;

use super::store::Store;
use crate::filetransfer::FileTransferParams;
use crate::system::bookmarks_client::BookmarksClient;
use crate::system::config_client::ConfigClient;
use crate::system::theme_provider::ThemeProvider;

/// Context holds data structures shared by the activities
pub struct Context {
    ft_params: Option<FileTransferParams>,
    bookmarks_client: Option<BookmarksClient>,
    config_client: ConfigClient,
    pub(crate) store: Store,
    pub(crate) terminal: TerminalBridge,
    theme_provider: ThemeProvider,
    error: Option<String>,
}

impl Context {
    /// Instantiates a new Context
    pub fn new(
        bookmarks_client: Option<BookmarksClient>,
        config_client: ConfigClient,
        theme_provider: ThemeProvider,
        error: Option<String>,
    ) -> Context {
        let mut ctx = Context {
            bookmarks_client,
            config_client,
            ft_params: None,
            store: Store::init(),
            terminal: TerminalBridge::new().expect("Could not initialize terminal"),
            theme_provider,
            error,
        };

        // Init terminal state
        let _ = ctx.terminal.enable_raw_mode();
        let _ = ctx.terminal.enter_alternate_screen();

        ctx
    }

    // -- getters

    pub fn ft_params(&self) -> Option<&FileTransferParams> {
        self.ft_params.as_ref()
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

    pub fn terminal(&mut self) -> &mut TerminalBridge {
        &mut self.terminal
    }

    // -- setter

    pub fn set_ftparams(&mut self, params: FileTransferParams) {
        self.ft_params = Some(params);
    }

    // -- error

    /// Set context error
    pub fn set_error(&mut self, err: String) {
        self.error = Some(err);
    }

    /// Get error message and remove it from the context
    pub fn error(&mut self) -> Option<String> {
        self.error.take()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        // Re-enable terminal stuff
        let _ = self.terminal.disable_raw_mode();
        let _ = self.terminal.leave_alternate_screen();
    }
}
