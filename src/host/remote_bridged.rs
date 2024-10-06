mod temp_mapped_file;

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use remotefs::fs::{Metadata, UnixPex};
use remotefs::{File, RemoteError, RemoteErrorType, RemoteFs};

use self::temp_mapped_file::TempMappedFile;
use super::{HostBridge, HostError, HostResult};

struct WriteStreamOp {
    path: PathBuf,
    metadata: Metadata,
    tempfile: TempMappedFile,
}

/// A remote host bridged over the local host
pub struct RemoteBridged {
    /// Remote fs client
    remote: Box<dyn RemoteFs>,
    /// Reminder used to finalize write stream
    write_stream_op: Option<WriteStreamOp>,
}

impl RemoteBridged {
    fn open_file_from_temp(&mut self, file: &Path) -> HostResult<Box<dyn Read + Send>> {
        let mut temp_file = TempMappedFile::new()?;

        self.remote
            .open_file(file, Box::new(temp_file.clone()))
            .map_err(HostError::from)?;

        // Sync changes
        temp_file.sync()?;

        // now return as read
        Ok(Box::new(temp_file))
    }
}

impl From<Box<dyn RemoteFs>> for RemoteBridged {
    fn from(remote: Box<dyn RemoteFs>) -> Self {
        RemoteBridged {
            remote,
            write_stream_op: None,
        }
    }
}

impl HostBridge for RemoteBridged {
    fn connect(&mut self) -> HostResult<()> {
        self.remote.connect().map(|_| ()).map_err(HostError::from)
    }

    fn disconnect(&mut self) -> HostResult<()> {
        self.remote.disconnect().map_err(HostError::from)
    }

    fn is_connected(&mut self) -> bool {
        self.remote.is_connected()
    }

    fn is_localhost(&self) -> bool {
        false
    }

    fn pwd(&mut self) -> HostResult<PathBuf> {
        debug!("Getting working directory");
        self.remote.pwd().map_err(HostError::from)
    }

    fn change_wrkdir(&mut self, new_dir: &Path) -> HostResult<PathBuf> {
        debug!("Changing working directory to {:?}", new_dir);
        self.remote.change_dir(new_dir).map_err(HostError::from)
    }

    fn mkdir_ex(&mut self, dir_name: &Path, ignore_existing: bool) -> HostResult<()> {
        debug!("Creating directory {:?}", dir_name);
        match self.remote.create_dir(dir_name, UnixPex::from(0o755)) {
            Ok(_) => Ok(()),
            Err(remotefs::RemoteError {
                kind: RemoteErrorType::DirectoryAlreadyExists,
                ..
            }) if ignore_existing => Ok(()),
            Err(e) => Err(HostError::from(e)),
        }
    }

    fn remove(&mut self, entry: &File) -> HostResult<()> {
        debug!("Removing {:?}", entry.path());
        if entry.is_dir() {
            self.remote
                .remove_dir_all(entry.path())
                .map_err(HostError::from)
        } else {
            self.remote
                .remove_file(entry.path())
                .map_err(HostError::from)
        }
    }

    fn rename(&mut self, entry: &File, dst_path: &Path) -> HostResult<()> {
        debug!("Renaming {:?} to {:?}", entry.path(), dst_path);
        self.remote
            .mov(entry.path(), dst_path)
            .map_err(HostError::from)
    }

    fn copy(&mut self, entry: &File, dst: &Path) -> HostResult<()> {
        debug!("Copying {:?} to {:?}", entry.path(), dst);
        self.remote.copy(entry.path(), dst).map_err(HostError::from)
    }

    fn stat(&mut self, path: &Path) -> HostResult<File> {
        debug!("Statting {:?}", path);
        self.remote.stat(path).map_err(HostError::from)
    }

    fn exists(&mut self, path: &Path) -> HostResult<bool> {
        debug!("Checking existence of {:?}", path);
        self.remote.exists(path).map_err(HostError::from)
    }

    fn list_dir(&mut self, path: &Path) -> HostResult<Vec<File>> {
        debug!("Listing directory {:?}", path);
        self.remote.list_dir(path).map_err(HostError::from)
    }

    fn setstat(&mut self, path: &Path, metadata: &Metadata) -> HostResult<()> {
        debug!("Setting metadata for {:?}", path);
        self.remote
            .setstat(path, metadata.clone())
            .map_err(HostError::from)
    }

    fn exec(&mut self, cmd: &str) -> HostResult<String> {
        debug!("Executing command: {}", cmd);
        self.remote
            .exec(cmd)
            .map(|(_, stdout)| stdout)
            .map_err(HostError::from)
    }

    fn symlink(&mut self, src: &Path, dst: &Path) -> HostResult<()> {
        debug!("Creating symlink from {:?} to {:?}", src, dst);
        self.remote.symlink(src, dst).map_err(HostError::from)
    }

    fn chmod(&mut self, path: &Path, pex: UnixPex) -> HostResult<()> {
        debug!("Changing permissions of {:?} to {:?}", path, pex);
        let stat = self.remote.stat(path).map_err(HostError::from)?;
        let mut metadata = stat.metadata.clone();
        metadata.mode = Some(pex);

        self.setstat(path, &metadata)
    }

    fn open_file(&mut self, file: &Path) -> HostResult<Box<dyn Read + Send>> {
        // try to use stream, otherwise download to a temporary file and return a reader
        match self.remote.open(file) {
            Ok(stream) => Ok(Box::new(stream)),
            Err(RemoteError {
                kind: RemoteErrorType::UnsupportedFeature,
                ..
            }) => self.open_file_from_temp(file),
            Err(e) => Err(HostError::from(e)),
        }
    }

    fn create_file(
        &mut self,
        file: &Path,
        metadata: &Metadata,
    ) -> HostResult<Box<dyn Write + Send>> {
        // try to use stream, otherwise download to a temporary file and return a reader
        match self.remote.create(file, metadata) {
            Ok(stream) => Ok(Box::new(stream)),
            Err(RemoteError {
                kind: RemoteErrorType::UnsupportedFeature,
                ..
            }) => {
                let tempfile = TempMappedFile::new()?;
                self.write_stream_op = Some(WriteStreamOp {
                    path: file.to_path_buf(),
                    metadata: metadata.clone(),
                    tempfile: tempfile.clone(),
                });

                Ok(Box::new(tempfile))
            }
            Err(e) => Err(HostError::from(e)),
        }
    }

    fn finalize_write(&mut self, _writer: Box<dyn Write + Send>) -> HostResult<()> {
        if let Some(WriteStreamOp {
            path,
            metadata,
            mut tempfile,
        }) = self.write_stream_op.take()
        {
            // sync
            tempfile.sync()?;
            // write file
            self.remote
                .create_file(&path, &metadata, Box::new(tempfile))?;
        }
        Ok(())
    }
}
