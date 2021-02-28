//! ## Parser
//!
//! `parser` is the module which provides utilities for parsing different kind of stuff

/*
*
*   Copyright (C) 2020-2021 Christian Visintin - christian.visintin1997@gmail.com
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
extern crate regex;
extern crate whoami;

// Locals
use crate::filetransfer::FileTransferProtocol;
#[cfg(not(test))] // NOTE: don't use configuration during tests
use crate::system::config_client::ConfigClient;
#[cfg(not(test))] // NOTE: don't use configuration during tests
use crate::system::environment;

// Ext
use chrono::format::ParseError;
use chrono::prelude::*;
use regex::Regex;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, SystemTime};

// Regex
lazy_static! {
    /**
     * Regex matches:
     *  - group 1: Some(protocol) | None
     *  - group 2: Some(user) | None
     *  - group 3: Address
     *  - group 4: Some(port) | None
     *  - group 5: Some(path) | None
     */
    static ref REMOTE_OPT_REGEX: Regex = Regex::new(r"(?:([a-z]+)://)?(?:([^@]+)@)?(?:([^:]+))(?::((?:[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])(?:[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])))?(?::([^:]+))?").ok().unwrap();
    /**
     * Regex matches:
     * - group 1: Version
     * E.g. termscp-0.3.2 => 0.3.2
     *      v0.4.0 => 0.4.0
     */
    static ref SEMVER_REGEX: Regex = Regex::new(r".*(:?[0-9]\.[0-9]\.[0-9])").unwrap();
}

pub struct RemoteOptions {
    pub hostname: String,
    pub port: u16,
    pub protocol: FileTransferProtocol,
    pub username: Option<String>,
    pub wrkdir: Option<PathBuf>,
}

/// ### parse_remote_opt
///
/// Parse remote option string. Returns in case of success a RemoteOptions struct
/// For ssh if username is not provided, current user will be used.
/// In case of error, message is returned
/// If port is missing default port will be used for each protocol
///     SFTP => 22
///     FTP => 21
/// The option string has the following syntax
/// [protocol://][username@]{address}[:port][:path]
/// The only argument which is mandatory is address
/// NOTE: possible strings
/// - 172.26.104.1
/// - root@172.26.104.1
/// - sftp://root@172.26.104.1
/// - sftp://172.26.104.1:4022
/// - sftp://172.26.104.1
/// - ...
///
pub fn parse_remote_opt(remote: &str) -> Result<RemoteOptions, String> {
    // Set protocol to default protocol
    #[cfg(not(test))] // NOTE: don't use configuration during tests
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
    #[cfg(test)] // NOTE: during test set protocol just to Sftp
    let mut protocol: FileTransferProtocol = FileTransferProtocol::Sftp;
    // Match against regex
    match REMOTE_OPT_REGEX.captures(remote) {
        Some(groups) => {
            // Match protocol
            let mut port: u16 = 22;
            if let Some(group) = groups.get(1) {
                // Set protocol from group
                let (m_protocol, m_port) = match FileTransferProtocol::from_str(group.as_str()) {
                    Ok(proto) => match proto {
                        FileTransferProtocol::Ftp(_) => (proto, 21),
                        FileTransferProtocol::Scp => (proto, 22),
                        FileTransferProtocol::Sftp => (proto, 22),
                    },
                    Err(_) => return Err(format!("Unknown protocol \"{}\"", group.as_str())),
                };
                // NOTE: tuple destructuring assignment is not supported yet :(
                protocol = m_protocol;
                port = m_port;
            }
            // Match user
            let username: Option<String> = match groups.get(2) {
                Some(group) => Some(group.as_str().to_string()),
                None => match protocol {
                    // If group is empty, set to current user
                    FileTransferProtocol::Scp | FileTransferProtocol::Sftp => {
                        Some(whoami::username())
                    }
                    _ => None,
                },
            };
            // Get address
            let hostname: String = match groups.get(3) {
                Some(group) => group.as_str().to_string(),
                None => return Err(String::from("Missing address")),
            };
            // Get port
            if let Some(group) = groups.get(4) {
                port = match group.as_str().parse::<u16>() {
                    Ok(p) => p,
                    Err(err) => return Err(format!("Bad port \"{}\": {}", group.as_str(), err)),
                };
            }
            // Get workdir
            let wrkdir: Option<PathBuf> = match groups.get(5) {
                Some(group) => Some(PathBuf::from(group.as_str())),
                None => None,
            };
            Ok(RemoteOptions {
                hostname,
                port,
                protocol,
                username,
                wrkdir,
            })
        }
        None => Err(String::from("Bad remote host syntax!")),
    }
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

/// ### parse_semver
///
/// Parse semver string
pub fn parse_semver(haystack: &str) -> Option<String> {
    match SEMVER_REGEX.captures(haystack) {
        Some(groups) => match groups.get(1) {
            Some(version) => Some(version.as_str().to_string()),
            None => None,
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::fmt::fmt_time;

    #[test]
    fn test_utils_parse_remote_opt() {
        // Base case
        let result: RemoteOptions = parse_remote_opt(&String::from("172.26.104.1"))
            .ok()
            .unwrap();
        assert_eq!(result.hostname, String::from("172.26.104.1"));
        assert_eq!(result.port, 22);
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert!(result.username.is_some());
        // User case
        let result: RemoteOptions = parse_remote_opt(&String::from("root@172.26.104.1"))
            .ok()
            .unwrap();
        assert_eq!(result.hostname, String::from("172.26.104.1"));
        assert_eq!(result.port, 22);
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert_eq!(result.username.unwrap(), String::from("root"));
        assert!(result.wrkdir.is_none());
        // User + port
        let result: RemoteOptions = parse_remote_opt(&String::from("root@172.26.104.1:8022"))
            .ok()
            .unwrap();
        assert_eq!(result.hostname, String::from("172.26.104.1"));
        assert_eq!(result.port, 8022);
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert_eq!(result.username.unwrap(), String::from("root"));
        assert!(result.wrkdir.is_none());
        // Port only
        let result: RemoteOptions = parse_remote_opt(&String::from("172.26.104.1:4022"))
            .ok()
            .unwrap();
        assert_eq!(result.hostname, String::from("172.26.104.1"));
        assert_eq!(result.port, 4022);
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert!(result.username.is_some());
        assert!(result.wrkdir.is_none());
        // Protocol
        let result: RemoteOptions = parse_remote_opt(&String::from("ftp://172.26.104.1"))
            .ok()
            .unwrap();
        assert_eq!(result.hostname, String::from("172.26.104.1"));
        assert_eq!(result.port, 21); // Fallback to ftp default
        assert_eq!(result.protocol, FileTransferProtocol::Ftp(false));
        assert!(result.username.is_none()); // Doesn't fall back
        assert!(result.wrkdir.is_none());
        // Protocol
        let result: RemoteOptions = parse_remote_opt(&String::from("sftp://172.26.104.1"))
            .ok()
            .unwrap();
        assert_eq!(result.hostname, String::from("172.26.104.1"));
        assert_eq!(result.port, 22); // Fallback to sftp default
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert!(result.username.is_some()); // Doesn't fall back
        assert!(result.wrkdir.is_none());
        let result: RemoteOptions = parse_remote_opt(&String::from("scp://172.26.104.1"))
            .ok()
            .unwrap();
        assert_eq!(result.hostname, String::from("172.26.104.1"));
        assert_eq!(result.port, 22); // Fallback to scp default
        assert_eq!(result.protocol, FileTransferProtocol::Scp);
        assert!(result.username.is_some()); // Doesn't fall back
        assert!(result.wrkdir.is_none());
        // Protocol + user
        let result: RemoteOptions = parse_remote_opt(&String::from("ftps://anon@172.26.104.1"))
            .ok()
            .unwrap();
        assert_eq!(result.hostname, String::from("172.26.104.1"));
        assert_eq!(result.port, 21); // Fallback to ftp default
        assert_eq!(result.protocol, FileTransferProtocol::Ftp(true));
        assert_eq!(result.username.unwrap(), String::from("anon"));
        assert!(result.wrkdir.is_none());
        // Path
        let result: RemoteOptions = parse_remote_opt(&String::from("root@172.26.104.1:8022:/var"))
            .ok()
            .unwrap();
        assert_eq!(result.hostname, String::from("172.26.104.1"));
        assert_eq!(result.port, 8022);
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert_eq!(result.username.unwrap(), String::from("root"));
        assert_eq!(result.wrkdir.unwrap(), PathBuf::from("/var"));
        // Port only
        let result: RemoteOptions = parse_remote_opt(&String::from("172.26.104.1:home"))
            .ok()
            .unwrap();
        assert_eq!(result.hostname, String::from("172.26.104.1"));
        assert_eq!(result.port, 22);
        assert_eq!(result.protocol, FileTransferProtocol::Sftp);
        assert!(result.username.is_some());
        assert_eq!(result.wrkdir.unwrap(), PathBuf::from("home"));
        // All together now
        let result: RemoteOptions =
            parse_remote_opt(&String::from("ftp://anon@172.26.104.1:8021:/tmp"))
                .ok()
                .unwrap();
        assert_eq!(result.hostname, String::from("172.26.104.1"));
        assert_eq!(result.port, 8021); // Fallback to ftp default
        assert_eq!(result.protocol, FileTransferProtocol::Ftp(false));
        assert_eq!(result.username.unwrap(), String::from("anon"));
        assert_eq!(result.wrkdir.unwrap(), PathBuf::from("/tmp"));
        // bad syntax
        assert!(parse_remote_opt(&String::from("omar://172.26.104.1")).is_err()); // Bad protocol
        assert!(parse_remote_opt(&String::from("omar://172.26.104.1:650000")).is_err());
        // Bad port
    }

    #[test]
    fn test_utils_parse_lstime() {
        // Good cases
        assert_eq!(
            fmt_time(
                parse_lstime("Nov 5 16:32", "%b %d %Y", "%b %d %H:%M")
                    .ok()
                    .unwrap(),
                "%m %d %M"
            )
            .as_str(),
            "11 05 32"
        );
        assert_eq!(
            fmt_time(
                parse_lstime("Dec 2 21:32", "%b %d %Y", "%b %d %H:%M")
                    .ok()
                    .unwrap(),
                "%m %d %M"
            )
            .as_str(),
            "12 02 32"
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

    #[test]
    fn test_utils_parse_semver() {
        assert_eq!(
            parse_semver("termscp-0.3.2").unwrap(),
            String::from("0.3.2")
        );
        assert_eq!(parse_semver("v0.4.1").unwrap(), String::from("0.4.1"),);
        assert_eq!(parse_semver("1.0.0").unwrap(), String::from("1.0.0"),);
        assert!(parse_semver("v1.1").is_none());
    }
}
