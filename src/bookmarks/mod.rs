//! ## Bookmarks
//!
//! `bookmarks` is the module which provides data types and de/serializer for bookmarks

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

pub mod serializer;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, std::fmt::Debug)]
/// ## UserHosts
///
/// UserHosts contains all the hosts saved by the user in the data storage
/// It contains both `Bookmark`
pub struct UserHosts {
    pub bookmarks: HashMap<String, Bookmark>,
    pub recents: HashMap<String, Bookmark>,
}

#[derive(Deserialize, Serialize, std::fmt::Debug, PartialEq)]
/// ## Bookmark
///
/// Bookmark describes a single bookmark entry in the user hosts storage
pub struct Bookmark {
    pub address: String,
    pub port: u16,
    pub protocol: String,
    pub username: String,
    pub password: Option<String>, // Password is optional; base64, aes-128 encrypted password
}

// Errors

/// ## SerializerError
///
/// Contains the error for serializer/deserializer
#[derive(std::fmt::Debug)]
pub struct SerializerError {
    kind: SerializerErrorKind,
    msg: Option<String>,
}

/// ## SerializerErrorKind
///
/// Describes the kind of error for the serializer/deserializer
#[derive(std::fmt::Debug, PartialEq)]
pub enum SerializerErrorKind {
    IoError,
    SerializationError,
    SyntaxError,
}

impl Default for UserHosts {
    fn default() -> Self {
        UserHosts {
            bookmarks: HashMap::new(),
            recents: HashMap::new(),
        }
    }
}

impl SerializerError {
    /// ### new
    ///
    /// Instantiate a new `SerializerError`
    pub fn new(kind: SerializerErrorKind) -> SerializerError {
        SerializerError { kind, msg: None }
    }

    /// ### new_ex
    ///
    /// Instantiates a new `SerializerError` with description message
    pub fn new_ex(kind: SerializerErrorKind, msg: String) -> SerializerError {
        let mut err: SerializerError = SerializerError::new(kind);
        err.msg = Some(msg);
        err
    }
}

impl std::fmt::Display for SerializerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let err: String = match &self.kind {
            SerializerErrorKind::IoError => String::from("IO error"),
            SerializerErrorKind::SerializationError => String::from("Serialization error"),
            SerializerErrorKind::SyntaxError => String::from("Syntax error"),
        };
        match &self.msg {
            Some(msg) => write!(f, "{} ({})", err, msg),
            None => write!(f, "{}", err),
        }
    }
}

// Tests

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bookmarks_bookmark_new() {
        let bookmark: Bookmark = Bookmark {
            address: String::from("192.168.1.1"),
            port: 22,
            protocol: String::from("SFTP"),
            username: String::from("root"),
            password: Some(String::from("password")),
        };
        let recent: Bookmark = Bookmark {
            address: String::from("192.168.1.2"),
            port: 22,
            protocol: String::from("SCP"),
            username: String::from("admin"),
            password: Some(String::from("password")),
        };
        let mut bookmarks: HashMap<String, Bookmark> = HashMap::with_capacity(1);
        bookmarks.insert(String::from("test"), bookmark);
        let mut recents: HashMap<String, Bookmark> = HashMap::with_capacity(1);
        recents.insert(String::from("ISO20201218T181432"), recent);
        let hosts: UserHosts = UserHosts {
            bookmarks: bookmarks,
            recents: recents,
        };
        // Verify
        let bookmark: &Bookmark = hosts.bookmarks.get(&String::from("test")).unwrap();
        assert_eq!(bookmark.address, String::from("192.168.1.1"));
        assert_eq!(bookmark.port, 22);
        assert_eq!(bookmark.protocol, String::from("SFTP"));
        assert_eq!(bookmark.username, String::from("root"));
        assert_eq!(
            *bookmark.password.as_ref().unwrap(),
            String::from("password")
        );
        let bookmark: &Bookmark = hosts
            .recents
            .get(&String::from("ISO20201218T181432"))
            .unwrap();
        assert_eq!(bookmark.address, String::from("192.168.1.2"));
        assert_eq!(bookmark.port, 22);
        assert_eq!(bookmark.protocol, String::from("SCP"));
        assert_eq!(bookmark.username, String::from("admin"));
        assert_eq!(
            *bookmark.password.as_ref().unwrap(),
            String::from("password")
        );
    }

    #[test]
    fn test_bookmarks_bookmark_errors() {
        let error: SerializerError = SerializerError::new(SerializerErrorKind::SyntaxError);
        assert_eq!(error.kind, SerializerErrorKind::SyntaxError);
        assert!(error.msg.is_none());
        assert_eq!(format!("{}", error), String::from("Syntax error"));
        let error: SerializerError =
            SerializerError::new_ex(SerializerErrorKind::SyntaxError, String::from("bad syntax"));
        assert_eq!(error.kind, SerializerErrorKind::SyntaxError);
        assert!(error.msg.is_some());
        assert_eq!(
            format!("{}", error),
            String::from("Syntax error (bad syntax)")
        );
        // Fmt
        assert_eq!(
            format!("{}", SerializerError::new(SerializerErrorKind::IoError)),
            String::from("IO error")
        );
        assert_eq!(
            format!(
                "{}",
                SerializerError::new(SerializerErrorKind::SerializationError)
            ),
            String::from("Serialization error")
        );
    }
}
