//! ## Utils
//!
//! `utils` is the module which provides utilities of different kind

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
extern crate whoami;

use crate::ui::activities::auth_activity::ScpProtocol;

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
    remote: &String,
) -> Result<(String, u16, ScpProtocol, Option<String>), String> {
    let mut wrkstr: String = remote.clone();
    let address: String;
    let mut port: u16 = 22;
    let mut protocol: ScpProtocol = ScpProtocol::Sftp;
    let mut username: Option<String> = None;
    // Split string by '://'
    let tokens: Vec<&str> = wrkstr.split("://").collect();
    // If length is > 1, then token[0] is protocol
    match tokens.len() {
        1 => {},
        2 => {
            // Parse protocol
            match tokens[0] {
                "sftp" => {
                    // Set protocol to sftp
                    protocol = ScpProtocol::Sftp;
                    // Set port to default (22)
                    port = 22;
                }
                "ftp" | "ftps" => {
                    // Set protocol to fpt
                    protocol = ScpProtocol::Ftp;
                    // Set port to default (21)
                    port = 21;
                }
                _ => return Err(format!("Unknown protocol '{}'", tokens[0])),
            }
            wrkstr = String::from(tokens[1]); // Wrkstr becomes tokens[1]
        },
        _ => return Err(String::from("Bad syntax")), // Too many tokens...
    }
    // Set username to default if sftp
    if protocol == ScpProtocol::Sftp {
        // Set username to current username
        username = Some(whoami::username());
    }
    // Split wrkstring by '@'
    let tokens: Vec<&str> = wrkstr.split("@").collect();
    match tokens.len() {
        1 => {}
        2 => {
            // Username is first token
            username = Some(String::from(tokens[0]));
            // Update wrkstr
            wrkstr = String::from(tokens[1]);
        },
        _ => return Err(String::from("Bad syntax")), // Too many tokens...
    }
    // Split wrkstring by ':'
    let tokens: Vec<&str> = wrkstr.split(":").collect();
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
                Err(_) => return Err(format!("Port must be a number in range [0-65535], but is '{}'", tokens[1]))
            };
        },
        _ => return Err(String::from("Bad syntax")), // Too many tokens...
    }
    Ok((address, port, protocol, username))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_utils_parse_remote_opt() {
        // Base case
        let result: (String, u16, ScpProtocol, Option<String>) = parse_remote_opt(&String::from("172.26.104.1")).ok().unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 22);
        assert_eq!(result.2, ScpProtocol::Sftp);
        assert!(result.3.is_some());
        // User case
        let result: (String, u16, ScpProtocol, Option<String>) = parse_remote_opt(&String::from("root@172.26.104.1")).ok().unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 22);
        assert_eq!(result.2, ScpProtocol::Sftp);
        assert_eq!(result.3.unwrap(), String::from("root"));
        // User + port
        let result: (String, u16, ScpProtocol, Option<String>) = parse_remote_opt(&String::from("root@172.26.104.1:8022")).ok().unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 8022);
        assert_eq!(result.2, ScpProtocol::Sftp);
        assert_eq!(result.3.unwrap(), String::from("root"));
        // Port only
        let result: (String, u16, ScpProtocol, Option<String>) = parse_remote_opt(&String::from("172.26.104.1:4022")).ok().unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 4022);
        assert_eq!(result.2, ScpProtocol::Sftp);
        assert!(result.3.is_some());
        // Protocol
        let result: (String, u16, ScpProtocol, Option<String>) = parse_remote_opt(&String::from("ftp://172.26.104.1")).ok().unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 21); // Fallback to ftp default
        assert_eq!(result.2, ScpProtocol::Ftp);
        assert!(result.3.is_none()); // Doesn't fall back
        // Protocol + user
        let result: (String, u16, ScpProtocol, Option<String>) = parse_remote_opt(&String::from("ftp://anon@172.26.104.1")).ok().unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 21); // Fallback to ftp default
        assert_eq!(result.2, ScpProtocol::Ftp);
        assert_eq!(result.3.unwrap(), String::from("anon"));
        // All together now
        let result: (String, u16, ScpProtocol, Option<String>) = parse_remote_opt(&String::from("ftp://anon@172.26.104.1:8021")).ok().unwrap();
        assert_eq!(result.0, String::from("172.26.104.1"));
        assert_eq!(result.1, 8021); // Fallback to ftp default
        assert_eq!(result.2, ScpProtocol::Ftp);
        assert_eq!(result.3.unwrap(), String::from("anon"));

        // bad syntax
        assert!(parse_remote_opt(&String::from("://172.26.104.1")).is_err()); // Missing protocol
        assert!(parse_remote_opt(&String::from("omar://172.26.104.1")).is_err()); // Bad protocol
        assert!(parse_remote_opt(&String::from("172.26.104.1:abc")).is_err()); // Bad port
    }

}
