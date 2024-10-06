mod activity_manager;
mod cli;
mod config;
mod explorer;
mod filetransfer;
mod host;
mod support;
mod system;
mod ui;
mod utils;

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

use std::env;
use std::path::Path;
use std::time::Duration;

use self::activity_manager::{ActivityManager, NextActivity};
use self::cli::{Args, ArgsSubcommands, RemoteArgs, RunOpts, Task};
use self::system::logging::{self, LogLevel};

const EXIT_CODE_SUCCESS: i32 = 0;
const EXIT_CODE_ERROR: i32 = 1;
const TERMSCP_VERSION: &str = env!("CARGO_PKG_VERSION");
const TERMSCP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let args: Args = argh::from_env();
    // Parse args
    let run_opts: RunOpts = match parse_args(args) {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(255);
        }
    };
    // Setup logging
    if let Err(err) = logging::init(run_opts.log_level) {
        eprintln!("Failed to initialize logging: {err}");
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
    let run_opts = match args.nested {
        Some(ArgsSubcommands::Update(_)) => RunOpts::update(),
        Some(ArgsSubcommands::LoadTheme(args)) => RunOpts::import_theme(args.theme),
        Some(ArgsSubcommands::Config(_)) => RunOpts::config(),
        None => {
            let mut run_opts: RunOpts = RunOpts::default();
            // Version
            if args.version {
                return Err(format!(
                    "termscp - {TERMSCP_VERSION} - Developed by {TERMSCP_AUTHORS}",
                ));
            }
            // Logging
            if args.debug {
                run_opts.log_level = LogLevel::Trace;
            } else if args.quiet {
                run_opts.log_level = LogLevel::Off;
            }
            // Match ticks
            run_opts.ticks = Duration::from_millis(args.ticks);
            // Remote argument
            match RemoteArgs::try_from(&args) {
                Err(err) => return Err(err),
                Ok(remote) => {
                    // Set params
                    run_opts.remote = remote;
                    // In this case the first activity will be FileTransfer
                    run_opts.task = Task::Activity(NextActivity::FileTransfer);
                }
            }

            // Local directory
            if let Some(localdir) = run_opts.remote.local_dir.as_deref() {
                if let Err(err) = env::set_current_dir(localdir) {
                    return Err(format!("Bad working directory argument: {err}"));
                }
            }

            run_opts
        }
    };

    Ok(run_opts)
}

/// Run task and return rc
fn run(run_opts: RunOpts) -> i32 {
    match run_opts.task {
        Task::ImportTheme(theme) => run_import_theme(&theme),
        Task::InstallUpdate => run_install_update(),
        Task::Activity(activity) => run_activity(activity, run_opts.ticks, run_opts.remote),
    }
}

fn run_import_theme(theme: &Path) -> i32 {
    match support::import_theme(theme) {
        Ok(_) => {
            println!("Theme has been successfully imported!");
            EXIT_CODE_ERROR
        }
        Err(err) => {
            eprintln!("{err}");
            EXIT_CODE_ERROR
        }
    }
}

fn run_install_update() -> i32 {
    match support::install_update() {
        Ok(msg) => {
            println!("{msg}");
            EXIT_CODE_SUCCESS
        }
        Err(err) => {
            eprintln!("Could not install update: {err}");
            EXIT_CODE_ERROR
        }
    }
}

fn run_activity(activity: NextActivity, ticks: Duration, remote_args: RemoteArgs) -> i32 {
    // Create activity manager (and context too)
    let mut manager: ActivityManager = match ActivityManager::new(ticks) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("Could not start activity manager: {err}");
            return EXIT_CODE_ERROR;
        }
    };

    // Set file transfer params if set
    if let Err(err) = manager.configure_remote_args(remote_args) {
        eprintln!("{err}");
        return EXIT_CODE_ERROR;
    }

    manager.run(activity);

    EXIT_CODE_SUCCESS
}
