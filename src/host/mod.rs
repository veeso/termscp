//! ## Host
//!
//! `host` is the module which provides functionalities to host file system

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

use std::fs::{self, File, Metadata, OpenOptions};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
// Metadata ext
#[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
use std::os::unix::fs::MetadataExt;

// Locals
use crate::fs::{FsDirectory, FsEntry, FsFile};

/// ## HostErrorType
///
/// HostErrorType provides an overview of the specific host error
#[derive(PartialEq, std::fmt::Debug)]
pub enum HostErrorType {
    NoSuchFileOrDirectory,
    ReadonlyFile,
    DirNotAccessible,
    FileNotAccessible,
    FileAlreadyExists,
    CouldNotCreateFile,
    DeleteFailed,
}

/// ### HostError
///
/// HostError is a wrapper for the error type and the exact io error

pub struct HostError {
    pub error: HostErrorType,
    pub ioerr: Option<std::io::Error>,
}

impl HostError {
    /// ### new
    ///
    /// Instantiates a new HostError
    pub(crate) fn new(error: HostErrorType, errno: Option<std::io::Error>) -> HostError {
        HostError {
            error,
            ioerr: errno,
        }
    }
}

impl std::fmt::Display for HostError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let code_str: &str = match self.error {
            HostErrorType::NoSuchFileOrDirectory => "No such file or directory",
            HostErrorType::ReadonlyFile => "File is readonly",
            HostErrorType::DirNotAccessible => "Could not access directory",
            HostErrorType::FileNotAccessible => "Could not access directory",
            HostErrorType::FileAlreadyExists => "File already exists",
            HostErrorType::CouldNotCreateFile => "Could not create file",
            HostErrorType::DeleteFailed => "Could not delete file",
        };
        match &self.ioerr {
            Some(err) => write!(f, "{}: {}", code_str, err),
            None => write!(f, "{}", code_str),
        }
    }
}

/// ## Localhost
///
/// Localhost is the entity which holds the information about the current directory and host.
/// It provides functions to navigate across the local host file system
pub struct Localhost {
    wrkdir: PathBuf,
    files: Vec<FsEntry>,
}

impl Localhost {
    /// ### new
    ///
    /// Instantiates a new Localhost struct
    pub fn new(wrkdir: PathBuf) -> Result<Localhost, HostError> {
        let mut host: Localhost = Localhost {
            wrkdir,
            files: Vec::new(),
        };
        // Check if dir exists
        if !host.file_exists(host.wrkdir.as_path()) {
            return Err(HostError::new(HostErrorType::NoSuchFileOrDirectory, None));
        }
        // Retrieve files for provided path
        host.files = match host.scan_dir(host.wrkdir.as_path()) {
            Ok(files) => files,
            Err(err) => return Err(err),
        };
        Ok(host)
    }

    /// ### pwd
    ///
    /// Print working directory
    pub fn pwd(&self) -> PathBuf {
        self.wrkdir.clone()
    }

    /// ### list_dir
    ///
    /// List files in current directory
    #[allow(dead_code)]
    pub fn list_dir(&self) -> Vec<FsEntry> {
        self.files.clone()
    }

    /// ### change_wrkdir
    ///
    /// Change working directory with the new provided directory
    pub fn change_wrkdir(&mut self, new_dir: PathBuf) -> Result<PathBuf, HostError> {
        // Check whether directory exists
        if !self.file_exists(new_dir.as_path()) {
            return Err(HostError::new(HostErrorType::NoSuchFileOrDirectory, None));
        }
        let prev_dir: PathBuf = self.wrkdir.clone(); // Backup location
                                                     // Update working directory
        self.wrkdir = new_dir;
        // Scan new directory
        self.files = match self.scan_dir(self.wrkdir.as_path()) {
            Ok(files) => files,
            Err(err) => {
                // Restore directory
                self.wrkdir = prev_dir;
                return Err(err);
            }
        };
        Ok(self.wrkdir.clone())
    }

    /// ### mkdir
    ///
    /// Make a directory at path and update the file list (only if relative)
    pub fn mkdir(&mut self, dir_name: &Path) -> Result<(), HostError> {
        self.mkdir_ex(dir_name, false)
    }

    /// ### mkdir_ex
    ///
    /// Extended option version of makedir.
    /// ignex: don't report error if directory already exists
    pub fn mkdir_ex(&mut self, dir_name: &Path, ignex: bool) -> Result<(), HostError> {
        let dir_path: PathBuf = match dir_name.is_absolute() {
            true => PathBuf::from(dir_name),
            false => {
                let mut dir_path: PathBuf = self.wrkdir.clone();
                dir_path.push(dir_name);
                dir_path
            }
        };
        // If dir already exists, return Error
        if dir_path.exists() {
            match ignex {
                true => return Ok(()),
                false => return Err(HostError::new(HostErrorType::FileAlreadyExists, None)),
            }
        }
        match std::fs::create_dir(dir_path) {
            Ok(_) => {
                // Update dir
                if dir_name.is_relative() {
                    self.files = match self.scan_dir(self.wrkdir.as_path()) {
                        Ok(f) => f,
                        Err(err) => return Err(err),
                    };
                }
                Ok(())
            }
            Err(err) => Err(HostError::new(HostErrorType::CouldNotCreateFile, Some(err))),
        }
    }

    /// ### remove
    ///
    /// Remove file entry
    pub fn remove(&mut self, entry: &FsEntry) -> Result<(), HostError> {
        match entry {
            FsEntry::Directory(dir) => {
                // If file doesn't exist; return error
                if !dir.abs_path.as_path().exists() {
                    return Err(HostError::new(HostErrorType::NoSuchFileOrDirectory, None));
                }
                // Remove
                match std::fs::remove_dir_all(dir.abs_path.as_path()) {
                    Ok(_) => {
                        // Update dir
                        self.files = match self.scan_dir(self.wrkdir.as_path()) {
                            Ok(f) => f,
                            Err(err) => return Err(err),
                        };
                        Ok(())
                    }
                    Err(err) => Err(HostError::new(HostErrorType::DeleteFailed, Some(err))),
                }
            }
            FsEntry::File(file) => {
                // If file doesn't exist; return error
                if !file.abs_path.as_path().exists() {
                    return Err(HostError::new(HostErrorType::NoSuchFileOrDirectory, None));
                }
                // Remove
                match std::fs::remove_file(file.abs_path.as_path()) {
                    Ok(_) => {
                        // Update dir
                        self.files = match self.scan_dir(self.wrkdir.as_path()) {
                            Ok(f) => f,
                            Err(err) => return Err(err),
                        };
                        Ok(())
                    }
                    Err(err) => Err(HostError::new(HostErrorType::DeleteFailed, Some(err))),
                }
            }
        }
    }

    /// ### rename
    ///
    /// Rename file or directory to new name
    pub fn rename(&mut self, entry: &FsEntry, dst_path: &Path) -> Result<(), HostError> {
        let abs_path: PathBuf = match entry {
            FsEntry::Directory(dir) => dir.abs_path.clone(),
            FsEntry::File(f) => f.abs_path.clone(),
        };
        match std::fs::rename(abs_path.as_path(), dst_path) {
            Ok(_) => {
                // Scan dir
                self.files = match self.scan_dir(self.wrkdir.as_path()) {
                    Ok(f) => f,
                    Err(err) => return Err(err),
                };
                Ok(())
            }
            Err(err) => Err(HostError::new(HostErrorType::CouldNotCreateFile, Some(err))),
        }
    }

    /// ### stat
    ///
    /// Stat file and create a FsEntry
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    pub fn stat(&self, path: &Path) -> Result<FsEntry, HostError> {
        let attr: Metadata = match fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(err) => return Err(HostError::new(HostErrorType::FileNotAccessible, Some(err))),
        };
        let file_name: String = String::from(path.file_name().unwrap().to_str().unwrap_or(""));
        // Match dir / file
        Ok(match path.is_dir() {
            true => FsEntry::Directory(FsDirectory {
                name: file_name,
                abs_path: PathBuf::from(path),
                last_change_time: attr.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                last_access_time: attr.accessed().unwrap_or(SystemTime::UNIX_EPOCH),
                creation_time: attr.created().unwrap_or(SystemTime::UNIX_EPOCH),
                readonly: attr.permissions().readonly(),
                symlink: match fs::read_link(path) {
                    Ok(p) => Some(p),
                    Err(_) => None,
                },
                user: Some(attr.uid()),
                group: Some(attr.gid()),
                unix_pex: Some(self.u32_to_mode(attr.mode())),
            }),
            false => {
                // Is File
                let extension: Option<String> = match path.extension() {
                    Some(s) => Some(String::from(s.to_str().unwrap_or(""))),
                    None => None,
                };
                FsEntry::File(FsFile {
                    name: file_name,
                    abs_path: PathBuf::from(path),
                    last_change_time: attr.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                    last_access_time: attr.accessed().unwrap_or(SystemTime::UNIX_EPOCH),
                    creation_time: attr.created().unwrap_or(SystemTime::UNIX_EPOCH),
                    readonly: attr.permissions().readonly(),
                    size: attr.len() as usize,
                    ftype: extension,
                    symlink: match fs::read_link(path) {
                        Ok(p) => Some(p),
                        Err(_) => None,
                    },
                    user: Some(attr.uid()),
                    group: Some(attr.gid()),
                    unix_pex: Some(self.u32_to_mode(attr.mode())),
                })
            }
        })
    }

    /// ### stat
    ///
    /// Stat file and create a FsEntry
    #[cfg(target_os = "windows")]
    #[cfg(not(tarpaulin_include))]
    pub fn stat(&self, path: &Path) -> Result<FsEntry, HostError> {
        let attr: Metadata = match fs::metadata(path.clone()) {
            Ok(metadata) => metadata,
            Err(err) => return Err(HostError::new(HostErrorType::FileNotAccessible, Some(err))),
        };
        let file_name: String = String::from(path.file_name().unwrap().to_str().unwrap_or(""));
        // Match dir / file
        Ok(match path.is_dir() {
            true => FsEntry::Directory(FsDirectory {
                name: file_name,
                abs_path: PathBuf::from(path),
                last_change_time: attr.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                last_access_time: attr.accessed().unwrap_or(SystemTime::UNIX_EPOCH),
                creation_time: attr.created().unwrap_or(SystemTime::UNIX_EPOCH),
                readonly: attr.permissions().readonly(),
                symlink: match fs::read_link(path) {
                    Ok(p) => Some(p),
                    Err(_) => None,
                },
                user: None,
                group: None,
                unix_pex: None,
            }),
            false => {
                // Is File
                let extension: Option<String> = match path.extension() {
                    Some(s) => Some(String::from(s.to_str().unwrap_or(""))),
                    None => None,
                };
                FsEntry::File(FsFile {
                    name: file_name,
                    abs_path: PathBuf::from(path),
                    last_change_time: attr.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                    last_access_time: attr.accessed().unwrap_or(SystemTime::UNIX_EPOCH),
                    creation_time: attr.created().unwrap_or(SystemTime::UNIX_EPOCH),
                    readonly: attr.permissions().readonly(),
                    size: attr.len() as usize,
                    ftype: extension,
                    symlink: match fs::read_link(path) {
                        Ok(p) => Some(p),
                        Err(_) => None,
                    },
                    user: None,
                    group: None,
                    unix_pex: None,
                })
            }
        })
    }

    /// ### open_file_read
    ///
    /// Open file for read
    pub fn open_file_read(&self, file: &Path) -> Result<File, HostError> {
        if !self.file_exists(file) {
            return Err(HostError::new(HostErrorType::NoSuchFileOrDirectory, None));
        }
        match OpenOptions::new()
            .create(false)
            .read(true)
            .write(false)
            .open(file)
        {
            Ok(f) => Ok(f),
            Err(err) => Err(HostError::new(HostErrorType::FileNotAccessible, Some(err))),
        }
    }

    /// ### open_file_write
    ///
    /// Open file for write
    pub fn open_file_write(&self, file: &Path) -> Result<File, HostError> {
        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file)
        {
            Ok(f) => Ok(f),
            Err(err) => match self.file_exists(file) {
                true => Err(HostError::new(HostErrorType::ReadonlyFile, Some(err))),
                false => Err(HostError::new(HostErrorType::FileNotAccessible, Some(err))),
            },
        }
    }

    /// ### file_exists
    ///
    /// Returns whether provided file path exists
    fn file_exists(&self, path: &Path) -> bool {
        path.exists()
    }

    /// ### scan_dir
    ///
    /// Get content of the current directory as a list of fs entry (Windows)
    pub fn scan_dir(&self, dir: &Path) -> Result<Vec<FsEntry>, HostError> {
        let entries = match std::fs::read_dir(dir) {
            Ok(e) => e,
            Err(err) => return Err(HostError::new(HostErrorType::DirNotAccessible, Some(err))),
        };
        let mut fs_entries: Vec<FsEntry> = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                fs_entries.push(match self.stat(entry.path().as_path()) {
                    Ok(entry) => entry,
                    Err(err) => return Err(err),
                });
            }
        }
        Ok(fs_entries)
    }

    /// ### u32_to_mode
    ///
    /// Return string with format xxxxxx to tuple of permissions (user, group, others)
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn u32_to_mode(&self, mode: u32) -> (u8, u8, u8) {
        let user: u8 = ((mode >> 6) & 0x7) as u8;
        let group: u8 = ((mode >> 3) & 0x7) as u8;
        let others: u8 = (mode & 0x7) as u8;
        (user, group, others)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    use std::fs::File;
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    use std::io::Write;

    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    use std::os::unix::fs::{symlink, PermissionsExt};

    #[test]
    fn test_host_error_new() {
        let error: HostError = HostError::new(HostErrorType::CouldNotCreateFile, None);
        assert_eq!(error.error, HostErrorType::CouldNotCreateFile);
        assert!(error.ioerr.is_none());
    }

    #[test]
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn test_host_localhost_new() {
        let host: Localhost = Localhost::new(PathBuf::from("/bin")).ok().unwrap();
        assert_eq!(host.wrkdir, PathBuf::from("/bin"));
        // Scan dir
        let entries = std::fs::read_dir(PathBuf::from("/bin").as_path()).unwrap();
        let mut counter: usize = 0;
        for _ in entries {
            counter = counter + 1;
        }
        assert_eq!(host.files.len(), counter);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_host_localhost_new() {
        let host: Localhost = Localhost::new(PathBuf::from("C:\\users")).ok().unwrap();
        assert_eq!(host.wrkdir, PathBuf::from("C:\\users"));
        // Scan dir
        let entries = std::fs::read_dir(PathBuf::from("C:\\users").as_path()).unwrap();
        let mut counter: usize = 0;
        for _ in entries {
            counter = counter + 1;
        }
        assert_eq!(host.files.len(), counter);
    }

    #[test]
    #[should_panic]
    fn test_host_localhost_new_bad() {
        Localhost::new(PathBuf::from("/omargabber/123/345"))
            .ok()
            .unwrap();
    }

    #[test]
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn test_host_localhost_pwd() {
        let host: Localhost = Localhost::new(PathBuf::from("/bin")).ok().unwrap();
        assert_eq!(host.pwd(), PathBuf::from("/bin"));
    }

    #[test]
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn test_host_localhost_list_files() {
        let host: Localhost = Localhost::new(PathBuf::from("/bin")).ok().unwrap();
        // Scan dir
        let entries = std::fs::read_dir(PathBuf::from("/bin").as_path()).unwrap();
        let mut counter: usize = 0;
        for _ in entries {
            counter = counter + 1;
        }
        assert_eq!(host.list_dir().len(), counter);
    }

    #[test]
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn test_host_localhost_change_dir() {
        let mut host: Localhost = Localhost::new(PathBuf::from("/dev")).ok().unwrap();
        let new_dir: PathBuf = PathBuf::from("/dev");
        assert!(host.change_wrkdir(new_dir.clone()).is_ok());
        // Verify new files
        // Scan dir
        let entries = std::fs::read_dir(PathBuf::from(new_dir).as_path()).unwrap();
        let mut counter: usize = 0;
        for _ in entries {
            counter = counter + 1;
        }
        assert_eq!(host.files.len(), counter);
    }

    #[test]
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    #[should_panic]
    fn test_host_localhost_change_dir_failed() {
        let mut host: Localhost = Localhost::new(PathBuf::from("/bin")).ok().unwrap();
        let new_dir: PathBuf = PathBuf::from("/omar/gabber/123/456");
        assert!(host.change_wrkdir(new_dir.clone()).is_ok());
    }

    #[test]
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn test_host_localhost_open_read() {
        let host: Localhost = Localhost::new(PathBuf::from("/bin")).ok().unwrap();
        // Create temp file
        let file: tempfile::NamedTempFile = create_sample_file();
        assert!(host.open_file_read(file.path()).is_ok());
    }

    #[test]
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    #[should_panic]
    fn test_host_localhost_open_read_err_no_such_file() {
        let host: Localhost = Localhost::new(PathBuf::from("/bin")).ok().unwrap();
        assert!(host
            .open_file_read(PathBuf::from("/bin/foo-bar-test-omar-123-456-789.txt").as_path())
            .is_ok());
    }

    #[test]
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn test_host_localhost_open_read_err_not_accessible() {
        let host: Localhost = Localhost::new(PathBuf::from("/bin")).ok().unwrap();
        let file: tempfile::NamedTempFile = create_sample_file();
        //let mut perms = fs::metadata(file.path())?.permissions();
        fs::set_permissions(file.path(), PermissionsExt::from_mode(0o222)).unwrap();
        //fs::set_permissions(file.path(), perms)?;
        assert!(host.open_file_read(file.path()).is_err());
    }

    #[test]
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn test_host_localhost_open_write() {
        let host: Localhost = Localhost::new(PathBuf::from("/bin")).ok().unwrap();
        // Create temp file
        let file: tempfile::NamedTempFile = create_sample_file();
        assert!(host.open_file_write(file.path()).is_ok());
    }

    #[test]
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn test_host_localhost_open_write_err() {
        let host: Localhost = Localhost::new(PathBuf::from("/bin")).ok().unwrap();
        let file: tempfile::NamedTempFile = create_sample_file();
        //let mut perms = fs::metadata(file.path())?.permissions();
        fs::set_permissions(file.path(), PermissionsExt::from_mode(0o444)).unwrap();
        //fs::set_permissions(file.path(), perms)?;
        assert!(host.open_file_write(file.path()).is_err());
    }
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    #[test]
    fn test_host_localhost_symlinks() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        // Create sample file
        assert!(File::create(format!("{}/foo.txt", tmpdir.path().display()).as_str()).is_ok());
        // Create symlink
        assert!(symlink(
            format!("{}/foo.txt", tmpdir.path().display()),
            format!("{}/bar.txt", tmpdir.path().display())
        )
        .is_ok());
        // Get dir
        let host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        let files: Vec<FsEntry> = host.list_dir();
        println!("Entries {:?}", files);
        // Verify files
        let file_0: &FsEntry = files.get(0).unwrap();
        match file_0 {
            FsEntry::File(file_0) => {
                if file_0.name == String::from("foo.txt") {
                    assert!(file_0.symlink.is_none());
                } else {
                    assert_eq!(
                        *file_0.symlink.as_ref().unwrap(),
                        PathBuf::from(format!("{}/foo.txt", tmpdir.path().display()))
                    );
                }
            }
            _ => panic!("expected entry 0 to be file: {:?}", file_0),
        };
        // Verify simlink
        let file_1: &FsEntry = files.get(1).unwrap();
        match file_1 {
            FsEntry::File(file_1) => {
                if file_1.name == String::from("bar.txt") {
                    assert_eq!(
                        *file_1.symlink.as_ref().unwrap(),
                        PathBuf::from(format!("{}/foo.txt", tmpdir.path().display()))
                    );
                } else {
                    assert!(file_1.symlink.is_none());
                }
            }
            _ => panic!("expected entry 0 to be file: {:?}", file_1),
        };
    }

    #[test]
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn test_host_localhost_mkdir() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        let mut host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        let files: Vec<FsEntry> = host.list_dir();
        assert_eq!(files.len(), 0); // There should be 0 files now
        assert!(host.mkdir(PathBuf::from("test_dir").as_path()).is_ok());
        let files: Vec<FsEntry> = host.list_dir();
        assert_eq!(files.len(), 1); // There should be 1 file now
                                    // Try to re-create directory
        assert!(host.mkdir(PathBuf::from("test_dir").as_path()).is_err());
        // Try abs path
        assert!(host
            .mkdir_ex(PathBuf::from("/tmp/test_dir_123456789").as_path(), true)
            .is_ok());
    }

    #[test]
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn test_host_localhost_remove() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        // Create sample file
        assert!(File::create(format!("{}/foo.txt", tmpdir.path().display()).as_str()).is_ok());
        let mut host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        let files: Vec<FsEntry> = host.list_dir();
        assert_eq!(files.len(), 1); // There should be 1 file now
                                    // Remove file
        assert!(host.remove(files.get(0).unwrap()).is_ok());
        // There should be 0 files now
        let files: Vec<FsEntry> = host.list_dir();
        assert_eq!(files.len(), 0); // There should be 0 files now
                                    // Create directory
        assert!(host.mkdir(PathBuf::from("test_dir").as_path()).is_ok());
        // Delete directory
        let files: Vec<FsEntry> = host.list_dir();
        assert_eq!(files.len(), 1); // There should be 1 file now
        assert!(host.remove(files.get(0).unwrap()).is_ok());
    }

    #[test]
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn test_host_localhost_rename() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        // Create sample file
        let src_path: PathBuf =
            PathBuf::from(format!("{}/foo.txt", tmpdir.path().display()).as_str());
        assert!(File::create(src_path.as_path()).is_ok());
        let mut host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        let files: Vec<FsEntry> = host.list_dir();
        assert_eq!(files.len(), 1); // There should be 1 file now
        assert_eq!(get_filename(files.get(0).unwrap()), String::from("foo.txt"));
        // Rename file
        let dst_path: PathBuf =
            PathBuf::from(format!("{}/bar.txt", tmpdir.path().display()).as_str());
        assert!(host
            .rename(files.get(0).unwrap(), dst_path.as_path())
            .is_ok());
        // There should be still 1 file now, but named bar.txt
        let files: Vec<FsEntry> = host.list_dir();
        assert_eq!(files.len(), 1); // There should be 0 files now
        assert_eq!(get_filename(files.get(0).unwrap()), String::from("bar.txt"));
        // Fail
        let bad_path: PathBuf = PathBuf::from("/asdailsjoidoewojdijow/ashdiuahu");
        assert!(host
            .rename(files.get(0).unwrap(), bad_path.as_path())
            .is_err());
    }

    /// ### create_sample_file
    ///
    /// Create a sample file
    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn create_sample_file() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(
            tmpfile,
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\nMauris ultricies consequat eros,\nnec scelerisque magna imperdiet metus.\n"
        )
        .unwrap();
        tmpfile
    }

    #[cfg(any(target_os = "unix", target_os = "macos", target_os = "linux"))]
    fn get_filename(entry: &FsEntry) -> String {
        match entry {
            FsEntry::Directory(d) => d.name.clone(),
            FsEntry::File(f) => f.name.clone(),
        }
    }
}
