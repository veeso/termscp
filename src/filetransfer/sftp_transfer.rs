//! ## SFTP_Transfer
//!
//! `sftp_transfer` is the module which provides the implementation for the SFTP file transfer

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
extern crate ssh2;

// Locals
use super::{FileTransfer, FileTransferError, FileTransferErrorType};
use crate::fs::{FsDirectory, FsEntry, FsFile};

// Includes
use ssh2::{FileStat, OpenFlags, OpenType, Session, Sftp};
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// ## SftpFileTransfer
///
/// SFTP file transfer structure
pub struct SftpFileTransfer {
    session: Option<Session>,
    sftp: Option<Sftp>,
    wrkdir: PathBuf,
}

impl Default for SftpFileTransfer {
    fn default() -> Self {
        Self::new()
    }
}

impl SftpFileTransfer {
    /// ### new
    ///
    /// Instantiates a new SftpFileTransfer
    pub fn new() -> SftpFileTransfer {
        SftpFileTransfer {
            session: None,
            sftp: None,
            wrkdir: PathBuf::from("~"),
        }
    }

    /// ### get_abs_path
    ///
    /// Get absolute path from path argument and check if it exists
    fn get_remote_path(&self, p: &Path) -> Result<PathBuf, FileTransferError> {
        match p.is_relative() {
            true => {
                let mut root: PathBuf = self.wrkdir.clone();
                root.push(p);
                match self.sftp.as_ref().unwrap().realpath(root.as_path()) {
                    Ok(p) => match self.sftp.as_ref().unwrap().stat(p.as_path()) {
                        Ok(_) => Ok(p),
                        Err(err) => Err(FileTransferError::new_ex(
                            FileTransferErrorType::NoSuchFileOrDirectory,
                            format!("{}", err),
                        )),
                    },
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::NoSuchFileOrDirectory,
                        format!("{}", err),
                    )),
                }
            }
            false => match self.sftp.as_ref().unwrap().realpath(p) {
                Ok(p) => match self.sftp.as_ref().unwrap().stat(p.as_path()) {
                    Ok(_) => Ok(p),
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::NoSuchFileOrDirectory,
                        format!("{}", err),
                    )),
                },
                Err(_) => Err(FileTransferError::new(
                    FileTransferErrorType::NoSuchFileOrDirectory,
                )),
            },
        }
    }

    /// ### get_abs_path
    ///
    /// Returns absolute path on remote, but without errors
    fn get_abs_path(&self, p: &Path) -> PathBuf {
        match p.is_relative() {
            true => {
                let mut root: PathBuf = self.wrkdir.clone();
                root.push(p);
                match self.sftp.as_ref().unwrap().realpath(root.as_path()) {
                    Ok(p) => p,
                    Err(_) => root,
                }
            }
            false => PathBuf::from(p),
        }
    }

    /// ### make_fsentry
    ///
    /// Make fsentry from path and metadata
    fn make_fsentry(&mut self, path: &Path, metadata: &FileStat) -> FsEntry {
        // Get common parameters
        let file_name: String = String::from(path.file_name().unwrap().to_str().unwrap_or(""));
        let file_type: Option<String> = match path.extension() {
            Some(ext) => Some(String::from(ext.to_str().unwrap_or(""))),
            None => None,
        };
        let uid: Option<u32> = metadata.uid;
        let gid: Option<u32> = metadata.gid;
        let pex: Option<(u8, u8, u8)> = match metadata.perm {
            Some(perms) => Some((
                ((perms >> 6) & 0x7) as u8,
                ((perms >> 3) & 0x7) as u8,
                (perms & 0x7) as u8,
            )),
            None => None,
        };
        let size: u64 = metadata.size.unwrap_or(0);
        let mut atime: SystemTime = SystemTime::UNIX_EPOCH;
        atime = atime
            .checked_add(Duration::from_secs(metadata.atime.unwrap_or(0)))
            .unwrap_or(SystemTime::UNIX_EPOCH);
        let mut mtime: SystemTime = SystemTime::UNIX_EPOCH;
        mtime = mtime
            .checked_add(Duration::from_secs(metadata.mtime.unwrap_or(0)))
            .unwrap_or(SystemTime::UNIX_EPOCH);
        // Check if symlink
        let is_symlink: bool = metadata.file_type().is_symlink();
        let symlink: Option<Box<FsEntry>> = match is_symlink {
            true => {
                // Read symlink
                match self.sftp.as_ref().unwrap().readlink(path) {
                    Ok(p) => match self.stat(p.as_path()) {
                        Ok(entry) => Some(Box::new(entry)),
                        Err(_) => None, // Ignore errors
                    },
                    Err(_) => None,
                }
            }
            false => None,
        };
        // Is a directory?
        match metadata.is_dir() {
            true => FsEntry::Directory(FsDirectory {
                name: file_name,
                abs_path: PathBuf::from(path),
                last_change_time: mtime,
                last_access_time: atime,
                creation_time: SystemTime::UNIX_EPOCH,
                readonly: false,
                symlink,
                user: uid,
                group: gid,
                unix_pex: pex,
            }),
            false => FsEntry::File(FsFile {
                name: file_name,
                abs_path: PathBuf::from(path),
                size: size as usize,
                ftype: file_type,
                last_change_time: mtime,
                last_access_time: atime,
                creation_time: SystemTime::UNIX_EPOCH,
                readonly: false,
                symlink,
                user: uid,
                group: gid,
                unix_pex: pex,
            }),
        }
    }
}

impl FileTransfer for SftpFileTransfer {
    /// ### connect
    ///
    /// Connect to the remote server
    fn connect(
        &mut self,
        address: String,
        port: u16,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<Option<String>, FileTransferError> {
        // Setup tcp stream
        let tcp: TcpStream = match TcpStream::connect(format!("{}:{}", address, port)) {
            Ok(stream) => stream,
            Err(err) => {
                return Err(FileTransferError::new_ex(
                    FileTransferErrorType::BadAddress,
                    format!("{}", err),
                ))
            }
        };
        // Create session
        let mut session: Session = match Session::new() {
            Ok(s) => s,
            Err(err) => {
                return Err(FileTransferError::new_ex(
                    FileTransferErrorType::ConnectionError,
                    format!("{}", err),
                ))
            }
        };
        // Set TCP stream
        session.set_tcp_stream(tcp);
        // Open connection
        if let Err(err) = session.handshake() {
            return Err(FileTransferError::new_ex(
                FileTransferErrorType::ConnectionError,
                format!("{}", err),
            ));
        }
        let username: String = match username {
            Some(u) => u,
            None => String::from(""),
        };
        // Try authenticating with user agent
        if session.userauth_agent(username.as_str()).is_err() {
            // Try authentication with password then
            if let Err(err) = session.userauth_password(
                username.as_str(),
                password.unwrap_or_else(|| String::from("")).as_str(),
            ) {
                return Err(FileTransferError::new_ex(
                    FileTransferErrorType::AuthenticationFailed,
                    format!("{}", err),
                ));
            }
        }
        // Set blocking to true
        session.set_blocking(true);
        // Get Sftp client
        let sftp: Sftp = match session.sftp() {
            Ok(s) => s,
            Err(err) => {
                return Err(FileTransferError::new_ex(
                    FileTransferErrorType::ProtocolError,
                    format!("{}", err),
                ))
            }
        };
        // Get working directory
        self.wrkdir = match sftp.realpath(PathBuf::from(".").as_path()) {
            Ok(p) => p,
            Err(err) => {
                return Err(FileTransferError::new_ex(
                    FileTransferErrorType::ProtocolError,
                    format!("{}", err),
                ))
            }
        };
        // Set session
        let banner: Option<String> = match session.banner() {
            Some(s) => Some(String::from(s)),
            None => None,
        };
        self.session = Some(session);
        // Set sftp
        self.sftp = Some(sftp);
        Ok(banner)
    }

    /// ### disconnect
    ///
    /// Disconnect from the remote server
    fn disconnect(&mut self) -> Result<(), FileTransferError> {
        match self.session.as_ref() {
            Some(session) => {
                // Disconnect (greet server with 'Mandi' as they do in Friuli)
                match session.disconnect(None, "Mandi!", None) {
                    Ok(()) => {
                        // Set session and sftp to none
                        self.session = None;
                        self.sftp = None;
                        Ok(())
                    }
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::ConnectionError,
                        format!("{}", err),
                    )),
                }
            }
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### is_connected
    ///
    /// Indicates whether the client is connected to remote
    fn is_connected(&self) -> bool {
        self.session.is_some()
    }

    /// ### pwd
    ///
    /// Print working directory
    fn pwd(&mut self) -> Result<PathBuf, FileTransferError> {
        match self.sftp {
            Some(_) => Ok(self.wrkdir.clone()),
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### change_dir
    ///
    /// Change working directory
    fn change_dir(&mut self, dir: &Path) -> Result<PathBuf, FileTransferError> {
        match self.sftp.as_ref() {
            Some(_) => {
                // Change working directory
                self.wrkdir = match self.get_remote_path(dir) {
                    Ok(p) => p,
                    Err(err) => return Err(err),
                };
                Ok(self.wrkdir.clone())
            }
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### list_dir
    ///
    /// List directory entries
    fn list_dir(&mut self, path: &Path) -> Result<Vec<FsEntry>, FileTransferError> {
        match self.sftp.as_ref() {
            Some(sftp) => {
                // Get path
                let dir: PathBuf = match self.get_remote_path(path) {
                    Ok(p) => p,
                    Err(err) => return Err(err),
                };
                // Get files
                match sftp.readdir(dir.as_path()) {
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::DirStatFailed,
                        format!("{}", err),
                    )),
                    Ok(files) => {
                        // Allocate vector
                        let mut entries: Vec<FsEntry> = Vec::with_capacity(files.len());
                        // Iterate over files
                        for (path, metadata) in files {
                            entries.push(self.make_fsentry(path.as_path(), &metadata));
                        }
                        Ok(entries)
                    }
                }
            }
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### mkdir
    ///
    /// Make directory
    fn mkdir(&mut self, dir: &Path) -> Result<(), FileTransferError> {
        match self.sftp.as_ref() {
            Some(sftp) => {
                // Make directory
                let path: PathBuf = self.get_abs_path(PathBuf::from(dir).as_path());
                match sftp.mkdir(path.as_path(), 0o775) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::FileCreateDenied,
                        format!("{}", err),
                    )),
                }
            }
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### remove
    ///
    /// Remove a file or a directory
    fn remove(&mut self, file: &FsEntry) -> Result<(), FileTransferError> {
        if self.sftp.is_none() {
            return Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            ));
        }
        // Match if file is a file or a directory
        match file {
            FsEntry::File(f) => {
                // Remove file
                match self.sftp.as_ref().unwrap().unlink(f.abs_path.as_path()) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::PexError,
                        format!("{}", err),
                    )),
                }
            }
            FsEntry::Directory(d) => {
                // Remove recursively
                // Get directory files
                let directory_content: Vec<FsEntry> = match self.list_dir(d.abs_path.as_path()) {
                    Ok(entries) => entries,
                    Err(err) => return Err(err),
                };
                for entry in directory_content.iter() {
                    if let Err(err) = self.remove(&entry) {
                        return Err(err);
                    }
                }
                // Finally remove directory
                match self.sftp.as_ref().unwrap().rmdir(d.abs_path.as_path()) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::PexError,
                        format!("{}", err),
                    )),
                }
            }
        }
    }

    /// ### rename
    ///
    /// Rename file or a directory
    fn rename(&mut self, file: &FsEntry, dst: &Path) -> Result<(), FileTransferError> {
        match self.sftp.as_ref() {
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
            Some(sftp) => {
                // Resolve destination path
                let abs_dst: PathBuf = self.get_abs_path(dst);
                // Get abs path of entry
                let abs_src: PathBuf = match file {
                    FsEntry::Directory(dir) => dir.abs_path.clone(),
                    FsEntry::File(file) => file.abs_path.clone(),
                };
                match sftp.rename(abs_src.as_path(), abs_dst.as_path(), None) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::FileCreateDenied,
                        format!("{}", err),
                    )),
                }
            }
        }
    }

    /// ### stat
    ///
    /// Stat file and return FsEntry
    fn stat(&mut self, path: &Path) -> Result<FsEntry, FileTransferError> {
        match self.sftp.as_ref() {
            Some(sftp) => {
                // Get path
                let dir: PathBuf = match self.get_remote_path(path) {
                    Ok(p) => p,
                    Err(err) => return Err(err),
                };
                // Get file
                match sftp.stat(dir.as_path()) {
                    Ok(metadata) => Ok(self.make_fsentry(dir.as_path(), &metadata)),
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::NoSuchFileOrDirectory,
                        format!("{}", err),
                    )),
                }
            }
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### send_file
    ///
    /// Send file to remote
    /// File name is referred to the name of the file as it will be saved
    /// Data contains the file data
    fn send_file(
        &mut self,
        local: &FsFile,
        file_name: &Path,
    ) -> Result<Box<dyn Write>, FileTransferError> {
        match self.sftp.as_ref() {
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
            Some(sftp) => {
                let remote_path: PathBuf = self.get_abs_path(file_name);
                // Calculate file mode
                let mode: i32 = match local.unix_pex {
                    None => 0o644,
                    Some((u, g, o)) => ((u as i32) << 6) + ((g as i32) << 3) + (o as i32),
                };
                match sftp.open_mode(
                    remote_path.as_path(),
                    OpenFlags::WRITE | OpenFlags::CREATE | OpenFlags::APPEND | OpenFlags::TRUNCATE,
                    mode,
                    OpenType::File,
                ) {
                    Ok(file) => Ok(Box::new(BufWriter::with_capacity(65536, file))),
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::FileCreateDenied,
                        format!("{}", err),
                    )),
                }
            }
        }
    }

    /// ### recv_file
    ///
    /// Receive file from remote with provided name
    fn recv_file(&mut self, file: &FsFile) -> Result<Box<dyn Read>, FileTransferError> {
        match self.sftp.as_ref() {
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
            Some(sftp) => {
                // Get remote file name
                let remote_path: PathBuf = match self.get_remote_path(file.abs_path.as_path()) {
                    Ok(p) => p,
                    Err(err) => return Err(err),
                };
                // Open remote file
                match sftp.open(remote_path.as_path()) {
                    Ok(file) => Ok(Box::new(BufReader::with_capacity(8192, file))),
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::NoSuchFileOrDirectory,
                        format!("{}", err),
                    )),
                }
            }
        }
    }

    /// ### on_sent
    ///
    /// Finalize send method. This method must be implemented only if necessary.
    /// The purpose of this method is to finalize the connection with the peer when writing data.
    /// This is necessary for some protocols such as FTP.
    /// You must call this method each time you want to finalize the write of the remote file.
    fn on_sent(&mut self, _writable: Box<dyn Write>) -> Result<(), FileTransferError> {
        Ok(())
    }

    /// ### on_recv
    ///
    /// Finalize recv method. This method must be implemented only if necessary.
    /// The purpose of this method is to finalize the connection with the peer when reading data.
    /// This mighe be necessary for some protocols.
    /// You must call this method each time you want to finalize the read of the remote file.
    fn on_recv(&mut self, _readable: Box<dyn Read>) -> Result<(), FileTransferError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_filetransfer_sftp_new() {
        let client: SftpFileTransfer = SftpFileTransfer::new();
        assert!(client.session.is_none());
        assert!(client.sftp.is_none());
        assert_eq!(client.wrkdir, PathBuf::from("~"));
        assert_eq!(client.is_connected(), false);
    }

    #[test]
    fn test_filetransfer_sftp_connect() {
        let mut client: SftpFileTransfer = SftpFileTransfer::new();
        assert_eq!(client.is_connected(), false);
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and sftp
        assert!(client.session.is_some());
        assert!(client.sftp.is_some());
        assert_eq!(client.wrkdir, PathBuf::from("/"));
        assert_eq!(client.is_connected(), true);
        // Disconnect
        assert!(client.disconnect().is_ok());
        assert_eq!(client.is_connected(), false);
    }

    #[test]
    fn test_filetransfer_sftp_bad_auth() {
        let mut client: SftpFileTransfer = SftpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("badpassword"))
            )
            .is_err());
    }

    #[test]
    fn test_filetransfer_sftp_no_credentials() {
        let mut client: SftpFileTransfer = SftpFileTransfer::new();
        assert!(client
            .connect(String::from("test.rebex.net"), 22, None, None)
            .is_err());
    }

    #[test]
    fn test_filetransfer_sftp_bad_server() {
        let mut client: SftpFileTransfer = SftpFileTransfer::new();
        assert!(client
            .connect(
                String::from("mybadserver.veryverybad.awful"),
                22,
                None,
                None
            )
            .is_err());
    }

    #[test]
    fn test_filetransfer_sftp_pwd() {
        let mut client: SftpFileTransfer = SftpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and sftp
        assert!(client.session.is_some());
        assert!(client.sftp.is_some());
        assert_eq!(client.wrkdir, PathBuf::from("/"));
        // Pwd
        assert_eq!(client.wrkdir.clone(), client.pwd().ok().unwrap());
        // Disconnect
        assert!(client.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_sftp_cwd() {
        let mut client: SftpFileTransfer = SftpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and sftp
        assert!(client.session.is_some());
        assert!(client.sftp.is_some());
        assert_eq!(client.wrkdir, PathBuf::from("/"));
        // Pwd
        assert_eq!(client.wrkdir.clone(), client.pwd().ok().unwrap());
        // Cwd (relative)
        assert!(client.change_dir(PathBuf::from("pub/").as_path()).is_ok());
        assert_eq!(client.wrkdir, PathBuf::from("/pub"));
        // Cwd (absolute)
        assert!(client.change_dir(PathBuf::from("/").as_path()).is_ok());
        assert_eq!(client.wrkdir, PathBuf::from("/"));
        // Disconnect
        assert!(client.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_sftp_cwd_error() {
        let mut client: SftpFileTransfer = SftpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Cwd (abs)
        assert!(client
            .change_dir(PathBuf::from("/omar/gabber").as_path())
            .is_err());
        // Cwd (rel)
        assert!(client
            .change_dir(PathBuf::from("gomar/pett").as_path())
            .is_err());
        // Disconnect
        assert!(client.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_sftp_ls() {
        let mut client: SftpFileTransfer = SftpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and sftp
        assert!(client.session.is_some());
        assert!(client.sftp.is_some());
        assert_eq!(client.wrkdir, PathBuf::from("/"));
        // List dir
        let pwd: PathBuf = client.pwd().ok().unwrap();
        let files: Vec<FsEntry> = client.list_dir(pwd.as_path()).ok().unwrap();
        assert_eq!(files.len(), 3); // There are 3 files
                                    // Disconnect
        assert!(client.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_sftp_stat() {
        let mut client: SftpFileTransfer = SftpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and sftp
        assert!(client.session.is_some());
        assert!(client.sftp.is_some());
        assert_eq!(client.wrkdir, PathBuf::from("/"));
        let file: FsEntry = client
            .stat(PathBuf::from("readme.txt").as_path())
            .ok()
            .unwrap();
        if let FsEntry::File(file) = file {
            assert_eq!(file.abs_path, PathBuf::from("/readme.txt"));
        } else {
            panic!("Expected readme.txt to be a file");
        }
    }

    #[test]
    fn test_filetransfer_sftp_recv() {
        let mut client: SftpFileTransfer = SftpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and sftp
        assert!(client.session.is_some());
        assert!(client.sftp.is_some());
        assert_eq!(client.wrkdir, PathBuf::from("/"));
        let file: FsFile = FsFile {
            name: String::from("readme.txt"),
            abs_path: PathBuf::from("/readme.txt"),
            last_change_time: SystemTime::UNIX_EPOCH,
            last_access_time: SystemTime::UNIX_EPOCH,
            creation_time: SystemTime::UNIX_EPOCH,
            size: 0,
            ftype: Some(String::from("txt")), // File type
            readonly: true,
            symlink: None,             // UNIX only
            user: Some(0),             // UNIX only
            group: Some(0),            // UNIX only
            unix_pex: Some((6, 4, 4)), // UNIX only
        };
        // Receive file
        assert!(client.recv_file(&file).is_ok());
        // Disconnect
        assert!(client.disconnect().is_ok());
    }
    #[test]
    fn test_filetransfer_sftp_recv_failed_nosuchfile() {
        let mut client: SftpFileTransfer = SftpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and sftp
        assert!(client.session.is_some());
        assert!(client.sftp.is_some());
        assert_eq!(client.wrkdir, PathBuf::from("/"));
        // Receive file
        let file: FsFile = FsFile {
            name: String::from("omar.txt"),
            abs_path: PathBuf::from("/omar.txt"),
            last_change_time: SystemTime::UNIX_EPOCH,
            last_access_time: SystemTime::UNIX_EPOCH,
            creation_time: SystemTime::UNIX_EPOCH,
            size: 0,
            ftype: Some(String::from("txt")), // File type
            readonly: true,
            symlink: None,             // UNIX only
            user: Some(0),             // UNIX only
            group: Some(0),            // UNIX only
            unix_pex: Some((6, 4, 4)), // UNIX only
        };
        assert!(client.recv_file(&file).is_err());
        // Disconnect
        assert!(client.disconnect().is_ok());
    }

    // NOTE: other functions doesn't work with this test SFTP server

    /* NOTE: the server doesn't allow you to create directories
    #[test]
    fn test_filetransfer_sftp_mkdir() {
        let mut client: SftpFileTransfer = SftpFileTransfer::new();
        assert!(client.connect(String::from("test.rebex.net"), 22, Some(String::from("demo")), Some(String::from("password"))).is_ok());
        let dir: String = String::from("foo");
        // Mkdir
        assert!(client.mkdir(dir).is_ok());
        // cwd
        assert!(client.change_dir(PathBuf::from("foo/").as_path()).is_ok());
        assert_eq!(client.wrkdir, PathBuf::from("/foo"));
        // Disconnect
        assert!(client.disconnect().is_ok());
    }
    */
}
