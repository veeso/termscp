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

// External libs
use getopts::Options;
use std::env;

/// ### print_usage
///
/// Print usage

fn print_usage(opts: Options) {
    let brief = format!("Usage: termscp [Options]... Remote");
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //Program CLI options
    // TODO: insert opts here
    //Process options
    let mut opts = Options::new();
    // opts.optopt("c", "command", "Specify command to run. Shell returns after running the command", "<command>");
    // opts.optopt("C", "config", "Specify YAML configuration file", "<config>");
    // opts.optopt("l", "lang", "Specify shell language", "<ru|рус>");
    // opts.optopt("s", "shell", "Force the shell binary path", "</bin/bash>");
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
    // TODO: ...
    std::process::exit(0);
}
