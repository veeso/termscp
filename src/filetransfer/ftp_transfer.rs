//! ## Ftp_transfer
//!
//! `ftp_transfer` is the module which provides the implementation for the FTP/FTPS file transfer

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
extern crate chrono;
extern crate ftp;
extern crate regex;

use super::{FileTransfer, FileTransferError, FileTransferErrorType};
use crate::fs::{FsDirectory, FsEntry, FsFile};
use crate::utils::lstime_to_systime;

// Includes
use ftp::{FtpStream, FtpError};
use ftp::openssl::ssl::{SslContext, SslMethod};
use regex::Regex;
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// ## FtpFileTransfer
/// 
/// Ftp file transfer struct
pub struct FtpFileTransfer {
    stream: Option<FtpStream>,
    ftps: bool,
}

impl FtpFileTransfer {

    /// ### new
    /// 
    /// Instantiates a new `FtpFileTransfer`
    pub fn new(ftps: bool) -> FtpFileTransfer {
        FtpFileTransfer {
            stream: None,
            ftps: ftps,
        }
    }

}

impl FileTransfer for FtpFileTransfer {

    /// ### connect
    ///
    /// Connect to the remote server

    fn connect(
        &mut self,
        address: String,
        port: u16,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<(), FileTransferError> {
        // Get stream
        let mut stream: FtpStream = match FtpStream::connect(format!("{}:{}", address, port)) {
            Ok(stream) => stream,
            Err(err) => return Err(FileTransferError::new_ex(FileTransferErrorType::ConnectionError, format!("{}", err))),
        };
        // If SSL, open secure session
        if self.ftps {
            let mut ctx = SslContext::builder(SslMethod::tls()).unwrap();
            let ctx = ctx.build();
            if let Err(err) = stream.into_secure(ctx) {
                return Err(FileTransferError::new_ex(FileTransferErrorType::SslError, format!("{}", err)))
            }
        }
        // If username / password...
        if let Some(username) = username {
            if let Err(err) = stream.login(username.as_str(), match password {
                Some(pwd) => pwd.as_ref(),
                None => ""
            }) {
                return Err(FileTransferError::new_ex(FileTransferErrorType::AuthenticationFailed, format!("{}", err)))
            }
        }
        // Set stream
        self.stream = Some(stream);
        // Return OK
        Ok(())
    }

    /// ### disconnect
    ///
    /// Disconnect from the remote server

    fn disconnect(&mut self) -> Result<(), FileTransferError> {
        match self.stream {
            Some(stream) => match stream.quit() {
                Ok(_) => Ok(()),
                Err(err) => Err(FileTransferError::new_ex(FileTransferErrorType::ConnectionError, format!("{}", err)))
            },
            None => Err(FileTransferError::new(FileTransferErrorType::UninitializedSession))
        }
    }

    /// ### is_connected
    ///
    /// Indicates whether the client is connected to remote
    fn is_connected(&self) -> bool {
        match self.stream {
            Some(_) => true,
            None => false,
        }
    }

    /// ### pwd
    ///
    /// Print working directory

    fn pwd(&self) -> Result<PathBuf, FileTransferError> {
        match self.stream {
            Some(stream) => match stream.pwd() {
                Ok(path) => Ok(PathBuf::from(path.as_str())),
                Err(err) => Err(FileTransferError::new_ex(FileTransferErrorType::ConnectionError, format!("{}", err)))
            },
            None => Err(FileTransferError::new(FileTransferErrorType::UninitializedSession))
        }
    }

    /// ### change_dir
    ///
    /// Change working directory

    fn change_dir(&mut self, dir: &Path) -> Result<PathBuf, FileTransferError> {
        match self.stream {
            Some(stream) => match stream.cwd(&dir.as_os_str().to_string_lossy()) {
                Ok(_) => Ok(PathBuf::from(dir)),
                Err(err) => Err(FileTransferError::new_ex(FileTransferErrorType::ConnectionError, format!("{}", err))),
            }
            None => Err(FileTransferError::new(FileTransferErrorType::UninitializedSession))
        }
    }

    /// ### list_dir
    ///
    /// List directory entries

    fn list_dir(&self, path: &Path) -> Result<Vec<FsEntry>, FileTransferError> {
        // Prepare list regex 
        // NOTE: about this damn regex <https://stackoverflow.com/questions/32480890/is-there-a-regex-to-parse-the-values-from-an-ftp-directory-listing>
        lazy_static! {
            static ref LS_RE: Regex = Regex::new(r#"^([\-ld])([\-rwxs]{9})\s+(\d+)\s+(\w+)\s+(\w+)\s+(\d+)\s+(\w{3}\s+\d{1,2}\s+(?:\d{1,2}:\d{1,2}|\d{4}))\s+(.+)$"#).unwrap();
        }
        match self.stream {
            Some(stream) => match stream.list(Some(&path.as_os_str().to_string_lossy())) {
                Ok(entries) => {
                    // Prepare result
                    let mut result: Vec<FsEntry> = Vec::with_capacity(entries.len());
                    // Iterate over entries
                    for entry in entries.iter() {
                        // Apply regex to result
                        if let Some(metadata) = LS_RE.captures(entry) { // String matches regex
                            // NOTE: metadata fmt: (regex, file_type, permissions, link_count, uid, gid, filesize, mtime, filename)
                            // Expected 7 + 1 (8) values: + 1 cause regex is repeated at 0
                            if metadata.len() < 8 {
                                continue
                            }
                            // Collect metadata
                            // Get if is directory and if is symlink
                            let (is_dir, is_symlink): (bool, bool) = match metadata.get(1).unwrap().as_str() {
                                "-" => (false, false),
                                "l" => (false, true),
                                "d" => (true, false),
                                _ => continue, // Ignore special files
                            };
                            // Check string length (unix pex)
                            if metadata.get(2).unwrap().as_str().len() < 9 {
                                continue;
                            }
                            // Get unix pex
                            let unix_pex: (u8, u8, u8) = {
                                let owner_pex: u8 = {
                                    let mut count: u8 = 0;
                                    for (i, c) in metadata.get(2).unwrap().as_str()[0..3].chars().enumerate() {
                                        match c {
                                            '-' => {},
                                            _ => count = count + match i {
                                                0 => 4,
                                                1 => 2,
                                                2 => 1,
                                                _ => 0,
                                            }
                                        }
                                    }
                                    count
                                };
                                let group_pex: u8 = {
                                    let mut count: u8 = 0;
                                    for (i, c) in metadata.get(2).unwrap().as_str()[3..6].chars().enumerate() {
                                        match c {
                                            '-' => {},
                                            _ => count = count + match i {
                                                0 => 4,
                                                1 => 2,
                                                2 => 1,
                                                _ => 0,
                                            }
                                        }
                                    }
                                    count
                                };
                                let others_pex: u8 = {
                                    let mut count: u8 = 0;
                                    for (i, c) in metadata.get(2).unwrap().as_str()[6..9].chars().enumerate() {
                                        match c {
                                            '-' => {},
                                            _ => count = count + match i {
                                                0 => 4,
                                                1 => 2,
                                                2 => 1,
                                                _ => 0,
                                            }
                                        }
                                    }
                                    count
                                };
                                (owner_pex, group_pex, others_pex)
                            };
                            // Parse mtime and convert to SystemTime
                            let mtime: SystemTime = match lstime_to_systime(metadata.get(7).unwrap().as_str(), "%b %d %Y", "%b %d %H:%M") {
                                Ok(t) => t,
                                Err(_) => continue
                            };
                            // Get uid
                            let uid: Option<u32> = match metadata.get(4).unwrap().as_str().parse::<u32>() {
                                Ok(uid) => Some(uid),
                                Err(_) => None
                            };
                            // Get gid
                            let gid: Option<u32> = match metadata.get(5).unwrap().as_str().parse::<u32>() {
                                Ok(gid) => Some(gid),
                                Err(_) => None
                            };
                            // Get filesize
                            let filesize: usize = match metadata.get(6).unwrap().as_str().parse::<usize>() {
                                Ok(sz) => sz,
                                Err(_) => continue
                            };
                            let file_name: String = String::from(metadata.get(8).unwrap().as_str());
                            let mut abs_path: PathBuf = PathBuf::from(path);
                            let extension: Option<String> = match abs_path.as_path().extension() {
                                None => None,
                                Some(s) => Some(String::from(s.to_string_lossy()))
                            };
                            abs_path.push(file_name.as_str());
                            // Return
                            // Push to entries
                            result.push(match is_dir {
                                true => FsEntry::Directory(FsDirectory {
                                    name: file_name,
                                    abs_path: abs_path,
                                    last_change_time: mtime,
                                    last_access_time: mtime,
                                    creation_time: mtime,
                                    readonly: false,
                                    symlink: None,
                                    user: uid,
                                    group: gid,
                                    unix_pex: Some(unix_pex),
                                }),
                                false => FsEntry::File(FsFile {
                                    name: file_name,
                                    abs_path: abs_path,
                                    last_change_time: mtime,
                                    last_access_time: mtime,
                                    creation_time: mtime,
                                    size: filesize,
                                    ftype: extension,
                                    readonly: false,
                                    symlink: None,
                                    user: uid,
                                    group: gid,
                                    unix_pex: Some(unix_pex),
                                })
                            })
                        }
                    }
                    Ok(result)
                }
                Err(err) => Err(FileTransferError::new_ex(FileTransferErrorType::DirStatFailed, format!("{}", err))),
            }
            None => Err(FileTransferError::new(FileTransferErrorType::UninitializedSession))
        }
    }

    /// ### mkdir
    ///
    /// Make directory
    fn mkdir(&self, dir: &Path) -> Result<(), FileTransferError> {
        match self.stream {
            Some(stream) => {},
            None => Err(FileTransferError::new(FileTransferErrorType::UninitializedSession))
        }
    }

    /// ### remove
    ///
    /// Remove a file or a directory
    fn remove(&self, file: &FsEntry) -> Result<(), FileTransferError> {
        match self.stream {
            Some(stream) => {},
            None => Err(FileTransferError::new(FileTransferErrorType::UninitializedSession))
        }
    }

    /// ### rename
    ///
    /// Rename file or a directory
    fn rename(&self, file: &FsEntry, dst: &Path) -> Result<(), FileTransferError> {
        match self.stream {
            Some(stream) => {},
            None => Err(FileTransferError::new(FileTransferErrorType::UninitializedSession))
        }
    }

    /// ### stat
    /// 
    /// Stat file and return FsEntry
    fn stat(&self, path: &Path) -> Result<FsEntry, FileTransferError> {
        match self.stream {
            Some(stream) => Err(FileTransferError::new(FileTransferErrorType::UnsupportedFeature)),
            None => Err(FileTransferError::new(FileTransferErrorType::UninitializedSession))
        }
    }

    /// ### send_file
    ///
    /// Send file to remote
    /// File name is referred to the name of the file as it will be saved
    /// Data contains the file data
    /// Returns file and its size
    fn send_file(&self, file_name: &Path) -> Result<Box<dyn Write>, FileTransferError> {
        match self.stream {
            Some(stream) => {},
            None => Err(FileTransferError::new(FileTransferErrorType::UninitializedSession))
        }
    }

    /// ### recv_file
    ///
    /// Receive file from remote with provided name
    /// Returns file and its size
    fn recv_file(&self, file_name: &Path) -> Result<(Box<dyn Read>, usize), FileTransferError> {
        match self.stream {
            Some(stream) => {},
            None => Err(FileTransferError::new(FileTransferErrorType::UninitializedSession))
        }
    }

}
