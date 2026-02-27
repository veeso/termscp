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
extern crate log;
#[macro_use]
extern crate magic_crypt;

use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

use self::activity_manager::{ActivityManager, NextActivity};
use self::cli::{Args, ArgsSubcommands, RemoteArgs, RunOpts, Task};
use self::system::logging::{self, LogLevel};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_BUILD_DATE: &str = env!("VERGEN_BUILD_TIMESTAMP");
const APP_GIT_BRANCH: &str = env!("VERGEN_GIT_BRANCH");
const APP_GIT_HASH: &str = env!("VERGEN_GIT_SHA");
const TERMSCP_VERSION: &str = env!("CARGO_PKG_VERSION");
const TERMSCP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[inline]
fn git_hash() -> &'static str {
    APP_GIT_HASH[0..8].as_ref()
}

fn main() -> MainResult<()> {
    let args: Args = argh::from_env();
    // Parse args
    let run_opts: RunOpts = match parse_args(args) {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("{err}");
            return Err(err.into());
        }
    };
    // Setup logging
    if let Err(err) = logging::init(run_opts.log_level) {
        eprintln!("Failed to initialize logging: {err}");
    }
    info!(
        "{APP_NAME} v{TERMSCP_VERSION} ({APP_GIT_BRANCH}, {git_hash}, {APP_BUILD_DATE}) - Developed by {TERMSCP_AUTHORS}",
        git_hash = git_hash()
    );
    // Run
    info!("Starting activity manager...");
    run(run_opts)
}

/// Parse arguments
/// In case of success returns `RunOpts`
/// in case something is wrong returns the error message
fn parse_args(args: Args) -> Result<RunOpts, String> {
    let run_opts = match args.nested {
        Some(ArgsSubcommands::Update(_)) => RunOpts::update(),
        Some(ArgsSubcommands::ImportSshHosts(subargs)) => {
            RunOpts::import_ssh_hosts(subargs.ssh_config, !args.wno_keyring)
        }
        Some(ArgsSubcommands::ImportTheme(args)) => RunOpts::import_theme(args.theme),
        Some(ArgsSubcommands::Config(_)) => RunOpts::config(),
        None => {
            let mut run_opts: RunOpts = RunOpts::default();

            // Version
            if args.version {
                run_opts.task = Task::Version;
                return Ok(run_opts);
            }
            // Logging
            if args.debug {
                run_opts.log_level = LogLevel::Trace;
            } else if args.quiet {
                run_opts.log_level = LogLevel::Off;
            }
            // set keyring
            if args.wno_keyring {
                run_opts.keyring = false;
            }
            // Match ticks
            run_opts.ticks = Duration::from_millis(args.ticks);
            // Remote argument
            match RemoteArgs::try_from(&args) {
                Err(err) => return Err(err),
                Ok(remote) => {
                    // Set params
                    run_opts.remote = remote;
                }
            }

            // set activity based on remote state
            run_opts.task = if run_opts.remote.remote.is_none() {
                Task::Activity(NextActivity::Authentication)
            } else {
                Task::Activity(NextActivity::FileTransfer)
            };

            // Local directory
            if let Some(localdir) = run_opts.remote.local_dir.as_deref()
                && let Err(err) = env::set_current_dir(localdir)
            {
                return Err(format!("Bad working directory argument: {err}"));
            }

            run_opts
        }
    };

    Ok(run_opts)
}

/// Run task and return rc
fn run(run_opts: RunOpts) -> MainResult<()> {
    match run_opts.task {
        Task::ImportSshHosts(ssh_config) => run_import_ssh_hosts(ssh_config, run_opts.keyring),
        Task::ImportTheme(theme) => run_import_theme(&theme),
        Task::InstallUpdate => run_install_update(),
        Task::Activity(activity) => {
            run_activity(activity, run_opts.ticks, run_opts.remote, run_opts.keyring)
        }
        Task::Version => print_version(),
    }
}

fn print_version() -> MainResult<()> {
    println!(
        "{APP_NAME} v{TERMSCP_VERSION} ({APP_GIT_BRANCH}, {git_hash}, {APP_BUILD_DATE}) - Developed by {TERMSCP_AUTHORS}",
        git_hash = git_hash()
    );

    Ok(())
}

fn run_import_ssh_hosts(ssh_config_path: Option<PathBuf>, keyring: bool) -> MainResult<()> {
    support::import_ssh_hosts(ssh_config_path, keyring)
        .map(|_| {
            println!("SSH hosts have been successfully imported!");
        })
        .map_err(|err| {
            eprintln!("{err}");
            err.into()
        })
}

fn run_import_theme(theme: &Path) -> MainResult<()> {
    match support::import_theme(theme) {
        Ok(_) => {
            println!("Theme has been successfully imported!");
            Ok(())
        }
        Err(err) => {
            eprintln!("{err}");
            Err(err.into())
        }
    }
}

fn run_install_update() -> MainResult<()> {
    match support::install_update() {
        Ok(msg) => {
            println!("{msg}");
            Ok(())
        }
        Err(err) => {
            eprintln!("Could not install update: {err}");
            Err(err.into())
        }
    }
}

fn run_activity(
    activity: NextActivity,
    ticks: Duration,
    remote_args: RemoteArgs,
    keyring: bool,
) -> MainResult<()> {
    // Create activity manager (and context too)
    let mut manager: ActivityManager = match ActivityManager::new(ticks, keyring) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("Could not start activity manager: {err}");
            return Err(err.into());
        }
    };

    // Set file transfer params if set
    if let Err(err) = manager.configure_remote_args(remote_args) {
        eprintln!("{err}");
        return Err(err.into());
    }

    manager.run(activity);

    Ok(())
}
