use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use remotefs::fs::{Metadata, UnixPex};
use remotefs::File;

use super::HostResult;

/// Trait to bridge a remote filesystem to the host filesystem
///
/// In case of `Localhost` this should be effortless, while for remote hosts this should
/// implement a real bridge when the resource is first loaded on the local
///  filesystem and then processed on the remote.
pub trait HostBridge {
    /// Print working directory
    fn pwd(&mut self) -> HostResult<PathBuf>;

    /// Change working directory with the new provided directory
    fn change_wrkdir(&mut self, new_dir: &Path) -> HostResult<PathBuf>;

    /// Make a directory at path and update the file list (only if relative)
    fn mkdir(&mut self, dir_name: &Path) -> HostResult<()> {
        self.mkdir_ex(dir_name, false)
    }

    /// Extended option version of makedir.
    /// ignex: don't report error if directory already exists
    fn mkdir_ex(&mut self, dir_name: &Path, ignore_existing: bool) -> HostResult<()>;

    /// Remove file entry
    fn remove(&mut self, entry: &File) -> HostResult<()>;

    /// Rename file or directory to new name
    fn rename(&mut self, entry: &File, dst_path: &Path) -> HostResult<()>;

    /// Copy file to destination path
    fn copy(&mut self, entry: &File, dst: &Path) -> HostResult<()>;

    /// Stat file and create a File
    fn stat(&mut self, path: &Path) -> HostResult<File>;

    /// Returns whether provided file path exists
    fn exists(&mut self, path: &Path) -> HostResult<bool>;

    /// Get content of a directory
    fn list_dir(&mut self, path: &Path) -> HostResult<Vec<File>>;

    /// Set file stat
    fn setstat(&mut self, path: &Path, metadata: &Metadata) -> HostResult<()>;

    /// Execute a command on localhost
    fn exec(&mut self, cmd: &str) -> HostResult<String>;

    /// Create a symlink from src to dst
    fn symlink(&mut self, src: &Path, dst: &Path) -> HostResult<()>;

    /// Change file mode to file, according to UNIX permissions
    fn chmod(&mut self, path: &Path, pex: UnixPex) -> HostResult<()>;

    /// Open file for reading
    fn open_file(&mut self, file: &Path) -> HostResult<Box<dyn Read + Send>>;

    /// Open file for writing
    fn create_file(
        &mut self,
        file: &Path,
        metadata: &Metadata,
    ) -> HostResult<Box<dyn Write + Send>>;

    /// Finalize write operation
    fn finalize_write(&mut self, writer: Box<dyn Write + Send>) -> HostResult<()>;
}
