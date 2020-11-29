/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
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

const TERMSCP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const TERMSCP_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

// Crates
extern crate getopts;
extern crate rpassword;

// External libs
use getopts::Options;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

// Include
mod activity_manager;
mod filetransfer;
mod fs;
mod host;
mod ui;
mod utils;

// namespaces
use activity_manager::{ActivityManager, NextActivity};
use filetransfer::FileTransferProtocol;

/// ### print_usage
///
/// Print usage

fn print_usage(opts: Options) {
    let brief = format!("Usage: termscp [Options]... [protocol:user@address:port]");
    print!("{}", opts.usage(&brief));
    println!("\nPlease, report issues to <https://github.com/ChristianVisintin/TermSCP>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //Program CLI options
    let mut address: Option<String> = None; // None
    let mut port: u16 = 22; // Default port
    let mut username: Option<String> = None; // Default username
    let password: Option<String>; // Default password
    let mut protocol: FileTransferProtocol = FileTransferProtocol::Sftp; // Default protocol
    let mut ticks: Duration = Duration::from_micros(50);
    //Process options
    // FIXME: there is no way to provide password from CLI atm
    let mut opts = Options::new();
    opts.optopt("T", "ticks", "Set UI ticks; default 50µs", "<µs>");
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
            "TermSCP - {} - Developed by {}",
            TERMSCP_VERSION, TERMSCP_AUTHORS,
        );
        std::process::exit(255);
    }
    // Match ticks
    if let Some(val) = matches.opt_str("T") {
        match val.parse::<usize>() {
            Ok(val) => ticks = Duration::from_micros(val as u64),
            Err(_) => {
                eprintln!("Ticks is not a number '{}'", val);
                print_usage(opts);
                std::process::exit(255);
            }
        }
    }
    // Check free args
    let extra_args: Vec<String> = matches.free.clone();
    if let Some(remote) = extra_args.get(0) {
        // Parse address
        match utils::parse_remote_opt(remote) {
            Ok((addr, portn, proto, user)) => {
                // Set params
                address = Some(addr);
                port = portn;
                protocol = proto;
                username = user;
            }
            Err(err) => {
                eprintln!("Bad address option: {}", err);
                print_usage(opts);
                std::process::exit(255);
            }
        }
    }
    // Get working directory
    let wrkdir: PathBuf = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => PathBuf::from("/"),
    };
    // Create activity manager
    let mut manager: ActivityManager = match ActivityManager::new(&wrkdir, ticks) {
        Ok(m) => m,
        Err(_) => {
            eprintln!("Invalid directory '{}'", wrkdir.display());
            std::process::exit(255);
        }
    };
    // Initialize client if necessary
    let mut start_activity: NextActivity = NextActivity::Authentication;
    if let Some(address) = address {
        // Ask password
        password = match rpassword::read_password_from_tty(Some("Password: ")) {
            Ok(p) => {
                if p.len() > 0 {
                    Some(p)
                } else {
                    None
                }
            }
            Err(_) => {
                eprintln!("Could not read password from prompt");
                std::process::exit(255);
            }
        };
        // In this case the first activity will be FileTransfer
        start_activity = NextActivity::FileTransfer;
        manager.set_filetransfer_params(address, port, protocol, username, password);
    }
    // Run
    manager.run(start_activity);
    // Then return
    std::process::exit(0);
}
