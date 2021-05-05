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
extern crate magic_crypt;
extern crate rpassword;

// External libs
use getopts::Options;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

// Include
mod activity_manager;
mod bookmarks;
mod config;
mod filetransfer;
mod fs;
mod host;
mod system;
mod ui;
mod utils;

// namespaces
use activity_manager::{ActivityManager, NextActivity};
use filetransfer::FileTransferProtocol;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    //Program CLI options
    let mut address: Option<String> = None; // None
    let mut port: u16 = 22; // Default port
    let mut username: Option<String> = None; // Default username
    let mut password: Option<String> = None; // Default password
    let mut remote_wrkdir: Option<PathBuf> = None;
    let mut protocol: FileTransferProtocol = FileTransferProtocol::Sftp; // Default protocol
    let mut ticks: Duration = Duration::from_millis(10);
    //Process options
    let mut opts = Options::new();
    opts.optopt("P", "password", "Provide password from CLI", "<password>");
    opts.optopt("T", "ticks", "Set UI ticks; default 10ms", "<ms>");
    opts.optflag("v", "version", "");
    opts.optflag("h", "help", "Print this menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            std::process::exit(255);
        }
    };
    // Help
    if matches.opt_present("h") {
        print_usage(opts);
        std::process::exit(255);
    }
    // Version
    if matches.opt_present("v") {
        eprintln!(
            "termscp - {} - Developed by {}",
            TERMSCP_VERSION, TERMSCP_AUTHORS,
        );
        std::process::exit(255);
    }
    // Match password
    if let Some(passwd) = matches.opt_str("P") {
        password = Some(passwd);
    }
    // Match ticks
    if let Some(val) = matches.opt_str("T") {
        match val.parse::<usize>() {
            Ok(val) => ticks = Duration::from_millis(val as u64),
            Err(_) => {
                eprintln!("Ticks is not a number '{}'", val);
                print_usage(opts);
                std::process::exit(255);
            }
        }
    }
    // Check free args
    let extra_args: Vec<String> = matches.free;
    // Remote argument
    if let Some(remote) = extra_args.get(0) {
        // Parse address
        match utils::parser::parse_remote_opt(remote) {
            Ok(host_opts) => {
                // Set params
                address = Some(host_opts.hostname);
                port = host_opts.port;
                protocol = host_opts.protocol;
                username = host_opts.username;
                remote_wrkdir = host_opts.wrkdir;
            }
            Err(err) => {
                eprintln!("Bad address option: {}", err);
                print_usage(opts);
                std::process::exit(255);
            }
        }
    }
    // Local directory
    if let Some(localdir) = extra_args.get(1) {
        // Change working directory if local dir is set
        let localdir: PathBuf = PathBuf::from(localdir);
        if let Err(err) = env::set_current_dir(localdir.as_path()) {
            eprintln!("Bad working directory argument: {}", err);
            std::process::exit(255);
        }
    }
    // Get working directory
    let wrkdir: PathBuf = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => PathBuf::from("/"),
    };
    // Initialize client if necessary
    let mut start_activity: NextActivity = NextActivity::Authentication;
    if address.is_some() {
        if password.is_none() {
            // Ask password if unspecified
            password = match rpassword::read_password_from_tty(Some("Password: ")) {
                Ok(p) => {
                    if p.is_empty() {
                        None
                    } else {
                        Some(p)
                    }
                }
                Err(_) => {
                    eprintln!("Could not read password from prompt");
                    std::process::exit(255);
                }
            };
        }
        // In this case the first activity will be FileTransfer
        start_activity = NextActivity::FileTransfer;
    }
    // Create activity manager (and context too)
    let mut manager: ActivityManager = match ActivityManager::new(&wrkdir, ticks) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("Could not start activity manager: {}", err);
            std::process::exit(255);
        }
    };
    // Set file transfer params if set
    if let Some(address) = address {
        manager.set_filetransfer_params(address, port, protocol, username, password, remote_wrkdir);
    }
    // Run
    manager.run(start_activity);
    // Then return
    std::process::exit(0);
}
