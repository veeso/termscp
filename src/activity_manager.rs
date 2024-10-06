//! ## ActivityManager
//!
//! `activity_manager` is the module which provides run methods and handling for activities

// Deps
// Namespaces
use std::path::PathBuf;
use std::time::Duration;

use remotefs_ssh::SshKeyStorage as SshKeyStorageTrait;

use crate::filetransfer::{FileTransferParams, FileTransferProtocol, HostBridgeParams};
use crate::host::HostError;
use crate::system::bookmarks_client::BookmarksClient;
use crate::system::config_client::ConfigClient;
use crate::system::environment;
use crate::system::sshkey_storage::SshKeyStorage;
use crate::system::theme_provider::ThemeProvider;
use crate::ui::activities::auth::AuthActivity;
use crate::ui::activities::filetransfer::FileTransferActivity;
use crate::ui::activities::setup::SetupActivity;
use crate::ui::activities::{Activity, ExitReason};
use crate::ui::context::Context;
use crate::utils::{fmt, tty};

/// NextActivity identifies the next identity to run once the current has ended
pub enum NextActivity {
    Authentication,
    FileTransfer,
    SetupActivity,
}

/// The activity manager takes care of running activities and handling them until the application has ended
pub struct ActivityManager {
    context: Option<Context>,
    ticks: Duration,
}

impl ActivityManager {
    /// Initializes a new Activity Manager
    pub fn new(ticks: Duration) -> Result<ActivityManager, HostError> {
        // Prepare Context
        // Initialize configuration client
        let (config_client, error_config): (ConfigClient, Option<String>) =
            match Self::init_config_client() {
                Ok(cli) => (cli, None),
                Err(err) => {
                    error!("Failed to initialize config client: {}", err);
                    (ConfigClient::degraded(), Some(err))
                }
            };
        let (bookmarks_client, error_bookmark) = match Self::init_bookmarks_client() {
            Ok(cli) => (cli, None),
            Err(err) => (None, Some(err)),
        };
        let error = error_config.or(error_bookmark);
        let theme_provider: ThemeProvider = Self::init_theme_provider();
        let ctx: Context = Context::new(bookmarks_client, config_client, theme_provider, error);
        Ok(ActivityManager {
            context: Some(ctx),
            ticks,
        })
    }

    /// Set file transfer params
    pub fn set_filetransfer_params(
        &mut self,
        mut params: FileTransferParams,
        password: Option<&str>,
    ) -> Result<(), String> {
        // Set password if provided
        if params.password_missing() {
            if let Some(password) = password {
                params.set_default_secret(password.to_string());
            } else if matches!(
                params.protocol,
                FileTransferProtocol::Scp | FileTransferProtocol::Sftp,
            ) && params.params.generic_params().is_some()
            {
                // * if protocol is SCP or SFTP check whether a SSH key is registered for this remote, in case not ask password
                let storage = SshKeyStorage::from(self.context.as_ref().unwrap().config());
                let generic_params = params.params.generic_params().unwrap();
                if storage
                    .resolve(
                        &generic_params.address,
                        &generic_params
                            .username
                            .clone()
                            .unwrap_or(whoami::username()),
                    )
                    .is_none()
                {
                    debug!(
                        "storage could not find any suitable key for {}... prompting for password",
                        generic_params.address
                    );
                    self.prompt_password(&mut params)?;
                } else {
                    debug!(
                        "a key is already set for {}; password is not required",
                        generic_params.address
                    );
                }
            } else {
                self.prompt_password(&mut params)?;
            }
        }
        // Put params into the context
        self.context.as_mut().unwrap().set_ftparams(params);
        Ok(())
    }

    /// Prompt user for password to set into params.
    fn prompt_password(&mut self, params: &mut FileTransferParams) -> Result<(), String> {
        let ctx = self.context.as_mut().unwrap();

        match tty::read_secret_from_tty(ctx.terminal(), "Password: ") {
            Err(err) => Err(format!("Could not read password: {err}")),
            Ok(Some(secret)) => {
                debug!(
                    "Read password from tty: {}",
                    fmt::shadow_password(secret.as_str())
                );
                params.set_default_secret(secret);
                Ok(())
            }
            Ok(None) => Ok(()),
        }
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
                    r#"Could not resolve bookmark name: "{bookmark_name}" no such bookmark"#
                )),
                Some(params) => self.set_filetransfer_params(params, password),
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
        let ctx: Context = match self.context.take() {
            Some(ctx) => ctx,
            None => {
                error!("Failed to start FileTransferActivity: context is None");
                return None;
            }
        };
        // If ft params is None, return None
        let remote_params: &FileTransferParams = match ctx.ft_params() {
            Some(ft_params) => ft_params,
            None => {
                error!("Failed to start FileTransferActivity: file transfer params is None");
                return None;
            }
        };

        // get local path:
        // - if set in file transfer params, get it from there
        // - otherwise is env current dir
        // - otherwise is /
        let local_wrkdir = remote_params
            .local_path
            .clone()
            .or(std::env::current_dir().ok())
            .unwrap_or(PathBuf::from("/"));

        // TODO: get host params from prev activity
        let host_bridge_params = HostBridgeParams::Localhost(local_wrkdir);

        let mut activity: FileTransferActivity =
            FileTransferActivity::new(host_bridge_params, remote_params, self.ticks);
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
                            Err(err) => Err(format!("Could not read configuration: {err}")),
                        }
                    }
                    None => Err(String::from(
                        "Your system doesn't provide a configuration directory",
                    )),
                }
            }
            Err(err) => Err(format!(
                "Could not initialize configuration directory: {err}"
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
