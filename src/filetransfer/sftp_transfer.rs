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
use super::{FileTransfer, FileTransferError};
use crate::fs::{FsDirectory, FsEntry, FsFile};

// Includes
use ssh2::{Session, Sftp};
use std::fs::File;
use std::io::{Read, Seek, Write};
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
                        Ok(_) => Ok(PathBuf::from(p)),
                        Err(_) => Err(FileTransferError::NoSuchFileOrDirectory),
                    },
                    Err(_) => Err(FileTransferError::NoSuchFileOrDirectory),
                }
            }
            false => match self.sftp.as_ref().unwrap().realpath(p) {
                Ok(p) => match self.sftp.as_ref().unwrap().stat(p.as_path()) {
                    Ok(_) => Ok(PathBuf::from(p)),
                    Err(_) => Err(FileTransferError::NoSuchFileOrDirectory),
                },
                Err(_) => Err(FileTransferError::NoSuchFileOrDirectory),
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
    ) -> Result<(), FileTransferError> {
        // Setup tcp stream
        let tcp: TcpStream = match TcpStream::connect(format!("{}:{}", address, port)) {
            Ok(stream) => stream,
            Err(_) => return Err(FileTransferError::BadAddress),
        };
        // Create session
        let mut session: Session = match Session::new() {
            Ok(s) => s,
            Err(_) => return Err(FileTransferError::ConnectionError),
        };
        // Set TCP stream
        session.set_tcp_stream(tcp);
        // Open connection
        if let Err(_) = session.handshake() {
            return Err(FileTransferError::ConnectionError);
        }
        // Try authentication
        if let Err(_) = session.userauth_password(
            username.unwrap_or(String::from("")).as_str(),
            password.unwrap_or(String::from("")).as_str(),
        ) {
            return Err(FileTransferError::AuthenticationFailed);
        }
        // Set blocking to true
        session.set_blocking(true);
        // Get Sftp client
        let sftp: Sftp = match session.sftp() {
            Ok(s) => s,
            Err(_) => return Err(FileTransferError::ProtocolError),
        };
        // Get working directory
        self.wrkdir = match sftp.realpath(PathBuf::from(".").as_path()) {
            Ok(p) => p,
            Err(_) => return Err(FileTransferError::ProtocolError),
        };
        // Set session
        self.session = Some(session);
        // Set sftp
        self.sftp = Some(sftp);
        Ok(())
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
                    Err(_) => Err(FileTransferError::ConnectionError),
                }
            }
            None => Err(FileTransferError::UninitializedSession),
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
    fn pwd(&self) -> Result<PathBuf, FileTransferError> {
        match self.sftp {
            Some(_) => Ok(self.wrkdir.clone()),
            None => Err(FileTransferError::UninitializedSession),
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
            None => Err(FileTransferError::UninitializedSession),
        }
    }

    /// ### list_dir
    ///
    /// List directory entries
    fn list_dir(&self, path: &Path) -> Result<Vec<FsEntry>, FileTransferError> {
        match self.sftp.as_ref() {
            Some(sftp) => {
                // Get path
                let dir: PathBuf = match self.get_remote_path(path) {
                    Ok(p) => p,
                    Err(err) => return Err(err),
                };
                // Get files
                match sftp.readdir(dir.as_path()) {
                    Err(_) => return Err(FileTransferError::DirStatFailed),
                    Ok(files) => {
                        // Allocate vector
                        let mut entries: Vec<FsEntry> = Vec::with_capacity(files.len());
                        // Iterate over files
                        for (path, metadata) in files {
                            // Get common parameters
                            let file_name: String =
                                String::from(path.file_name().unwrap().to_str().unwrap_or(""));
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
                            let symlink: Option<PathBuf> = match is_symlink {
                                true => {
                                    // Read symlink
                                    match sftp.readlink(path.as_path()) {
                                        Ok(p) => Some(p),
                                        Err(_) => None,
                                    }
                                }
                                false => None,
                            };
                            // Is a directory?
                            match metadata.is_dir() {
                                true => {
                                    entries.push(FsEntry::Directory(FsDirectory {
                                        name: file_name,
                                        abs_path: path.clone(),
                                        last_change_time: mtime,
                                        last_access_time: atime,
                                        creation_time: SystemTime::UNIX_EPOCH,
                                        readonly: false,
                                        symlink: symlink,
                                        user: uid,
                                        group: gid,
                                        unix_pex: pex,
                                    }));
                                }
                                false => {
                                    entries.push(FsEntry::File(FsFile {
                                        name: file_name,
                                        abs_path: path.clone(),
                                        size: size as usize,
                                        ftype: file_type,
                                        last_change_time: mtime,
                                        last_access_time: atime,
                                        creation_time: SystemTime::UNIX_EPOCH,
                                        readonly: false,
                                        symlink: symlink,
                                        user: uid,
                                        group: gid,
                                        unix_pex: pex,
                                    }));
                                }
                            }
                        }
                        Ok(entries)
                    }
                }
            }
            None => Err(FileTransferError::UninitializedSession),
        }
    }

    /// ### mkdir
    ///
    /// Make directory
    fn mkdir(&self, dir: &Path) -> Result<(), FileTransferError> {
        match self.sftp.as_ref() {
            Some(sftp) => {
                // Make directory
                let path: PathBuf = self.get_abs_path(PathBuf::from(dir).as_path());
                match sftp.mkdir(path.as_path(), 0o775) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(FileTransferError::FileCreateDenied),
                }
            }
            None => Err(FileTransferError::UninitializedSession),
        }
    }

    /// ### remove
    ///
    /// Remove a file or a directory
    fn remove(&self, file: &FsEntry) -> Result<(), FileTransferError> {
        match self.sftp.as_ref() {
            None => Err(FileTransferError::UninitializedSession),
            Some(sftp) => {
                // Match if file is a file or a directory
                match file {
                    FsEntry::File(f) => {
                        // Remove file
                        match sftp.unlink(f.abs_path.as_path()) {
                            Ok(_) => Ok(()),
                            Err(_) => Err(FileTransferError::FileReadonly),
                        }
                    }
                    FsEntry::Directory(d) => {
                        // Remove recursively
                        // Get directory files
                        match self.list_dir(d.abs_path.as_path()) {
                            Ok(entries) => {
                                // Remove each entry
                                for entry in entries {
                                    if let Err(err) = self.remove(&entry) {
                                        return Err(err);
                                    }
                                }
                                // Finally remove directory
                                match sftp.rmdir(d.abs_path.as_path()) {
                                    Ok(_) => Ok(()),
                                    Err(_) => Err(FileTransferError::FileReadonly),
                                }
                            }
                            Err(err) => return Err(err),
                        }
                    }
                }
            }
        }
    }

    /// ### rename
    ///
    /// Rename file or a directory
    fn rename(&self, file: &FsEntry, dst: &Path) -> Result<(), FileTransferError> {
        match self.sftp.as_ref() {
            None => Err(FileTransferError::UninitializedSession),
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
                    Err(_) => Err(FileTransferError::FileCreateDenied),
                }
            }
        }
    }

    /// ### send_file
    ///
    /// Send file to remote
    /// File name is referred to the name of the file as it will be saved
    /// Data contains the file data
    fn send_file(&self, file_name: &Path, file: &mut File) -> Result<(), FileTransferError> {
        match self.sftp.as_ref() {
            None => Err(FileTransferError::UninitializedSession),
            Some(sftp) => {
                let remote_path: PathBuf = self.get_abs_path(file_name);
                // Get file size
                //let file_size: usize = file.seek(std::io::SeekFrom::End(0)).unwrap_or(0) as usize;
                // rewind
                if let Err(err) = file.seek(std::io::SeekFrom::Start(0)) {
                    return Err(FileTransferError::IoErr(err));
                };
                // Open remote file
                match sftp.create(remote_path.as_path()) {
                    Ok(mut rhnd) => {
                        //let mut total_bytes_written: usize = 0;
                        loop {
                            // Read till you can
                            let mut buffer: [u8; 8192] = [0; 8192];
                            match file.read(&mut buffer) {
                                Ok(bytes_read) => {
                                    //total_bytes_written += bytes_read;
                                    if bytes_read == 0 {
                                        break;
                                    } else {
                                        // Write bytes
                                        if let Err(err) = rhnd.write(&buffer) {
                                            return Err(FileTransferError::IoErr(err));
                                        }
                                    }
                                }
                                Err(err) => return Err(FileTransferError::IoErr(err)),
                            }
                        }
                        Ok(())
                    }
                    Err(_) => return Err(FileTransferError::FileCreateDenied),
                }
            }
        }
    }

    /// ### recv_file
    ///
    /// Receive file from remote with provided name
    fn recv_file(&self, file_name: &Path, dest_file: &mut File) -> Result<(), FileTransferError> {
        match self.sftp.as_ref() {
            None => Err(FileTransferError::UninitializedSession),
            Some(sftp) => {
                // Get remote file name
                let remote_path: PathBuf = match self.get_remote_path(file_name) {
                    Ok(p) => p,
                    Err(err) => return Err(err),
                };
                // Open remote file
                match sftp.open(remote_path.as_path()) {
                    Ok(mut rhnd) => {
                        // let file_size: usize = rhnd.seek(std::io::SeekFrom::End(0)).unwrap_or(0) as usize;
                        // rewind
                        if let Err(err) = rhnd.seek(std::io::SeekFrom::Start(0)) {
                            return Err(FileTransferError::IoErr(err));
                        };
                        // Write local file
                        // let mut total_bytes_written: usize = 0;
                        loop {
                            // Read till you can
                            let mut buffer: [u8; 8192] = [0; 8192];
                            match rhnd.read(&mut buffer) {
                                Ok(bytes_read) => {
                                    // total_bytes_written += bytes_read;
                                    if bytes_read == 0 {
                                        break;
                                    } else {
                                        // Write bytes
                                        if let Err(err) = dest_file.write(&buffer) {
                                            return Err(FileTransferError::IoErr(err));
                                        }
                                    }
                                }
                                Err(err) => return Err(FileTransferError::IoErr(err)),
                            }
                        }
                        Ok(())
                    }
                    Err(_) => return Err(FileTransferError::NoSuchFileOrDirectory),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::OpenOptions;

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
        assert_eq!(client.wrkdir, client.pwd().ok().unwrap());
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
        assert_eq!(client.wrkdir, client.pwd().ok().unwrap());
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
        let files: Vec<FsEntry> = client
            .list_dir(client.pwd().ok().unwrap().as_path())
            .ok()
            .unwrap();
        assert_eq!(files.len(), 3); // There are 3 files
                                    // Disconnect
        assert!(client.disconnect().is_ok());
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
        // Open dest file
        let dst_file: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        let mut dst_file_hnd: File = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(dst_file.path())
            .unwrap();
        // Receive file
        assert!(client
            .recv_file(PathBuf::from("readme.txt").as_path(), &mut dst_file_hnd)
            .is_ok());
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
        // Open dest file
        let dst_file: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        let mut dst_file_hnd: File = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(dst_file.path())
            .unwrap();
        // Receive file
        assert!(client
            .recv_file(PathBuf::from("omar.txt").as_path(), &mut dst_file_hnd)
            .is_err());
        // Disconnect
        assert!(client.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_sftp_recv_failed_io() {
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
        // Open dest file
        let dst_file: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        let mut dst_file_hnd: File = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(dst_file.path())
            .unwrap();
        assert!(writeln!(dst_file_hnd, "Test test test").is_ok());
        let mut dst_file_hnd: File = OpenOptions::new().read(true).open(dst_file.path()).unwrap();
        // Receive file
        assert!(client
            .recv_file(PathBuf::from("readme.txt").as_path(), &mut dst_file_hnd)
            .is_err());
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
