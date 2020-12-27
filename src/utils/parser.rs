//! ## Parser
//!
//! `parser` is the module which provides utilities for parsing different kind of stuff

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

// Dependencies
extern crate chrono;
extern crate whoami;

// Locals
use crate::filetransfer::FileTransferProtocol;
use crate::system::config_client::ConfigClient;
use crate::system::environment;

// Ext
use chrono::format::ParseError;
use chrono::prelude::*;
use std::str::FromStr;
use std::time::{Duration, SystemTime};

/// ### parse_remote_opt
///
/// Parse remote option string. Returns in case of success a tuple made of (address, port, protocol, username)
/// For ssh if username is not provided, current user will be used.
/// In case of error, message is returned
/// If port is missing default port will be used for each protocol
///     SFTP => 22
///     FTP => 21
/// The option string has the following syntax
/// [protocol]://[username]@{address}:[port]
/// The only argument which is mandatory is address
/// NOTE: possible strings
/// - 172.26.104.1
/// - root@172.26.104.1
/// - sftp://root@172.26.104.1
/// - sftp://172.26.104.1:4022
/// - sftp://172.26.104.1
/// - ...
///
pub fn parse_remote_opt(
    remote: &str,
) -> Result<(String, u16, FileTransferProtocol, Option<String>), String> {
    let mut wrkstr: String = remote.to_string();
    let address: String;
    let mut port: u16 = 22;
    let mut username: Option<String> = None;
    // Set protocol to default protocol
    let mut protocol: FileTransferProtocol = match environment::init_config_dir() {
        Ok(p) => match p {
            Some(p) => {
                // Create config client
                let (config_path, ssh_key_path) = environment::get_config_paths(p.as_path());
                match ConfigClient::new(config_path.as_path(), ssh_key_path.as_path()) {
                    Ok(cli) => cli.get_default_protocol(),
                    Err(_) => FileTransferProtocol::Sftp,
                }
            }
            None => FileTransferProtocol::Sftp,
        },
        Err(_) => FileTransferProtocol::Sftp,
    };
    // Split string by '://'
    let tokens: Vec<&str> = wrkstr.split("://").collect();
    // If length is > 1, then token[0] is protocol
    match tokens.len() {
        1 => {}
        2 => {
            // Parse protocol
            let (m_protocol, m_port) = match FileTransferProtocol::from_str(tokens[0]) {
                Ok(proto) => match proto {
                    FileTransferProtocol::Ftp(_) => (proto, 21),
                    FileTransferProtocol::Scp => (proto, 22),
                    FileTransferProtocol::Sftp => (proto, 22),
                },
                Err(_) => return Err(format!("Unknown protocol '{}'", tokens[0])),
            };
            protocol = m_protocol;
            port = m_port;
            wrkstr = String::from(tokens[1]); // Wrkstr becomes tokens[1]
        }
        _ => return Err(String::from("Bad syntax")), // Too many tokens...
    }
    // Set username to default if sftp or scp
    if matches!(
        protocol,
        FileTransferProtocol::Sftp | FileTransferProtocol::Scp
    ) {
        // Set username to current username
        username = Some(whoami::username());
    }
    // Split wrkstring by '@'
    let tokens: Vec<&str> = wrkstr.split('@').collect();
    match tokens.len() {
        1 => {}
        2 => {
            // Username is first token
            username = Some(String::from(tokens[0]));
            // Update wrkstr
            wrkstr = String::from(tokens[1]);
        }
        _ => return Err(String::from("Bad syntax")), // Too many tokens...
    }
    // Split wrkstring by ':'
    let tokens: Vec<&str> = wrkstr.split(':').collect();
    match tokens.len() {
        1 => {
            // Address is wrkstr
            address = wrkstr.clone();
        }
        2 => {
            // Address is first token
            address = String::from(tokens[0]);
            // Port is second str
            port = match tokens[1].parse::<u16>() {
                Ok(val) => val,
                Err(_) => {
                    return Err(format!(
                        "Port must be a number in range [0-65535], but is '{}'",
                        tokens[1]
                    ))
                }
            };
        }
        _ => return Err(String::from("Bad syntax")), // Too many tokens...
    }
    Ok((address, port, protocol, username))
}

/// ### parse_lstime
///
/// Convert ls syntax time to System Time
/// ls time has two possible syntax:
/// 1. if year is current: %b %d %H:%M (e.g. Nov 5 13:46)
/// 2. else: %b %d %Y (e.g. Nov 5 2019)
pub fn parse_lstime(tm: &str, fmt_year: &str, fmt_hours: &str) -> Result<SystemTime, ParseError> {
    let datetime: NaiveDateTime = match NaiveDate::parse_from_str(tm, fmt_year) {
        Ok(date) => {
            // Case 2.
            // Return NaiveDateTime from NaiveDate with time 00:00:00
            date.and_hms(0, 0, 0)
        }
        Err(_) => {
            // Might be case 1.
            // We need to add Current Year at the end of the string
            let this_year: i32 = Utc::now().year();
            let date_time_str: String = format!("{} {}", tm, this_year);
            // Now parse
            match NaiveDateTime::parse_from_str(
                date_time_str.as_ref(),
                format!("{} %Y", fmt_hours).as_ref(),
            ) {
                Ok(dt) => dt,
                Err(err) => return Err(err),
            }
        }
    };
    // Convert datetime to system time
    let sys_time: SystemTime = SystemTime::UNIX_EPOCH;
    Ok(sys_time
        .checked_add(Duration::from_secs(datetime.timestamp() as u64))
        .unwrap_or(SystemTime::UNIX_EPOCH))
}

/// ### parse_datetime
///
/// Parse date time string representation and transform it into `SystemTime`
pub fn parse_datetime(tm: &str, fmt: &str) -> Result<SystemTime, ParseError> {
    match NaiveDateTime::parse_from_str(tm, fmt) {
        Ok(dt) => {
            let sys_time: SystemTime = SystemTime::UNIX_EPOCH;
            Ok(sys_time
                .checked_add(Duration::from_secs(dt.timestamp() as u64))
                .unwrap_or(SystemTime::UNIX_EPOCH))
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_utils_parse_remote_opt() {
        // Base case
        let result: (String, u16, FileTransferProtocol, Option<String>) =
            parse_remote_opt(&String::from("172.26.104.1"))
                .ok()
                .unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 22);
        assert_eq!(result.2, FileTransferProtocol::Sftp);
        assert!(result.3.is_some());
        // User case
        let result: (String, u16, FileTransferProtocol, Option<String>) =
            parse_remote_opt(&String::from("root@172.26.104.1"))
                .ok()
                .unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 22);
        assert_eq!(result.2, FileTransferProtocol::Sftp);
        assert_eq!(result.3.unwrap(), String::from("root"));
        // User + port
        let result: (String, u16, FileTransferProtocol, Option<String>) =
            parse_remote_opt(&String::from("root@172.26.104.1:8022"))
                .ok()
                .unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 8022);
        assert_eq!(result.2, FileTransferProtocol::Sftp);
        assert_eq!(result.3.unwrap(), String::from("root"));
        // Port only
        let result: (String, u16, FileTransferProtocol, Option<String>) =
            parse_remote_opt(&String::from("172.26.104.1:4022"))
                .ok()
                .unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 4022);
        assert_eq!(result.2, FileTransferProtocol::Sftp);
        assert!(result.3.is_some());
        // Protocol
        let result: (String, u16, FileTransferProtocol, Option<String>) =
            parse_remote_opt(&String::from("ftp://172.26.104.1"))
                .ok()
                .unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 21); // Fallback to ftp default
        assert_eq!(result.2, FileTransferProtocol::Ftp(false));
        assert!(result.3.is_none()); // Doesn't fall back
                                     // Protocol
        let result: (String, u16, FileTransferProtocol, Option<String>) =
            parse_remote_opt(&String::from("sftp://172.26.104.1"))
                .ok()
                .unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 22); // Fallback to sftp default
        assert_eq!(result.2, FileTransferProtocol::Sftp);
        assert!(result.3.is_some()); // Doesn't fall back
        let result: (String, u16, FileTransferProtocol, Option<String>) =
            parse_remote_opt(&String::from("scp://172.26.104.1"))
                .ok()
                .unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 22); // Fallback to scp default
        assert_eq!(result.2, FileTransferProtocol::Scp);
        assert!(result.3.is_some()); // Doesn't fall back
                                     // Protocol + user
        let result: (String, u16, FileTransferProtocol, Option<String>) =
            parse_remote_opt(&String::from("ftps://anon@172.26.104.1"))
                .ok()
                .unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 21); // Fallback to ftp default
        assert_eq!(result.2, FileTransferProtocol::Ftp(true));
        assert_eq!(result.3.unwrap(), String::from("anon"));
        // All together now
        let result: (String, u16, FileTransferProtocol, Option<String>) =
            parse_remote_opt(&String::from("ftp://anon@172.26.104.1:8021"))
                .ok()
                .unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 8021); // Fallback to ftp default
        assert_eq!(result.2, FileTransferProtocol::Ftp(false));
        assert_eq!(result.3.unwrap(), String::from("anon"));

        // bad syntax
        assert!(parse_remote_opt(&String::from("://172.26.104.1")).is_err()); // Missing protocol
        assert!(parse_remote_opt(&String::from("omar://172.26.104.1")).is_err()); // Bad protocol
        assert!(parse_remote_opt(&String::from("172.26.104.1:abc")).is_err()); // Bad port
    }

    #[test]
    fn test_utils_parse_lstime() {
        // Good cases
        assert_eq!(
            parse_lstime("Nov 5 16:32", "%b %d %Y", "%b %d %H:%M")
                .ok()
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1604593920)
        );
        assert_eq!(
            parse_lstime("Dec 2 21:32", "%b %d %Y", "%b %d %H:%M")
                .ok()
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1606944720)
        );
        assert_eq!(
            parse_lstime("Nov 5 2018", "%b %d %Y", "%b %d %H:%M")
                .ok()
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1541376000)
        );
        assert_eq!(
            parse_lstime("Mar 18 2018", "%b %d %Y", "%b %d %H:%M")
                .ok()
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1521331200)
        );
        // bad cases
        assert!(parse_lstime("Oma 31 2018", "%b %d %Y", "%b %d %H:%M").is_err());
        assert!(parse_lstime("Feb 31 2018", "%b %d %Y", "%b %d %H:%M").is_err());
        assert!(parse_lstime("Feb 15 25:32", "%b %d %Y", "%b %d %H:%M").is_err());
    }

    #[test]
    fn test_utils_parse_datetime() {
        assert_eq!(
            parse_datetime("04-08-14  03:09PM", "%d-%m-%y %I:%M%p")
                .ok()
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1407164940)
        );
        // Not enough argument for datetime
        assert!(parse_datetime("04-08-14", "%d-%m-%y").is_err());
    }
}
