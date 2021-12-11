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
use super::store::Store;
use crate::filetransfer::FileTransferParams;
use crate::system::config_client::ConfigClient;
use crate::system::theme_provider::ThemeProvider;

use tuirealm::terminal::TerminalBridge;

/// Context holds data structures shared by the activities
pub struct Context {
    ft_params: Option<FileTransferParams>,
    config_client: ConfigClient,
    pub(crate) store: Store,
    pub(crate) terminal: TerminalBridge,
    theme_provider: ThemeProvider,
    error: Option<String>,
}

impl Context {
    /// Instantiates a new Context
    pub fn new(
        config_client: ConfigClient,
        theme_provider: ThemeProvider,
        error: Option<String>,
    ) -> Context {
        Context {
            ft_params: None,
            config_client,
            store: Store::init(),
            terminal: TerminalBridge::new().expect("Could not initialize terminal"),
            theme_provider,
            error,
        }
    }

    // -- getters

    pub fn ft_params(&self) -> Option<&FileTransferParams> {
        self.ft_params.as_ref()
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
        let _ = self.terminal.clear_screen();
    }
}
