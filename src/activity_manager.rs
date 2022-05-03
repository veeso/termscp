//! ## ActivityManager
//!
//! `activity_manager` is the module which provides run methods and handling for activities

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
// Deps
use crate::filetransfer::FileTransferParams;
use crate::host::{HostError, Localhost};
use crate::system::bookmarks_client::BookmarksClient;
use crate::system::config_client::ConfigClient;
use crate::system::environment;
use crate::system::theme_provider::ThemeProvider;
use crate::ui::activities::{
    auth::AuthActivity, filetransfer::FileTransferActivity, setup::SetupActivity, Activity,
    ExitReason,
};
use crate::ui::context::Context;

// Namespaces
use std::path::{Path, PathBuf};
use std::time::Duration;

/// ### NextActivity
///
/// NextActivity identifies the next identity to run once the current has ended
pub enum NextActivity {
    Authentication,
    FileTransfer,
    SetupActivity,
}

/// ### ActivityManager
///
/// The activity manager takes care of running activities and handling them until the application has ended
pub struct ActivityManager {
    context: Option<Context>,
    ticks: Duration,
    local_dir: PathBuf,
}

impl ActivityManager {
    /// Initializes a new Activity Manager
    pub fn new(local_dir: &Path, ticks: Duration) -> Result<ActivityManager, HostError> {
        // Prepare Context
        // Initialize configuration client
        let (config_client, error): (ConfigClient, Option<String>) =
            match Self::init_config_client() {
                Ok(cli) => (cli, None),
                Err(err) => {
                    error!("Failed to initialize config client: {}", err);
                    (ConfigClient::degraded(), Some(err))
                }
            };
        let (bookmarks_client, error) = match Self::init_bookmarks_client() {
            Ok(cli) => (cli, None),
            Err(err) => (None, Some(err)),
        };
        let theme_provider: ThemeProvider = Self::init_theme_provider();
        let ctx: Context = Context::new(bookmarks_client, config_client, theme_provider, error);
        Ok(ActivityManager {
            context: Some(ctx),
            local_dir: local_dir.to_path_buf(),
            ticks,
        })
    }

    /// Set file transfer params
    pub fn set_filetransfer_params(&mut self, params: FileTransferParams) {
        // Put params into the context
        self.context.as_mut().unwrap().set_ftparams(params);
    }

    /// Resolve provided bookmark name and set it as file transfer params.
    /// Returns error if bookmark is not found
    pub fn resolve_bookmark_name(
        &mut self,
        bookmark_name: &str,
        password: Option<&str>,
    ) -> Result<(), String> {
        if let Some(bookmarks_client) = self.context.as_mut().unwrap().bookmarks_client_mut() {
            match bookmarks_client.get_bookmark(bookmark_name) {
                None => Err(format!(
                    r#"Could not resolve bookmark name: "{}" no such bookmark"#,
                    bookmark_name
                )),
                Some(params) => {
                    self.context.as_mut().unwrap().set_ftparams(params);
                    // Set password if provided
                    // TODO: move read password from tty to utils and create a method to check whether ft params require password
                    Ok(())
                }
            }
        } else {
            Err(String::from(
                "Could not resolve bookmark name: bookmarks client not initialized",
            ))
        }
    }

    ///
    /// Loop for activity manager. You need to provide the activity to start with
    /// Returns the exitcode
    pub fn run(&mut self, launch_activity: NextActivity) {
        let mut current_activity: Option<NextActivity> = Some(launch_activity);
        loop {
            current_activity = match current_activity {
                Some(activity) => match activity {
                    NextActivity::Authentication => self.run_authentication(),
                    NextActivity::FileTransfer => self.run_filetransfer(),
                    NextActivity::SetupActivity => self.run_setup(),
                },
                None => break, // Exit
            }
        }
        // Drop context
        drop(self.context.take());
    }

    // -- Activity Loops

    /// Loop for Authentication activity.
    /// Returns when activity terminates.
    /// Returns the next activity to run
    fn run_authentication(&mut self) -> Option<NextActivity> {
        info!("Starting AuthActivity...");
        // Prepare activity
        let mut activity: AuthActivity = AuthActivity::new(self.ticks);
        // Prepare result
        let result: Option<NextActivity>;
        // Get context
        let ctx: Context = match self.context.take() {
            Some(ctx) => ctx,
            None => {
                error!("Failed to start AuthActivity: context is None");
                return None;
            }
        };
        // Create activity
        activity.on_create(ctx);
        loop {
            // Draw activity
            activity.on_draw();
            // Check if has to be terminated
            if let Some(exit_reason) = activity.will_umount() {
                match exit_reason {
                    ExitReason::Quit => {
                        info!("AuthActivity terminated due to 'Quit'");
                        result = None;
                        break;
                    }
                    ExitReason::EnterSetup => {
                        // User requested activity
                        info!("AuthActivity terminated due to 'EnterSetup'");
                        result = Some(NextActivity::SetupActivity);
                        break;
                    }
                    ExitReason::Connect => {
                        // User submitted, set next activity
                        info!("AuthActivity terminated due to 'Connect'");
                        result = Some(NextActivity::FileTransfer);
                        break;
                    }
                    _ => { /* Nothing to do */ }
                }
            }
        }
        // Destroy activity
        self.context = activity.on_destroy();
        info!("AuthActivity destroyed");
        result
    }

    /// Loop for FileTransfer activity.
    /// Returns when activity terminates.
    /// Returns the next activity to run
    fn run_filetransfer(&mut self) -> Option<NextActivity> {
        info!("Starting FileTransferActivity");
        // Get context
        let mut ctx: Context = match self.context.take() {
            Some(ctx) => ctx,
            None => {
                error!("Failed to start FileTransferActivity: context is None");
                return None;
            }
        };
        // If ft params is None, return None
        let ft_params: &FileTransferParams = match ctx.ft_params() {
            Some(ft_params) => ft_params,
            None => {
                error!("Failed to start FileTransferActivity: file transfer params is None");
                return None;
            }
        };
        // Prepare activity
        let host: Localhost = match Localhost::new(self.local_dir.clone()) {
            Ok(host) => host,
            Err(err) => {
                // Set error in context
                error!("Failed to initialize localhost: {}", err);
                ctx.set_error(format!("Could not initialize localhost: {}", err));
                return None;
            }
        };
        let mut activity: FileTransferActivity =
            FileTransferActivity::new(host, ft_params, self.ticks);
        // Prepare result
        let result: Option<NextActivity>;
        // Create activity
        activity.on_create(ctx);
        loop {
            // Draw activity
            activity.on_draw();
            // Check if has to be terminated
            if let Some(exit_reason) = activity.will_umount() {
                match exit_reason {
                    ExitReason::Quit => {
                        info!("FileTransferActivity terminated due to 'Quit'");
                        result = None;
                        break;
                    }
                    ExitReason::Disconnect => {
                        // User disconnected, set next activity to authentication
                        info!("FileTransferActivity terminated due to 'Authentication'");
                        result = Some(NextActivity::Authentication);
                        break;
                    }
                    _ => { /* Nothing to do */ }
                }
            }
        }
        // Destroy activity
        self.context = activity.on_destroy();
        result
    }

    /// `SetupActivity` run loop.
    /// Returns when activity terminates.
    /// Returns the next activity to run
    fn run_setup(&mut self) -> Option<NextActivity> {
        // Prepare activity
        let mut activity: SetupActivity = SetupActivity::new(self.ticks);
        // Get context
        let ctx: Context = match self.context.take() {
            Some(ctx) => ctx,
            None => {
                error!("Failed to start SetupActivity: context is None");
                return None;
            }
        };
        // Create activity
        activity.on_create(ctx);
        loop {
            // Draw activity
            activity.on_draw();
            // Check if activity has terminated
            if let Some(ExitReason::Quit) = activity.will_umount() {
                info!("SetupActivity terminated due to 'Quit'");
                break;
            }
        }
        // Destroy activity
        self.context = activity.on_destroy();
        // This activity always returns to AuthActivity
        Some(NextActivity::Authentication)
    }

    // -- misc

    fn init_bookmarks_client() -> Result<Option<BookmarksClient>, String> {
        // Get config dir
        match environment::init_config_dir() {
            Ok(path) => {
                // If some configure client, otherwise do nothing; don't bother users telling them that bookmarks are not supported on their system.
                if let Some(config_dir_path) = path {
                    let bookmarks_file: PathBuf =
                        environment::get_bookmarks_paths(config_dir_path.as_path());
                    // Initialize client
                    BookmarksClient::new(bookmarks_file.as_path(), config_dir_path.as_path(), 16)
                        .map(Option::Some)
                        .map_err(|e| {
                            format!(
                                "Could not initialize bookmarks (at \"{}\", \"{}\"): {}",
                                bookmarks_file.display(),
                                config_dir_path.display(),
                                e
                            )
                        })
                } else {
                    Ok(None)
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Initialize configuration client
    fn init_config_client() -> Result<ConfigClient, String> {
        // Get config dir
        match environment::init_config_dir() {
            Ok(config_dir) => {
                match config_dir {
                    Some(config_dir) => {
                        // Get config client paths
                        let (config_path, ssh_dir): (PathBuf, PathBuf) =
                            environment::get_config_paths(config_dir.as_path());
                        match ConfigClient::new(config_path.as_path(), ssh_dir.as_path()) {
                            Ok(cli) => Ok(cli),
                            Err(err) => Err(format!("Could not read configuration: {}", err)),
                        }
                    }
                    None => Err(String::from(
                        "Your system doesn't provide a configuration directory",
                    )),
                }
            }
            Err(err) => Err(format!(
                "Could not initialize configuration directory: {}",
                err
            )),
        }
    }

    fn init_theme_provider() -> ThemeProvider {
        match environment::init_config_dir() {
            Ok(config_dir) => {
                match config_dir {
                    Some(config_dir) => {
                        // Get config client paths
                        let theme_path: PathBuf = environment::get_theme_path(config_dir.as_path());
                        match ThemeProvider::new(theme_path.as_path()) {
                            Ok(provider) => provider,
                            Err(err) => {
                                error!("Could not initialize theme provider with file '{}': {}; using theme provider in degraded mode", theme_path.display(), err);
                                ThemeProvider::degraded()
                            }
                        }
                    }
                    None => {
                        error!("This system doesn't provide a configuration directory; using theme provider in degraded mode");
                        ThemeProvider::degraded()
                    }
                }
            }
            Err(err) => {
                error!("Could not initialize configuration directory: {}; using theme provider in degraded mode", err);
                ThemeProvider::degraded()
            }
        }
    }
}
