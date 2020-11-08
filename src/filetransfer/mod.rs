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

use std::path::PathBuf;
use std::fs::File;

use crate::fs::FsEntry;

/// ## FileTransferProtocol
///
/// This enum defines the different transfer protocol available in TermSCP

#[derive(PartialEq, Clone)]
pub enum FileTransferProtocol {
    Scp,
    Sftp,
    Ftps,
}

/// ## FileTransferError
///
/// FileTransferError defines the possible errors available for a file transfer

#[derive(PartialEq, Clone)]
pub enum FileTransferError {
    ConnectionError,
    BadAddress,
    AuthenticationFailed,
    NoSuchFileOrDirectory,
    DirStatFailed,
    FileReadonly,
    DownloadError,
    UnknownError,
}

/// ## FileTransfer
/// 
/// File transfer trait must be implemented by all the file transfers and defines the method used by a generic file transfer

pub trait FileTransfer {

    /// ### connect
    /// 
    /// Connect to the remote server

    fn connect(&mut self, address: String, port: usize, username: Option<String>, password: Option<String>) -> Result<(), FileTransferError>;

    /// ### disconnect
    /// 
    /// Disconnect from the remote server

    fn disconnect(&mut self) -> Result<(), FileTransferError>;

    /// ### pwd
    /// 
    /// Print working directory

    fn pwd(&self) -> Result<PathBuf, FileTransferError>;

    /// ### change_dir
    /// 
    /// Change working directory

    fn change_dir(&mut self, dir: PathBuf) -> Result<PathBuf, FileTransferError>;

    /// ### list_dir
    /// 
    /// List directory entries

    fn list_dir(&self) -> Result<Vec<FsEntry>, FileTransferError>;

    /// ### send_file
    /// 
    /// Send file to remote
    /// File name is referred to the name of the file as it will be saved
    /// Data contains the file data
    fn send_file(&self, file_name: PathBuf, file: File) -> Result<(), FileTransferError>;

    /// ### recv_file
    /// 
    /// Receive file from remote with provided name
    fn recv_file(&self, file_name: PathBuf) -> Result<Vec<u8>, FileTransferError>;

}
