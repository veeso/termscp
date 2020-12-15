//! ## AuthActivity
//!
//! `auth_activity` is the module which implements the authentication activity

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
extern crate dirs;

// Locals
use super::{AuthActivity, Color, FileTransferProtocol, InputMode, PopupType, UserHosts};
use crate::bookmarks::serializer::BookmarkSerializer;
use crate::bookmarks::Bookmark;
use crate::utils::time_to_str;

// Ext
use std::path::PathBuf;
use std::time::SystemTime;

impl AuthActivity {
    /// ### read_bookmarks
    ///
    /// Read bookmarks from data file; Show popup if necessary
    pub(super) fn read_bookmarks(&mut self) {
        // Init bookmarks
        if let Some(bookmark_file) = self.init_bookmarks() {
            // Read
            if self.context.is_some() {
                match self
                    .context
                    .as_ref()
                    .unwrap()
                    .local
                    .open_file_read(bookmark_file.as_path())
                {
                    Ok(reader) => {
                        // Read bookmarks
                        let deserializer: BookmarkSerializer = BookmarkSerializer {};
                        match deserializer.deserialize(Box::new(reader)) {
                            Ok(bookmarks) => self.bookmarks = Some(bookmarks),
                            Err(err) => {
                                self.input_mode = InputMode::Popup(PopupType::Alert(
                                    Color::Yellow,
                                    format!(
                                        "Could not read bookmarks from \"{}\": {}",
                                        bookmark_file.display(),
                                        err
                                    ),
                                ))
                            }
                        }
                    }
                    Err(err) => {
                        self.input_mode = InputMode::Popup(PopupType::Alert(
                            Color::Yellow,
                            format!(
                                "Could not read bookmarks from \"{}\": {}",
                                bookmark_file.display(),
                                err
                            ),
                        ))
                    }
                }
            }
        }
    }

    /// ### del_bookmark
    ///
    /// Delete bookmark
    pub(super) fn del_bookmark(&mut self, name: String) {
        if let Some(hosts) = self.bookmarks.as_mut() {
            if hosts.bookmarks.contains_key(name.as_str()) {
                hosts.bookmarks.remove(name.as_str());
            }
        }
    }

    /// ### save_bookmark
    ///
    /// Save current input fields as a bookmark
    pub(super) fn save_bookmark(&mut self, name: String) {
        if let Ok(host) = self.make_user_host() {
            if let Some(hosts) = self.bookmarks.as_mut() {
                hosts.bookmarks.insert(name, host);
                // Write bookmarks
                self.write_bookmarks();
            }
        }
    }

    /// ### save_recent
    ///
    /// Save current input fields as a "recent"
    pub(super) fn save_recent(&mut self) {
        if let Ok(host) = self.make_user_host() {
            if let Some(hosts) = self.bookmarks.as_mut() {
                // Check if duplicated
                for recent_host in hosts.recents.values() {
                    if *recent_host == host {
                        // Don't save duplicates
                        return;
                    }
                }
                // If hosts size is bigger than 16; pop last
                if hosts.recents.len() >= 16 {
                    let mut keys: Vec<String> = Vec::with_capacity(hosts.recents.len());
                    for key in hosts.recents.keys() {
                        keys.push(key.clone());
                    }
                    // Sort keys; NOTE: most recent is the last element
                    keys.sort();
                    // Delete keys starting from the last one
                    for key in keys.iter() {
                        let _ = hosts.recents.remove(key);
                        // If length is < 16; break
                        if hosts.recents.len() < 16 {
                            break;
                        }
                    }
                }
                // Create name
                let name: String = time_to_str(SystemTime::now(), "ISO%Y%m%dT%H%M%S");
                // Save host to recents
                hosts.recents.insert(name, host);
                // Write bookmarks
                self.write_bookmarks();
            }
        }
    }

    /// ### make_user_host
    ///
    /// Make user host from current input fields
    fn make_user_host(&mut self) -> Result<Bookmark, ()> {
        // Check port
        let port: u16 = match self.port.parse::<usize>() {
            Ok(val) => {
                if val > 65535 {
                    self.input_mode = InputMode::Popup(PopupType::Alert(
                        Color::Red,
                        String::from("Specified port must be in range 0-65535"),
                    ));
                    return Err(());
                }
                val as u16
            }
            Err(_) => {
                self.input_mode = InputMode::Popup(PopupType::Alert(
                    Color::Red,
                    String::from("Specified port is not a number"),
                ));
                return Err(());
            }
        };
        Ok(Bookmark {
            address: self.address.clone(),
            port: port,
            protocol: match self.protocol {
                FileTransferProtocol::Ftp(secure) => match secure {
                    true => String::from("FTPS"),
                    false => String::from("FTP"),
                },
                FileTransferProtocol::Scp => String::from("SCP"),
                FileTransferProtocol::Sftp => String::from("SFTP"),
            },
            username: self.username.clone(),
        })
    }

    /// ### write_bookmarks
    ///
    /// Write bookmarks to file
    fn write_bookmarks(&mut self) {
        if self.bookmarks.is_some() {
            if self.context.is_some() {
                // Open file for write
                if let Some(bookmarks_file) = self.init_bookmarks() {
                    match self
                        .context
                        .as_ref()
                        .unwrap()
                        .local
                        .open_file_write(bookmarks_file.as_path())
                    {
                        Ok(writer) => {
                            let serializer: BookmarkSerializer = BookmarkSerializer {};
                            if let Err(err) = serializer
                                .serialize(Box::new(writer), &self.bookmarks.as_ref().unwrap())
                            {
                                self.input_mode = InputMode::Popup(PopupType::Alert(
                                    Color::Yellow,
                                    format!(
                                        "Could not write default bookmarks at \"{}\": {}",
                                        bookmarks_file.display(),
                                        err
                                    ),
                                ));
                            }
                        }
                        Err(err) => {
                            self.input_mode = InputMode::Popup(PopupType::Alert(
                                Color::Yellow,
                                format!(
                                    "Could not write default bookmarks at \"{}\": {}",
                                    bookmarks_file.display(),
                                    err
                                ),
                            ))
                        }
                    }
                }
            }
        }
    }

    /// ### init_bookmarks
    ///
    /// Initialize bookmarks directory
    /// Returns bookmark path
    fn init_bookmarks(&mut self) -> Option<PathBuf> {
        // Get file
        lazy_static! {
            static ref CONF_DIR: Option<PathBuf> = dirs::config_dir();
        }
        if CONF_DIR.is_some() {
            // Get path of bookmarks
            let mut p: PathBuf = CONF_DIR.as_ref().unwrap().clone();
            // Append termscp dir
            p.push("termscp/");
            // Mkdir if doesn't exist
            if self.context.is_some() {
                if let Err(err) = self
                    .context
                    .as_mut()
                    .unwrap()
                    .local
                    .mkdir_ex(p.as_path(), true)
                {
                    // Show popup
                    self.input_mode = InputMode::Popup(PopupType::Alert(
                        Color::Yellow,
                        format!(
                            "Could not create configuration directory at \"{}\": {}",
                            p.display(),
                            err
                        ),
                    ));
                    // Return None
                    return None;
                }
            }
            // Append bookmarks.toml
            p.push("bookmarks.toml");
            // If bookmarks.toml doesn't exist, initializae it
            if self.context.is_some() {
                if !self
                    .context
                    .as_ref()
                    .unwrap()
                    .local
                    .file_exists(p.as_path())
                {
                    // Write file
                    let default_hosts: UserHosts = Default::default();
                    match self
                        .context
                        .as_ref()
                        .unwrap()
                        .local
                        .open_file_write(p.as_path())
                    {
                        Ok(writer) => {
                            let serializer: BookmarkSerializer = BookmarkSerializer {};
                            // Serialize and write
                            if let Err(err) = serializer.serialize(Box::new(writer), &default_hosts)
                            {
                                self.input_mode = InputMode::Popup(PopupType::Alert(
                                    Color::Yellow,
                                    format!(
                                        "Could not write default bookmarks at \"{}\": {}",
                                        p.display(),
                                        err
                                    ),
                                ));
                                return None;
                            }
                        }
                        Err(err) => {
                            self.input_mode = InputMode::Popup(PopupType::Alert(
                                Color::Yellow,
                                format!(
                                    "Could not write default bookmarks at \"{}\": {}",
                                    p.display(),
                                    err
                                ),
                            ));
                            return None;
                        }
                    }
                }
            }
            // return path
            Some(p)
        } else {
            None
        }
    }
}
