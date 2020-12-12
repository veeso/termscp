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
#[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
extern crate users;

use crate::utils::{fmt_pex, time_to_str};

use bytesize::ByteSize;
use std::path::PathBuf;
use std::time::SystemTime;
#[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
use users::get_user_by_uid;

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
    pub symlink: Option<Box<FsEntry>>,  // UNIX only
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
    pub symlink: Option<Box<FsEntry>>,  // UNIX only
    pub user: Option<u32>,              // UNIX only
    pub group: Option<u32>,             // UNIX only
    pub unix_pex: Option<(u8, u8, u8)>, // UNIX only
}

impl FsEntry {
    /// ### get_abs_path
    ///
    /// Get absolute path from `FsEntry`
    pub fn get_abs_path(&self) -> PathBuf {
        match self {
            FsEntry::Directory(dir) => dir.abs_path.clone(),
            FsEntry::File(file) => file.abs_path.clone(),
        }
    }

    /// ### get_name
    ///
    /// Get file name from `FsEntry`
    pub fn get_name(&self) -> String {
        match self {
            FsEntry::Directory(dir) => dir.name.clone(),
            FsEntry::File(file) => file.name.clone(),
        }
    }

    /// ### get_last_change_time
    ///
    /// Get last change time from `FsEntry`
    pub fn get_last_change_time(&self) -> SystemTime {
        match self {
            FsEntry::Directory(dir) => dir.last_change_time,
            FsEntry::File(file) => file.last_change_time,
        }
    }

    /// ### get_last_access_time
    ///
    /// Get access time from `FsEntry`
    pub fn get_last_access_time(&self) -> SystemTime {
        match self {
            FsEntry::Directory(dir) => dir.last_access_time,
            FsEntry::File(file) => file.last_access_time,
        }
    }

    /// ### get_creation_time
    ///
    /// Get creation time from `FsEntry`
    pub fn get_creation_time(&self) -> SystemTime {
        match self {
            FsEntry::Directory(dir) => dir.creation_time,
            FsEntry::File(file) => file.creation_time,
        }
    }

    /// ### get_size
    ///
    /// Get size from `FsEntry`. For directories is always 4096
    pub fn get_size(&self) -> usize {
        match self {
            FsEntry::Directory(_) => 4096,
            FsEntry::File(file) => file.size,
        }
    }

    /// ### get_ftype
    ///
    /// Get file type from `FsEntry`. For directories is always None
    pub fn get_ftype(&self) -> Option<String> {
        match self {
            FsEntry::Directory(_) => None,
            FsEntry::File(file) => file.ftype.clone(),
        }
    }

    /// ### get_user
    ///
    /// Get uid from `FsEntry`
    pub fn get_user(&self) -> Option<u32> {
        match self {
            FsEntry::Directory(dir) => dir.user,
            FsEntry::File(file) => file.user,
        }
    }

    /// ### get_group
    ///
    /// Get gid from `FsEntry`
    pub fn get_group(&self) -> Option<u32> {
        match self {
            FsEntry::Directory(dir) => dir.group,
            FsEntry::File(file) => file.group,
        }
    }

    /// ### get_unix_pex
    ///
    /// Get unix pex from `FsEntry`
    pub fn get_unix_pex(&self) -> Option<(u8, u8, u8)> {
        match self {
            FsEntry::Directory(dir) => dir.unix_pex,
            FsEntry::File(file) => file.unix_pex,
        }
    }

    /// ### is_symlink
    ///
    /// Returns whether the `FsEntry` is a symlink
    pub fn is_symlink(&self) -> bool {
        match self {
            FsEntry::Directory(dir) => dir.symlink.is_some(),
            FsEntry::File(file) => file.symlink.is_some(),
        }
    }

    /// ### get_realfile
    ///
    /// Return the real file pointed by a `FsEntry`
    pub fn get_realfile(&self) -> FsEntry {
        match self {
            FsEntry::Directory(dir) => match &dir.symlink {
                Some(symlink) => symlink.get_realfile(),
                None => self.clone(),
            },
            FsEntry::File(file) => match &file.symlink {
                Some(symlink) => symlink.get_realfile(),
                None => self.clone(),
            },
        }
    }
}

impl std::fmt::Display for FsEntry {
    /// ### fmt_ls
    ///
    /// Format File Entry as `ls` does
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Create mode string
        let mut mode: String = String::with_capacity(10);
        let file_type: char = match self.is_symlink() {
            true => 'l',
            false => 'd',
        };
        mode.push(file_type);
        match self.get_unix_pex() {
            None => mode.push_str("?????????"),
            Some((owner, group, others)) => mode.push_str(fmt_pex(owner, group, others).as_str()),
        }
        // Get username
        let username: String = match self.get_user() {
            Some(uid) => match get_user_by_uid(uid) {
                Some(user) => user.name().to_string_lossy().to_string(),
                None => uid.to_string(),
            },
            None => String::from("0"),
        };
        // Get group
        /*
        let group: String = match self.get_group() {
            Some(gid) => match get_group_by_gid(gid) {
                Some(group) => group.name().to_string_lossy().to_string(),
                None => gid.to_string(),
            },
            None => String::from("0"),
        };
        */
        // Get byte size
        let size: ByteSize = ByteSize(self.get_size() as u64);
        // Get date
        let datetime: String = time_to_str(self.get_last_change_time(), "%b %d %Y %H:%M");
        // Set file name (or elide if too long)
        let name: String = self.get_name();
        let name: String = match name.len() >= 24 {
            false => name,
            true => format!("{}...", &name.as_str()[0..20]),
        };
        write!(
            f,
            "{:24}\t{:12}\t{:12}\t{:9}\t{:17}",
            name, mode, username, size, datetime
        )
    }

    /// ### fmt_ls
    ///
    /// Format File Entry as `ls` does
    #[cfg(target_os = "windows")]
    #[cfg(not(tarpaulin_include))]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Create mode string
        let mut mode: String = String::with_capacity(10);
        let file_type: char = match self.is_symlink() {
            true => 'l',
            false => 'd',
        };
        mode.push(file_type);
        match self.get_unix_pex() {
            None => mode.push_str("?????????"),
            Some((owner, group, others)) => mode.push_str(fmt_pex(owner, group, others).as_str()),
        }
        // Get username
        let username: usize = match self.get_user() {
            Some(uid) => uid,
            None => 0,
        };
        // Get group
        /*
        let group: String = match self.get_group() {
            Some(gid) => match get_group_by_gid(gid) {
                Some(group) => group.name().to_string_lossy().to_string(),
                None => gid.to_string(),
            },
            None => String::from("0"),
        };
        */
        // Get byte size
        let size: ByteSize = ByteSize(self.get_size() as u64);
        // Get date
        let datetime: String = time_to_str(self.get_last_change_time(), "%b %d %Y %H:%M");
        // Set file name (or elide if too long)
        let name: String = self.get_name();
        let name: String = match name.len() >= 24 {
            false => name,
            true => format!("{}...", &name.as_str()[0..20]),
        };
        write!(
            f,
            "{:24}\t{:12}\t{:12}\t{:9}\t{:17}",
            name, mode, username, size, datetime
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fs_fsentry_dir() {
        let t_now: SystemTime = SystemTime::now();
        let entry: FsEntry = FsEntry::Directory(FsDirectory {
            name: String::from("foo"),
            abs_path: PathBuf::from("/foo"),
            last_change_time: t_now,
            last_access_time: t_now,
            creation_time: t_now,
            readonly: false,
            symlink: None,             // UNIX only
            user: Some(0),             // UNIX only
            group: Some(0),            // UNIX only
            unix_pex: Some((7, 5, 5)), // UNIX only
        });
        assert_eq!(entry.get_abs_path(), PathBuf::from("/foo"));
        assert_eq!(entry.get_name(), String::from("foo"));
        assert_eq!(entry.get_last_access_time(), t_now);
        assert_eq!(entry.get_last_change_time(), t_now);
        assert_eq!(entry.get_creation_time(), t_now);
        assert_eq!(entry.get_size(), 4096);
        assert_eq!(entry.get_ftype(), None);
        assert_eq!(entry.get_user(), Some(0));
        assert_eq!(entry.get_group(), Some(0));
        assert_eq!(entry.is_symlink(), false);
        assert_eq!(entry.get_unix_pex(), Some((7, 5, 5)));
    }

    #[test]
    fn test_fs_fsentry_file() {
        let t_now: SystemTime = SystemTime::now();
        let entry: FsEntry = FsEntry::File(FsFile {
            name: String::from("bar.txt"),
            abs_path: PathBuf::from("/bar.txt"),
            last_change_time: t_now,
            last_access_time: t_now,
            creation_time: t_now,
            size: 8192,
            readonly: false,
            ftype: Some(String::from("txt")),
            symlink: None,             // UNIX only
            user: Some(0),             // UNIX only
            group: Some(0),            // UNIX only
            unix_pex: Some((6, 4, 4)), // UNIX only
        });
        assert_eq!(entry.get_abs_path(), PathBuf::from("/bar.txt"));
        assert_eq!(entry.get_name(), String::from("bar.txt"));
        assert_eq!(entry.get_last_access_time(), t_now);
        assert_eq!(entry.get_last_change_time(), t_now);
        assert_eq!(entry.get_creation_time(), t_now);
        assert_eq!(entry.get_size(), 8192);
        assert_eq!(entry.get_ftype(), Some(String::from("txt")));
        assert_eq!(entry.get_user(), Some(0));
        assert_eq!(entry.get_group(), Some(0));
        assert_eq!(entry.get_unix_pex(), Some((6, 4, 4)));
        assert_eq!(entry.is_symlink(), false);
    }

    #[test]
    fn test_fs_fsentry_realfile_none() {
        let t_now: SystemTime = SystemTime::now();
        // With file...
        let entry: FsEntry = FsEntry::File(FsFile {
            name: String::from("bar.txt"),
            abs_path: PathBuf::from("/bar.txt"),
            last_change_time: t_now,
            last_access_time: t_now,
            creation_time: t_now,
            size: 8192,
            readonly: false,
            ftype: Some(String::from("txt")),
            symlink: None,             // UNIX only
            user: Some(0),             // UNIX only
            group: Some(0),            // UNIX only
            unix_pex: Some((6, 4, 4)), // UNIX only
        });
        // Symlink is None...
        assert_eq!(
            entry.get_realfile().get_abs_path(),
            PathBuf::from("/bar.txt")
        );
        // With directory...
        let entry: FsEntry = FsEntry::Directory(FsDirectory {
            name: String::from("foo"),
            abs_path: PathBuf::from("/foo"),
            last_change_time: t_now,
            last_access_time: t_now,
            creation_time: t_now,
            readonly: false,
            symlink: None,             // UNIX only
            user: Some(0),             // UNIX only
            group: Some(0),            // UNIX only
            unix_pex: Some((7, 5, 5)), // UNIX only
        });
        assert_eq!(entry.is_symlink(), true);
        assert_eq!(entry.get_realfile().get_abs_path(), PathBuf::from("/foo"));
    }

    #[test]
    fn test_fs_fsentry_realfile_some() {
        let t_now: SystemTime = SystemTime::now();
        // Prepare entries
        // root -> child -> target
        let entry_target: FsEntry = FsEntry::Directory(FsDirectory {
            name: String::from("projects"),
            abs_path: PathBuf::from("/home/cvisintin/projects"),
            last_change_time: t_now,
            last_access_time: t_now,
            creation_time: t_now,
            readonly: false,
            symlink: None,             // UNIX only
            user: Some(0),             // UNIX only
            group: Some(0),            // UNIX only
            unix_pex: Some((7, 7, 7)), // UNIX only
        });
        let entry_child: FsEntry = FsEntry::Directory(FsDirectory {
            name: String::from("projects"),
            abs_path: PathBuf::from("/develop/projects"),
            last_change_time: t_now,
            last_access_time: t_now,
            creation_time: t_now,
            readonly: false,
            symlink: Some(Box::new(entry_target)),
            user: Some(0),
            group: Some(0),
            unix_pex: Some((7, 7, 7)),
        });
        let entry_root: FsEntry = FsEntry::File(FsFile {
            name: String::from("projects"),
            abs_path: PathBuf::from("/projects"),
            last_change_time: t_now,
            last_access_time: t_now,
            creation_time: t_now,
            size: 8,
            readonly: false,
            ftype: None,
            symlink: Some(Box::new(entry_child)),
            user: Some(0),
            group: Some(0),
            unix_pex: Some((7, 7, 7)),
        });
        // get real file
        let real_file: FsEntry = entry_root.get_realfile();
        // real file must be projects in /home/cvisintin
        assert_eq!(
            real_file.get_abs_path(),
            PathBuf::from("/home/cvisintin/projects")
        );
    }
}
