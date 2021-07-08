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
extern crate getopts;
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
use getopts::{Matches, Options};
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
    let args: Vec<String> = env::args().collect();
    //Program CLI options
    let mut run_opts: RunOpts = RunOpts::default();
    //Process options
    let mut opts = Options::new();
    opts.optflag("c", "config", "Open termscp configuration");
    opts.optflag("q", "quiet", "Disable logging");
    opts.optopt("t", "theme", "Import specified theme", "<path>");
    opts.optopt("P", "password", "Provide password from CLI", "<password>");
    opts.optopt("T", "ticks", "Set UI ticks; default 10ms", "<ms>");
    opts.optflag("v", "version", "");
    opts.optflag("h", "help", "Print this menu");
    let matches: Matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            std::process::exit(255);
        }
    };
    // Parse args
    if let Err(err) = parse_run_opts(&mut run_opts, matches) {
        if let Some(err) = err {
            eprintln!("{}", err);
        } else {
            print_usage(opts);
        }
        std::process::exit(255);
    }
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

/// ### print_usage
///
/// Print usage
fn print_usage(opts: Options) {
    let brief = String::from(
        "Usage: termscp [options]... [protocol://user@address:port:wrkdir] [local-wrkdir]",
    );
    print!("{}", opts.usage(&brief));
    println!("\nPlease, report issues to <https://github.com/veeso/termscp>");
    println!("Please, consider supporting the author <https://www.buymeacoffee.com/veeso>")
}

/// ### parse_run_opts
///
/// Parse run options; in case something is wrong returns the error message
fn parse_run_opts(run_opts: &mut RunOpts, opts: Matches) -> Result<(), Option<String>> {
    // Help
    if opts.opt_present("h") {
        return Err(None);
    }
    // Version
    if opts.opt_present("v") {
        return Err(Some(format!(
            "termscp - {} - Developed by {}",
            TERMSCP_VERSION, TERMSCP_AUTHORS,
        )));
    }
    // Setup activity?
    if opts.opt_present("c") {
        run_opts.task = Task::Activity(NextActivity::SetupActivity);
    }
    // Logging
    if opts.opt_present("q") {
        run_opts.log_enabled = false;
    }
    // Match password
    if let Some(passwd) = opts.opt_str("P") {
        run_opts.password = Some(passwd);
    }
    // Match ticks
    if let Some(val) = opts.opt_str("T") {
        match val.parse::<usize>() {
            Ok(val) => run_opts.ticks = Duration::from_millis(val as u64),
            Err(_) => {
                return Err(Some(format!("Ticks is not a number: '{}'", val)));
            }
        }
    }
    // @! extra modes
    if let Some(theme) = opts.opt_str("t") {
        run_opts.task = Task::ImportTheme(PathBuf::from(theme));
    }
    // @! Ordinary mode
    // Check free args
    let extra_args: Vec<String> = opts.free;
    // Remote argument
    if let Some(remote) = extra_args.get(0) {
        // Parse address
        match utils::parser::parse_remote_opt(remote) {
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
                return Err(Some(format!("Bad address option: {}", err)));
            }
        }
    }
    // Local directory
    if let Some(localdir) = extra_args.get(1) {
        // Change working directory if local dir is set
        let localdir: PathBuf = PathBuf::from(localdir);
        if let Err(err) = env::set_current_dir(localdir.as_path()) {
            return Err(Some(format!("Bad working directory argument: {}", err)));
        }
    }
    Ok(())
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
