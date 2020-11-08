//! ## Host
//!
//! `host` is the module which provides functionalities to host file system

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

use std::fs::{self, Metadata};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
// Metadata ext
#[cfg(any(unix, macos, linux))]
extern crate users;
#[cfg(any(unix, macos, linux))]
use std::os::unix::fs::{MetadataExt, PermissionsExt};
#[cfg(any(unix, macos, linux))]
use users::{get_group_by_gid, get_user_by_uid};

// Locals
use crate::fs::{FsDirectory, FsEntry, FsFile};

/// ## HostError
///
/// HostError provides errors related to local host file system
pub enum HostError {
    NoSuchFileOrDirectory,
    ReadonlyFile,
    DirNotAccessible,
}

/// ## Localhost
///
/// Localhost is the entity which holds the information about the current directory and host.
/// It provides functions to navigate across the local host file system
pub struct Localhost {
    wrkdir: PathBuf,
    files: Vec<FsEntry>,
}

impl Localhost {
    /// ### new
    ///
    /// Instantiates a new Localhost struct
    pub fn new(wrkdir: PathBuf) -> Result<Localhost, HostError> {
        let mut host: Localhost = Localhost {
            wrkdir: wrkdir,
            files: Vec::new(),
        };
        // Check if dir exists
        if !host.file_exists(host.wrkdir.as_path()) {
            return Err(HostError::NoSuchFileOrDirectory);
        }
        // Retrieve files for provided path
        host.files = match host.scan_dir() {
            Ok(files) => files,
            Err(err) => return Err(err),
        };
        Ok(host)
    }

    /// ### pwd
    ///
    /// Print working directory
    pub fn pwd(&self) -> PathBuf {
        self.wrkdir.clone()
    }

    /// ### list_dir
    ///
    /// List files in current directory
    pub fn list_dir(&self) -> Vec<FsEntry> {
        self.files.clone()
    }

    /// ### change_wrkdir
    ///
    /// Change working directory with the new provided directory
    pub fn change_wrkdir(&mut self, new_dir: PathBuf) -> Result<PathBuf, HostError> {
        // Check whether directory exists
        if !self.file_exists(new_dir.as_path()) {
            return Err(HostError::NoSuchFileOrDirectory);
        }
        let prev_dir: PathBuf = self.wrkdir.clone(); // Backup location
                                                     // Update working directory
        self.wrkdir = new_dir;
        // Scan new directory
        self.files = match self.scan_dir() {
            Ok(files) => files,
            Err(err) => {
                // Restore directory
                self.wrkdir = prev_dir;
                return Err(err);
            }
        };
        Ok(self.wrkdir.clone())
    }

    /// ### file_exists
    ///
    /// Returns whether provided file path exists
    fn file_exists(&self, path: &Path) -> bool {
        path.exists()
    }

    /// ### scan_dir
    ///
    /// Get content of the current directory as a list of fs entry (Windows)
    #[cfg(any(unix, macos, linux))]
    fn scan_dir(&self) -> Result<Vec<FsEntry>, HostError> {
        let entries = match std::fs::read_dir(self.wrkdir.as_path()) {
            Ok(e) => e,
            Err(_) => return Err(HostError::DirNotAccessible),
        };
        let mut fs_entries: Vec<FsEntry> = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                let path: PathBuf = entry.path();
                let attr: Metadata = fs::metadata(path.clone()).unwrap();
                let is_symlink: bool = attr.file_type().is_symlink();
                // Get user stuff
                let user: Option<String> = match get_user_by_uid(attr.uid()) {
                    Some(user) => String::from(user.name().to_str().unwrap_or("")),
                    None => None,
                };
                let group: Option<String> = match get_group_by_gid(attr.gid()) {
                    Some(gruop) => String::from(group.name().to_str().unwrap_or("")),
                    None => None,
                };
                // Match dir / file
                fs_entries.push(match path.is_dir() {
                    true => {
                        // Is dir
                        FsEntry::Directory(FsDirectory {
                            name: path.file_name(),
                            last_change_time: attr.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                            last_access_time: attr.accessed().unwrap_or(SystemTime::UNIX_EPOCH),
                            creation_time: attr.created().unwrap_or(SystemTime::UNIX_EPOCH),
                            readonly: attr.permissions().readonly(),
                            symlink: match is_symlink {
                                true => {
                                    // Read link
                                    match fs::read_link(path) {
                                        Ok(p) => Some(p),
                                        Err(_) => None,
                                    }
                                }
                                false => None,
                            },
                            user: user,
                            group: group,
                            unix_pex: Some(self.u32_to_mode(attr.mode())),
                        })
                    }
                    false => {
                        // Is File
                        let extension: Option<String> = match path.extension() {
                            Some(s) => Some(String::from(s.to_str().unwrap_or(""))),
                            None => None,
                        };
                        FsEntry::File(FsFile {
                            name: path.file_name(),
                            last_change_time: attr.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                            last_access_time: attr.accessed().unwrap_or(SystemTime::UNIX_EPOCH),
                            creation_time: attr.created().unwrap_or(SystemTime::UNIX_EPOCH),
                            readonly: attr.permissions().readonly(),
                            size: attr.len() as usize,
                            ftype: extension,
                            symlink: match is_symlink {
                                true => {
                                    // Read link
                                    match fs::read_link(path) {
                                        Ok(p) => Some(p),
                                        Err(_) => None,
                                    }
                                }
                                false => None,
                            },
                            user: user,
                            group: group,
                            unix_pex: Some(self.u32_to_mode(attr.mode())),
                        })
                    }
                });
            }
        }
        Ok(fs_entries)
    }

    /// ### scan_dir
    ///
    /// Get content of the current directory as a list of fs entry (Windows)
    #[cfg(target_os = "windows")]
    #[cfg(not(tarpaulin_include))]
    fn scan_dir(&self) -> Result<Vec<FsEntry>, HostError> {
        let entries = match std::fs::read_dir(self.wrkdir.as_path()) {
            Ok(e) => e,
            Err(_) => return Err(HostError::DirNotAccessible),
        };
        let mut fs_entries: Vec<FsEntry> = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                let path: PathBuf = entry.path();
                let attr: Metadata = fs::metadata(path.clone()).unwrap();
                fs_entries.push(match path.is_dir() {
                    true => {
                        // Is dir
                        FsEntry::Directory(FsDirectory {
                            name: path,
                            last_change_time: attr.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                            last_access_time: attr.accessed().unwrap_or(SystemTime::UNIX_EPOCH),
                            creation_time: attr.created().unwrap_or(SystemTime::UNIX_EPOCH),
                            readonly: attr.permissions().readonly(),
                            symlink: None,
                            user: None,
                            group: None,
                            unix_pex: None,
                        })
                    }
                    false => {
                        // Is File
                        let extension: Option<String> = match path.extension() {
                            Some(s) => Some(String::from(s.to_str().unwrap_or(""))),
                            None => None,
                        };
                        FsEntry::File(FsFile {
                            name: path,
                            last_change_time: attr.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                            last_access_time: attr.accessed().unwrap_or(SystemTime::UNIX_EPOCH),
                            creation_time: attr.created().unwrap_or(SystemTime::UNIX_EPOCH),
                            readonly: attr.permissions().readonly(),
                            size: attr.len() as usize,
                            ftype: extension,
                            symlink: None,
                            user: None,
                            group: None,
                            unix_pex: None,
                        })
                    }
                });
            }
        }
        Ok(fs_entries)
    }

    /// ### u32_to_mode
    ///
    /// Return string with format xxxxxx to tuple of permissions (user, group, others)
    #[cfg(any(unix, macos, linux))]
    fn u32_to_mode(&self, mode: u32) -> (u8, u8, u8) {
        let user: u8 = ((mode >> 6) & 0x7) as u8;
        let group: u8 = ((mode >> 3) & 0x7) as u8;
        let others: u8 = (mode & 0x7) as u8;
        (user, group, others)
    }
}
