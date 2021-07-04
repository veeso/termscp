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

impl Default for UserHosts {
    fn default() -> Self {
        Self {
            bookmarks: HashMap::new(),
            recents: HashMap::new(),
        }
    }
}

// Tests

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_bookmarks_default() {
        let bookmarks: UserHosts = UserHosts::default();
        assert_eq!(bookmarks.bookmarks.len(), 0);
        assert_eq!(bookmarks.recents.len(), 0);
    }

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
}
