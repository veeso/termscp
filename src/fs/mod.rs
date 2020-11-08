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

use std::path::PathBuf;
use std::time::Instant;

/// ## FsDirectory
///
/// Directory provides an interface to file system directories

pub struct FsDirectory {
    pub name: PathBuf,
    pub last_change_time: Instant,
    pub last_access_time: Instant,
    pub creation_time: Instant,
    pub readonly: bool,
    pub symlink: Option<PathBuf>,       // UNIX only
    pub user: Option<String>,           // UNIX only
    pub group: Option<String>,          // UNIX only
    pub unix_pex: Option<(u8, u8, u8)>, // UNIX only
}

/// ### FsFile
///
/// FsFile provides an interface to file system files

pub struct FsFile {
    pub name: PathBuf,
    pub last_change_time: Instant,
    pub last_access_time: Instant,
    pub creation_time: Instant,
    pub size: usize,
    pub ftype: String, // File type
    pub readonly: bool,
    pub symlink: Option<PathBuf>,       // UNIX only
    pub user: Option<String>,           // UNIX only
    pub group: Option<String>,          // UNIX only
    pub unix_pex: Option<(u8, u8, u8)>, // UNIX only
}
