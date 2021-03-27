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
extern crate regex;

use super::{FileTransfer, FileTransferError, FileTransferErrorType};
use crate::fs::{FsDirectory, FsEntry, FsFile};
use crate::utils::parser::{parse_datetime, parse_lstime};

// Includes
use ftp4::native_tls::TlsConnector;
use ftp4::FtpStream;
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

    /// ### parse_list_line
    ///
    /// Parse a line of LIST command output and instantiates an FsEntry from it
    fn parse_list_line(&self, path: &Path, line: &str) -> Result<FsEntry, ()> {
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
    fn parse_unix_list_line(&self, path: &Path, line: &str) -> Result<FsEntry, ()> {
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
                let (is_dir, _is_symlink): (bool, bool) = match metadata.get(1).unwrap().as_str() {
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
                let file_name: String = String::from(metadata.get(8).unwrap().as_str());
                // Check if file_name is '.' or '..'
                if file_name.as_str() == "." || file_name.as_str() == ".." {
                    return Err(());
                }
                let mut abs_path: PathBuf = PathBuf::from(path);
                abs_path.push(file_name.as_str());
                // get extension
                let extension: Option<String> = abs_path
                    .as_path()
                    .extension()
                    .map(|s| String::from(s.to_string_lossy()));
                // Return
                // Push to entries
                Ok(match is_dir {
                    true => FsEntry::Directory(FsDirectory {
                        name: file_name,
                        abs_path,
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
                        abs_path,
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
                // Get extension
                let extension: Option<String> = abs_path
                    .as_path()
                    .extension()
                    .map(|s| String::from(s.to_string_lossy()));
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
        let mut stream: FtpStream = match FtpStream::connect(format!("{}:{}", address, port)) {
            Ok(stream) => stream,
            Err(err) => {
                return Err(FileTransferError::new_ex(
                    FileTransferErrorType::ConnectionError,
                    format!("{}", err),
                ))
            }
        };
        // If SSL, open secure session
        if self.ftps {
            let ctx = match TlsConnector::builder()
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true)
                .build()
            {
                Ok(tls) => tls,
                Err(err) => {
                    return Err(FileTransferError::new_ex(
                        FileTransferErrorType::SslError,
                        format!("{}", err),
                    ))
                }
            };
            stream = match stream.into_secure(ctx, address.as_str()) {
                Ok(s) => s,
                Err(err) => {
                    return Err(FileTransferError::new_ex(
                        FileTransferErrorType::SslError,
                        format!("{}", err),
                    ))
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
        if let Err(err) = stream.login(username.as_str(), password.as_str()) {
            return Err(FileTransferError::new_ex(
                FileTransferErrorType::AuthenticationFailed,
                format!("{}", err),
            ));
        }
        // Set stream
        self.stream = Some(stream);
        // Return OK
        Ok(self.stream.as_ref().unwrap().get_welcome_msg())
    }

    /// ### disconnect
    ///
    /// Disconnect from the remote server

    fn disconnect(&mut self) -> Result<(), FileTransferError> {
        match &mut self.stream {
            Some(stream) => match stream.quit() {
                Ok(_) => Ok(()),
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::ConnectionError,
                    format!("{}", err),
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
        match &mut self.stream {
            Some(stream) => match stream.pwd() {
                Ok(path) => Ok(PathBuf::from(path.as_str())),
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::ConnectionError,
                    format!("{}", err),
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
        match &mut self.stream {
            Some(stream) => match stream.cwd(&dir.to_string_lossy()) {
                Ok(_) => Ok(PathBuf::from(dir)),
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::ConnectionError,
                    format!("{}", err),
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
        Err(FileTransferError::new(
            FileTransferErrorType::UnsupportedFeature,
        ))
    }

    /// ### list_dir
    ///
    /// List directory entries

    fn list_dir(&mut self, path: &Path) -> Result<Vec<FsEntry>, FileTransferError> {
        match &mut self.stream {
            Some(stream) => match stream.list(Some(&path.to_string_lossy())) {
                Ok(entries) => {
                    // Prepare result
                    let mut result: Vec<FsEntry> = Vec::with_capacity(entries.len());
                    // Iterate over entries
                    for entry in entries.iter() {
                        if let Ok(file) = self.parse_list_line(path, entry) {
                            result.push(file);
                        }
                    }
                    Ok(result)
                }
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::DirStatFailed,
                    format!("{}", err),
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
        match &mut self.stream {
            Some(stream) => match stream.mkdir(&dir.to_string_lossy()) {
                Ok(_) => Ok(()),
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::FileCreateDenied,
                    format!("{}", err),
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
        match fsentry {
            // Match fs entry...
            FsEntry::File(file) => {
                // Remove file directly
                match self.stream.as_mut().unwrap().rm(file.name.as_ref()) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::PexError,
                        format!("{}", err),
                    )),
                }
            }
            FsEntry::Directory(dir) => {
                // Get directory files
                match self.list_dir(dir.abs_path.as_path()) {
                    Ok(files) => {
                        // Remove recursively files
                        for file in files.iter() {
                            if let Err(err) = self.remove(&file) {
                                return Err(FileTransferError::new_ex(
                                    FileTransferErrorType::PexError,
                                    format!("{}", err),
                                ));
                            }
                        }
                        // Once all files in directory have been deleted, remove directory
                        match self.stream.as_mut().unwrap().rmdir(dir.name.as_str()) {
                            Ok(_) => Ok(()),
                            Err(err) => Err(FileTransferError::new_ex(
                                FileTransferErrorType::PexError,
                                format!("{}", err),
                            )),
                        }
                    }
                    Err(err) => Err(FileTransferError::new_ex(
                        FileTransferErrorType::DirStatFailed,
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
        match &mut self.stream {
            Some(stream) => {
                // Get name
                let src_name: String = match file {
                    FsEntry::Directory(dir) => dir.name.clone(),
                    FsEntry::File(file) => file.name.clone(),
                };
                let dst_name: PathBuf = match dst.file_name() {
                    Some(p) => PathBuf::from(p),
                    None => {
                        return Err(FileTransferError::new_ex(
                            FileTransferErrorType::FileCreateDenied,
                            String::from("Invalid destination name"),
                        ))
                    }
                };
                // Only names are supported
                match stream.rename(src_name.as_str(), &dst_name.as_path().to_string_lossy()) {
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
        match &mut self.stream {
            Some(stream) => match stream.put_with_stream(&file_name.to_string_lossy()) {
                Ok(writer) => Ok(Box::new(writer)), // NOTE: don't use BufWriter here, since already returned by the library
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::FileCreateDenied,
                    format!("{}", err),
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
        match &mut self.stream {
            Some(stream) => match stream.get(&file.abs_path.as_path().to_string_lossy()) {
                Ok(reader) => Ok(Box::new(reader)), // NOTE: don't use BufReader here, since already returned by the library
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::NoSuchFileOrDirectory,
                    format!("{}", err),
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
        match &mut self.stream {
            Some(stream) => match stream.finalize_put_stream(writable) {
                Ok(_) => Ok(()),
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::ProtocolError,
                    format!("{}", err),
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
        match &mut self.stream {
            Some(stream) => match stream.finalize_get(readable) {
                Ok(_) => Ok(()),
                Err(err) => Err(FileTransferError::new_ex(
                    FileTransferErrorType::ProtocolError,
                    format!("{}", err),
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
    use crate::utils::fmt::fmt_time;
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
    fn test_filetransfer_ftp_parse_list_line_unix() {
        let ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Simple file
        let fs_entry: FsEntry = ftp
            .parse_list_line(
                PathBuf::from("/tmp").as_path(),
                "-rw-rw-r-- 1 root  dialout  8192 Nov 5 2018 omar.txt",
            )
            .ok()
            .unwrap();
        if let FsEntry::File(file) = fs_entry {
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
        } else {
            panic!("Expected file, got directory");
        }
        // Simple file with number as gid, uid
        let fs_entry: FsEntry = ftp
            .parse_list_line(
                PathBuf::from("/tmp").as_path(),
                "-rwxr-xr-x 1 0  9  4096 Nov 5 16:32 omar.txt",
            )
            .ok()
            .unwrap();
        if let FsEntry::File(file) = fs_entry {
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
        } else {
            panic!("Expected file, got directory");
        }
        // Directory
        let fs_entry: FsEntry = ftp
            .parse_list_line(
                PathBuf::from("/tmp").as_path(),
                "drwxrwxr-x 1 0  9  4096 Nov 5 2018 docs",
            )
            .ok()
            .unwrap();
        if let FsEntry::Directory(dir) = fs_entry {
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
        } else {
            panic!("Expected directory, got directory");
        }
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
        let ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Simple file
        let fs_entry: FsEntry = ftp
            .parse_list_line(
                PathBuf::from("/tmp").as_path(),
                "04-08-14  03:09PM  8192 omar.txt",
            )
            .ok()
            .unwrap();
        if let FsEntry::File(file) = fs_entry {
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
        } else {
            panic!("Expected file, got directory");
        }
        // Directory
        let fs_entry: FsEntry = ftp
            .parse_list_line(
                PathBuf::from("/tmp").as_path(),
                "04-08-14  03:09PM  <DIR> docs",
            )
            .ok()
            .unwrap();
        if let FsEntry::Directory(dir) = fs_entry {
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
        } else {
            panic!("Expected directory, got directory");
        }
        // Error
        assert!(ftp
            .parse_list_line(PathBuf::from("/").as_path(), "04-08-14  omar.txt")
            .is_err());
    }

    #[test]
    fn test_filetransfer_ftp_connect_unsecure_anonymous() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Connect
        assert!(ftp
            .connect(String::from("speedtest.tele2.net"), 21, None, None)
            .is_ok());
        // Pwd
        assert_eq!(ftp.pwd().ok().unwrap(), PathBuf::from("/"));
        // Disconnect
        assert!(ftp.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_ftp_connect_unsecure_username() {
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
        // Disconnect
        assert!(ftp.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_ftp_connect_secure() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(true);
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
        // Disconnect
        assert!(ftp.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_ftp_change_dir() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Connect
        assert!(ftp
            .connect(String::from("speedtest.tele2.net"), 21, None, None)
            .is_ok());
        // Pwd
        assert_eq!(ftp.pwd().ok().unwrap(), PathBuf::from("/"));
        // Cwd
        assert!(ftp.change_dir(PathBuf::from("upload/").as_path()).is_ok());
        // Pwd
        assert_eq!(ftp.pwd().ok().unwrap(), PathBuf::from("/upload"));
        // Disconnect
        assert!(ftp.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_ftp_copy() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Connect
        assert!(ftp
            .connect(String::from("speedtest.tele2.net"), 21, None, None)
            .is_ok());
        // Pwd
        assert_eq!(ftp.pwd().ok().unwrap(), PathBuf::from("/"));
        // Copy
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
        assert!(ftp
            .copy(&FsEntry::File(file), &Path::new("/tmp/dest.txt"))
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
        println!("{:?}", ftp.list_dir(PathBuf::from("/").as_path()));
        let files: Vec<FsEntry> = ftp.list_dir(PathBuf::from("/").as_path()).ok().unwrap();
        // There should be at least 1 file
        assert!(files.len() > 0);
        // Disconnect
        assert!(ftp.disconnect().is_ok());
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn test_filetransfer_ftp_list_dir_unix_syntax() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Connect
        assert!(ftp
            .connect(String::from("speedtest.tele2.net"), 21, None, None)
            .is_ok());
        // Pwd
        assert_eq!(ftp.pwd().ok().unwrap(), PathBuf::from("/"));
        // List dir
        println!("{:?}", ftp.list_dir(PathBuf::from("/").as_path()));
        let files: Vec<FsEntry> = ftp.list_dir(PathBuf::from("/").as_path()).ok().unwrap();
        // There should be at least 1 file
        assert!(files.len() > 0);
        // Disconnect
        assert!(ftp.disconnect().is_ok());
    }

    /* NOTE: they don't work
    #[test]
    fn test_filetransfer_ftp_recv() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Connect
        assert!(ftp.connect(String::from("test.rebex.net"), 21, Some(String::from("demo")), Some(String::from("password"))).is_ok());
        // Pwd
        assert_eq!(ftp.pwd().ok().unwrap(), PathBuf::from("/"));
        // Recv 100KB
        assert!(ftp.recv_file(PathBuf::from("readme.txt").as_path()).is_ok());
        // Disconnect
        assert!(ftp.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_ftp_send() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Connect
        assert!(ftp.connect(String::from("speedtest.tele2.net"), 21, None, None).is_ok());
        // Pwd
        assert_eq!(ftp.pwd().ok().unwrap(), PathBuf::from("/"));
        // Cwd
        assert!(ftp.change_dir(PathBuf::from("upload/").as_path()).is_ok());
        // Pwd
        assert_eq!(ftp.pwd().ok().unwrap(), PathBuf::from("/upload"));
        // Send a sample file 100KB
        assert!(ftp.send_file(PathBuf::from("test.txt").as_path()).is_ok());
        // Disconnect
        assert!(ftp.disconnect().is_ok());
    }*/

    #[test]
    fn test_filetransfer_ftp_exec() {
        let mut ftp: FtpFileTransfer = FtpFileTransfer::new(false);
        // Connect
        assert!(ftp
            .connect(String::from("speedtest.tele2.net"), 21, None, None)
            .is_ok());
        // Pwd
        assert!(ftp.exec("echo 1;").is_err());
        // Disconnect
        assert!(ftp.disconnect().is_ok());
    }

    #[test]
    fn test_filetransfer_ftp_find() {
        let mut client: FtpFileTransfer = FtpFileTransfer::new(false);
        // Connect
        assert!(client
            .connect(
                String::from("test.rebex.net"),
                21,
                Some(String::from("demo")),
                Some(String::from("password"))
            )
            .is_ok());
        // Pwd
        assert_eq!(client.pwd().ok().unwrap(), PathBuf::from("/"));
        // Search for file (let's search for pop3-*.png); there should be 2
        let search_res: Vec<FsEntry> = client.find("pop3-*.png").ok().unwrap();
        assert_eq!(search_res.len(), 2);
        // verify names
        assert_eq!(search_res[0].get_name(), "pop3-browser.png");
        assert_eq!(search_res[1].get_name(), "pop3-console-client.png");
        // Search directory
        let search_res: Vec<FsEntry> = client.find("pub").ok().unwrap();
        assert_eq!(search_res.len(), 1);
        // Disconnect
        assert!(client.disconnect().is_ok());
        // Verify err
        assert!(client.find("pippo").is_err());
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
        assert!(ftp.pwd().is_err());
        assert!(ftp.stat(Path::new("/tmp")).is_err());
        assert!(ftp.recv_file(&file).is_err());
        assert!(ftp.send_file(&file, Path::new("/tmp/omar.txt")).is_err());
    }
}
