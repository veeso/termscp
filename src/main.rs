const TERMSCP_VERSION: &str = env!("CARGO_PKG_VERSION");
const TERMSCP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

// Crates
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate magic_crypt;

// External libs
use std::env;
use std::path::PathBuf;
use std::time::Duration;

// Include
mod activity_manager;
mod cli_opts;
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
use cli_opts::{Args, BookmarkParams, HostParams, Remote, RunOpts, Task};
use filetransfer::FileTransferParams;
use system::logging::{self, LogLevel};

fn main() {
    let args: Args = argh::from_env();
    // Parse args
    let run_opts: RunOpts = match parse_args(args) {
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
    info!("termscp {} started!", TERMSCP_VERSION);
    // Run
    info!("Starting activity manager...");
    let rc = run(run_opts);
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
            parse_remote_address(remote.as_str())
                .map(|x| Remote::Host(HostParams::new(x, args.password.as_deref())))
        }
    } else {
        Ok(Remote::None)
    }
}

/// Parse remote address
fn parse_remote_address(remote: &str) -> Result<FileTransferParams, String> {
    utils::parser::parse_remote_opt(remote).map_err(|e| format!("Bad address option: {}", e))
}

/// ### run
///
/// Run task and return rc
fn run(run_opts: RunOpts) -> i32 {
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
            match run_opts.remote {
                Remote::Bookmark(BookmarkParams { name, password }) => {
                    if let Err(err) = manager.resolve_bookmark_name(&name, password.as_deref()) {
                        eprintln!("{}", err);
                        return 1;
                    }
                }
                Remote::Host(HostParams { params, password }) => {
                    if let Err(err) = manager.set_filetransfer_params(params, password.as_deref()) {
                        eprintln!("{}", err);
                        return 1;
                    }
                }
                Remote::None => {}
            }
            manager.run(activity);
            0
        }
    }
}
