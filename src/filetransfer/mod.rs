//! ## FileTransfer
//!
//! `filetransfer` is the module which provides the trait file transfers must implement and the different file transfers

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

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::fs::FsEntry;

// Transfers
pub mod ftp_transfer;
pub mod sftp_transfer;

/// ## FileTransferProtocol
///
/// This enum defines the different transfer protocol available in TermSCP

#[derive(std::cmp::PartialEq, std::fmt::Debug, std::clone::Clone)]
pub enum FileTransferProtocol {
    Sftp,
    Ftp(bool), // Bool is for secure (true => ftps)
}

/// ## FileTransferError
///
/// FileTransferError defines the possible errors available for a file transfer

pub struct FileTransferError {
    code: FileTransferErrorType,
    msg: Option<String>,
}

/// ## FileTransferErrorType
///
/// FileTransferErrorType defines the possible errors available for a file transfer

pub enum FileTransferErrorType {
    AuthenticationFailed,
    BadAddress,
    ConnectionError,
    SslError,
    DirStatFailed,
    FileCreateDenied,
    FileReadonly,
    IoErr(std::io::Error),
    NoSuchFileOrDirectory,
    ProtocolError,
    UninitializedSession,
    UnsupportedFeature,
}

impl FileTransferError {
    /// ### new
    ///
    /// Instantiates a new FileTransferError
    pub fn new(code: FileTransferErrorType) -> FileTransferError {
        FileTransferError {
            code: code,
            msg: None,
        }
    }

    /// ### new_ex
    ///
    /// Instantiates a new FileTransferError with message
    pub fn new_ex(code: FileTransferErrorType, msg: String) -> FileTransferError {
        let mut err: FileTransferError = FileTransferError::new(code);
        err.msg = Some(msg);
        err
    }
}

impl std::fmt::Display for FileTransferError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let err: String = match &self.code {
            FileTransferErrorType::AuthenticationFailed => {
                String::from("Authentication failed")
            }
            FileTransferErrorType::BadAddress => String::from("Bad address syntax"),
            FileTransferErrorType::ConnectionError => String::from("Connection error"),
            FileTransferErrorType::DirStatFailed => String::from("Could not stat directory"),
            FileTransferErrorType::FileCreateDenied => String::from("Failed to create file"),
            FileTransferErrorType::FileReadonly => String::from("File is readonly"),
            FileTransferErrorType::IoErr(err) => format!("IO Error: {}", err),
            FileTransferErrorType::NoSuchFileOrDirectory => {
                String::from("No such file or directory")
            }
            FileTransferErrorType::ProtocolError => String::from("Protocol error"),
            FileTransferErrorType::SslError => String::from("SSL error"),
            FileTransferErrorType::UninitializedSession => String::from("Uninitialized session"),
            FileTransferErrorType::UnsupportedFeature => String::from("Unsupported feature"),
        };
        match &self.msg {
            Some(msg) => write!(f, "{} ({})", err, msg),
            None => write!(f, "{}", err),
        }
    }
}

/// ## FileTransfer
///
/// File transfer trait must be implemented by all the file transfers and defines the method used by a generic file transfer

pub trait FileTransfer {
    /// ### connect
    ///
    /// Connect to the remote server

    fn connect(
        &mut self,
        address: String,
        port: u16,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<(), FileTransferError>;

    /// ### disconnect
    ///
    /// Disconnect from the remote server

    fn disconnect(&mut self) -> Result<(), FileTransferError>;

    /// ### is_connected
    ///
    /// Indicates whether the client is connected to remote
    fn is_connected(&self) -> bool;

    /// ### pwd
    ///
    /// Print working directory

    fn pwd(&self) -> Result<PathBuf, FileTransferError>;

    /// ### change_dir
    ///
    /// Change working directory

    fn change_dir(&mut self, dir: &Path) -> Result<PathBuf, FileTransferError>;

    /// ### list_dir
    ///
    /// List directory entries

    fn list_dir(&self, path: &Path) -> Result<Vec<FsEntry>, FileTransferError>;

    /// ### mkdir
    ///
    /// Make directory
    fn mkdir(&self, dir: &Path) -> Result<(), FileTransferError>;

    /// ### remove
    ///
    /// Remove a file or a directory
    fn remove(&self, file: &FsEntry) -> Result<(), FileTransferError>;

    /// ### rename
    ///
    /// Rename file or a directory
    fn rename(&self, file: &FsEntry, dst: &Path) -> Result<(), FileTransferError>;

    /// ### stat
    ///
    /// Stat file and return FsEntry
    fn stat(&self, path: &Path) -> Result<FsEntry, FileTransferError>;

    /// ### send_file
    ///
    /// Send file to remote
    /// File name is referred to the name of the file as it will be saved
    /// Data contains the file data
    /// Returns file and its size
    fn send_file(&self, file_name: &Path) -> Result<Box<dyn Write>, FileTransferError>;

    /// ### recv_file
    ///
    /// Receive file from remote with provided name
    /// Returns file and its size
    fn recv_file(&self, file_name: &Path) -> Result<(Box<dyn Read>, usize), FileTransferError>;
}
