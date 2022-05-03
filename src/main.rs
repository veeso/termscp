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

// External libs
use argh::FromArgs;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

// Include
mod activity_manager;
mod config;
mod explorer;
mod filetransfer;
mod host;
mod support;
mod system;
mod ui;
mod utils;

// namespaces
use activity_manager::{ActivityManager, NextActivity};
use filetransfer::FileTransferParams;
use system::logging::{self, LogLevel};

enum Task {
    Activity(NextActivity),
    ImportTheme(PathBuf),
    InstallUpdate,
}

#[derive(FromArgs)]
#[argh(description = "
where positional can be: [address] [local-wrkdir]

Address syntax can be:

    - `protocol://user@address:port:wrkdir` for protocols such as Sftp, Scp, Ftp
    - `s3://bucket-name@region:profile:/wrkdir` for Aws S3 protocol

Please, report issues to <https://github.com/veeso/termscp>
Please, consider supporting the author <https://ko-fi.com/veeso>")]
struct Args {
    #[argh(
        switch,
        short = 'b',
        description = "resolve address argument as a bookmark name"
    )]
    address_as_bookmark: bool,
    #[argh(switch, short = 'c', description = "open termscp configuration")]
    config: bool,
    #[argh(switch, short = 'D', description = "enable TRACE log level")]
    debug: bool,
    #[argh(option, short = 'P', description = "provide password from CLI")]
    password: Option<String>,
    #[argh(switch, short = 'q', description = "disable logging")]
    quiet: bool,
    #[argh(option, short = 't', description = "import specified theme")]
    theme: Option<String>,
    #[argh(
        switch,
        short = 'u',
        description = "update termscp to the latest version"
    )]
    update: bool,
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
    remote: Remote,
    ticks: Duration,
    log_level: LogLevel,
    task: Task,
}

impl Default for RunOpts {
    fn default() -> Self {
        Self {
            remote: Remote::None,
            ticks: Duration::from_millis(10),
            log_level: LogLevel::Info,
            task: Task::Activity(NextActivity::Authentication),
        }
    }
}

enum Remote {
    Bookmark(BookmarkParams),
    Host(FileTransferParams),
    None,
}

struct BookmarkParams {
    name: String,
    password: Option<String>,
}

impl BookmarkParams {
    pub fn new<S: AsRef<str>>(name: S, password: Option<S>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            password: password.map(|x| x.as_ref().to_string()),
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
    if let Err(err) = logging::init(run_opts.log_level) {
        eprintln!("Failed to initialize logging: {}", err);
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
    info!("termscp terminated with exitcode {}", rc);
    // Then return
    std::process::exit(rc);
}

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
    if args.debug {
        run_opts.log_level = LogLevel::Trace;
    } else if args.quiet {
        run_opts.log_level = LogLevel::Off;
    }
    // Match ticks
    run_opts.ticks = Duration::from_millis(args.ticks);
    // @! extra modes
    if let Some(theme) = args.theme.as_deref() {
        run_opts.task = Task::ImportTheme(PathBuf::from(theme));
    }
    if args.update {
        run_opts.task = Task::InstallUpdate;
    }
    // @! Ordinary mode
    // Remote argument
    match parse_address_arg(&args) {
        Err(err) => return Err(err),
        Ok(Remote::None) => {}
        Ok(remote) => {
            // Set params
            run_opts.remote = remote;
            // In this case the first activity will be FileTransfer
            run_opts.task = Task::Activity(NextActivity::FileTransfer);
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

/// Parse address argument from cli args
fn parse_address_arg(args: &Args) -> Result<Remote, String> {
    if let Some(remote) = args.positional.get(0) {
        if args.address_as_bookmark {
            Ok(Remote::Bookmark(BookmarkParams::new(
                remote,
                args.password.as_ref(),
            )))
        } else {
            // Parse address
            parse_remote_address(remote.as_str(), args.password.as_deref()).map(Remote::Host)
        }
    } else {
        Ok(Remote::None)
    }
}

/// Parse remote address
fn parse_remote_address(
    remote: &str,
    password: Option<&str>,
) -> Result<FileTransferParams, String> {
    match utils::parser::parse_remote_opt(remote) {
        Ok(mut remote) => {
            // If password is provided, set password
            if let Some(passwd) = password {
                if let Some(mut params) = remote.params.mut_generic_params() {
                    params.password = Some(passwd.to_string());
                }
            }
            Ok(remote)
        }
        Err(err) => Err(format!("Bad address option: {}", err)),
    }
}

/// Read password from tty if address is specified
fn read_password(run_opts: &mut RunOpts) -> Result<(), String> {
    // Initialize client if necessary
    if let Some(remote) = run_opts.remote.as_mut() {
        // Ask password for generic params
        if let Some(mut params) = remote.params.mut_generic_params() {
            // Ask password only if generic protocol params
            if params.password.is_none() {
                // Ask password if unspecified
                params.password = match rpassword::read_password_from_tty(Some("Password: ")) {
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
        Task::InstallUpdate => match support::install_update() {
            Ok(msg) => {
                println!("{}", msg);
                0
            }
            Err(err) => {
                eprintln!("Could not install update: {}", err);
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
            if let Some(remote) = run_opts.remote.take() {
                manager.set_filetransfer_params(remote);
            }
            // TODO: resolve bookmark name
            manager.run(activity);
            0
        }
    }
}
