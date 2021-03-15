//! ## ActivityManager
//!
//! `activity_manager` is the module which provides run methods and handling for activities

/*
*
*   Copyright (C) 2020-2021 Christian Visintin - christian.visintin1997@gmail.com
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

use std::path::PathBuf;

// Deps
use crate::filetransfer::FileTransferProtocol;
use crate::host::{HostError, Localhost};
use crate::system::config_client::ConfigClient;
use crate::system::environment;
use crate::ui::activities::{
    auth_activity::AuthActivity, filetransfer_activity::FileTransferActivity,
    filetransfer_activity::FileTransferParams, setup_activity::SetupActivity, Activity,
};
use crate::ui::context::Context;

// Namespaces
use std::thread::sleep;
use std::time::Duration;

/// ### NextActivity
///
/// NextActivity identified the next identity to run once the current has ended
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
    ftparams: Option<FileTransferParams>,
    interval: Duration,
}

impl ActivityManager {
    /// ### new
    ///
    /// Initializes a new Activity Manager
    pub fn new(local_dir: &PathBuf, interval: Duration) -> Result<ActivityManager, HostError> {
        // Prepare Context
        let host: Localhost = match Localhost::new(local_dir.clone()) {
            Ok(h) => h,
            Err(e) => return Err(e),
        };
        // Initialize configuration client
        let (config_client, error): (Option<ConfigClient>, Option<String>) =
            match Self::init_config_client() {
                Ok(cli) => (Some(cli), None),
                Err(err) => (None, Some(err)),
            };
        let ctx: Context = Context::new(host, config_client, error);
        Ok(ActivityManager {
            context: Some(ctx),
            ftparams: None,
            interval,
        })
    }

    /// ### set_filetransfer_params
    ///
    /// Set file transfer params
    pub fn set_filetransfer_params(
        &mut self,
        address: String,
        port: u16,
        protocol: FileTransferProtocol,
        username: Option<String>,
        password: Option<String>,
        entry_directory: Option<PathBuf>,
    ) {
        self.ftparams = Some(FileTransferParams {
            address,
            port,
            protocol,
            username,
            password,
            entry_directory,
        });
    }

    /// ### run
    ///
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

    /// ### run_authentication
    ///
    /// Loop for Authentication activity.
    /// Returns when activity terminates.
    /// Returns the next activity to run
    fn run_authentication(&mut self) -> Option<NextActivity> {
        // Prepare activity
        let mut activity: AuthActivity = AuthActivity::default();
        // Prepare result
        let result: Option<NextActivity>;
        // Get context
        let ctx: Context = match self.context.take() {
            Some(ctx) => ctx,
            None => return None,
        };
        // Create activity
        activity.on_create(ctx);
        loop {
            // Draw activity
            activity.on_draw();
            // Check if has to be terminated
            if activity.quit {
                // Quit activities
                result = None;
                break;
            }
            if activity.setup {
                // User requested activity
                result = Some(NextActivity::SetupActivity);
                break;
            }
            if activity.submit {
                // User submitted, set next activity
                result = Some(NextActivity::FileTransfer);
                // Get params
                self.ftparams = Some(FileTransferParams {
                    address: activity.address.clone(),
                    port: activity.port.parse::<u16>().ok().unwrap(),
                    username: match activity.username.len() {
                        0 => None,
                        _ => Some(activity.username.clone()),
                    },
                    password: match activity.password.len() {
                        0 => None,
                        _ => Some(activity.password.clone()),
                    },
                    protocol: activity.protocol,
                    entry_directory: None, // Has use only when accessing with address
                });
                break;
            }
            // Sleep for ticks
            sleep(self.interval);
        }
        // Destroy activity
        self.context = activity.on_destroy();
        result
    }

    /// ### run_filetransfer
    ///
    /// Loop for FileTransfer activity.
    /// Returns when activity terminates.
    /// Returns the next activity to run
    fn run_filetransfer(&mut self) -> Option<NextActivity> {
        if self.ftparams.is_none() {
            return Some(NextActivity::Authentication);
        }
        // Prepare activity
        let mut activity: FileTransferActivity =
            FileTransferActivity::new(self.ftparams.take().unwrap());
        // Prepare result
        let result: Option<NextActivity>;
        // Get context
        let ctx: Context = match self.context.take() {
            Some(ctx) => ctx,
            None => return None,
        };
        // Create activity
        activity.on_create(ctx);
        loop {
            // Draw activity
            activity.on_draw();
            // Check if has to be terminated
            if activity.quit {
                // Quit activities
                result = None;
                break;
            }
            if activity.disconnected {
                // User disconnected, set next activity to authentication
                result = Some(NextActivity::Authentication);
                break;
            }
            // Sleep for ticks
            sleep(self.interval);
        }
        // Destroy activity
        self.context = activity.on_destroy();
        result
    }

    /// ### run_setup
    ///
    /// `SetupActivity` run loop.
    /// Returns when activity terminates.
    /// Returns the next activity to run
    fn run_setup(&mut self) -> Option<NextActivity> {
        // Prepare activity
        let mut activity: SetupActivity = SetupActivity::default();
        // Get context
        let ctx: Context = match self.context.take() {
            Some(ctx) => ctx,
            None => return None,
        };
        // Create activity
        activity.on_create(ctx);
        loop {
            // Draw activity
            activity.on_draw();
            // Check if activity has terminated
            if activity.quit {
                break;
            }
            // Sleep for ticks
            sleep(self.interval);
        }
        // Destroy activity
        self.context = activity.on_destroy();
        // This activity always returns to AuthActivity
        Some(NextActivity::Authentication)
    }

    // -- misc

    /// ### init_config_client
    ///
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
                        "Your system doesn't support configuration paths",
                    )),
                }
            }
            Err(err) => Err(format!(
                "Could not initialize configuration directory: {}",
                err
            )),
        }
    }
}
