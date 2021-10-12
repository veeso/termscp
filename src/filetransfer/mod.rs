//! ## FileTransfer
//!
//! `filetransfer` is the module which provides the trait file transfers must implement and the different file transfers

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
// locals
use crate::fs::{FsEntry, FsFile};
// ext
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;
use wildmatch::WildMatch;
// exports
pub mod params;
mod transfer;

// -- export types
pub use params::{FileTransferParams, ProtocolParams};
pub use transfer::{FtpFileTransfer, S3FileTransfer, ScpFileTransfer, SftpFileTransfer};

/// ## FileTransferProtocol
///
/// This enum defines the different transfer protocol available in termscp

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum FileTransferProtocol {
    Sftp,
    Scp,
    Ftp(bool), // Bool is for secure (true => ftps)
    AwsS3,
}

/// ## FileTransferError
///
/// FileTransferError defines the possible errors available for a file transfer
#[derive(Debug)]
pub struct FileTransferError {
    code: FileTransferErrorType,
    msg: Option<String>,
}

/// ## FileTransferErrorType
///
/// FileTransferErrorType defines the possible errors available for a file transfer
#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum FileTransferErrorType {
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Bad address syntax")]
    BadAddress,
    #[error("Connection error")]
    ConnectionError,
    #[error("SSL error")]
    SslError,
    #[error("Could not stat directory")]
    DirStatFailed,
    #[error("Directory already exists")]
    DirectoryAlreadyExists,
    #[error("Failed to create file")]
    FileCreateDenied,
    #[error("No such file or directory")]
    NoSuchFileOrDirectory,
    #[error("Not enough permissions")]
    PexError,
    #[error("Protocol error")]
    ProtocolError,
    #[error("Uninitialized session")]
    UninitializedSession,
    #[error("Unsupported feature")]
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

    /// ### kind
    ///
    /// Returns the error kind
    pub fn kind(&self) -> FileTransferErrorType {
        self.code
    }
}

impl std::fmt::Display for FileTransferError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.msg {
            Some(msg) => write!(f, "{} ({})", self.code, msg),
            None => write!(f, "{}", self.code),
        }
    }
}

/// ## FileTransferResult
///
/// Result type returned by a `FileTransfer` implementation
pub type FileTransferResult<T> = Result<T, FileTransferError>;

/// ## FileTransfer
///
/// File transfer trait must be implemented by all the file transfers and defines the method used by a generic file transfer
pub trait FileTransfer {
    /// ### connect
    ///
    /// Connect to the remote server
    /// Can return banner / welcome message on success
    fn connect(&mut self, params: &ProtocolParams) -> FileTransferResult<Option<String>>;

    /// ### disconnect
    ///
    /// Disconnect from the remote server
    fn disconnect(&mut self) -> FileTransferResult<()>;

    /// ### is_connected
    ///
    /// Indicates whether the client is connected to remote
    fn is_connected(&self) -> bool;

    /// ### pwd
    ///
    /// Print working directory

    fn pwd(&mut self) -> FileTransferResult<PathBuf>;

    /// ### change_dir
    ///
    /// Change working directory

    fn change_dir(&mut self, dir: &Path) -> FileTransferResult<PathBuf>;

    /// ### copy
    ///
    /// Copy file to destination
    fn copy(&mut self, src: &FsEntry, dst: &Path) -> FileTransferResult<()>;

    /// ### list_dir
    ///
    /// List directory entries

    fn list_dir(&mut self, path: &Path) -> FileTransferResult<Vec<FsEntry>>;

    /// ### mkdir
    ///
    /// Make directory
    /// In case the directory already exists, it must return an Error of kind `FileTransferErrorType::DirectoryAlreadyExists`
    fn mkdir(&mut self, dir: &Path) -> FileTransferResult<()>;

    /// ### remove
    ///
    /// Remove a file or a directory
    fn remove(&mut self, file: &FsEntry) -> FileTransferResult<()>;

    /// ### rename
    ///
    /// Rename file or a directory
    fn rename(&mut self, file: &FsEntry, dst: &Path) -> FileTransferResult<()>;

    /// ### stat
    ///
    /// Stat file and return FsEntry
    fn stat(&mut self, path: &Path) -> FileTransferResult<FsEntry>;

    /// ### exec
    ///
    /// Execute a command on remote host
    fn exec(&mut self, cmd: &str) -> FileTransferResult<String>;

    /// ### send_file
    ///
    /// Send file to remote
    /// File name is referred to the name of the file as it will be saved
    /// Data contains the file data
    /// Returns file and its size.
    /// By default returns unsupported feature
    fn send_file(
        &mut self,
        _local: &FsFile,
        _file_name: &Path,
    ) -> FileTransferResult<Box<dyn Write>> {
        Err(FileTransferError::new(
            FileTransferErrorType::UnsupportedFeature,
        ))
    }

    /// ### recv_file
    ///
    /// Receive file from remote with provided name
    /// Returns file and its size
    /// By default returns unsupported feature
    fn recv_file(&mut self, _file: &FsFile) -> FileTransferResult<Box<dyn Read>> {
        Err(FileTransferError::new(
            FileTransferErrorType::UnsupportedFeature,
        ))
    }

    /// ### on_sent
    ///
    /// Finalize send method.
    /// This method must be implemented only if necessary; in case you don't need it, just return `Ok(())`
    /// The purpose of this method is to finalize the connection with the peer when writing data.
    /// This is necessary for some protocols such as FTP.
    /// You must call this method each time you want to finalize the write of the remote file.
    /// By default this function returns already `Ok(())`
    fn on_sent(&mut self, _writable: Box<dyn Write>) -> FileTransferResult<()> {
        Ok(())
    }

    /// ### on_recv
    ///
    /// Finalize recv method.
    /// This method must be implemented only if necessary; in case you don't need it, just return `Ok(())`
    /// The purpose of this method is to finalize the connection with the peer when reading data.
    /// This mighe be necessary for some protocols.
    /// You must call this method each time you want to finalize the read of the remote file.
    /// By default this function returns already `Ok(())`
    fn on_recv(&mut self, _readable: Box<dyn Read>) -> FileTransferResult<()> {
        Ok(())
    }

    /// ### send_file_wno_stream
    ///
    /// Send a file to remote WITHOUT using streams.
    /// This method SHOULD be implemented ONLY when streams are not supported by the current file transfer.
    /// The developer implementing the filetransfer user should FIRST try with `send_file` followed by `on_sent`
    /// If the function returns error kind() `UnsupportedFeature`, then he should call this function.
    /// By default this function uses the streams function to copy content from reader to writer
    fn send_file_wno_stream(
        &mut self,
        src: &FsFile,
        dest: &Path,
        mut reader: Box<dyn Read>,
    ) -> FileTransferResult<()> {
        match self.is_connected() {
            true => {
                let mut stream = self.send_file(src, dest)?;
                io::copy(&mut reader, &mut stream).map_err(|e| {
                    FileTransferError::new_ex(FileTransferErrorType::ProtocolError, e.to_string())
                })?;
                self.on_sent(stream)
            }
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### recv_file_wno_stream
    ///
    /// Receive a file from remote WITHOUT using streams.
    /// This method SHOULD be implemented ONLY when streams are not supported by the current file transfer.
    /// The developer implementing the filetransfer user should FIRST try with `send_file` followed by `on_sent`
    /// If the function returns error kind() `UnsupportedFeature`, then he should call this function.
    /// For safety reasons this function doesn't accept the `Write` trait, but the destination path.
    /// By default this function uses the streams function to copy content from reader to writer
    fn recv_file_wno_stream(&mut self, src: &FsFile, dest: &Path) -> FileTransferResult<()> {
        match self.is_connected() {
            true => {
                let mut writer = File::create(dest).map_err(|e| {
                    FileTransferError::new_ex(
                        FileTransferErrorType::FileCreateDenied,
                        format!("Could not open local file: {}", e),
                    )
                })?;
                let mut stream = self.recv_file(src)?;
                io::copy(&mut stream, &mut writer)
                    .map(|_| ())
                    .map_err(|e| {
                        FileTransferError::new_ex(
                            FileTransferErrorType::ProtocolError,
                            e.to_string(),
                        )
                    })?;
                self.on_recv(stream)
            }
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### find
    ///
    /// Find files from current directory (in all subdirectories) whose name matches the provided search
    /// Search supports wildcards ('?', '*')
    fn find(&mut self, search: &str) -> FileTransferResult<Vec<FsEntry>> {
        match self.is_connected() {
            true => {
                // Starting from current directory, iter dir
                match self.pwd() {
                    Ok(p) => self.iter_search(p.as_path(), &WildMatch::new(search)),
                    Err(err) => Err(err),
                }
            }
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### iter_search
    ///
    /// Search recursively in `dir` for file matching the wildcard.
    /// NOTE: DON'T RE-IMPLEMENT THIS FUNCTION, unless the file transfer provides a faster way to do so
    /// NOTE: don't call this method from outside; consider it as private
    fn iter_search(&mut self, dir: &Path, filter: &WildMatch) -> FileTransferResult<Vec<FsEntry>> {
        let mut drained: Vec<FsEntry> = Vec::new();
        // Scan directory
        match self.list_dir(dir) {
            Ok(entries) => {
                /* For each entry:
                - if is dir: call iter_search with `dir`
                    - push `iter_search` result to `drained`
                - if is file: check if it matches `filter`
                    - if it matches `filter`: push to to filter
                */
                for entry in entries.iter() {
                    match entry {
                        FsEntry::Directory(dir) => {
                            // If directory name, matches wildcard, push it to drained
                            if filter.matches(dir.name.as_str()) {
                                drained.push(FsEntry::Directory(dir.clone()));
                            }
                            drained.append(&mut self.iter_search(dir.abs_path.as_path(), filter)?);
                        }
                        FsEntry::File(file) => {
                            if filter.matches(file.name.as_str()) {
                                drained.push(FsEntry::File(file.clone()));
                            }
                        }
                    }
                }
                Ok(drained)
            }
            Err(err) => Err(err),
        }
    }
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
            FileTransferProtocol::AwsS3 => "S3",
        })
    }
}

impl std::str::FromStr for FileTransferProtocol {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "FTP" => Ok(FileTransferProtocol::Ftp(false)),
            "FTPS" => Ok(FileTransferProtocol::Ftp(true)),
            "SCP" => Ok(FileTransferProtocol::Scp),
            "SFTP" => Ok(FileTransferProtocol::Sftp),
            "S3" => Ok(FileTransferProtocol::AwsS3),
            _ => Err(s.to_string()),
        }
    }
}

// Tests

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;
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
        assert_eq!(
            FileTransferProtocol::from_str("S3").ok().unwrap(),
            FileTransferProtocol::AwsS3
        );
        assert_eq!(
            FileTransferProtocol::from_str("s3").ok().unwrap(),
            FileTransferProtocol::AwsS3
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
        assert_eq!(FileTransferProtocol::AwsS3.to_string(), String::from("S3"));
    }

    #[test]
    fn test_filetransfer_mod_error() {
        let err: FileTransferError = FileTransferError::new_ex(
            FileTransferErrorType::NoSuchFileOrDirectory,
            String::from("non va una mazza"),
        );
        assert_eq!(*err.msg.as_ref().unwrap(), String::from("non va una mazza"));
        assert_eq!(
            format!("{}", err),
            String::from("No such file or directory (non va una mazza)")
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
        let err = FileTransferError::new(FileTransferErrorType::UnsupportedFeature);
        assert_eq!(err.kind(), FileTransferErrorType::UnsupportedFeature);
    }
}
