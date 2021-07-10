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

const TERMSCP_VERSION: &str = env!("CARGO_PKG_VERSION");
const TERMSCP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

// Crates
extern crate argh;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate magic_crypt;
extern crate rpassword;

// External libs
use argh::FromArgs;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

// Include
mod activity_manager;
mod config;
mod filetransfer;
mod fs;
mod host;
mod support;
mod system;
mod ui;
mod utils;

// namespaces
use activity_manager::{ActivityManager, NextActivity};
use filetransfer::FileTransferProtocol;
use system::logging;

enum Task {
    Activity(NextActivity),
    ImportTheme(PathBuf),
}

#[derive(FromArgs)]
#[argh(description = "
where positional can be: [protocol://user@address:port:wrkdir] [local-wrkdir]

Please, report issues to <https://github.com/veeso/termscp>
Please, consider supporting the author <https://www.buymeacoffee.com/veeso>")]
struct Args {
    #[argh(switch, short = 'c', description = "open termscp configuration")]
    config: bool,
    #[argh(option, short = 'P', description = "provide password from CLI")]
    password: Option<String>,
    #[argh(switch, short = 'q', description = "disable logging")]
    quiet: bool,
    #[argh(option, short = 't', description = "import specified theme")]
    theme: Option<String>,
    #[argh(
        option,
        short = 'T',
        default = "10",
        description = "set UI ticks; default 10ms"
    )]
    ticks: u64,
    #[argh(switch, short = 'v', description = "print version")]
    version: bool,
    // -- positional
    #[argh(
        positional,
        description = "protocol://user@address:port:wrkdir local-wrkdir"
    )]
    positional: Vec<String>,
}

struct RunOpts {
    address: Option<String>,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    remote_wrkdir: Option<PathBuf>,
    protocol: FileTransferProtocol,
    ticks: Duration,
    log_enabled: bool,
    task: Task,
}

impl Default for RunOpts {
    fn default() -> Self {
        Self {
            address: None,
            port: 22,
            username: None,
            password: None,
            remote_wrkdir: None,
            protocol: FileTransferProtocol::Sftp,
            ticks: Duration::from_millis(10),
            log_enabled: true,
            task: Task::Activity(NextActivity::Authentication),
        }
    }
}

fn main() {
    let args: Args = argh::from_env();
    // Parse args
    let mut run_opts: RunOpts = match parse_args(args) {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(255);
        }
    };
    // Setup logging
    if run_opts.log_enabled {
        if let Err(err) = logging::init() {
            eprintln!("Failed to initialize logging: {}", err);
        }
    }
    // Read password from remote
    if let Err(err) = read_password(&mut run_opts) {
        eprintln!("{}", err);
        std::process::exit(255);
    }
    info!("termscp {} started!", TERMSCP_VERSION);
    // Run
    info!("Starting activity manager...");
    let rc: i32 = run(run_opts);
    info!("termscp terminated");
    // Then return
    std::process::exit(rc);
}

/// ### parse_args
///
/// Parse arguments
/// In case of success returns `RunOpts`
/// in case something is wrong returns the error message
fn parse_args(args: Args) -> Result<RunOpts, String> {
    let mut run_opts: RunOpts = RunOpts::default();
    // Version
    if args.version {
        return Err(format!(
            "termscp - {} - Developed by {}",
            TERMSCP_VERSION, TERMSCP_AUTHORS,
        ));
    }
    // Setup activity?
    if args.config {
        run_opts.task = Task::Activity(NextActivity::SetupActivity);
    }
    // Logging
    if args.quiet {
        run_opts.log_enabled = false;
    }
    // Match password
    if let Some(passwd) = args.password {
        run_opts.password = Some(passwd);
    }
    // Match ticks
    run_opts.ticks = Duration::from_millis(args.ticks);
    // @! extra modes
    if let Some(theme) = args.theme {
        run_opts.task = Task::ImportTheme(PathBuf::from(theme));
    }
    // @! Ordinary mode
    // Remote argument
    if let Some(remote) = args.positional.get(0) {
        // Parse address
        match utils::parser::parse_remote_opt(remote.as_str()) {
            Ok(host_opts) => {
                // Set params
                run_opts.address = Some(host_opts.hostname);
                run_opts.port = host_opts.port;
                run_opts.protocol = host_opts.protocol;
                run_opts.username = host_opts.username;
                run_opts.remote_wrkdir = host_opts.wrkdir;
                // In this case the first activity will be FileTransfer
                run_opts.task = Task::Activity(NextActivity::FileTransfer);
            }
            Err(err) => {
                return Err(format!("Bad address option: {}", err));
            }
        }
    }
    // Local directory
    if let Some(localdir) = args.positional.get(1) {
        // Change working directory if local dir is set
        let localdir: PathBuf = PathBuf::from(localdir);
        if let Err(err) = env::set_current_dir(localdir.as_path()) {
            return Err(format!("Bad working directory argument: {}", err));
        }
    }
    Ok(run_opts)
}

/// ### read_password
///
/// Read password from tty if address is specified
fn read_password(run_opts: &mut RunOpts) -> Result<(), String> {
    // Initialize client if necessary
    if run_opts.address.is_some() {
        debug!("User has specified remote options: address: {:?}, port: {:?}, protocol: {:?}, user: {:?}, password: {}", run_opts.address, run_opts.port, run_opts.protocol, run_opts.username, utils::fmt::shadow_password(run_opts.password.as_deref().unwrap_or("")));
        if run_opts.password.is_none() {
            // Ask password if unspecified
            run_opts.password = match rpassword::read_password_from_tty(Some("Password: ")) {
                Ok(p) => {
                    if p.is_empty() {
                        None
                    } else {
                        debug!(
                            "Read password from tty: {}",
                            utils::fmt::shadow_password(p.as_str())
                        );
                        Some(p)
                    }
                }
                Err(_) => {
                    return Err("Could not read password from prompt".to_string());
                }
            };
        }
    }
    Ok(())
}

/// ### run
///
/// Run task and return rc
fn run(mut run_opts: RunOpts) -> i32 {
    match run_opts.task {
        Task::ImportTheme(theme) => match support::import_theme(theme.as_path()) {
            Ok(_) => {
                println!("Theme has been successfully imported!");
                0
            }
            Err(err) => {
                eprintln!("{}", err);
                1
            }
        },
        Task::Activity(activity) => {
            // Get working directory
            let wrkdir: PathBuf = match env::current_dir() {
                Ok(dir) => dir,
                Err(_) => PathBuf::from("/"),
            };
            // Create activity manager (and context too)
            let mut manager: ActivityManager =
                match ActivityManager::new(wrkdir.as_path(), run_opts.ticks) {
                    Ok(m) => m,
                    Err(err) => {
                        eprintln!("Could not start activity manager: {}", err);
                        return 1;
                    }
                };
            // Set file transfer params if set
            if let Some(address) = run_opts.address.take() {
                manager.set_filetransfer_params(
                    address,
                    run_opts.port,
                    run_opts.protocol,
                    run_opts.username,
                    run_opts.password,
                    run_opts.remote_wrkdir,
                );
            }
            manager.run(activity);
            0
        }
    }
}
