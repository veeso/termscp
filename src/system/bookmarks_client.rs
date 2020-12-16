//! ## BookmarksClient
//!
//! `bookmarks_client` is the module which provides an API between the Bookmarks module and the system

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

// Deps
extern crate magic_crypt;
extern crate rand;

// Local
use crate::bookmarks::serializer::BookmarkSerializer;
use crate::bookmarks::{Bookmark, SerializerError, SerializerErrorKind, UserHosts};
use crate::filetransfer::FileTransferProtocol;
use crate::utils::time_to_str;
// Ext
use magic_crypt::MagicCryptTrait;
use rand::{distributions::Alphanumeric, Rng};
use std::fs::{OpenOptions, Permissions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// ## BookmarksClient
///
/// BookmarksClient provides a layer between the host system and the bookmarks module
pub struct BookmarksClient {
    hosts: UserHosts,
    bookmarks_file: PathBuf,
    key: String,
}

impl BookmarksClient {
    /// ### BookmarksClient
    ///
    /// Instantiates a new BookmarksClient
    /// Bookmarks file path must be provided
    /// Key file must be provided
    pub fn new(bookmarks_file: &Path, key_file: &Path) -> Result<BookmarksClient, SerializerError> {
        // Create default hosts
        let default_hosts: UserHosts = Default::default();
        // If key file doesn't exist, create key, otherwise read it
        let key: String = match key_file.exists() {
            true => match BookmarksClient::load_key(key_file) {
                Ok(key) => key,
                Err(err) => return Err(err),
            },
            false => match BookmarksClient::generate_key(key_file) {
                Ok(key) => key,
                Err(err) => return Err(err),
            },
        };
        let mut client: BookmarksClient = BookmarksClient {
            hosts: default_hosts,
            bookmarks_file: PathBuf::from(bookmarks_file),
            key,
        };
        // If bookmark file doesn't exist, initialize it
        if !bookmarks_file.exists() {
            if let Err(err) = client.write_bookmarks() {
                return Err(err);
            }
        } else {
            // Load bookmarks from file
            if let Err(err) = client.read_bookmarks() {
                return Err(err);
            }
        }
        // Load key
        Ok(client)
    }

    /// ### iter_bookmarks
    ///
    /// Iterate over bookmarks keys
    pub fn iter_bookmarks(&self) -> Box<dyn Iterator<Item = &String> + '_> {
        Box::new(self.hosts.bookmarks.keys())
    }

    /// ### get_bookmark
    ///
    /// Get bookmark associated to key
    pub fn get_bookmark(
        &self,
        key: &str,
    ) -> Option<(String, u16, FileTransferProtocol, String, Option<String>)> {
        let entry: &Bookmark = self.hosts.bookmarks.get(key)?;
        Some((
            entry.address.clone(),
            entry.port,
            match entry.protocol.to_ascii_uppercase().as_str() {
                "FTP" => FileTransferProtocol::Ftp(false),
                "FTPS" => FileTransferProtocol::Ftp(true),
                "SCP" => FileTransferProtocol::Scp,
                _ => FileTransferProtocol::Sftp,
            },
            entry.username.clone(),
            match &entry.password {
                // Decrypted password if Some; if decryption fails return None
                Some(pwd) => match self.decrypt_str(pwd.as_str()) {
                    Ok(decrypted_pwd) => Some(decrypted_pwd),
                    Err(_) => None,
                },
                None => None,
            },
        ))
    }

    /// ### add_recent
    ///
    /// Add a new recent to bookmarks
    pub fn add_bookmark(
        &mut self,
        name: String,
        addr: String,
        port: u16,
        protocol: FileTransferProtocol,
        username: String,
        password: Option<String>,
    ) {
        if name.is_empty() {
            panic!("Bookmark name can't be empty");
        }
        // Make bookmark
        let host: Bookmark = self.make_bookmark(addr, port, protocol, username, password);
        self.hosts.bookmarks.insert(name, host);
    }

    /// ### del_bookmark
    ///
    /// Delete entry from bookmarks
    pub fn del_bookmark(&mut self, name: &str) {
        let _ = self.hosts.bookmarks.remove(name);
    }
    /// ### iter_recents
    ///
    /// Iterate over recents keys
    pub fn iter_recents(&self) -> Box<dyn Iterator<Item = &String> + '_> {
        Box::new(self.hosts.recents.keys())
    }

    /// ### get_recent
    ///
    /// Get recent associated to key
    pub fn get_recent(&self, key: &str) -> Option<(String, u16, FileTransferProtocol, String)> {
        // NOTE: password is not decrypted; recents will never have password
        let entry: &Bookmark = self.hosts.recents.get(key)?;
        Some((
            entry.address.clone(),
            entry.port,
            match entry.protocol.to_ascii_uppercase().as_str() {
                "FTP" => FileTransferProtocol::Ftp(false),
                "FTPS" => FileTransferProtocol::Ftp(true),
                "SCP" => FileTransferProtocol::Scp,
                _ => FileTransferProtocol::Sftp,
            },
            entry.username.clone(),
        ))
    }

    /// ### add_recent
    ///
    /// Add a new recent to bookmarks
    pub fn add_recent(
        &mut self,
        addr: String,
        port: u16,
        protocol: FileTransferProtocol,
        username: String,
    ) {
        // Make bookmark
        let host: Bookmark = self.make_bookmark(addr, port, protocol, username, None);
        // Check if duplicated
        for recent_host in self.hosts.recents.values() {
            if *recent_host == host {
                // Don't save duplicates
                return;
            }
        }
        // If hosts size is bigger than 16; pop last
        if self.hosts.recents.len() >= 16 {
            let mut keys: Vec<String> = Vec::with_capacity(self.hosts.recents.len());
            for key in self.hosts.recents.keys() {
                keys.push(key.clone());
            }
            // Sort keys; NOTE: most recent is the last element
            keys.sort();
            // Delete keys starting from the last one
            for key in keys.iter() {
                let _ = self.hosts.recents.remove(key);
                // If length is < 16; break
                if self.hosts.recents.len() < 16 {
                    break;
                }
            }
        }
        let name: String = time_to_str(SystemTime::now(), "ISO%Y%m%dT%H%M%S");
        self.hosts.recents.insert(name, host);
    }

    /// ### del_recent
    ///
    /// Delete entry from recents
    pub fn del_recent(&mut self, name: &str) {
        let _ = self.hosts.recents.remove(name);
    }

    /// ### write_bookmarks
    ///
    /// Write bookmarks to file
    pub fn write_bookmarks(&self) -> Result<(), SerializerError> {
        // Open file
        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.bookmarks_file.as_path())
        {
            Ok(writer) => {
                let serializer: BookmarkSerializer = BookmarkSerializer {};
                serializer.serialize(Box::new(writer), &self.hosts)
            }
            Err(err) => Err(SerializerError::new_ex(
                SerializerErrorKind::IoError,
                err.to_string(),
            )),
        }
    }

    /// ### read_bookmarks
    ///
    /// Read bookmarks from file
    fn read_bookmarks(&mut self) -> Result<(), SerializerError> {
        // Open bookmarks file for read
        match OpenOptions::new()
            .read(true)
            .open(self.bookmarks_file.as_path())
        {
            Ok(reader) => {
                // Deserialize
                let deserializer: BookmarkSerializer = BookmarkSerializer {};
                match deserializer.deserialize(Box::new(reader)) {
                    Ok(hosts) => {
                        self.hosts = hosts;
                        Ok(())
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(SerializerError::new_ex(
                SerializerErrorKind::IoError,
                err.to_string(),
            )),
        }
    }

    /// ### generate_key
    ///
    /// Generate a new AES key and write it to key file
    fn generate_key(key_file: &Path) -> Result<String, SerializerError> {
        // Generate 256 bytes (2048 bits) key
        let key: String = rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(256)
            .collect::<String>();
        // Write file
        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(key_file)
        {
            Ok(mut file) => {
                // Write key to file
                if let Err(err) = file.write_all(key.as_bytes()) {
                    return Err(SerializerError::new_ex(
                        SerializerErrorKind::IoError,
                        err.to_string(),
                    ));
                }
                // Set file to readonly
                let mut permissions: Permissions = file.metadata().unwrap().permissions();
                permissions.set_readonly(true);
                let _ = file.set_permissions(permissions);
                Ok(key)
            }
            Err(err) => Err(SerializerError::new_ex(
                SerializerErrorKind::IoError,
                err.to_string(),
            )),
        }
    }

    /// ### make_bookmark
    ///
    /// Make bookmark from credentials
    fn make_bookmark(
        &self,
        addr: String,
        port: u16,
        protocol: FileTransferProtocol,
        username: String,
        password: Option<String>,
    ) -> Bookmark {
        Bookmark {
            address: addr,
            port,
            username,
            protocol: match protocol {
                FileTransferProtocol::Ftp(secure) => match secure {
                    true => String::from("FTPS"),
                    false => String::from("FTP"),
                },
                FileTransferProtocol::Scp => String::from("SCP"),
                FileTransferProtocol::Sftp => String::from("SFTP"),
            },
            password: match password {
                Some(p) => Some(self.encrypt_str(p.as_str())), // Encrypt password if provided
                None => None,
            },
        }
    }

    /// ### load_key
    ///
    /// Load key from key_file
    fn load_key(key_file: &Path) -> Result<String, SerializerError> {
        match OpenOptions::new().read(true).open(key_file) {
            Ok(mut file) => {
                let mut key: String = String::with_capacity(256);
                match file.read_to_string(&mut key) {
                    Ok(_) => Ok(key),
                    Err(err) => Err(SerializerError::new_ex(
                        SerializerErrorKind::IoError,
                        err.to_string(),
                    )),
                }
            }
            Err(err) => Err(SerializerError::new_ex(
                SerializerErrorKind::IoError,
                err.to_string(),
            )),
        }
    }

    /// ### encrypt_str
    ///
    /// Encrypt provided string using AES-128. Encrypted buffer is then converted to BASE64
    fn encrypt_str(&self, txt: &str) -> String {
        let crypter = new_magic_crypt!(self.key.clone(), 128);
        crypter.encrypt_str_to_base64(txt.to_string())
    }

    /// ### decrypt_str
    ///
    /// Decrypt provided string using AES-128
    fn decrypt_str(&self, secret: &str) -> Result<String, SerializerError> {
        let crypter = new_magic_crypt!(self.key.clone(), 128);
        match crypter.decrypt_base64_to_string(secret.to_string()) {
            Ok(txt) => Ok(txt),
            Err(err) => Err(SerializerError::new_ex(
                SerializerErrorKind::SyntaxError,
                err.to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_system_bookmarks_new() {
        let tmp_dir: tempfile::TempDir = create_tmp_dir();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        // Initialize a new bookmarks client
        let client: BookmarksClient =
            BookmarksClient::new(cfg_path.as_path(), key_path.as_path()).unwrap();
        // Verify client
        assert_eq!(client.hosts.bookmarks.len(), 0);
        assert_eq!(client.hosts.recents.len(), 0);
        assert!(client.key.len() > 0);
        assert_eq!(client.bookmarks_file, cfg_path);
    }

    #[test]
    fn test_system_bookmarks_new_err() {
        assert!(
            BookmarksClient::new(Path::new("/tmp/oifoif/omar"), Path::new("/tmp/efnnu/omar"))
                .is_err()
        );
    }

    #[test]
    fn test_system_bookmarks_new_from_existing() {
        let tmp_dir: tempfile::TempDir = create_tmp_dir();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        // Initialize a new bookmarks client
        let mut client: BookmarksClient =
            BookmarksClient::new(cfg_path.as_path(), key_path.as_path()).unwrap();
        // Add some bookmarks
        client.add_bookmark(
            String::from("raspberry"),
            String::from("192.168.1.31"),
            22,
            FileTransferProtocol::Sftp,
            String::from("pi"),
            Some(String::from("mypassword")),
        );
        client.add_recent(
            String::from("192.168.1.31"),
            22,
            FileTransferProtocol::Sftp,
            String::from("pi"),
        );
        let recent_key: String = String::from(client.iter_recents().next().unwrap());
        assert!(client.write_bookmarks().is_ok());
        let key: String = client.key.clone();
        // Re-initialize a client
        let client: BookmarksClient =
            BookmarksClient::new(cfg_path.as_path(), key_path.as_path()).unwrap();
        // Verify it loaded parameters correctly
        assert_eq!(client.key, key);
        let bookmark: (String, u16, FileTransferProtocol, String, Option<String>) =
            client.get_bookmark(&String::from("raspberry")).unwrap();
        assert_eq!(bookmark.0, String::from("192.168.1.31"));
        assert_eq!(bookmark.1, 22);
        assert_eq!(bookmark.2, FileTransferProtocol::Sftp);
        assert_eq!(bookmark.3, String::from("pi"));
        assert_eq!(*bookmark.4.as_ref().unwrap(), String::from("mypassword"));
        let bookmark: (String, u16, FileTransferProtocol, String) =
            client.get_recent(&recent_key).unwrap();
        assert_eq!(bookmark.0, String::from("192.168.1.31"));
        assert_eq!(bookmark.1, 22);
        assert_eq!(bookmark.2, FileTransferProtocol::Sftp);
        assert_eq!(bookmark.3, String::from("pi"));
    }

    #[test]
    fn test_system_bookmarks_manipulate_bookmarks() {
        let tmp_dir: tempfile::TempDir = create_tmp_dir();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        // Initialize a new bookmarks client
        let mut client: BookmarksClient =
            BookmarksClient::new(cfg_path.as_path(), key_path.as_path()).unwrap();
        // Add bookmark
        client.add_bookmark(
            String::from("raspberry"),
            String::from("192.168.1.31"),
            22,
            FileTransferProtocol::Sftp,
            String::from("pi"),
            Some(String::from("mypassword")),
        );
        // Get bookmark
        let bookmark: (String, u16, FileTransferProtocol, String, Option<String>) =
            client.get_bookmark(&String::from("raspberry")).unwrap();
        assert_eq!(bookmark.0, String::from("192.168.1.31"));
        assert_eq!(bookmark.1, 22);
        assert_eq!(bookmark.2, FileTransferProtocol::Sftp);
        assert_eq!(bookmark.3, String::from("pi"));
        assert_eq!(*bookmark.4.as_ref().unwrap(), String::from("mypassword"));
        // Write bookmarks
        assert!(client.write_bookmarks().is_ok());
        // Delete bookmark
        client.del_bookmark(&String::from("raspberry"));
        // Get unexisting bookmark
        assert!(client.get_bookmark(&String::from("raspberry")).is_none());
        // Write bookmarks
        assert!(client.write_bookmarks().is_ok());
    }

    #[test]
    fn test_system_bookmarks_manipulate_recents() {
        let tmp_dir: tempfile::TempDir = create_tmp_dir();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        // Initialize a new bookmarks client
        let mut client: BookmarksClient =
            BookmarksClient::new(cfg_path.as_path(), key_path.as_path()).unwrap();
        // Add bookmark
        client.add_recent(
            String::from("192.168.1.31"),
            22,
            FileTransferProtocol::Sftp,
            String::from("pi"),
        );
        let key: String = String::from(client.iter_recents().next().unwrap());
        // Get bookmark
        let bookmark: (String, u16, FileTransferProtocol, String) =
            client.get_recent(&key).unwrap();
        assert_eq!(bookmark.0, String::from("192.168.1.31"));
        assert_eq!(bookmark.1, 22);
        assert_eq!(bookmark.2, FileTransferProtocol::Sftp);
        assert_eq!(bookmark.3, String::from("pi"));
        // Write bookmarks
        assert!(client.write_bookmarks().is_ok());
        // Delete bookmark
        client.del_recent(&key);
        // Get unexisting bookmark
        assert!(client.get_bookmark(&key).is_none());
        // Write bookmarks
        assert!(client.write_bookmarks().is_ok());
    }

    #[test]
    #[should_panic]
    fn test_system_bookmarks_add_bookmark_empty() {
        let tmp_dir: tempfile::TempDir = create_tmp_dir();
        let (cfg_path, key_path): (PathBuf, PathBuf) = get_paths(tmp_dir.path());
        // Initialize a new bookmarks client
        let mut client: BookmarksClient =
            BookmarksClient::new(cfg_path.as_path(), key_path.as_path()).unwrap();
        // Add bookmark
        client.add_bookmark(
            String::from(""),
            String::from("192.168.1.31"),
            22,
            FileTransferProtocol::Sftp,
            String::from("pi"),
            Some(String::from("mypassword")),
        );
    }

    /// ### get_paths
    ///
    /// Get paths for configuration and key for bookmarks
    fn get_paths(dir: &Path) -> (PathBuf, PathBuf) {
        let mut k: PathBuf = PathBuf::from(dir);
        let mut c: PathBuf = k.clone();
        k.push("bookmarks.key");
        c.push("bookmarks.toml");
        (c, k)
    }

    /// ### create_tmp_dir
    ///
    /// Create temporary directory
    fn create_tmp_dir() -> tempfile::TempDir {
        tempfile::TempDir::new().ok().unwrap()
    }
}
