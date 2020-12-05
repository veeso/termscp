//! ## SCP_Transfer
//!
//! `scps_transfer` is the module which provides the implementation for the SCP file transfer

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
extern crate regex;
extern crate ssh2;

// Locals
use super::{FileTransfer, FileTransferError, FileTransferErrorType};
use crate::fs::{FsDirectory, FsEntry, FsFile};
use crate::utils::lstime_to_systime;

// Includes
use regex::Regex;
use ssh2::{Channel, Session};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// ## ScpFileTransfer
///
/// SCP file transfer structure
pub struct ScpFileTransfer {
    session: Option<Session>,
    wrkdir: PathBuf,
}

impl ScpFileTransfer {
    /// ### new
    ///
    /// Instantiates a new ScpFileTransfer
    pub fn new() -> ScpFileTransfer {
        ScpFileTransfer {
            session: None,
            wrkdir: PathBuf::from("~"),
        }
    }

    /// ### parse_ls_output
    ///
    /// Parse a line of `ls -l` output and tokenize the output into a `FsEntry`
    fn parse_ls_output(&self, path: &Path, line: &str) -> Result<FsEntry, ()> {
        // Prepare list regex
        // NOTE: about this damn regex <https://stackoverflow.com/questions/32480890/is-there-a-regex-to-parse-the-values-from-an-ftp-directory-listing>
        lazy_static! {
            static ref LS_RE: Regex = Regex::new(r#"^([\-ld])([\-rwxs]{9})\s+(\d+)\s+(\w+)\s+(\w+)\s+(\d+)\s+(\w{3}\s+\d{1,2}\s+(?:\d{1,2}:\d{1,2}|\d{4}))\s+(.+)$"#).unwrap();
        }
        // Apply regex to result
        match LS_RE.captures(line) {
            // String matches regex
            Some(metadata) => {
                // NOTE: metadata fmt: (regex, file_type, permissions, link_count, uid, gid, filesize, mtime, filename)
                // Expected 7 + 1 (8) values: + 1 cause regex is repeated at 0
                if metadata.len() < 8 {
                    return Err(());
                }
                // Collect metadata
                // Get if is directory and if is symlink
                let (is_dir, is_symlink): (bool, bool) = match metadata.get(1).unwrap().as_str() {
                    "-" => (false, false),
                    "l" => (false, true),
                    "d" => (true, false),
                    _ => return Err(()), // Ignore special files
                };
                // Check string length (unix pex)
                if metadata.get(2).unwrap().as_str().len() < 9 {
                    return Err(());
                }
                // Get unix pex
                let unix_pex: (u8, u8, u8) = {
                    let owner_pex: u8 = {
                        let mut count: u8 = 0;
                        for (i, c) in metadata.get(2).unwrap().as_str()[0..3].chars().enumerate() {
                            match c {
                                '-' => {}
                                _ => {
                                    count = count
                                        + match i {
                                            0 => 4,
                                            1 => 2,
                                            2 => 1,
                                            _ => 0,
                                        }
                                }
                            }
                        }
                        count
                    };
                    let group_pex: u8 = {
                        let mut count: u8 = 0;
                        for (i, c) in metadata.get(2).unwrap().as_str()[3..6].chars().enumerate() {
                            match c {
                                '-' => {}
                                _ => {
                                    count = count
                                        + match i {
                                            0 => 4,
                                            1 => 2,
                                            2 => 1,
                                            _ => 0,
                                        }
                                }
                            }
                        }
                        count
                    };
                    let others_pex: u8 = {
                        let mut count: u8 = 0;
                        for (i, c) in metadata.get(2).unwrap().as_str()[6..9].chars().enumerate() {
                            match c {
                                '-' => {}
                                _ => {
                                    count = count
                                        + match i {
                                            0 => 4,
                                            1 => 2,
                                            2 => 1,
                                            _ => 0,
                                        }
                                }
                            }
                        }
                        count
                    };
                    (owner_pex, group_pex, others_pex)
                };
                // Parse mtime and convert to SystemTime
                let mtime: SystemTime = match lstime_to_systime(
                    metadata.get(7).unwrap().as_str(),
                    "%b %d %Y",
                    "%b %d %H:%M",
                ) {
                    Ok(t) => t,
                    Err(_) => return Err(()),
                };
                // Get uid
                let uid: Option<u32> = match metadata.get(4).unwrap().as_str().parse::<u32>() {
                    Ok(uid) => Some(uid),
                    Err(_) => None,
                };
                // Get gid
                let gid: Option<u32> = match metadata.get(5).unwrap().as_str().parse::<u32>() {
                    Ok(gid) => Some(gid),
                    Err(_) => None,
                };
                // Get filesize
                let filesize: usize = match metadata.get(6).unwrap().as_str().parse::<usize>() {
                    Ok(sz) => sz,
                    Err(_) => return Err(()),
                };
                // Get link and name
                let (file_name, symlink_path): (String, Option<PathBuf>) = match is_symlink {
                    true => self.get_name_and_link(metadata.get(8).unwrap().as_str()),
                    false => (String::from(metadata.get(8).unwrap().as_str()), None),
                };
                let mut abs_path: PathBuf = PathBuf::from(path);
                let extension: Option<String> = match abs_path.as_path().extension() {
                    None => None,
                    Some(s) => Some(String::from(s.to_string_lossy())),
                };
                abs_path.push(file_name.as_str());
                // Return
                // Push to entries
                Ok(match is_dir {
                    true => FsEntry::Directory(FsDirectory {
                        name: file_name,
                        abs_path: abs_path,
                        last_change_time: mtime,
                        last_access_time: mtime,
                        creation_time: mtime,
                        readonly: false,
                        symlink: symlink_path,
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
                        symlink: symlink_path,
                        user: uid,
                        group: gid,
                        unix_pex: Some(unix_pex),
                    }),
                })
            }
            None => Err(()),
        }
    }

    /// ### get_name_and_link
    ///
    /// Returns from a `ls -l` command output file name token, the name of the file and the symbolic link (if there is any)
    fn get_name_and_link(&self, token: &str) -> (String, Option<PathBuf>) {
        let tokens: Vec<&str> = token.split(" -> ").collect();
        let filename: String = String::from(*tokens.get(0).unwrap());
        let symlink: Option<PathBuf> = match tokens.get(1) {
            Some(s) => Some(PathBuf::from(s)),
            None => None,
        };
        (filename, symlink)
    }

    /// ### perform_shell_cmd_with
    ///
    /// Perform a shell command, but change directory to specified path first
    fn perform_shell_cmd_with_path(
        &mut self,
        path: &Path,
        cmd: &str,
    ) -> Result<String, FileTransferError> {
        self.perform_shell_cmd(format!("cd \"{}\"; {}", path.display(), cmd).as_str())
    }

    /// ### perform_shell_cmd
    ///
    /// Perform a shell command and read the output from shell
    /// This operation is, obviously, blocking.
    fn perform_shell_cmd(&mut self, cmd: &str) -> Result<String, FileTransferError> {
        match self.session.as_mut() {
            Some(session) => {
                // Create channel
                let mut channel: Channel = match session.channel_session() {
                    Ok(ch) => ch,
                    Err(err) => {
                        return Err(FileTransferError::new_ex(
                            FileTransferErrorType::ProtocolError,
                            format!("Could not open channel: {}", err),
                        ))
                    }
                };
                // Execute command
                if let Err(err) = channel.exec(cmd) {
                    return Err(FileTransferError::new_ex(
                        FileTransferErrorType::ProtocolError,
                        format!("Could not execute command \"{}\": {}", cmd, err),
                    ));
                }
                // Read output
                let mut output: String = String::new();
                match channel.read_to_string(&mut output) {
                    Ok(_) => {
                        // Wait close
                        let _ = channel.wait_close();
                        Ok(output)
                    }
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::ProtocolError,
                        format!("Could not read output: {}", err),
                    )),
                }
            }
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }
}

impl FileTransfer for ScpFileTransfer {
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
            Some(u) => u.clone(),
            None => String::from(""),
        };
        // Try authenticating with user agent
        if let Err(_) = session.userauth_agent(username.as_str()) {
            // Try authentication with password then
            if let Err(err) = session.userauth_password(
                username.as_str(),
                password.unwrap_or(String::from("")).as_str(),
            ) {
                return Err(FileTransferError::new_ex(
                    FileTransferErrorType::AuthenticationFailed,
                    format!("{}", err),
                ));
            }
        }
        // Get banner
        let banner: Option<String> = match session.banner() {
            Some(s) => Some(String::from(s)),
            None => None,
        };
        // Set session
        self.session = Some(session);
        // Get working directory
        match self.perform_shell_cmd("pwd") {
            Ok(output) => self.wrkdir = PathBuf::from(output.as_str().trim()),
            Err(err) => return Err(err),
        }
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
                        // Set session to none
                        self.session = None;
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
        match self.session.as_ref() {
            Some(_) => true,
            None => false,
        }
    }

    /// ### pwd
    ///
    /// Print working directory

    fn pwd(&mut self) -> Result<PathBuf, FileTransferError> {
        match self.is_connected() {
            true => Ok(self.wrkdir.clone()),
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### change_dir
    ///
    /// Change working directory

    fn change_dir(&mut self, dir: &Path) -> Result<PathBuf, FileTransferError> {
        match self.is_connected() {
            true => {
                let p: PathBuf = self.wrkdir.clone();
                let remote_path: PathBuf = match dir.is_absolute() {
                    true => PathBuf::from(dir),
                    false => {
                        let mut p: PathBuf = PathBuf::from(".");
                        p.push(dir);
                        p
                    }
                };
                // Change directory
                match self.perform_shell_cmd_with_path(
                    p.as_path(),
                    format!("cd \"{}\"; echo $?; pwd", remote_path.display()).as_str(),
                ) {
                    Ok(output) => {
                        // Trim
                        let output: String = String::from(output.as_str().trim());
                        // Check if output starts with 0; should be 0{PWD}
                        match output.as_str().starts_with("0") {
                            true => {
                                // Set working directory
                                self.wrkdir = PathBuf::from(&output.as_str()[1..].trim());
                                Ok(self.wrkdir.clone())
                            }
                            false => Err(FileTransferError::new_ex(
                                // No such file or directory
                                FileTransferErrorType::NoSuchFileOrDirectory,
                                format!("\"{}\"", dir.display()),
                            )),
                        }
                    }
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::ProtocolError,
                        format!("{}", err),
                    )),
                }
            }
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### list_dir
    ///
    /// List directory entries

    fn list_dir(&mut self, path: &Path) -> Result<Vec<FsEntry>, FileTransferError> {
        match self.is_connected() {
            true => {
                // Send ls -l to path
                let p: PathBuf = self.wrkdir.clone();
                match self.perform_shell_cmd_with_path(
                    p.as_path(),
                    format!("ls -l \"{}\"", path.display()).as_str(),
                ) {
                    Ok(output) => {
                        // Split output by (\r)\n
                        let lines: Vec<&str> = output.as_str().lines().collect();
                        let mut entries: Vec<FsEntry> = Vec::with_capacity(lines.len());
                        for line in lines.iter() {
                            // First line must always be ignored
                            // Parse row, if ok push to entries
                            if let Ok(entry) = self.parse_ls_output(path, line) {
                                entries.push(entry);
                            }
                        }
                        Ok(entries)
                    }
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::ProtocolError,
                        format!("{}", err),
                    )),
                }
            }
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### mkdir
    ///
    /// Make directory
    /// You must return error in case the directory already exists
    fn mkdir(&mut self, dir: &Path) -> Result<(), FileTransferError> {
        match self.is_connected() {
            true => {
                let p: PathBuf = self.wrkdir.clone();
                // Mkdir dir && echo 0
                match self.perform_shell_cmd_with_path(
                    p.as_path(),
                    format!("mkdir \"{}\"; echo $?", dir.display()).as_str(),
                ) {
                    Ok(output) => {
                        // Check if output is 0
                        match output.as_str().trim() == "0" {
                            true => Ok(()), // Directory created
                            false => Err(FileTransferError::new_ex(
                                // Could not create directory
                                FileTransferErrorType::FileCreateDenied,
                                format!("\"{}\"", dir.display()),
                            )),
                        }
                    }
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::ProtocolError,
                        format!("{}", err),
                    )),
                }
            }
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### remove
    ///
    /// Remove a file or a directory
    fn remove(&mut self, file: &FsEntry) -> Result<(), FileTransferError> {
        // Yay, we have rm -rf here :D
        match self.is_connected() {
            true => {
                // Get path
                let path: PathBuf = match file {
                    FsEntry::Directory(dir) => dir.abs_path.clone(),
                    FsEntry::File(file) => file.abs_path.clone(),
                };
                let p: PathBuf = self.wrkdir.clone();
                match self.perform_shell_cmd_with_path(
                    p.as_path(),
                    format!("rm -rf \"{}\"; echo $?", path.display()).as_str(),
                ) {
                    Ok(output) => {
                        // Check if output is 0
                        match output.as_str().trim() == "0" {
                            true => Ok(()), // Directory created
                            false => Err(FileTransferError::new_ex(
                                // Could not create directory
                                FileTransferErrorType::PexError,
                                format!("\"{}\"", path.display()),
                            )),
                        }
                    }
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::ProtocolError,
                        format!("{}", err),
                    )),
                }
            }
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### rename
    ///
    /// Rename file or a directory
    fn rename(&mut self, file: &FsEntry, dst: &Path) -> Result<(), FileTransferError> {
        match self.is_connected() {
            true => {
                // Get path
                let path: PathBuf = match file {
                    FsEntry::Directory(dir) => dir.abs_path.clone(),
                    FsEntry::File(file) => file.abs_path.clone(),
                };
                let p: PathBuf = self.wrkdir.clone();
                match self.perform_shell_cmd_with_path(
                    p.as_path(),
                    format!("mv -f \"{}\" {}\"; echo $?", path.display(), dst.display()).as_str(),
                ) {
                    Ok(output) => {
                        // Check if output is 0
                        match output.as_str().trim() == "0" {
                            true => Ok(()), // File renamed
                            false => Err(FileTransferError::new_ex(
                                // Could not move file
                                FileTransferErrorType::PexError,
                                format!("\"{}\"", path.display()),
                            )),
                        }
                    }
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::ProtocolError,
                        format!("{}", err),
                    )),
                }
            }
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### stat
    ///
    /// Stat file and return FsEntry
    fn stat(&mut self, path: &Path) -> Result<FsEntry, FileTransferError> {
        if path.is_dir() {
            return Err(FileTransferError::new_ex(
                FileTransferErrorType::UnsupportedFeature,
                String::from("stat is not supported for directories"),
            ));
        }
        match self.is_connected() {
            true => {
                let p: PathBuf = self.wrkdir.clone();
                match self.perform_shell_cmd_with_path(
                    p.as_path(),
                    format!("ls -l \"{}\"", path.display()).as_str(),
                ) {
                    Ok(line) => {
                        // Parse ls line
                        let parent: PathBuf = match path.parent() {
                            Some(p) => PathBuf::from(p),
                            None => {
                                return Err(FileTransferError::new_ex(
                                    FileTransferErrorType::UnsupportedFeature,
                                    String::from("Path has no parent"),
                                ))
                            }
                        };
                        match self.parse_ls_output(parent.as_path(), line.as_str()) {
                            Ok(entry) => Ok(entry),
                            Err(_) => Err(FileTransferError::new(
                                FileTransferErrorType::NoSuchFileOrDirectory,
                            )),
                        }
                    }
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::ProtocolError,
                        format!("{}", err),
                    )),
                }
            }
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

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
    ) -> Result<Box<dyn Write>, FileTransferError> {
        match self.session.as_ref() {
            Some(session) => {
                // Set blocking to true
                session.set_blocking(true);
                // Calculate file mode
                let mode: i32 = match local.unix_pex {
                    None => 0o644,
                    Some((u, g, o)) => ((u as i32) << 6) + ((g as i32) << 3) + (o as i32),
                };
                // Calculate mtime, atime
                let times: (u64, u64) = {
                    let mtime: u64 = match local
                        .last_change_time
                        .duration_since(SystemTime::UNIX_EPOCH)
                    {
                        Ok(durr) => durr.as_secs() as u64,
                        Err(_) => 0,
                    };
                    let atime: u64 = match local
                        .last_access_time
                        .duration_since(SystemTime::UNIX_EPOCH)
                    {
                        Ok(durr) => durr.as_secs() as u64,
                        Err(_) => 0,
                    };
                    (mtime, atime)
                };
                match session.scp_send(file_name, mode, local.size as u64, Some(times)) {
                    Ok(channel) => Ok(Box::new(channel)),
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::ProtocolError,
                        format!("{}", err),
                    )),
                }
            }
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### recv_file
    ///
    /// Receive file from remote with provided name
    /// Returns file and its size
    fn recv_file(&mut self, file: &FsFile) -> Result<Box<dyn Read>, FileTransferError> {
        match self.session.as_ref() {
            Some(session) => {
                // Set blocking to true
                session.set_blocking(true);
                match session.scp_recv(file.abs_path.as_path()) {
                    Ok(reader) => Ok(Box::new(reader.0)),
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::ProtocolError,
                        format!("{}", err),
                    )),
                }
            }
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### on_sent
    ///
    /// Finalize send method.
    /// This method must be implemented only if necessary; in case you don't need it, just return `Ok(())`
    /// The purpose of this method is to finalize the connection with the peer when writing data.
    /// This is necessary for some protocols such as FTP.
    /// You must call this method each time you want to finalize the write of the remote file.
    fn on_sent(&mut self, _writable: Box<dyn Write>) -> Result<(), FileTransferError> {
        if let Some(session) = self.session.as_ref() {
            // Set blocking to false
            session.set_blocking(false);
        }
        Ok(())
    }

    /// ### on_recv
    ///
    /// Finalize recv method.
    /// This method must be implemented only if necessary; in case you don't need it, just return `Ok(())`
    /// The purpose of this method is to finalize the connection with the peer when reading data.
    /// This mighe be necessary for some protocols.
    /// You must call this method each time you want to finalize the read of the remote file.
    fn on_recv(&mut self, _readable: Box<dyn Read>) -> Result<(), FileTransferError> {
        if let Some(session) = self.session.as_ref() {
            // Set blocking to false
            session.set_blocking(false);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_filetransfer_scp_new() {
        let client: ScpFileTransfer = ScpFileTransfer::new();
        assert!(client.session.is_none());
        assert_eq!(client.is_connected(), false);
    }

    #[test]
    fn test_filetransfer_scp_connect() {
        let mut client: ScpFileTransfer = ScpFileTransfer::new();
        assert_eq!(client.is_connected(), false);
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and scp
        assert!(client.session.is_some());
        assert_eq!(client.is_connected(), true);
        // Disconnect
        assert!(client.disconnect().is_ok());
        assert_eq!(client.is_connected(), false);
    }
    #[test]
    fn test_filetransfer_scp_bad_auth() {
        let mut client: ScpFileTransfer = ScpFileTransfer::new();
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
    fn test_filetransfer_scp_no_credentials() {
        let mut client: ScpFileTransfer = ScpFileTransfer::new();
        assert!(client
            .connect(String::from("test.rebex.net"), 22, None, None)
            .is_err());
    }

    #[test]
    fn test_filetransfer_scp_bad_server() {
        let mut client: ScpFileTransfer = ScpFileTransfer::new();
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
    fn test_filetransfer_scp_pwd() {
        let mut client: ScpFileTransfer = ScpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and scp
        assert!(client.session.is_some());
        // Pwd
        assert_eq!(client.pwd().ok().unwrap(), PathBuf::from("/"));
        // Disconnect
        assert!(client.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_scp_cwd() {
        let mut client: ScpFileTransfer = ScpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and scp
        assert!(client.session.is_some());
        // Cwd (relative)
        assert!(client.change_dir(PathBuf::from("pub/").as_path()).is_ok());
        // Cwd (absolute)
        assert!(client.change_dir(PathBuf::from("/pub").as_path()).is_ok());
        // Disconnect
        assert!(client.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_scp_cwd_error() {
        let mut client: ScpFileTransfer = ScpFileTransfer::new();
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
    fn test_filetransfer_scp_ls() {
        let mut client: ScpFileTransfer = ScpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and scp
        assert!(client.session.is_some());
        // List dir
        let pwd: PathBuf = client.pwd().ok().unwrap();
        let files: Vec<FsEntry> = client.list_dir(pwd.as_path()).ok().unwrap();
        assert_eq!(files.len(), 5); // There are 5 files
                                    // Disconnect
        assert!(client.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_scp_stat() {
        let mut client: ScpFileTransfer = ScpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and scp
        assert!(client.session.is_some());
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
    fn test_filetransfer_scp_recv() {
        let mut client: ScpFileTransfer = ScpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and scp
        assert!(client.session.is_some());
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
    fn test_filetransfer_scp_recv_failed_nosuchfile() {
        let mut client: ScpFileTransfer = ScpFileTransfer::new();
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                22,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Check session and scp
        assert!(client.session.is_some());
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
    // NOTE: other functions doesn't work with this test scp server

    /* NOTE: the server doesn't allow you to create directories
    #[test]
    fn test_filetransfer_scp_mkdir() {
        let mut client: ScpFileTransfer = ScpFileTransfer::new();
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
