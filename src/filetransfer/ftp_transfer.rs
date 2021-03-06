//! ## Ftp_transfer
//!
//! `ftp_transfer` is the module which provides the implementation for the FTP/FTPS file transfer

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
// Dependencies
extern crate chrono;
extern crate ftp4;
#[cfg(os_target = "windows")]
extern crate path_slash;
extern crate regex;

use super::{FileTransfer, FileTransferError, FileTransferErrorType};
use crate::fs::{FsDirectory, FsEntry, FsFile};
use crate::utils::fmt::{fmt_time, shadow_password};
use crate::utils::parser::{parse_datetime, parse_lstime};

// Includes
use ftp4::native_tls::TlsConnector;
use ftp4::{types::FileType, FtpStream};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{
    io::{Read, Write},
    ops::Range,
};

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
        FtpFileTransfer { stream: None, ftps }
    }

    /// ### resolve
    ///
    /// Fix provided path; on Windows fixes the backslashes, converting them to slashes
    /// While on POSIX does nothing
    #[cfg(target_os = "windows")]
    fn resolve(p: &Path) -> PathBuf {
        PathBuf::from(path_slash::PathExt::to_slash_lossy(p).as_str())
    }

    #[cfg(target_family = "unix")]
    fn resolve(p: &Path) -> PathBuf {
        p.to_path_buf()
    }

    /// ### parse_list_line
    ///
    /// Parse a line of LIST command output and instantiates an FsEntry from it
    fn parse_list_line(&mut self, path: &Path, line: &str) -> Result<FsEntry, ()> {
        // Try to parse using UNIX syntax
        match self.parse_unix_list_line(path, line) {
            Ok(entry) => Ok(entry),
            Err(_) => match self.parse_dos_list_line(path, line) {
                // If UNIX parsing fails, try DOS
                Ok(entry) => Ok(entry),
                Err(_) => Err(()),
            },
        }
    }

    /// ### parse_unix_list_line
    ///
    /// Try to parse a "LIST" output command line in UNIX format.
    /// Returns error if syntax is not UNIX compliant.
    /// UNIX syntax has the following syntax:
    /// {FILE_TYPE}{UNIX_PEX} {HARD_LINKS} {USER} {GROUP} {SIZE} {DATE} {FILENAME}
    /// -rw-r--r--   1 cvisintin  staff   4968 27 Dic 10:46 CHANGELOG.md
    fn parse_unix_list_line(&mut self, path: &Path, line: &str) -> Result<FsEntry, ()> {
        // Prepare list regex
        // NOTE: about this damn regex <https://stackoverflow.com/questions/32480890/is-there-a-regex-to-parse-the-values-from-an-ftp-directory-listing>
        lazy_static! {
            static ref LS_RE: Regex = Regex::new(r#"^([\-ld])([\-rwxs]{9})\s+(\d+)\s+(\w+)\s+(\w+)\s+(\d+)\s+(\w{3}\s+\d{1,2}\s+(?:\d{1,2}:\d{1,2}|\d{4}))\s+(.+)$"#).unwrap();
        }
        debug!("Parsing LIST (UNIX) line: '{}'", line);
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
                let (mut is_dir, is_symlink): (bool, bool) = match metadata.get(1).unwrap().as_str()
                {
                    "-" => (false, false),
                    "l" => (false, true),
                    "d" => (true, false),
                    _ => return Err(()), // Ignore special files
                };
                // Check string length (unix pex)
                if metadata.get(2).unwrap().as_str().len() < 9 {
                    return Err(());
                }

                let pex = |range: Range<usize>| {
                    let mut count: u8 = 0;
                    for (i, c) in metadata.get(2).unwrap().as_str()[range].chars().enumerate() {
                        match c {
                            '-' => {}
                            _ => {
                                count += match i {
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

                // Get unix pex
                let unix_pex = (pex(0..3), pex(3..6), pex(6..9));

                // Parse mtime and convert to SystemTime
                let mtime: SystemTime = match parse_lstime(
                    metadata.get(7).unwrap().as_str(),
                    "%b %d %Y",
                    "%b %d %H:%M",
                ) {
                    Ok(t) => t,
                    Err(_) => SystemTime::UNIX_EPOCH,
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
                let filesize: usize = metadata
                    .get(6)
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap_or(0);
                // Split filename if required
                let (file_name, symlink_path): (String, Option<PathBuf>) = match is_symlink {
                    true => self.get_name_and_link(metadata.get(8).unwrap().as_str()),
                    false => (String::from(metadata.get(8).unwrap().as_str()), None),
                };
                // Check if file_name is '.' or '..'
                if file_name.as_str() == "." || file_name.as_str() == ".." {
                    debug!("File name is {}; ignoring entry", file_name);
                    return Err(());
                }
                // Get symlink
                let symlink: Option<Box<FsEntry>> = symlink_path.map(|p| {
                    Box::new(match p.to_string_lossy().ends_with('/') {
                        true => {
                            // NOTE: is_dir becomes true
                            is_dir = true;
                            FsEntry::Directory(FsDirectory {
                                name: p
                                    .file_name()
                                    .unwrap_or(&std::ffi::OsStr::new(""))
                                    .to_string_lossy()
                                    .to_string(),
                                abs_path: p.clone(),
                                last_change_time: mtime,
                                last_access_time: mtime,
                                creation_time: mtime,
                                readonly: false,
                                symlink: None,
                                user: uid,
                                group: gid,
                                unix_pex: Some(unix_pex),
                            })
                        }
                        false => FsEntry::File(FsFile {
                            name: p
                                .file_name()
                                .unwrap_or(&std::ffi::OsStr::new(""))
                                .to_string_lossy()
                                .to_string(),
                            abs_path: p.clone(),
                            last_change_time: mtime,
                            last_access_time: mtime,
                            creation_time: mtime,
                            readonly: false,
                            symlink: None,
                            size: filesize,
                            ftype: p.extension().map(|s| String::from(s.to_string_lossy())),
                            user: uid,
                            group: gid,
                            unix_pex: Some(unix_pex),
                        }),
                    })
                });
                let mut abs_path: PathBuf = PathBuf::from(path);
                abs_path.push(file_name.as_str());
                let abs_path: PathBuf = Self::resolve(abs_path.as_path());
                // get extension
                let extension: Option<String> = abs_path
                    .as_path()
                    .extension()
                    .map(|s| String::from(s.to_string_lossy()));
                // Return
                debug!("Follows LIST line '{}' attributes", line);
                debug!("Is directory? {}", is_dir);
                debug!("Is symlink? {}", is_symlink);
                debug!("name: {}", file_name);
                debug!("abs_path: {}", abs_path.display());
                debug!("last_change_time: {}", fmt_time(mtime, "%Y-%m-%dT%H:%M:%S"));
                debug!("last_access_time: {}", fmt_time(mtime, "%Y-%m-%dT%H:%M:%S"));
                debug!("creation_time: {}", fmt_time(mtime, "%Y-%m-%dT%H:%M:%S"));
                debug!("symlink: {:?}", symlink);
                debug!("user: {:?}", uid);
                debug!("group: {:?}", gid);
                debug!("unix_pex: {:?}", unix_pex);
                debug!("---------------------------------------");
                // Push to entries
                Ok(match is_dir {
                    true => FsEntry::Directory(FsDirectory {
                        name: file_name,
                        abs_path,
                        last_change_time: mtime,
                        last_access_time: mtime,
                        creation_time: mtime,
                        readonly: false,
                        symlink,
                        user: uid,
                        group: gid,
                        unix_pex: Some(unix_pex),
                    }),
                    false => FsEntry::File(FsFile {
                        name: file_name,
                        abs_path,
                        last_change_time: mtime,
                        last_access_time: mtime,
                        creation_time: mtime,
                        size: filesize,
                        ftype: extension,
                        readonly: false,
                        symlink,
                        user: uid,
                        group: gid,
                        unix_pex: Some(unix_pex),
                    }),
                })
            }
            None => Err(()),
        }
    }

    /// ### parse_dos_list_line
    ///
    /// Try to parse a "LIST" output command line in DOS format.
    /// Returns error if syntax is not DOS compliant.
    /// DOS syntax has the following syntax:
    /// {DATE} {TIME} {<DIR> | SIZE} {FILENAME}
    /// 10-19-20  03:19PM <DIR> pub
    /// 04-08-14  03:09PM 403   readme.txt
    fn parse_dos_list_line(&self, path: &Path, line: &str) -> Result<FsEntry, ()> {
        // Prepare list regex
        // NOTE: you won't find this regex on the internet. It seems I'm the only person in the world who needs this
        lazy_static! {
            static ref DOS_RE: Regex = Regex::new(
                r#"^(\d{2}\-\d{2}\-\d{2}\s+\d{2}:\d{2}\s*[AP]M)\s+(<DIR>)?([\d,]*)\s+(.+)$"#
            )
            .unwrap();
        }
        debug!("Parsing LIST (DOS) line: '{}'", line);
        // Apply regex to result
        match DOS_RE.captures(line) {
            // String matches regex
            Some(metadata) => {
                // NOTE: metadata fmt: (regex, date_time, is_dir?, file_size?, file_name)
                // Expected 4 + 1 (5) values: + 1 cause regex is repeated at 0
                if metadata.len() < 5 {
                    return Err(());
                }
                // Parse date time
                let time: SystemTime =
                    match parse_datetime(metadata.get(1).unwrap().as_str(), "%d-%m-%y %I:%M%p") {
                        Ok(t) => t,
                        Err(_) => SystemTime::UNIX_EPOCH, // Don't return error
                    };
                // Get if is a directory
                let is_dir: bool = metadata.get(2).is_some();
                // Get file size
                let file_size: usize = match is_dir {
                    true => 0, // If is directory, filesize is 0
                    false => match metadata.get(3) {
                        // If is file, parse arg 3
                        Some(val) => val.as_str().parse::<usize>().unwrap_or(0),
                        None => 0, // Should not happen
                    },
                };
                // Get file name
                let file_name: String = String::from(metadata.get(4).unwrap().as_str());
                // Get absolute path
                let mut abs_path: PathBuf = PathBuf::from(path);
                abs_path.push(file_name.as_str());
                let abs_path: PathBuf = Self::resolve(abs_path.as_path());
                // Get extension
                let extension: Option<String> = abs_path
                    .as_path()
                    .extension()
                    .map(|s| String::from(s.to_string_lossy()));
                debug!("Follows LIST line '{}' attributes", line);
                debug!("Is directory? {}", is_dir);
                debug!("name: {}", file_name);
                debug!("abs_path: {}", abs_path.display());
                debug!("last_change_time: {}", fmt_time(time, "%Y-%m-%dT%H:%M:%S"));
                debug!("last_access_time: {}", fmt_time(time, "%Y-%m-%dT%H:%M:%S"));
                debug!("creation_time: {}", fmt_time(time, "%Y-%m-%dT%H:%M:%S"));
                debug!("---------------------------------------");
                // Return entry
                Ok(match is_dir {
                    true => FsEntry::Directory(FsDirectory {
                        name: file_name,
                        abs_path,
                        last_change_time: time,
                        last_access_time: time,
                        creation_time: time,
                        readonly: false,
                        symlink: None,
                        user: None,
                        group: None,
                        unix_pex: None,
                    }),
                    false => FsEntry::File(FsFile {
                        name: file_name,
                        abs_path,
                        last_change_time: time,
                        last_access_time: time,
                        creation_time: time,
                        size: file_size,
                        ftype: extension,
                        readonly: false,
                        symlink: None,
                        user: None,
                        group: None,
                        unix_pex: None,
                    }),
                })
            }
            None => Err(()), // Invalid syntax
        }
    }

    /// ### get_name_and_link
    ///
    /// Returns from a `ls -l` command output file name token, the name of the file and the symbolic link (if there is any)
    fn get_name_and_link(&self, token: &str) -> (String, Option<PathBuf>) {
        let tokens: Vec<&str> = token.split(" -> ").collect();
        let filename: String = String::from(*tokens.get(0).unwrap());
        let symlink: Option<PathBuf> = tokens.get(1).map(PathBuf::from);
        (filename, symlink)
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
    ) -> Result<Option<String>, FileTransferError> {
        // Get stream
        info!("Connecting to {}:{}", address, port);
        let mut stream: FtpStream = match FtpStream::connect(format!("{}:{}", address, port)) {
            Ok(stream) => stream,
            Err(err) => {
                error!("Failed to connect: {}", err);
                return Err(FileTransferError::new_ex(
                    FileTransferErrorType::ConnectionError,
                    err.to_string(),
                ));
            }
        };
        // If SSL, open secure session
        if self.ftps {
            info!("Setting up TLS stream...");
            let ctx = match TlsConnector::builder()
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true)
                .build()
            {
                Ok(tls) => tls,
                Err(err) => {
                    error!("Failed to setup TLS stream: {}", err);
                    return Err(FileTransferError::new_ex(
                        FileTransferErrorType::SslError,
                        err.to_string(),
                    ));
                }
            };
            stream = match stream.into_secure(ctx, address.as_str()) {
                Ok(s) => s,
                Err(err) => {
                    error!("Failed to setup TLS stream: {}", err);
                    return Err(FileTransferError::new_ex(
                        FileTransferErrorType::SslError,
                        err.to_string(),
                    ));
                }
            };
        }
        // Login (use anonymous if credentials are unspecified)
        let username: String = match username {
            Some(u) => u,
            None => String::from("anonymous"),
        };
        let password: String = match password {
            Some(pwd) => pwd,
            None => String::new(),
        };
        info!(
            "Signin in with username: {}, password: {}",
            username,
            shadow_password(password.as_str())
        );
        if let Err(err) = stream.login(username.as_str(), password.as_str()) {
            error!("Login failed: {}", err);
            return Err(FileTransferError::new_ex(
                FileTransferErrorType::AuthenticationFailed,
                err.to_string(),
            ));
        }
        debug!("Setting transfer type to Binary");
        // Initialize file type
        if let Err(err) = stream.transfer_type(FileType::Binary) {
            error!("Failed to set transfer type to binary: {}", err);
            return Err(FileTransferError::new_ex(
                FileTransferErrorType::ProtocolError,
                err.to_string(),
            ));
        }
        // Set stream
        self.stream = Some(stream);
        info!("Connection successfully established");
        // Return OK
        Ok(self.stream.as_ref().unwrap().get_welcome_msg())
    }

    /// ### disconnect
    ///
    /// Disconnect from the remote server

    fn disconnect(&mut self) -> Result<(), FileTransferError> {
        info!("Disconnecting from FTP server...");
        match &mut self.stream {
            Some(stream) => match stream.quit() {
                Ok(_) => {
                    self.stream = None;
                    Ok(())
                }
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::ConnectionError,
                    err.to_string(),
                )),
            },
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### is_connected
    ///
    /// Indicates whether the client is connected to remote
    fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    /// ### pwd
    ///
    /// Print working directory

    fn pwd(&mut self) -> Result<PathBuf, FileTransferError> {
        info!("PWD");
        match &mut self.stream {
            Some(stream) => match stream.pwd() {
                Ok(path) => Ok(PathBuf::from(path.as_str())),
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::ConnectionError,
                    err.to_string(),
                )),
            },
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### change_dir
    ///
    /// Change working directory

    fn change_dir(&mut self, dir: &Path) -> Result<PathBuf, FileTransferError> {
        let dir: PathBuf = Self::resolve(dir);
        info!("Changing directory to {}", dir.display());
        match &mut self.stream {
            Some(stream) => match stream.cwd(&dir.as_path().to_string_lossy()) {
                Ok(_) => Ok(dir),
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::ConnectionError,
                    err.to_string(),
                )),
            },
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### copy
    ///
    /// Copy file to destination
    fn copy(&mut self, _src: &FsEntry, _dst: &Path) -> Result<(), FileTransferError> {
        // FTP doesn't support file copy
        debug!("COPY issues (will fail, since unsupported)");
        Err(FileTransferError::new(
            FileTransferErrorType::UnsupportedFeature,
        ))
    }

    /// ### list_dir
    ///
    /// List directory entries

    fn list_dir(&mut self, path: &Path) -> Result<Vec<FsEntry>, FileTransferError> {
        let dir: PathBuf = Self::resolve(path);
        info!("LIST dir {}", dir.display());
        match &mut self.stream {
            Some(stream) => match stream.list(Some(&dir.as_path().to_string_lossy())) {
                Ok(entries) => {
                    debug!("Got {} lines in LIST result", entries.len());
                    // Prepare result
                    let mut result: Vec<FsEntry> = Vec::with_capacity(entries.len());
                    // Iterate over entries
                    for entry in entries.iter() {
                        if let Ok(file) = self.parse_list_line(dir.as_path(), entry) {
                            result.push(file);
                        }
                    }
                    debug!(
                        "{} out of {} were valid entries",
                        result.len(),
                        entries.len()
                    );
                    Ok(result)
                }
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::DirStatFailed,
                    err.to_string(),
                )),
            },
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### mkdir
    ///
    /// Make directory
    fn mkdir(&mut self, dir: &Path) -> Result<(), FileTransferError> {
        let dir: PathBuf = Self::resolve(dir);
        info!("MKDIR {}", dir.display());
        match &mut self.stream {
            Some(stream) => match stream.mkdir(&dir.as_path().to_string_lossy()) {
                Ok(_) => Ok(()),
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::FileCreateDenied,
                    err.to_string(),
                )),
            },
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### remove
    ///
    /// Remove a file or a directory
    fn remove(&mut self, fsentry: &FsEntry) -> Result<(), FileTransferError> {
        if self.stream.is_none() {
            return Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            ));
        }
        info!("Removing entry {}", fsentry.get_abs_path().display());
        let wrkdir: PathBuf = self.pwd()?;
        match fsentry {
            // Match fs entry...
            FsEntry::File(file) => {
                // Go to parent directory
                if let Some(parent_dir) = file.abs_path.parent() {
                    debug!("Changing wrkdir to {}", parent_dir.display());
                    self.change_dir(parent_dir)?;
                }
                debug!("entry is a file; removing file {}", file.abs_path.display());
                // Remove file directly
                let result = self
                    .stream
                    .as_mut()
                    .unwrap()
                    .rm(file.name.as_ref())
                    .map(|_| ())
                    .map_err(|e| {
                        FileTransferError::new_ex(FileTransferErrorType::PexError, e.to_string())
                    });
                // Go to source directory
                match self.change_dir(wrkdir.as_path()) {
                    Err(err) => Err(err),
                    Ok(_) => result,
                }
            }
            FsEntry::Directory(dir) => {
                // Get directory files
                debug!("Entry is a directory; iterating directory entries");
                let result = match self.list_dir(dir.abs_path.as_path()) {
                    Ok(files) => {
                        // Remove recursively files
                        debug!("Removing {} entries from directory...", files.len());
                        for file in files.iter() {
                            if let Err(err) = self.remove(&file) {
                                return Err(FileTransferError::new_ex(
                                    FileTransferErrorType::PexError,
                                    err.to_string(),
                                ));
                            }
                        }
                        // Once all files in directory have been deleted, remove directory
                        debug!("Finally removing directory {}...", dir.name);
                        // Enter parent directory
                        if let Some(parent_dir) = dir.abs_path.parent() {
                            debug!(
                                "Changing wrkdir to {} to delete directory {}",
                                parent_dir.display(),
                                dir.name
                            );
                            self.change_dir(parent_dir)?;
                        }
                        match self.stream.as_mut().unwrap().rmdir(dir.name.as_str()) {
                            Ok(_) => {
                                debug!("Removed {}", dir.abs_path.display());
                                Ok(())
                            }
                            Err(err) => Err(FileTransferError::new_ex(
                                FileTransferErrorType::PexError,
                                err.to_string(),
                            )),
                        }
                    }
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::DirStatFailed,
                        err.to_string(),
                    )),
                };
                // Restore directory
                match self.change_dir(wrkdir.as_path()) {
                    Err(err) => Err(err),
                    Ok(_) => result,
                }
            }
        }
    }

    /// ### rename
    ///
    /// Rename file or a directory
    fn rename(&mut self, file: &FsEntry, dst: &Path) -> Result<(), FileTransferError> {
        let dst: PathBuf = Self::resolve(dst);
        info!(
            "Renaming {} to {}",
            file.get_abs_path().display(),
            dst.display()
        );
        match &mut self.stream {
            Some(stream) => {
                // Get name
                let src_name: String = match file {
                    FsEntry::Directory(dir) => dir.name.clone(),
                    FsEntry::File(file) => file.name.clone(),
                };
                // Only names are supported
                match stream.rename(src_name.as_str(), &dst.as_path().to_string_lossy()) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::FileCreateDenied,
                        err.to_string(),
                    )),
                }
            }
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### stat
    ///
    /// Stat file and return FsEntry
    fn stat(&mut self, _path: &Path) -> Result<FsEntry, FileTransferError> {
        match &mut self.stream {
            Some(_) => Err(FileTransferError::new(
                FileTransferErrorType::UnsupportedFeature,
            )),
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### exec
    ///
    /// Execute a command on remote host
    fn exec(&mut self, _cmd: &str) -> Result<String, FileTransferError> {
        Err(FileTransferError::new(
            FileTransferErrorType::UnsupportedFeature,
        ))
    }

    /// ### send_file
    ///
    /// Send file to remote
    /// File name is referred to the name of the file as it will be saved
    /// Data contains the file data
    /// Returns file and its size
    fn send_file(
        &mut self,
        _local: &FsFile,
        file_name: &Path,
    ) -> Result<Box<dyn Write>, FileTransferError> {
        let file_name: PathBuf = Self::resolve(file_name);
        info!("Sending file {}", file_name.display());
        match &mut self.stream {
            Some(stream) => match stream.put_with_stream(&file_name.as_path().to_string_lossy()) {
                Ok(writer) => Ok(Box::new(writer)), // NOTE: don't use BufWriter here, since already returned by the library
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::FileCreateDenied,
                    err.to_string(),
                )),
            },
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
        info!("Receiving file {}", file.abs_path.display());
        match &mut self.stream {
            Some(stream) => match stream.get(&file.abs_path.as_path().to_string_lossy()) {
                Ok(reader) => Ok(Box::new(reader)), // NOTE: don't use BufReader here, since already returned by the library
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::NoSuchFileOrDirectory,
                    err.to_string(),
                )),
            },
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
    fn on_sent(&mut self, writable: Box<dyn Write>) -> Result<(), FileTransferError> {
        info!("Finalizing put stream");
        match &mut self.stream {
            Some(stream) => match stream.finalize_put_stream(writable) {
                Ok(_) => Ok(()),
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::ProtocolError,
                    err.to_string(),
                )),
            },
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### on_recv
    ///
    /// Finalize recv method.
    /// This method must be implemented only if necessary; in case you don't need it, just return `Ok(())`
    /// The purpose of this method is to finalize the connection with the peer when reading data.
    /// This mighe be necessary for some protocols.
    /// You must call this method each time you want to finalize the read of the remote file.
    fn on_recv(&mut self, readable: Box<dyn Read>) -> Result<(), FileTransferError> {
        info!("Finalizing get");
        match &mut self.stream {
            Some(stream) => match stream.finalize_get(readable) {
                Ok(_) => Ok(()),
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::ProtocolError,
                    err.to_string(),
                )),
            },
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::file::open_file;
    use crate::utils::fmt::fmt_time;
    #[cfg(feature = "with-containers")]
    use crate::utils::test_helpers::write_file;
    use crate::utils::test_helpers::{create_sample_file_entry, make_fsentry};

    use pretty_assertions::assert_eq;
    use std::io::{Read, Write};
    use std::time::Duration;

    #[test]
    fn test_filetransfer_ftp_new() {
        let ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        assert_eq!(ftp.ftps, false);
        assert!(ftp.stream.is_none());
        // FTPS
        let ftp: FtpFileTransfer = FtpFileTransfer::new(true);
        assert_eq!(ftp.ftps, true);
        assert!(ftp.stream.is_none());
    }

    #[test]
    #[cfg(feature = "with-containers")]
    fn test_filetransfer_ftp_server() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Sample file
        let (entry, file): (FsFile, tempfile::NamedTempFile) = create_sample_file_entry();
        // Connect
        #[cfg(not(feature = "github-actions"))]
        let hostname: String = String::from("127.0.0.1");
        #[cfg(feature = "github-actions")]
        let hostname: String = String::from("127.0.0.1");
        assert!(ftp
            .connect(
                hostname,
                10021,
                Some(String::from("test")),
                Some(String::from("test")),
            )
            .is_ok());
        assert_eq!(ftp.is_connected(), true);
        // Get pwd
        assert_eq!(ftp.pwd().unwrap(), PathBuf::from("/"));
        // List dir (dir is empty)
        assert_eq!(ftp.list_dir(&Path::new("/")).unwrap().len(), 0);
        // Make directory
        assert!(ftp.mkdir(PathBuf::from("/home").as_path()).is_ok());
        // Make directory (err)
        assert!(ftp.mkdir(PathBuf::from("/root/pommlar").as_path()).is_err());
        // Change directory
        assert!(ftp.change_dir(PathBuf::from("/home").as_path()).is_ok());
        // Change directory (err)
        assert!(ftp
            .change_dir(PathBuf::from("/tmp/oooo/aaaa/eee").as_path())
            .is_err());
        // Copy (not supported)
        assert!(ftp
            .copy(&FsEntry::File(entry.clone()), PathBuf::from("/").as_path())
            .is_err());
        // Exec (not supported)
        assert!(ftp.exec("echo 1;").is_err());
        // Upload 2 files
        let mut writable = ftp
            .send_file(&entry, PathBuf::from("omar.txt").as_path())
            .ok()
            .unwrap();
        write_file(&file, &mut writable);
        assert!(ftp.on_sent(writable).is_ok());
        let mut writable = ftp
            .send_file(&entry, PathBuf::from("README.md").as_path())
            .ok()
            .unwrap();
        write_file(&file, &mut writable);
        assert!(ftp.on_sent(writable).is_ok());
        // Upload file (err)
        assert!(ftp
            .send_file(&entry, PathBuf::from("/ommlar/omarone").as_path())
            .is_err());
        // List dir
        let list: Vec<FsEntry> = ftp.list_dir(PathBuf::from("/home").as_path()).ok().unwrap();
        assert_eq!(list.len(), 2);
        // Find
        assert!(ftp.change_dir(PathBuf::from("/").as_path()).is_ok());
        assert_eq!(ftp.find("*.txt").ok().unwrap().len(), 1);
        assert_eq!(ftp.find("*.md").ok().unwrap().len(), 1);
        assert_eq!(ftp.find("*.jpeg").ok().unwrap().len(), 0);
        assert!(ftp.change_dir(PathBuf::from("/home").as_path()).is_ok());
        // Rename
        assert!(ftp.mkdir(PathBuf::from("/uploads").as_path()).is_ok());
        assert!(ftp
            .rename(
                list.get(0).unwrap(),
                PathBuf::from("/uploads/README.txt").as_path()
            )
            .is_ok());
        // Rename (err)
        assert!(ftp
            .rename(list.get(0).unwrap(), PathBuf::from("OMARONE").as_path())
            .is_err());
        let dummy: FsEntry = FsEntry::File(FsFile {
            name: String::from("cucumber.txt"),
            abs_path: PathBuf::from("/cucumber.txt"),
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
        });
        assert!(ftp
            .rename(&dummy, PathBuf::from("/a/b/c").as_path())
            .is_err());
        // Remove
        assert!(ftp.remove(list.get(1).unwrap()).is_ok());
        assert!(ftp.remove(list.get(1).unwrap()).is_err());
        // Receive file
        let mut writable = ftp
            .send_file(&entry, PathBuf::from("/uploads/README.txt").as_path())
            .ok()
            .unwrap();
        write_file(&file, &mut writable);
        assert!(ftp.on_sent(writable).is_ok());
        let file: FsFile = ftp
            .list_dir(PathBuf::from("/uploads").as_path())
            .ok()
            .unwrap()
            .get(0)
            .unwrap()
            .clone()
            .unwrap_file();
        let mut readable = ftp.recv_file(&file).ok().unwrap();
        let mut data: Vec<u8> = vec![0; 1024];
        assert!(readable.read(&mut data).is_ok());
        assert!(ftp.on_recv(readable).is_ok());
        // Receive file (err)
        assert!(ftp.recv_file(&entry).is_err());
        // Cleanup
        assert!(ftp.change_dir(PathBuf::from("/").as_path()).is_ok());
        assert!(ftp
            .remove(&make_fsentry(PathBuf::from("/home"), true))
            .is_ok());
        assert!(ftp
            .remove(&make_fsentry(PathBuf::from("/uploads"), true))
            .is_ok());
        // Disconnect
        assert!(ftp.disconnect().is_ok());
        assert_eq!(ftp.is_connected(), false);
    }

    #[test]
    #[cfg(feature = "with-containers")]
    fn test_filetransfer_ftp_server_bad_auth() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Connect
        assert!(ftp
            .connect(
                String::from("127.0.0.1"),
                10021,
                Some(String::from("omar")),
                Some(String::from("ommlar")),
            )
            .is_err());
    }

    #[test]
    #[cfg(feature = "with-containers")]
    fn test_filetransfer_ftp_no_credentials() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        assert!(ftp
            .connect(String::from("127.0.0.1"), 10021, None, None)
            .is_err());
    }

    #[test]
    fn test_filetransfer_ftp_server_bad_server() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Connect
        assert!(ftp
            .connect(
                String::from("mybadserver.veryverybad.awful"),
                21,
                Some(String::from("omar")),
                Some(String::from("ommlar")),
            )
            .is_err());
    }

    #[test]
    fn test_filetransfer_ftp_parse_list_line_unix() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Simple file
        let file: FsFile = ftp
            .parse_list_line(
                PathBuf::from("/tmp").as_path(),
                "-rw-rw-r-- 1 root  dialout  8192 Nov 5 2018 omar.txt",
            )
            .ok()
            .unwrap()
            .unwrap_file();
        assert_eq!(file.abs_path, PathBuf::from("/tmp/omar.txt"));
        assert_eq!(file.name, String::from("omar.txt"));
        assert_eq!(file.size, 8192);
        assert!(file.symlink.is_none());
        assert_eq!(file.user, None);
        assert_eq!(file.group, None);
        assert_eq!(file.unix_pex.unwrap(), (6, 6, 4));
        assert_eq!(
            file.last_access_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1541376000)
        );
        assert_eq!(
            file.last_change_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1541376000)
        );
        assert_eq!(
            file.creation_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1541376000)
        );
        // Simple file with number as gid, uid
        let file: FsFile = ftp
            .parse_list_line(
                PathBuf::from("/tmp").as_path(),
                "-rwxr-xr-x 1 0  9  4096 Nov 5 16:32 omar.txt",
            )
            .ok()
            .unwrap()
            .unwrap_file();
        assert_eq!(file.abs_path, PathBuf::from("/tmp/omar.txt"));
        assert_eq!(file.name, String::from("omar.txt"));
        assert_eq!(file.size, 4096);
        assert!(file.symlink.is_none());
        assert_eq!(file.user, Some(0));
        assert_eq!(file.group, Some(9));
        assert_eq!(file.unix_pex.unwrap(), (7, 5, 5));
        assert_eq!(
            fmt_time(file.last_access_time, "%m %d %M").as_str(),
            "11 05 32"
        );
        assert_eq!(
            fmt_time(file.last_change_time, "%m %d %M").as_str(),
            "11 05 32"
        );
        assert_eq!(
            fmt_time(file.creation_time, "%m %d %M").as_str(),
            "11 05 32"
        );
        // Directory
        let dir: FsDirectory = ftp
            .parse_list_line(
                PathBuf::from("/tmp").as_path(),
                "drwxrwxr-x 1 0  9  4096 Nov 5 2018 docs",
            )
            .ok()
            .unwrap()
            .unwrap_dir();
        assert_eq!(dir.abs_path, PathBuf::from("/tmp/docs"));
        assert_eq!(dir.name, String::from("docs"));
        assert!(dir.symlink.is_none());
        assert_eq!(dir.user, Some(0));
        assert_eq!(dir.group, Some(9));
        assert_eq!(dir.unix_pex.unwrap(), (7, 7, 5));
        assert_eq!(
            dir.last_access_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1541376000)
        );
        assert_eq!(
            dir.last_change_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1541376000)
        );
        assert_eq!(
            dir.creation_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1541376000)
        );
        assert_eq!(dir.readonly, false);
        // Error
        assert!(ftp
            .parse_list_line(
                PathBuf::from("/").as_path(),
                "drwxrwxr-x 1 0  9  Nov 5 2018 docs"
            )
            .is_err());
    }

    #[test]
    fn test_filetransfer_ftp_parse_list_line_dos() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Simple file
        let file: FsFile = ftp
            .parse_list_line(
                PathBuf::from("/tmp").as_path(),
                "04-08-14  03:09PM  8192 omar.txt",
            )
            .ok()
            .unwrap()
            .unwrap_file();
        assert_eq!(file.abs_path, PathBuf::from("/tmp/omar.txt"));
        assert_eq!(file.name, String::from("omar.txt"));
        assert_eq!(file.size, 8192);
        assert!(file.symlink.is_none());
        assert_eq!(file.user, None);
        assert_eq!(file.group, None);
        assert_eq!(file.unix_pex, None);
        assert_eq!(
            file.last_access_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1407164940)
        );
        assert_eq!(
            file.last_change_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1407164940)
        );
        assert_eq!(
            file.creation_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1407164940)
        );
        // Directory
        let dir: FsDirectory = ftp
            .parse_list_line(
                PathBuf::from("/tmp").as_path(),
                "04-08-14  03:09PM  <DIR> docs",
            )
            .ok()
            .unwrap()
            .unwrap_dir();
        assert_eq!(dir.abs_path, PathBuf::from("/tmp/docs"));
        assert_eq!(dir.name, String::from("docs"));
        assert!(dir.symlink.is_none());
        assert_eq!(dir.user, None);
        assert_eq!(dir.group, None);
        assert_eq!(dir.unix_pex, None);
        assert_eq!(
            dir.last_access_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1407164940)
        );
        assert_eq!(
            dir.last_change_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1407164940)
        );
        assert_eq!(
            dir.creation_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .unwrap(),
            Duration::from_secs(1407164940)
        );
        assert_eq!(dir.readonly, false);
        // Error
        assert!(ftp
            .parse_list_line(PathBuf::from("/").as_path(), "04-08-14  omar.txt")
            .is_err());
    }

    #[test]
    fn test_filetransfer_ftp_list_dir_dos_syntax() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Connect
        assert!(ftp
            .connect(
                String::from("test.rebex.net"),
                21,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Pwd
        assert_eq!(ftp.pwd().ok().unwrap(), PathBuf::from("/"));
        // List dir
        let files: Vec<FsEntry> = ftp.list_dir(PathBuf::from("/").as_path()).ok().unwrap();
        // There should be at least 1 file
        assert!(files.len() > 0);
        // Disconnect
        assert!(ftp.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_ftp_get_name_and_link() {
        let client: FtpFileTransfer = FtpFileTransfer::new(false);
        assert_eq!(
            client.get_name_and_link("Cargo.toml"),
            (String::from("Cargo.toml"), None)
        );
        assert_eq!(
            client.get_name_and_link("Cargo -> Cargo.toml"),
            (String::from("Cargo"), Some(PathBuf::from("Cargo.toml")))
        );
    }

    #[test]
    fn test_filetransfer_ftp_uninitialized() {
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
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        assert!(ftp.change_dir(Path::new("/tmp")).is_err());
        assert!(ftp.disconnect().is_err());
        assert!(ftp.list_dir(Path::new("/tmp")).is_err());
        assert!(ftp.mkdir(Path::new("/tmp")).is_err());
        assert!(ftp
            .remove(&make_fsentry(PathBuf::from("/nowhere"), false))
            .is_err());
        assert!(ftp
            .rename(
                &make_fsentry(PathBuf::from("/nowhere"), false),
                PathBuf::from("/culonia").as_path()
            )
            .is_err());
        assert!(ftp.pwd().is_err());
        assert!(ftp.stat(Path::new("/tmp")).is_err());
        assert!(ftp.recv_file(&file).is_err());
        assert!(ftp.send_file(&file, Path::new("/tmp/omar.txt")).is_err());
        let (_, temp): (FsFile, tempfile::NamedTempFile) = create_sample_file_entry();
        let readable: Box<dyn Read> = Box::new(std::fs::File::open(temp.path()).unwrap());
        assert!(ftp.on_recv(readable).is_err());
        let (_, temp): (FsFile, tempfile::NamedTempFile) = create_sample_file_entry();
        let writable: Box<dyn Write> =
            Box::new(open_file(temp.path(), true, true, true).ok().unwrap());
        assert!(ftp.on_sent(writable).is_err());
    }
}
