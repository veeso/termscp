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

use crate::fs::{FsEntry, FsFile};

// Transfers
pub mod ftp_transfer;
pub mod scp_transfer;
pub mod sftp_transfer;

/// ## FileTransferProtocol
///
/// This enum defines the different transfer protocol available in TermSCP

#[derive(PartialEq, std::fmt::Debug, std::clone::Clone, Copy)]
pub enum FileTransferProtocol {
    Sftp,
    Scp,
    Ftp(bool), // Bool is for secure (true => ftps)
}

/// ## FileTransferError
///
/// FileTransferError defines the possible errors available for a file transfer
#[derive(std::fmt::Debug)]
pub struct FileTransferError {
    code: FileTransferErrorType,
    msg: Option<String>,
}

/// ## FileTransferErrorType
///
/// FileTransferErrorType defines the possible errors available for a file transfer
#[allow(dead_code)]
#[derive(std::fmt::Debug)]
pub enum FileTransferErrorType {
    AuthenticationFailed,
    BadAddress,
    ConnectionError,
    SslError,
    DirStatFailed,
    FileCreateDenied,
    IoErr(std::io::Error),
    NoSuchFileOrDirectory,
    PexError,
    ProtocolError,
    UninitializedSession,
    UnsupportedFeature,
}

impl FileTransferError {
    /// ### new
    ///
    /// Instantiates a new FileTransferError
    pub fn new(code: FileTransferErrorType) -> FileTransferError {
        FileTransferError { code, msg: None }
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
            FileTransferErrorType::AuthenticationFailed => String::from("Authentication failed"),
            FileTransferErrorType::BadAddress => String::from("Bad address syntax"),
            FileTransferErrorType::ConnectionError => String::from("Connection error"),
            FileTransferErrorType::DirStatFailed => String::from("Could not stat directory"),
            FileTransferErrorType::FileCreateDenied => String::from("Failed to create file"),
            FileTransferErrorType::IoErr(err) => format!("IO error: {}", err),
            FileTransferErrorType::NoSuchFileOrDirectory => {
                String::from("No such file or directory")
            }
            FileTransferErrorType::PexError => String::from("Not enough permissions"),
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
    /// Can return banner / welcome message on success

    fn connect(
        &mut self,
        address: String,
        port: u16,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<Option<String>, FileTransferError>;

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

    fn pwd(&mut self) -> Result<PathBuf, FileTransferError>;

    /// ### change_dir
    ///
    /// Change working directory

    fn change_dir(&mut self, dir: &Path) -> Result<PathBuf, FileTransferError>;

    /// ### copy
    ///
    /// Copy file to destination
    fn copy(&mut self, src: &FsEntry, dst: &Path) -> Result<(), FileTransferError>;

    /// ### list_dir
    ///
    /// List directory entries

    fn list_dir(&mut self, path: &Path) -> Result<Vec<FsEntry>, FileTransferError>;

    /// ### mkdir
    ///
    /// Make directory
    /// You must return error in case the directory already exists
    fn mkdir(&mut self, dir: &Path) -> Result<(), FileTransferError>;

    /// ### remove
    ///
    /// Remove a file or a directory
    fn remove(&mut self, file: &FsEntry) -> Result<(), FileTransferError>;

    /// ### rename
    ///
    /// Rename file or a directory
    fn rename(&mut self, file: &FsEntry, dst: &Path) -> Result<(), FileTransferError>;

    /// ### stat
    ///
    /// Stat file and return FsEntry
    fn stat(&mut self, path: &Path) -> Result<FsEntry, FileTransferError>;

    /// ### send_file
    ///
    /// Send file to remote
    /// File name is referred to the name of the file as it will be saved
    /// Data contains the file data
    /// Returns file and its size
    fn send_file(
        &mut self,
        local: &FsFile,
        file_name: &Path,
    ) -> Result<Box<dyn Write>, FileTransferError>;

    /// ### recv_file
    ///
    /// Receive file from remote with provided name
    /// Returns file and its size
    fn recv_file(&mut self, file: &FsFile) -> Result<Box<dyn Read>, FileTransferError>;

    /// ### on_sent
    ///
    /// Finalize send method.
    /// This method must be implemented only if necessary; in case you don't need it, just return `Ok(())`
    /// The purpose of this method is to finalize the connection with the peer when writing data.
    /// This is necessary for some protocols such as FTP.
    /// You must call this method each time you want to finalize the write of the remote file.
    fn on_sent(&mut self, writable: Box<dyn Write>) -> Result<(), FileTransferError>;

    /// ### on_recv
    ///
    /// Finalize recv method.
    /// This method must be implemented only if necessary; in case you don't need it, just return `Ok(())`
    /// The purpose of this method is to finalize the connection with the peer when reading data.
    /// This mighe be necessary for some protocols.
    /// You must call this method each time you want to finalize the read of the remote file.
    fn on_recv(&mut self, readable: Box<dyn Read>) -> Result<(), FileTransferError>;
}

// Traits

impl std::string::ToString for FileTransferProtocol {
    fn to_string(&self) -> String {
        String::from(match self {
            FileTransferProtocol::Ftp(secure) => match secure {
                true => "FTPS",
                false => "FTP",
            },
            FileTransferProtocol::Scp => "SCP",
            FileTransferProtocol::Sftp => "SFTP",
        })
    }
}

impl std::str::FromStr for FileTransferProtocol {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "FTP" => Ok(FileTransferProtocol::Ftp(false)),
            "FTPS" => Ok(FileTransferProtocol::Ftp(true)),
            "SCP" => Ok(FileTransferProtocol::Scp),
            "SFTP" => Ok(FileTransferProtocol::Sftp),
            _ => Err(()),
        }
    }
}

// Tests

#[cfg(test)]
mod tests {

    use super::*;

    use std::str::FromStr;
    use std::string::ToString;

    #[test]
    fn test_filetransfer_mod_protocol() {
        assert_eq!(
            FileTransferProtocol::Ftp(true),
            FileTransferProtocol::Ftp(true)
        );
        assert_eq!(
            FileTransferProtocol::Ftp(false),
            FileTransferProtocol::Ftp(false)
        );
        // From str
        assert_eq!(
            FileTransferProtocol::from_str("FTPS").ok().unwrap(),
            FileTransferProtocol::Ftp(true)
        );
        assert_eq!(
            FileTransferProtocol::from_str("ftps").ok().unwrap(),
            FileTransferProtocol::Ftp(true)
        );
        assert_eq!(
            FileTransferProtocol::from_str("FTP").ok().unwrap(),
            FileTransferProtocol::Ftp(false)
        );
        assert_eq!(
            FileTransferProtocol::from_str("ftp").ok().unwrap(),
            FileTransferProtocol::Ftp(false)
        );
        assert_eq!(
            FileTransferProtocol::from_str("SFTP").ok().unwrap(),
            FileTransferProtocol::Sftp
        );
        assert_eq!(
            FileTransferProtocol::from_str("sftp").ok().unwrap(),
            FileTransferProtocol::Sftp
        );
        assert_eq!(
            FileTransferProtocol::from_str("SCP").ok().unwrap(),
            FileTransferProtocol::Scp
        );
        assert_eq!(
            FileTransferProtocol::from_str("scp").ok().unwrap(),
            FileTransferProtocol::Scp
        );
        // Error
        assert!(FileTransferProtocol::from_str("dummy").is_err());
        // To String
        assert_eq!(
            FileTransferProtocol::Ftp(true).to_string(),
            String::from("FTPS")
        );
        assert_eq!(
            FileTransferProtocol::Ftp(false).to_string(),
            String::from("FTP")
        );
        assert_eq!(FileTransferProtocol::Scp.to_string(), String::from("SCP"));
        assert_eq!(FileTransferProtocol::Sftp.to_string(), String::from("SFTP"));
    }

    #[test]
    fn test_filetransfer_mod_error() {
        let err: FileTransferError = FileTransferError::new_ex(
            FileTransferErrorType::IoErr(std::io::Error::from(std::io::ErrorKind::AddrInUse)),
            String::from("non va una mazza"),
        );
        assert_eq!(*err.msg.as_ref().unwrap(), String::from("non va una mazza"));
        assert_eq!(
            format!("{}", err),
            String::from("IO error: address in use (non va una mazza)")
        );
        assert_eq!(
            format!(
                "{}",
                FileTransferError::new(FileTransferErrorType::AuthenticationFailed)
            ),
            String::from("Authentication failed")
        );
        assert_eq!(
            format!(
                "{}",
                FileTransferError::new(FileTransferErrorType::BadAddress)
            ),
            String::from("Bad address syntax")
        );
        assert_eq!(
            format!(
                "{}",
                FileTransferError::new(FileTransferErrorType::ConnectionError)
            ),
            String::from("Connection error")
        );
        assert_eq!(
            format!(
                "{}",
                FileTransferError::new(FileTransferErrorType::DirStatFailed)
            ),
            String::from("Could not stat directory")
        );
        assert_eq!(
            format!(
                "{}",
                FileTransferError::new(FileTransferErrorType::FileCreateDenied)
            ),
            String::from("Failed to create file")
        );
        assert_eq!(
            format!(
                "{}",
                FileTransferError::new(FileTransferErrorType::NoSuchFileOrDirectory)
            ),
            String::from("No such file or directory")
        );
        assert_eq!(
            format!(
                "{}",
                FileTransferError::new(FileTransferErrorType::PexError)
            ),
            String::from("Not enough permissions")
        );
        assert_eq!(
            format!(
                "{}",
                FileTransferError::new(FileTransferErrorType::ProtocolError)
            ),
            String::from("Protocol error")
        );
        assert_eq!(
            format!(
                "{}",
                FileTransferError::new(FileTransferErrorType::SslError)
            ),
            String::from("SSL error")
        );
        assert_eq!(
            format!(
                "{}",
                FileTransferError::new(FileTransferErrorType::UninitializedSession)
            ),
            String::from("Uninitialized session")
        );
        assert_eq!(
            format!(
                "{}",
                FileTransferError::new(FileTransferErrorType::UnsupportedFeature)
            ),
            String::from("Unsupported feature")
        );
    }
}
