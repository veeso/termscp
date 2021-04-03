//! ## Bookmarks
//!
//! `bookmarks` is the module which provides data types and de/serializer for bookmarks

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
pub mod serializer;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

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
#[derive(Error, Debug)]
pub enum SerializerErrorKind {
    #[error("IO error")]
    IoError,
    #[error("Serialization error")]
    SerializationError,
    #[error("Syntax error")]
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
        match &self.msg {
            Some(msg) => write!(f, "{} ({})", self.kind, msg),
            None => write!(f, "{}", self.kind),
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
        assert!(error.msg.is_none());
        assert_eq!(format!("{}", error), String::from("Syntax error"));
        let error: SerializerError =
            SerializerError::new_ex(SerializerErrorKind::SyntaxError, String::from("bad syntax"));
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
