use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt as _;
use std::path::{Path, PathBuf};

use filetime::FileTime;
use remotefs::fs::{FileType, Metadata, UnixPex};
use remotefs::File;

use super::{HostBridge, HostResult};
use crate::host::{HostError, HostErrorType};
use crate::utils::path;

/// Localhost is the entity which holds the information about the current directory and host.
/// It provides functions to navigate across the local host file system
pub struct Localhost {
    wrkdir: PathBuf,
    files: Vec<File>,
}

impl Localhost {
    /// Instantiates a new Localhost struct
    pub fn new(wrkdir: PathBuf) -> HostResult<Self> {
        debug!("Initializing localhost at {}", wrkdir.display());
        let mut host: Localhost = Localhost {
            wrkdir,
            files: Vec::new(),
        };
        // Check if dir exists
        let pwd = host.pwd()?;

        if !host.exists(&pwd)? {
            error!(
                "Failed to initialize localhost: {} doesn't exist",
                host.wrkdir.display()
            );
            return Err(HostError::new(
                HostErrorType::NoSuchFileOrDirectory,
                None,
                host.wrkdir.as_path(),
            ));
        }
        // Retrieve files for provided path
        host.files = match host.list_dir(&pwd) {
            Ok(files) => files,
            Err(err) => {
                error!(
                    "Failed to initialize localhost: could not scan wrkdir: {}",
                    err
                );
                return Err(err);
            }
        };
        info!("Localhost initialized with success");
        Ok(host)
    }

    /// Convert path to absolute path
    fn to_path(&self, p: &Path) -> PathBuf {
        path::absolutize(self.wrkdir.as_path(), p)
    }
}

impl HostBridge for Localhost {
    fn pwd(&mut self) -> HostResult<PathBuf> {
        Ok(self.wrkdir.clone())
    }

    fn change_wrkdir(&mut self, new_dir: &std::path::Path) -> HostResult<PathBuf> {
        let new_dir: PathBuf = self.to_path(new_dir);
        info!("Changing localhost directory to {}...", new_dir.display());
        // Check whether directory exists
        if !self.exists(new_dir.as_path())? {
            error!("Could not change directory: No such file or directory");
            return Err(HostError::new(
                HostErrorType::NoSuchFileOrDirectory,
                None,
                new_dir.as_path(),
            ));
        }
        // Change directory
        if let Err(err) = std::env::set_current_dir(new_dir.as_path()) {
            error!("Could not enter directory: {}", err);
            return Err(HostError::new(
                HostErrorType::NoSuchFileOrDirectory,
                Some(err),
                new_dir.as_path(),
            ));
        }
        let prev_dir: PathBuf = self.wrkdir.clone(); // Backup location
                                                     // Update working directory
                                                     // Change dir
        self.wrkdir = new_dir;
        // Scan new directory
        let pwd = self.pwd()?;
        self.files = match self.list_dir(&pwd) {
            Ok(files) => files,
            Err(err) => {
                error!("Could not scan new directory: {}", err);
                // Restore directory
                self.wrkdir = prev_dir;
                return Err(err);
            }
        };
        debug!("Changed directory to {}", self.wrkdir.display());
        Ok(self.wrkdir.clone())
    }

    fn mkdir_ex(&mut self, dir_name: &std::path::Path, ignore_existing: bool) -> HostResult<()> {
        let dir_path: PathBuf = self.to_path(dir_name);
        info!("Making directory {}", dir_path.display());
        // If dir already exists, return Error
        if dir_path.exists() {
            match ignore_existing {
                true => return Ok(()),
                false => {
                    return Err(HostError::new(
                        HostErrorType::FileAlreadyExists,
                        None,
                        dir_path.as_path(),
                    ))
                }
            }
        }
        match std::fs::create_dir(dir_path.as_path()) {
            Ok(_) => {
                // Update dir
                if dir_name.is_relative() {
                    let pwd = self.pwd()?;
                    self.files = self.list_dir(&pwd)?;
                }
                info!("Created directory {}", dir_path.display());
                Ok(())
            }
            Err(err) => {
                error!("Could not make directory: {}", err);
                Err(HostError::new(
                    HostErrorType::CouldNotCreateFile,
                    Some(err),
                    dir_path.as_path(),
                ))
            }
        }
    }

    fn remove(&mut self, entry: &File) -> HostResult<()> {
        if entry.is_dir() {
            // If file doesn't exist; return error
            debug!("Removing directory {}", entry.path().display());
            if !entry.path().exists() {
                error!("Directory doesn't exist");
                return Err(HostError::new(
                    HostErrorType::NoSuchFileOrDirectory,
                    None,
                    entry.path(),
                ));
            }
            // Remove
            match std::fs::remove_dir_all(entry.path()) {
                Ok(_) => {
                    // Update dir
                    let pwd = self.pwd()?;
                    self.files = self.list_dir(&pwd)?;
                    info!("Removed directory {}", entry.path().display());
                    Ok(())
                }
                Err(err) => {
                    error!("Could not remove directory: {}", err);
                    Err(HostError::new(
                        HostErrorType::DeleteFailed,
                        Some(err),
                        entry.path(),
                    ))
                }
            }
        } else {
            // If file doesn't exist; return error
            debug!("Removing file {}", entry.path().display());
            if !entry.path().exists() {
                error!("File doesn't exist");
                return Err(HostError::new(
                    HostErrorType::NoSuchFileOrDirectory,
                    None,
                    entry.path(),
                ));
            }
            // Remove
            match std::fs::remove_file(entry.path()) {
                Ok(_) => {
                    // Update dir
                    let pwd = self.pwd()?;
                    self.files = self.list_dir(&pwd)?;
                    info!("Removed file {}", entry.path().display());
                    Ok(())
                }
                Err(err) => {
                    error!("Could not remove file: {}", err);
                    Err(HostError::new(
                        HostErrorType::DeleteFailed,
                        Some(err),
                        entry.path(),
                    ))
                }
            }
        }
    }

    fn rename(&mut self, entry: &File, dst_path: &std::path::Path) -> HostResult<()> {
        match std::fs::rename(entry.path(), dst_path) {
            Ok(_) => {
                // Scan dir
                let pwd = self.pwd()?;
                self.files = self.list_dir(&pwd)?;
                debug!(
                    "Moved file {} to {}",
                    entry.path().display(),
                    dst_path.display()
                );
                Ok(())
            }
            Err(err) => {
                error!(
                    "Failed to move {} to {}: {}",
                    entry.path().display(),
                    dst_path.display(),
                    err
                );
                Err(HostError::new(
                    HostErrorType::CouldNotCreateFile,
                    Some(err),
                    entry.path(),
                ))
            }
        }
    }

    fn copy(&mut self, entry: &File, dst: &std::path::Path) -> HostResult<()> {
        // Get absolute path of dest
        let dst: PathBuf = self.to_path(dst);
        info!(
            "Copying file {} to {}",
            entry.path().display(),
            dst.display()
        );
        // Match entry
        if entry.is_dir() {
            // If destination path doesn't exist, create destination
            if !dst.exists() {
                debug!("Directory {} doesn't exist; creating it", dst.display());
                self.mkdir(dst.as_path())?;
            }
            // Scan dir
            let dir_files: Vec<File> = self.list_dir(entry.path())?;
            // Iterate files
            for dir_entry in dir_files.iter() {
                // Calculate dst
                let mut sub_dst: PathBuf = dst.clone();
                sub_dst.push(dir_entry.name());
                // Call function recursively
                self.copy(dir_entry, sub_dst.as_path())?;
            }
        } else {
            // Copy file
            // If destination path is a directory, push file name
            let dst: PathBuf = match dst.as_path().is_dir() {
                true => {
                    let mut p: PathBuf = dst.clone();
                    p.push(entry.name().as_str());
                    p
                }
                false => dst.clone(),
            };
            // Copy entry path to dst path
            if let Err(err) = std::fs::copy(entry.path(), dst.as_path()) {
                error!("Failed to copy file: {}", err);
                return Err(HostError::new(
                    HostErrorType::CouldNotCreateFile,
                    Some(err),
                    entry.path(),
                ));
            }
            info!("File copied");
        }
        // Reload directory if dst is pwd
        let pwd = self.pwd()?;
        match dst.is_dir() {
            true => {
                if dst == pwd {
                    self.files = self.list_dir(&pwd)?;
                } else if let Some(parent) = dst.parent() {
                    // If parent is pwd, scan directory
                    if parent == pwd {
                        self.files = self.list_dir(&pwd)?;
                    }
                }
            }
            false => {
                if let Some(parent) = dst.parent() {
                    // If parent is pwd, scan directory
                    if parent == pwd {
                        self.files = self.list_dir(&pwd)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn stat(&mut self, path: &std::path::Path) -> HostResult<File> {
        info!("Stating file {}", path.display());
        let path: PathBuf = self.to_path(path);
        let attr = match fs::metadata(path.as_path()) {
            Ok(metadata) => metadata,
            Err(err) => {
                error!("Could not read file metadata: {}", err);
                return Err(HostError::new(
                    HostErrorType::FileNotAccessible,
                    Some(err),
                    path.as_path(),
                ));
            }
        };
        let mut metadata = Metadata::from(attr);
        if let Ok(symlink) = fs::read_link(path.as_path()) {
            metadata.set_symlink(symlink);
            metadata.file_type = FileType::Symlink;
        }
        // Match dir / file
        Ok(File { path, metadata })
    }

    fn exists(&mut self, path: &Path) -> HostResult<bool> {
        Ok(path.exists())
    }

    fn list_dir(&mut self, path: &Path) -> HostResult<Vec<File>> {
        info!("Reading directory {}", path.display());
        match std::fs::read_dir(path) {
            Ok(e) => {
                let mut fs_entries: Vec<File> = Vec::new();
                for entry in e.flatten() {
                    // NOTE: 0.4.1, don't fail if stat for one file fails
                    match self.stat(entry.path().as_path()) {
                        Ok(entry) => fs_entries.push(entry),
                        Err(e) => error!("Failed to stat {}: {}", entry.path().display(), e),
                    }
                }
                Ok(fs_entries)
            }
            Err(err) => Err(HostError::new(
                HostErrorType::DirNotAccessible,
                Some(err),
                path,
            )),
        }
    }

    fn setstat(&mut self, path: &Path, metadata: &Metadata) -> HostResult<()> {
        debug!("Setting stat for file at {}", path.display());
        if let Some(mtime) = metadata.modified {
            let mtime = FileTime::from_system_time(mtime);
            debug!("setting mtime {:?}", mtime);
            filetime::set_file_mtime(path, mtime)
                .map_err(|e| HostError::new(HostErrorType::FileNotAccessible, Some(e), path))?;
        }
        if let Some(atime) = metadata.accessed {
            let atime = FileTime::from_system_time(atime);
            filetime::set_file_atime(path, atime)
                .map_err(|e| HostError::new(HostErrorType::FileNotAccessible, Some(e), path))?;
        }
        #[cfg(unix)]
        if let Some(mode) = metadata.mode {
            self.chmod(path, mode)?;
        }
        Ok(())
    }

    fn exec(&mut self, cmd: &str) -> HostResult<String> {
        // Make command
        let args: Vec<&str> = cmd.split(' ').collect();
        let cmd: &str = args.first().unwrap();
        let argv: &[&str] = &args[1..];
        info!("Executing command: {} {:?}", cmd, argv);
        match std::process::Command::new(cmd).args(argv).output() {
            Ok(output) => match std::str::from_utf8(&output.stdout) {
                Ok(s) => {
                    info!("Command output: {}", s);
                    Ok(s.to_string())
                }
                Err(_) => Ok(String::new()),
            },
            Err(err) => {
                error!("Failed to run command: {}", err);
                Err(HostError::new(
                    HostErrorType::ExecutionFailed,
                    Some(err),
                    self.wrkdir.as_path(),
                ))
            }
        }
    }

    fn symlink(&mut self, src: &Path, dst: &Path) -> HostResult<()> {
        if cfg!(windows) {
            warn!("Cannot create symlink on Windows");

            return Err(HostError::from(HostErrorType::NotImplemented));
        }

        let src = self.to_path(src);
        std::os::unix::fs::symlink(dst, src.as_path()).map_err(|e| {
            error!(
                "Failed to create symlink at {} pointing at {}: {}",
                src.display(),
                dst.display(),
                e
            );
            HostError::new(HostErrorType::CouldNotCreateFile, Some(e), src.as_path())
        })
    }

    fn chmod(&mut self, path: &std::path::Path, pex: UnixPex) -> HostResult<()> {
        if cfg!(windows) {
            warn!("Cannot set file mode on Windows");

            return Err(HostError::from(HostErrorType::NotImplemented));
        }

        let path: PathBuf = self.to_path(path);
        // Get metadta
        match fs::metadata(path.as_path()) {
            Ok(metadata) => {
                let mut mpex = metadata.permissions();
                mpex.set_mode(pex.into());
                match fs::set_permissions(path.as_path(), mpex) {
                    Ok(_) => {
                        info!("Changed mode for {} to {:?}", path.display(), pex);
                        Ok(())
                    }
                    Err(err) => {
                        error!("Could not change mode for file {}: {}", path.display(), err);
                        Err(HostError::new(
                            HostErrorType::FileNotAccessible,
                            Some(err),
                            path.as_path(),
                        ))
                    }
                }
            }
            Err(err) => {
                error!(
                    "Chmod failed; could not read metadata for file {}: {}",
                    path.display(),
                    err
                );
                Err(HostError::new(
                    HostErrorType::FileNotAccessible,
                    Some(err),
                    path.as_path(),
                ))
            }
        }
    }

    fn open_file(&mut self, file: &std::path::Path) -> HostResult<Box<dyn Read + Send>> {
        let file: PathBuf = self.to_path(file);
        info!("Opening file {} for read", file.display());
        if !self.exists(file.as_path())? {
            error!("File doesn't exist!");
            return Err(HostError::new(
                HostErrorType::NoSuchFileOrDirectory,
                None,
                file.as_path(),
            ));
        }
        match OpenOptions::new()
            .create(false)
            .read(true)
            .write(false)
            .open(file.as_path())
        {
            Ok(f) => Ok(Box::new(f)),
            Err(err) => {
                error!("Could not open file for read: {}", err);
                Err(HostError::new(
                    HostErrorType::FileNotAccessible,
                    Some(err),
                    file.as_path(),
                ))
            }
        }
    }

    fn create_file(&mut self, file: &Path) -> HostResult<Box<dyn Write + Send>> {
        let file: PathBuf = self.to_path(file);
        info!("Opening file {} for write", file.display());
        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file.as_path())
        {
            Ok(f) => Ok(Box::new(f)),
            Err(err) => {
                error!("Failed to open file: {}", err);
                match self.exists(file.as_path())? {
                    true => Err(HostError::new(
                        HostErrorType::ReadonlyFile,
                        Some(err),
                        file.as_path(),
                    )),
                    false => Err(HostError::new(
                        HostErrorType::FileNotAccessible,
                        Some(err),
                        file.as_path(),
                    )),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(unix)]
    use std::fs::File as StdFile;
    #[cfg(unix)]
    use std::io::Write;
    use std::ops::AddAssign;
    #[cfg(unix)]
    use std::os::unix::fs::{symlink, PermissionsExt};
    use std::time::{Duration, SystemTime};

    use pretty_assertions::assert_eq;

    use super::*;
    #[cfg(unix)]
    use crate::utils::test_helpers::make_fsentry;
    use crate::utils::test_helpers::{create_sample_file, make_file_at};

    #[test]
    fn test_host_error_new() {
        let error: HostError =
            HostError::new(HostErrorType::CouldNotCreateFile, None, Path::new("/tmp"));
        assert!(error.ioerr.is_none());
        assert_eq!(error.path.as_ref().unwrap(), Path::new("/tmp"));
    }

    #[test]
    #[cfg(unix)]
    fn test_host_localhost_new() {
        let host: Localhost = Localhost::new(PathBuf::from("/dev")).ok().unwrap();
        assert_eq!(host.wrkdir, PathBuf::from("/dev"));
        // Scan dir
        let entries = std::fs::read_dir(PathBuf::from("/dev").as_path()).unwrap();
        let mut counter: usize = 0;
        for _ in entries {
            counter += 1;
        }
        assert_eq!(host.files.len(), counter);
    }

    #[test]
    #[cfg(windows)]
    fn test_host_localhost_new() {
        let mut host: Localhost = Localhost::new(PathBuf::from("C:\\users")).ok().unwrap();
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
    #[cfg(unix)]
    fn test_host_localhost_pwd() {
        let mut host: Localhost = Localhost::new(PathBuf::from("/dev")).ok().unwrap();
        assert_eq!(host.pwd().unwrap(), PathBuf::from("/dev"));
    }

    #[test]
    #[cfg(unix)]
    fn test_host_localhost_change_dir() {
        let mut host: Localhost = Localhost::new(PathBuf::from("/dev")).ok().unwrap();
        let new_dir: PathBuf = PathBuf::from("/dev");
        assert!(host.change_wrkdir(new_dir.as_path()).is_ok());
        // Verify new files
        // Scan dir
        let entries = std::fs::read_dir(new_dir.as_path()).unwrap();
        let mut counter: usize = 0;
        for _ in entries {
            counter += 1;
        }
        assert_eq!(host.files.len(), counter);
    }

    #[test]
    #[cfg(unix)]
    #[should_panic]
    fn test_host_localhost_change_dir_failed() {
        let mut host: Localhost = Localhost::new(PathBuf::from("/dev")).ok().unwrap();
        let new_dir: PathBuf = PathBuf::from("/omar/gabber/123/456");
        assert!(host.change_wrkdir(new_dir.as_path()).is_ok());
    }

    #[test]
    #[cfg(unix)]
    fn test_host_localhost_open_read() {
        let mut host: Localhost = Localhost::new(PathBuf::from("/dev")).ok().unwrap();
        // Create temp file
        let file: tempfile::NamedTempFile = create_sample_file();
        assert!(host.open_file(file.path()).is_ok());
    }

    #[test]
    #[cfg(unix)]
    #[should_panic]
    fn test_host_localhost_open_read_err_no_such_file() {
        let mut host: Localhost = Localhost::new(PathBuf::from("/dev")).ok().unwrap();
        assert!(host
            .open_file(PathBuf::from("/bin/foo-bar-test-omar-123-456-789.txt").as_path())
            .is_ok());
    }

    #[test]
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    fn test_host_localhost_open_read_err_not_accessible() {
        let mut host: Localhost = Localhost::new(PathBuf::from("/dev")).ok().unwrap();
        let file: tempfile::NamedTempFile = create_sample_file();
        //let mut perms = fs::metadata(file.path())?.permissions();
        fs::set_permissions(file.path(), PermissionsExt::from_mode(0o222)).unwrap();
        //fs::set_permissions(file.path(), perms)?;
        assert!(host.open_file(file.path()).is_err());
    }

    #[test]
    #[cfg(unix)]
    fn test_host_localhost_open_write() {
        let mut host: Localhost = Localhost::new(PathBuf::from("/dev")).ok().unwrap();
        // Create temp file
        let file: tempfile::NamedTempFile = create_sample_file();
        assert!(host.create_file(file.path()).is_ok());
    }

    #[test]
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    fn test_host_localhost_open_write_err() {
        let mut host: Localhost = Localhost::new(PathBuf::from("/dev")).ok().unwrap();
        let file: tempfile::NamedTempFile = create_sample_file();
        //let mut perms = fs::metadata(file.path())?.permissions();
        fs::set_permissions(file.path(), PermissionsExt::from_mode(0o444)).unwrap();
        //fs::set_permissions(file.path(), perms)?;
        assert!(host.create_file(file.path()).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn test_host_localhost_symlinks() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        // Create sample file
        assert!(StdFile::create(format!("{}/foo.txt", tmpdir.path().display()).as_str()).is_ok());
        // Create symlink
        assert!(symlink(
            format!("{}/foo.txt", tmpdir.path().display()),
            format!("{}/bar.txt", tmpdir.path().display())
        )
        .is_ok());
        // Get dir
        let host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        let files: Vec<File> = host.files.clone();
        // Verify files
        let file_0: &File = files.get(0).unwrap();
        if file_0.name() == *"foo.txt" {
            assert!(file_0.metadata.symlink.is_none());
        } else {
            assert_eq!(
                file_0.metadata.symlink.as_ref().unwrap(),
                &PathBuf::from(format!("{}/foo.txt", tmpdir.path().display()))
            );
        }
        // Verify simlink
        let file_1: &File = files.get(1).unwrap();
        if file_1.name() == *"bar.txt" {
            assert_eq!(
                file_1.metadata.symlink.as_ref().unwrap(),
                &PathBuf::from(format!("{}/foo.txt", tmpdir.path().display()))
            );
        } else {
            assert!(file_1.metadata.symlink.is_none());
        }
    }

    #[test]
    #[cfg(unix)]
    fn test_host_localhost_mkdir() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        let mut host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        let files: Vec<File> = host.files.clone();
        assert_eq!(files.len(), 0); // There should be 0 files now
        assert!(host.mkdir(PathBuf::from("test_dir").as_path()).is_ok());
        let files: Vec<File> = host.files.clone();
        assert_eq!(files.len(), 1); // There should be 1 file now
                                    // Try to re-create directory
        assert!(host.mkdir(PathBuf::from("test_dir").as_path()).is_err());
        // Try abs path
        assert!(host
            .mkdir_ex(PathBuf::from("/tmp/test_dir_123456789").as_path(), true)
            .is_ok());
        // Fail
        assert!(host
            .mkdir_ex(
                PathBuf::from("/aaaa/oooooo/tmp/test_dir_123456789").as_path(),
                true
            )
            .is_err());
    }

    #[test]
    #[cfg(unix)]
    fn test_host_localhost_remove() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        // Create sample file
        assert!(StdFile::create(format!("{}/foo.txt", tmpdir.path().display()).as_str()).is_ok());
        let mut host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        let files: Vec<File> = host.files.clone();
        assert_eq!(files.len(), 1); // There should be 1 file now
                                    // Remove file
        assert!(host.remove(files.get(0).unwrap()).is_ok());
        // There should be 0 files now
        let files: Vec<File> = host.files.clone();
        assert_eq!(files.len(), 0); // There should be 0 files now
                                    // Create directory
        assert!(host.mkdir(PathBuf::from("test_dir").as_path()).is_ok());
        // Delete directory
        let files: Vec<File> = host.files.clone();
        assert_eq!(files.len(), 1); // There should be 1 file now
        assert!(host.remove(files.get(0).unwrap()).is_ok());
        // Remove unexisting directory
        assert!(host
            .remove(&make_fsentry(PathBuf::from("/a/b/c/d"), true))
            .is_err());
        assert!(host
            .remove(&make_fsentry(PathBuf::from("/aaaaaaa"), false))
            .is_err());
    }

    #[test]
    #[cfg(unix)]
    fn test_host_localhost_rename() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        // Create sample file
        let src_path: PathBuf =
            PathBuf::from(format!("{}/foo.txt", tmpdir.path().display()).as_str());
        assert!(StdFile::create(src_path.as_path()).is_ok());
        let mut host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        let files: Vec<File> = host.files.clone();
        assert_eq!(files.len(), 1); // There should be 1 file now
        assert_eq!(files.get(0).unwrap().name(), "foo.txt");
        // Rename file
        let dst_path: PathBuf =
            PathBuf::from(format!("{}/bar.txt", tmpdir.path().display()).as_str());
        assert!(host
            .rename(files.get(0).unwrap(), dst_path.as_path())
            .is_ok());
        // There should be still 1 file now, but named bar.txt
        let files: Vec<File> = host.files.clone();
        assert_eq!(files.len(), 1); // There should be 0 files now
        assert_eq!(files.get(0).unwrap().name(), "bar.txt");
        // Fail
        let bad_path: PathBuf = PathBuf::from("/asdailsjoidoewojdijow/ashdiuahu");
        assert!(host
            .rename(files.get(0).unwrap(), bad_path.as_path())
            .is_err());
    }

    #[test]
    fn should_setstat() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        let file: tempfile::NamedTempFile = create_sample_file();
        let mut host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        // stat
        let mut filemeta = host.stat(file.path()).unwrap();

        let mut new_atime = SystemTime::UNIX_EPOCH;
        new_atime.add_assign(Duration::from_secs(1612164210));

        let mut new_mtime = SystemTime::UNIX_EPOCH;
        new_mtime.add_assign(Duration::from_secs(1613160210));

        filemeta.metadata.accessed = Some(new_atime);
        filemeta.metadata.modified = Some(new_mtime);

        // setstat
        assert!(host.setstat(filemeta.path(), filemeta.metadata()).is_ok());
        let new_metadata = host.stat(file.path()).unwrap();

        assert_eq!(new_metadata.metadata().accessed, Some(new_atime));
        assert_eq!(new_metadata.metadata().modified, Some(new_mtime));
    }

    #[cfg(unix)]
    #[test]
    fn test_host_chmod() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        let file: tempfile::NamedTempFile = create_sample_file();
        let mut host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        // Chmod to file
        assert!(host.chmod(file.path(), UnixPex::from(0o755)).is_ok());
        // Chmod to dir
        assert!(host.chmod(tmpdir.path(), UnixPex::from(0o750)).is_ok());
        // Error
        assert!(host
            .chmod(
                Path::new("/tmp/krgiogoiegj/kwrgnoerig"),
                UnixPex::from(0o777)
            )
            .is_err());
    }

    #[cfg(unix)]
    #[test]
    fn test_host_copy_file_absolute() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        // Create file in tmpdir
        let mut file1_path: PathBuf = PathBuf::from(tmpdir.path());
        file1_path.push("foo.txt");
        // Write file 1
        let mut file1 = StdFile::create(file1_path.as_path()).ok().unwrap();
        assert!(file1.write_all(b"Hello world!\n").is_ok());
        // Get file 2 path
        let mut file2_path: PathBuf = PathBuf::from(tmpdir.path());
        file2_path.push("bar.txt");
        // Create host
        let mut host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        let file1_entry: File = host.files.get(0).unwrap().clone();
        assert_eq!(file1_entry.name(), String::from("foo.txt"));
        // Copy
        assert!(host.copy(&file1_entry, file2_path.as_path()).is_ok());
        // Verify host has two files
        assert_eq!(host.files.len(), 2);
        // Fail copy
        assert!(host
            .copy(
                &make_fsentry(PathBuf::from("/a/a7/a/a7a"), false),
                PathBuf::from("571k422i").as_path()
            )
            .is_err());
    }

    #[cfg(unix)]
    #[test]
    fn test_host_copy_file_relative() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        // Create file in tmpdir
        let mut file1_path: PathBuf = PathBuf::from(tmpdir.path());
        file1_path.push("foo.txt");
        // Write file 1
        let mut file1 = StdFile::create(file1_path.as_path()).ok().unwrap();
        assert!(file1.write_all(b"Hello world!\n").is_ok());
        // Get file 2 path
        let file2_path: PathBuf = PathBuf::from("bar.txt");
        // Create host
        let mut host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        let file1_entry: File = host.files.get(0).unwrap().clone();
        assert_eq!(file1_entry.name(), String::from("foo.txt"));
        // Copy
        assert!(host.copy(&file1_entry, file2_path.as_path()).is_ok());
        // Verify host has two files
        assert_eq!(host.files.len(), 2);
    }

    #[cfg(unix)]
    #[test]
    fn test_host_copy_directory_absolute() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        // Create directory in tmpdir
        let mut dir_src: PathBuf = PathBuf::from(tmpdir.path());
        dir_src.push("test_dir/");
        assert!(std::fs::create_dir(dir_src.as_path()).is_ok());
        // Create file in src dir
        let mut file1_path: PathBuf = dir_src.clone();
        file1_path.push("foo.txt");
        // Write file 1
        let mut file1 = StdFile::create(file1_path.as_path()).ok().unwrap();
        assert!(file1.write_all(b"Hello world!\n").is_ok());
        // Copy dir src to dir ddest
        let mut dir_dest: PathBuf = PathBuf::from(tmpdir.path());
        dir_dest.push("test_dest_dir/");
        // Create host
        let mut host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        let dir_src_entry: File = host.files.get(0).unwrap().clone();
        assert_eq!(dir_src_entry.name(), String::from("test_dir"));
        // Copy
        assert!(host.copy(&dir_src_entry, dir_dest.as_path()).is_ok());
        // Verify host has two files
        assert_eq!(host.files.len(), 2);
        // Verify dir_dest contains foo.txt
        let mut test_file_path: PathBuf = dir_dest.clone();
        test_file_path.push("foo.txt");
        assert!(host.stat(test_file_path.as_path()).is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn test_host_copy_directory_relative() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        // Create directory in tmpdir
        let mut dir_src: PathBuf = PathBuf::from(tmpdir.path());
        dir_src.push("test_dir/");
        assert!(std::fs::create_dir(dir_src.as_path()).is_ok());
        // Create file in src dir
        let mut file1_path: PathBuf = dir_src.clone();
        file1_path.push("foo.txt");
        // Write file 1
        let mut file1 = StdFile::create(file1_path.as_path()).ok().unwrap();
        assert!(file1.write_all(b"Hello world!\n").is_ok());
        // Copy dir src to dir ddest
        let dir_dest: PathBuf = PathBuf::from("test_dest_dir/");
        // Create host
        let mut host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        let dir_src_entry: File = host.files.get(0).unwrap().clone();
        assert_eq!(dir_src_entry.name(), String::from("test_dir"));
        // Copy
        assert!(host.copy(&dir_src_entry, dir_dest.as_path()).is_ok());
        // Verify host has two files
        assert_eq!(host.files.len(), 2);
        // Verify dir_dest contains foo.txt
        let mut test_file_path: PathBuf = dir_dest.clone();
        test_file_path.push("foo.txt");
        assert!(host.stat(test_file_path.as_path()).is_ok());
    }

    #[test]
    fn test_host_exec() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        let mut host: Localhost = Localhost::new(PathBuf::from(tmpdir.path())).ok().unwrap();
        // Execute
        assert!(host.exec("echo 5").ok().unwrap().as_str().contains("5"));
    }

    #[cfg(unix)]
    #[test]
    fn should_create_symlink() {
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        let dir_path: &Path = tmpdir.path();
        // Make file
        assert!(make_file_at(dir_path, "pippo.txt").is_ok());
        let mut host: Localhost = Localhost::new(PathBuf::from(dir_path)).ok().unwrap();
        let mut p = dir_path.to_path_buf();
        p.push("pippo.txt");
        // Make symlink
        assert!(host.symlink(Path::new("link.txt"), p.as_path()).is_ok());
        // Fail symlink
        assert!(host.symlink(Path::new("link.txt"), p.as_path()).is_err());
        assert!(host
            .symlink(Path::new("/tmp/oooo/aaaa"), p.as_path())
            .is_err());
    }

    #[test]
    fn test_host_fmt_error() {
        let err: HostError = HostError::new(
            HostErrorType::CouldNotCreateFile,
            Some(std::io::Error::from(std::io::ErrorKind::AddrInUse)),
            Path::new("/tmp"),
        );
        assert_eq!(
            format!("{err}"),
            String::from("Could not create file: address in use (/tmp)"),
        );
        assert_eq!(
            format!("{}", HostError::from(HostErrorType::DeleteFailed)),
            String::from("Could not delete file")
        );
        assert_eq!(
            format!("{}", HostError::from(HostErrorType::ExecutionFailed)),
            String::from("Command execution failed"),
        );
        assert_eq!(
            format!("{}", HostError::from(HostErrorType::DirNotAccessible)),
            String::from("Could not access directory"),
        );
        assert_eq!(
            format!("{}", HostError::from(HostErrorType::NoSuchFileOrDirectory)),
            String::from("No such file or directory")
        );
        assert_eq!(
            format!("{}", HostError::from(HostErrorType::ReadonlyFile)),
            String::from("File is readonly")
        );
        assert_eq!(
            format!("{}", HostError::from(HostErrorType::FileNotAccessible)),
            String::from("Could not access file")
        );
        assert_eq!(
            format!("{}", HostError::from(HostErrorType::FileAlreadyExists)),
            String::from("File already exists")
        );
    }
}
