//! ## Fs
//!
//! `fs` is the module which provides file system entities

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

extern crate bytesize;
#[cfg(any(unix, macos, linux))]
extern crate users;

use crate::utils::time_to_str;

use bytesize::ByteSize;
use std::path::PathBuf;
use std::time::SystemTime;
#[cfg(any(unix, macos, linux))]
use users::{get_group_by_gid, get_user_by_uid};

/// ## FsEntry
///
/// FsEntry represents a generic entry in a directory

#[derive(Clone, std::fmt::Debug)]
pub enum FsEntry {
    Directory(FsDirectory),
    File(FsFile),
}

/// ## FsDirectory
///
/// Directory provides an interface to file system directories

#[derive(Clone, std::fmt::Debug)]
pub struct FsDirectory {
    pub name: String,
    pub abs_path: PathBuf,
    pub last_change_time: SystemTime,
    pub last_access_time: SystemTime,
    pub creation_time: SystemTime,
    pub readonly: bool,
    pub symlink: Option<PathBuf>,       // UNIX only
    pub user: Option<u32>,              // UNIX only
    pub group: Option<u32>,             // UNIX only
    pub unix_pex: Option<(u8, u8, u8)>, // UNIX only
}

/// ### FsFile
///
/// FsFile provides an interface to file system files

#[derive(Clone, std::fmt::Debug)]
pub struct FsFile {
    pub name: String,
    pub abs_path: PathBuf,
    pub last_change_time: SystemTime,
    pub last_access_time: SystemTime,
    pub creation_time: SystemTime,
    pub size: usize,
    pub ftype: Option<String>, // File type
    pub readonly: bool,
    pub symlink: Option<PathBuf>,       // UNIX only
    pub user: Option<u32>,              // UNIX only
    pub group: Option<u32>,             // UNIX only
    pub unix_pex: Option<(u8, u8, u8)>, // UNIX only
}

impl std::fmt::Display for FsEntry {
    /// ### fmt_ls
    ///
    /// Format File Entry as `ls` does
    #[cfg(any(unix, macos, linux))]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FsEntry::Directory(dir) => {
                // Create mode string
                let mut mode: String = String::with_capacity(10);
                let file_type: char = match dir.symlink {
                    Some(_) => 'l',
                    None => 'd',
                };
                mode.push(file_type);
                match dir.unix_pex {
                    None => mode.push_str("?????????"),
                    Some((owner, group, others)) => {
                        let read: u8 = (owner >> 2) & 0x1;
                        let write: u8 = (owner >> 1) & 0x1;
                        let exec: u8 = owner & 0x1;
                        mode.push_str(match read {
                            1 => "r",
                            _ => "-",
                        });
                        mode.push_str(match write {
                            1 => "w",
                            _ => "-",
                        });
                        mode.push_str(match exec {
                            1 => "x",
                            _ => "-",
                        });
                        let read: u8 = (group >> 2) & 0x1;
                        let write: u8 = (group >> 1) & 0x1;
                        let exec: u8 = group & 0x1;
                        mode.push_str(match read {
                            1 => "r",
                            _ => "-",
                        });
                        mode.push_str(match write {
                            1 => "w",
                            _ => "-",
                        });
                        mode.push_str(match exec {
                            1 => "x",
                            _ => "-",
                        });
                        let read: u8 = (others >> 2) & 0x1;
                        let write: u8 = (others >> 1) & 0x1;
                        let exec: u8 = others & 0x1;
                        mode.push_str(match read {
                            1 => "r",
                            _ => "-",
                        });
                        mode.push_str(match write {
                            1 => "w",
                            _ => "-",
                        });
                        mode.push_str(match exec {
                            1 => "x",
                            _ => "-",
                        });
                    }
                }
                // Get username
                let username: String = match dir.user {
                    Some(uid) => match get_user_by_uid(uid) {
                        Some(user) => user.name().to_string_lossy().to_string(),
                        None => uid.to_string(),
                    },
                    None => String::from("0"),
                };
                // Get group
                let group: String = match dir.group {
                    Some(gid) => match get_group_by_gid(gid) {
                        Some(group) => group.name().to_string_lossy().to_string(),
                        None => gid.to_string(),
                    },
                    None => String::from("0"),
                };
                // Get byte size
                let size: String = String::from("4096");
                // Get date
                let datetime: String = time_to_str(dir.last_change_time, "%b %d %Y %M:%H");
                write!(
                    f,
                    "{:24}\t{:12}\t{:16}\t{:16}\t{:8}\t{:17}",
                    dir.name.as_str(),
                    mode,
                    username,
                    group,
                    size,
                    datetime
                )
            }
            FsEntry::File(file) => {
                // Create mode string
                let mut mode: String = String::with_capacity(10);
                let file_type: char = match file.symlink {
                    Some(_) => 'l',
                    None => '-',
                };
                mode.push(file_type);
                match file.unix_pex {
                    None => mode.push_str("?????????"),
                    Some((owner, group, others)) => {
                        let read: u8 = (owner >> 2) & 0x1;
                        let write: u8 = (owner >> 1) & 0x1;
                        let exec: u8 = owner & 0x1;
                        mode.push_str(match read {
                            1 => "r",
                            _ => "-",
                        });
                        mode.push_str(match write {
                            1 => "w",
                            _ => "-",
                        });
                        mode.push_str(match exec {
                            1 => "x",
                            _ => "-",
                        });
                        let read: u8 = (group >> 2) & 0x1;
                        let write: u8 = (group >> 1) & 0x1;
                        let exec: u8 = group & 0x1;
                        mode.push_str(match read {
                            1 => "r",
                            _ => "-",
                        });
                        mode.push_str(match write {
                            1 => "w",
                            _ => "-",
                        });
                        mode.push_str(match exec {
                            1 => "x",
                            _ => "-",
                        });
                        let read: u8 = (others >> 2) & 0x1;
                        let write: u8 = (others >> 1) & 0x1;
                        let exec: u8 = others & 0x1;
                        mode.push_str(match read {
                            1 => "r",
                            _ => "-",
                        });
                        mode.push_str(match write {
                            1 => "w",
                            _ => "-",
                        });
                        mode.push_str(match exec {
                            1 => "x",
                            _ => "-",
                        });
                    }
                }
                // Get username
                let username: String = match file.user {
                    Some(uid) => match get_user_by_uid(uid) {
                        Some(user) => user.name().to_string_lossy().to_string(),
                        None => uid.to_string(),
                    },
                    None => String::from("0"),
                };
                // Get group
                let group: String = match file.group {
                    Some(gid) => match get_group_by_gid(gid) {
                        Some(group) => group.name().to_string_lossy().to_string(),
                        None => gid.to_string(),
                    },
                    None => String::from("0"),
                };
                // Get byte size
                let size: ByteSize = ByteSize(file.size as u64);
                // Get date
                let datetime: String = time_to_str(file.last_change_time, "%b %d %Y %M:%H");
                write!(
                    f,
                    "{:24}\t{:12}\t{:16}\t{:16}\t{:8}\t{:17}",
                    file.name.as_str(),
                    mode,
                    username,
                    group,
                    size,
                    datetime
                )
            }
        }
    }

    /// ### fmt_ls
    ///
    /// Format File Entry as `ls` does
    #[cfg(target_os = "windows")]
    #[cfg(not(tarpaulin_include))]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FsEntry::Directory(dir) => {
                // Create mode string
                let mut mode: String = String::with_capacity(10);
                let file_type: char = match dir.symlink {
                    Some(_) => 'l',
                    None => 'd',
                };
                mode.push(file_type);
                match dir.unix_pex {
                    None => mode.push_str("?????????"),
                    Some((owner, group, others)) => {
                        let read: u8 = (owner >> 2) & 0x1;
                        let write: u8 = (owner >> 1) & 0x1;
                        let exec: u8 = owner & 0x1;
                        mode.push_str(match read {
                            1 => "r",
                            _ => "-",
                        });
                        mode.push_str(match write {
                            1 => "w",
                            _ => "-",
                        });
                        mode.push_str(match exec {
                            1 => "x",
                            _ => "-",
                        });
                        let read: u8 = (group >> 2) & 0x1;
                        let write: u8 = (group >> 1) & 0x1;
                        let exec: u8 = group & 0x1;
                        mode.push_str(match read {
                            1 => "r",
                            _ => "-",
                        });
                        mode.push_str(match write {
                            1 => "w",
                            _ => "-",
                        });
                        mode.push_str(match exec {
                            1 => "x",
                            _ => "-",
                        });
                        let read: u8 = (others >> 2) & 0x1;
                        let write: u8 = (others >> 1) & 0x1;
                        let exec: u8 = others & 0x1;
                        mode.push_str(match read {
                            1 => "r",
                            _ => "-",
                        });
                        mode.push_str(match write {
                            1 => "w",
                            _ => "-",
                        });
                        mode.push_str(match exec {
                            1 => "x",
                            _ => "-",
                        });
                    }
                }
                // Get username
                let username: String = match dir.user {
                    Some(uid) => uid.to_string(),
                    None => String::from("0"),
                };
                // Get group
                let group: String = match dir.group {
                    Some(gid) => gid.to_string(),
                    None => String::from("0"),
                };
                // Get byte size
                let size: String = String::from("4096");
                // Get date
                let datetime: String = time_to_str(dir.last_change_time, "%b %d %Y %M:%H");
                write!(
                    f,
                    "{:24}\t{:12}\t{:16}\t{:16}\t{:8}\t{:17}",
                    dir.name.as_str(),
                    mode,
                    username,
                    group,
                    size,
                    datetime
                )
            }
            FsEntry::File(file) => {
                // Create mode string
                let mut mode: String = String::with_capacity(10);
                let file_type: char = match file.symlink {
                    Some(_) => 'l',
                    None => '-',
                };
                mode.push(file_type);
                match file.unix_pex {
                    None => mode.push_str("?????????"),
                    Some((owner, group, others)) => {
                        let read: u8 = (owner >> 2) & 0x1;
                        let write: u8 = (owner >> 1) & 0x1;
                        let exec: u8 = owner & 0x1;
                        mode.push_str(match read {
                            1 => "r",
                            _ => "-",
                        });
                        mode.push_str(match write {
                            1 => "w",
                            _ => "-",
                        });
                        mode.push_str(match exec {
                            1 => "x",
                            _ => "-",
                        });
                        let read: u8 = (group >> 2) & 0x1;
                        let write: u8 = (group >> 1) & 0x1;
                        let exec: u8 = group & 0x1;
                        mode.push_str(match read {
                            1 => "r",
                            _ => "-",
                        });
                        mode.push_str(match write {
                            1 => "w",
                            _ => "-",
                        });
                        mode.push_str(match exec {
                            1 => "x",
                            _ => "-",
                        });
                        let read: u8 = (others >> 2) & 0x1;
                        let write: u8 = (others >> 1) & 0x1;
                        let exec: u8 = others & 0x1;
                        mode.push_str(match read {
                            1 => "r",
                            _ => "-",
                        });
                        mode.push_str(match write {
                            1 => "w",
                            _ => "-",
                        });
                        mode.push_str(match exec {
                            1 => "x",
                            _ => "-",
                        });
                    }
                }
                // Get username
                let username: String = match file.user {
                    Some(uid) => uid.to_string(),
                    None => String::from("0"),
                };
                // Get group
                let group: String = match file.group {
                    Some(gid) => gid.to_string(),
                    None => String::from("0"),
                };
                // Get byte size
                let size: ByteSize = ByteSize(file.size as u64);
                // Get date
                let datetime: String = time_to_str(file.last_change_time, "%b %d %Y %M:%H");
                write!(
                    f,
                    "{:24}\t{:12}\t{:16}\t{:16}\t{:8}\t{:17}",
                    file.name.as_str(),
                    mode,
                    username,
                    group,
                    size,
                    datetime
                )
            }
        }
    }
}
