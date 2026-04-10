//! ## SFTP
//!
//! Sftp remote fs implementation

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use remotefs::File;
use remotefs::fs::{
    Metadata, ReadStream, RemoteError, RemoteErrorType, RemoteFs, RemoteResult, UnixPex, Welcome,
    WriteStream,
};

use super::SshOpts;
use crate::SshSession;
use crate::ssh::backend::{Sftp as _, WriteMode};
use crate::utils::path as path_utils;

/// Sftp "filesystem" client
pub struct SftpFs<S>
where
    S: SshSession,
{
    session: Option<S>,
    sftp: Option<S::Sftp>,
    wrkdir: PathBuf,
    opts: SshOpts,
}

#[cfg(feature = "libssh2")]
#[cfg_attr(docsrs, doc(cfg(feature = "libssh2")))]
impl SftpFs<super::backend::LibSsh2Session> {
    /// Constructs a new [`SftpFs`] instance with the `libssh2` backend.
    pub fn libssh2(opts: SshOpts) -> Self {
        Self {
            session: None,
            sftp: None,
            wrkdir: PathBuf::from("/"),
            opts,
        }
    }
}

#[cfg(feature = "libssh")]
#[cfg_attr(docsrs, doc(cfg(feature = "libssh")))]
impl SftpFs<super::backend::LibSshSession> {
    /// Constructs a new [`SftpFs`] instance with the `libssh` backend.
    pub fn libssh(opts: SshOpts) -> Self {
        Self {
            session: None,
            sftp: None,
            wrkdir: PathBuf::from("/"),
            opts,
        }
    }
}

impl<S> SftpFs<S>
where
    S: SshSession,
{
    /// Get a reference to current `session` value.
    pub fn session(&mut self) -> Option<&mut S> {
        self.session.as_mut()
    }

    /// Get a reference to current `sftp` value.
    pub fn sftp(&mut self) -> Option<&mut S::Sftp> {
        self.sftp.as_mut()
    }

    // -- private

    /// Check connection status
    fn check_connection(&mut self) -> RemoteResult<()> {
        if self.is_connected() {
            Ok(())
        } else {
            Err(RemoteError::new(RemoteErrorType::NotConnected))
        }
    }
}

impl<S> RemoteFs for SftpFs<S>
where
    S: SshSession,
{
    fn connect(&mut self) -> RemoteResult<Welcome> {
        debug!("Initializing SFTP connection...");
        let mut session = S::connect(&self.opts)?;
        // Get working directory
        debug!("Getting working directory...");
        self.wrkdir = session
            .cmd("pwd")
            .map(|(_rc, output)| PathBuf::from(output.as_str().trim()))?;
        // Get Sftp client
        debug!("Getting SFTP client...");
        let sftp = match session.sftp() {
            Ok(s) => s,
            Err(err) => {
                error!("Could not get sftp client: {err}");
                return Err(RemoteError::new_ex(RemoteErrorType::ProtocolError, err));
            }
        };
        self.session = Some(session);
        self.sftp = Some(sftp);
        let banner = self.session.as_ref().unwrap().banner()?;
        debug!(
            "Connection established: '{}'; working directory {}",
            banner.as_deref().unwrap_or(""),
            self.wrkdir.display()
        );
        Ok(Welcome::default().banner(banner))
    }

    fn disconnect(&mut self) -> RemoteResult<()> {
        debug!("Disconnecting from remote...");
        if let Some(session) = self.session.as_ref() {
            // First free sftp
            self.sftp = None;
            // Disconnect (greet server with 'Mandi' as they do in Friuli)
            match session.disconnect() {
                Ok(_) => {
                    // Set session and sftp to none
                    self.session = None;
                    Ok(())
                }
                Err(err) => Err(RemoteError::new_ex(RemoteErrorType::ConnectionError, err)),
            }
        } else {
            Err(RemoteError::new(RemoteErrorType::NotConnected))
        }
    }

    fn is_connected(&mut self) -> bool {
        self.session
            .as_ref()
            .map(|x| x.authenticated().unwrap_or_default())
            .unwrap_or_default()
    }

    fn pwd(&mut self) -> RemoteResult<PathBuf> {
        self.check_connection()?;
        Ok(self.wrkdir.clone())
    }

    fn change_dir(&mut self, dir: &Path) -> RemoteResult<PathBuf> {
        self.check_connection()?;
        let dir = path_utils::absolutize(self.wrkdir.as_path(), dir);
        // Stat path to check if it exists. If it is a file, return error
        match self.stat(dir.as_path()) {
            Err(err) => Err(err),
            Ok(file) if file.is_dir() => {
                self.wrkdir = dir;
                debug!("Changed working directory to {}", self.wrkdir.display());
                Ok(self.wrkdir.clone())
            }
            Ok(_) => Err(RemoteError::new_ex(
                RemoteErrorType::BadFile,
                "expected directory, got file",
            )),
        }
    }

    fn list_dir(&mut self, path: &Path) -> RemoteResult<Vec<File>> {
        if let Some(sftp) = self.sftp.as_ref() {
            let path = path_utils::absolutize(self.wrkdir.as_path(), path);
            debug!("Reading directory content of {}", path.display());
            match sftp.readdir(path.as_path()) {
                Err(err) => Err(RemoteError::new_ex(RemoteErrorType::StatFailed, err)),
                Ok(files) => Ok(files),
            }
        } else {
            Err(RemoteError::new(RemoteErrorType::NotConnected))
        }
    }

    fn stat(&mut self, path: &Path) -> RemoteResult<File> {
        if let Some(sftp) = self.sftp.as_ref() {
            let path = path_utils::absolutize(self.wrkdir.as_path(), path);
            debug!("Collecting metadata for {}", path.display());
            sftp.stat(path.as_path()).map_err(|e| {
                error!("Stat failed: {e}");
                RemoteError::new_ex(RemoteErrorType::NoSuchFileOrDirectory, e)
            })
        } else {
            Err(RemoteError::new(RemoteErrorType::NotConnected))
        }
    }

    fn setstat(&mut self, path: &Path, metadata: Metadata) -> RemoteResult<()> {
        if let Some(sftp) = self.sftp.as_ref() {
            let path = path_utils::absolutize(self.wrkdir.as_path(), path);
            debug!("Setting metadata for {}", path.display());
            sftp.setstat(path.as_path(), metadata)
                .map(|_| ())
                .map_err(|e| {
                    error!("Setstat failed: {e}");
                    RemoteError::new_ex(RemoteErrorType::StatFailed, e)
                })
        } else {
            Err(RemoteError::new(RemoteErrorType::NotConnected))
        }
    }

    fn exists(&mut self, path: &Path) -> RemoteResult<bool> {
        match self.stat(path) {
            Ok(_) => Ok(true),
            Err(RemoteError {
                kind: RemoteErrorType::NoSuchFileOrDirectory,
                ..
            }) => Ok(false),
            Err(err) => Err(err),
        }
    }

    fn remove_file(&mut self, path: &Path) -> RemoteResult<()> {
        if let Some(sftp) = self.sftp.as_ref() {
            let path = path_utils::absolutize(self.wrkdir.as_path(), path);
            debug!("Remove file {}", path.display());
            sftp.unlink(path.as_path()).map_err(|e| {
                error!("Remove failed: {e}");
                RemoteError::new_ex(RemoteErrorType::CouldNotRemoveFile, e)
            })
        } else {
            Err(RemoteError::new(RemoteErrorType::NotConnected))
        }
    }

    fn remove_dir(&mut self, path: &Path) -> RemoteResult<()> {
        if let Some(sftp) = self.sftp.as_ref() {
            let path = path_utils::absolutize(self.wrkdir.as_path(), path);
            debug!("Remove dir {}", path.display());
            sftp.rmdir(path.as_path()).map_err(|e| {
                error!("Remove failed: {e}");
                RemoteError::new_ex(RemoteErrorType::CouldNotRemoveFile, e)
            })
        } else {
            Err(RemoteError::new(RemoteErrorType::NotConnected))
        }
    }

    fn remove_dir_all(&mut self, path: &Path) -> RemoteResult<()> {
        self.check_connection()?;
        let path = path_utils::absolutize(self.wrkdir.as_path(), path);
        if !self.exists(path.as_path()).ok().unwrap_or(false) {
            return Err(RemoteError::new(RemoteErrorType::NoSuchFileOrDirectory));
        }
        debug!("Removing directory {} recursively", path.display());
        match self
            .session
            .as_mut()
            .unwrap()
            .cmd(format!("rm -rf \"{}\"", path.display()))
        {
            Ok((0, _)) => Ok(()),
            Ok(_) => Err(RemoteError::new(RemoteErrorType::CouldNotRemoveFile)),
            Err(err) => Err(RemoteError::new_ex(RemoteErrorType::ProtocolError, err)),
        }
    }

    fn create_dir(&mut self, path: &Path, mode: UnixPex) -> RemoteResult<()> {
        self.check_connection()?;
        let path = path_utils::absolutize(self.wrkdir.as_path(), path);
        // Check if already exists
        debug!(
            "Creating directory {} (mode: {:o})",
            path.display(),
            u32::from(mode)
        );
        if self.exists(path.as_path())? {
            error!("directory {} already exists", path.display());
            return Err(RemoteError::new(RemoteErrorType::DirectoryAlreadyExists));
        }
        self.sftp
            .as_ref()
            .unwrap()
            .mkdir(path.as_path(), u32::from(mode) as i32)
            .map_err(|e| {
                error!("Create dir failed: {e}");
                RemoteError::new_ex(RemoteErrorType::FileCreateDenied, e)
            })
    }

    fn symlink(&mut self, path: &Path, target: &Path) -> RemoteResult<()> {
        self.check_connection()?;
        let path = path_utils::absolutize(self.wrkdir.as_path(), path);
        // Check if already exists
        debug!(
            "Creating symlink at {} pointing to {}",
            path.display(),
            target.display()
        );
        if !self.exists(target)? {
            error!("target {} doesn't exist", target.display());
            return Err(RemoteError::new(RemoteErrorType::NoSuchFileOrDirectory));
        }
        self.sftp
            .as_ref()
            .unwrap()
            .symlink(target, path.as_path())
            .map_err(|e| {
                error!("Symlink failed: {e}");
                RemoteError::new_ex(RemoteErrorType::FileCreateDenied, e)
            })
    }

    fn copy(&mut self, src: &Path, dest: &Path) -> RemoteResult<()> {
        self.check_connection()?;
        let src = path_utils::absolutize(self.wrkdir.as_path(), src);
        // check if file exists
        if !self.exists(src.as_path()).ok().unwrap_or(false) {
            return Err(RemoteError::new(RemoteErrorType::NoSuchFileOrDirectory));
        }
        let dest = path_utils::absolutize(self.wrkdir.as_path(), dest);
        debug!("Copying {} to {}", src.display(), dest.display());
        // Run `cp -rf`
        match self
            .session
            .as_mut()
            .unwrap()
            .cmd(format!("cp -rf \"{}\" \"{}\"", src.display(), dest.display()).as_str())
        {
            Ok((0, _)) => Ok(()),
            Ok(_) => Err(RemoteError::new_ex(
                // Could not copy file
                RemoteErrorType::FileCreateDenied,
                format!("\"{}\"", dest.display()),
            )),
            Err(err) => Err(RemoteError::new_ex(RemoteErrorType::ProtocolError, err)),
        }
    }

    fn mov(&mut self, src: &Path, dest: &Path) -> RemoteResult<()> {
        self.check_connection()?;
        let src = path_utils::absolutize(self.wrkdir.as_path(), src);
        // check if file exists
        if !self.exists(src.as_path()).ok().unwrap_or(false) {
            return Err(RemoteError::new(RemoteErrorType::NoSuchFileOrDirectory));
        }
        let dest = path_utils::absolutize(self.wrkdir.as_path(), dest);
        debug!("Moving {} to {}", src.display(), dest.display());
        self.sftp
            .as_ref()
            .unwrap()
            .rename(src.as_path(), dest.as_path())
            .map_err(|e| {
                error!("Move failed: {e}",);
                RemoteError::new_ex(RemoteErrorType::FileCreateDenied, e)
            })
    }

    fn exec(&mut self, cmd: &str) -> RemoteResult<(u32, String)> {
        self.check_connection()?;
        debug!(r#"Executing command "{cmd}""#);

        self.session
            .as_mut()
            .unwrap()
            .cmd_at(cmd, self.wrkdir.as_path())
    }

    fn append(&mut self, path: &Path, metadata: &Metadata) -> RemoteResult<WriteStream> {
        if let Some(sftp) = self.sftp.as_ref() {
            let path = path_utils::absolutize(self.wrkdir.as_path(), path);
            debug!("Opening file at {} for appending", path.display());
            let mode = metadata.mode.map(|x| u32::from(x) as i32).unwrap_or(0o644);
            sftp.open_write(path.as_path(), WriteMode::Append, mode)
                .map_err(|e| {
                    error!("Append failed: {e}",);
                    RemoteError::new_ex(RemoteErrorType::CouldNotOpenFile, e)
                })
        } else {
            Err(RemoteError::new(RemoteErrorType::NotConnected))
        }
    }

    fn create(&mut self, path: &Path, metadata: &Metadata) -> RemoteResult<WriteStream> {
        if let Some(sftp) = self.sftp.as_ref() {
            let path = path_utils::absolutize(self.wrkdir.as_path(), path);
            debug!("Creating file at {}", path.display());
            let mode = metadata.mode.map(|x| u32::from(x) as i32).unwrap_or(0o644);
            sftp.open_write(path.as_path(), WriteMode::Truncate, mode)
                .map_err(|e| {
                    error!("Create failed: {e}",);
                    RemoteError::new_ex(RemoteErrorType::FileCreateDenied, e)
                })
        } else {
            Err(RemoteError::new(RemoteErrorType::NotConnected))
        }
    }

    fn open(&mut self, path: &Path) -> RemoteResult<ReadStream> {
        self.check_connection()?;
        let path = path_utils::absolutize(self.wrkdir.as_path(), path);
        // check if file exists
        if !self.exists(path.as_path()).ok().unwrap_or(false) {
            return Err(RemoteError::new(RemoteErrorType::NoSuchFileOrDirectory));
        }
        debug!("Opening file at {}", path.display());
        self.sftp
            .as_ref()
            .unwrap()
            .open_read(path.as_path())
            .map_err(|e| {
                error!("Open failed: {e}",);
                RemoteError::new_ex(RemoteErrorType::CouldNotOpenFile, e)
            })
    }

    // -- override (std::io::copy is VERY slow on SFTP <https://github.com/remotefs-rs/remotefs-rs/issues/6>)

    fn append_file(
        &mut self,
        path: &Path,
        metadata: &Metadata,
        mut reader: Box<dyn Read + Send>,
    ) -> RemoteResult<u64> {
        if self.is_connected() {
            let mut stream = self.append(path, metadata)?;
            trace!("Opened remote file");
            let mut bytes: usize = 0;
            let transfer_size = metadata.size as usize;
            let mut buffer: [u8; 65535] = [0; 65535];
            while bytes < transfer_size {
                let bytes_read = reader.read(&mut buffer).map_err(|e| {
                    error!("Failed to read from file: {e}",);
                    RemoteError::new_ex(RemoteErrorType::IoError, e)
                })?;
                let mut delta = 0;
                while delta < bytes_read {
                    delta += stream.write(&buffer[delta..bytes_read]).map_err(|e| {
                        error!("Failed to write to stream: {e}",);
                        RemoteError::new_ex(RemoteErrorType::IoError, e)
                    })?;
                }
                bytes += bytes_read;
            }
            self.on_written(stream)?;
            trace!("Written {bytes} bytes to destination",);
            Ok(bytes as u64)
        } else {
            Err(RemoteError::new(RemoteErrorType::NotConnected))
        }
    }

    fn create_file(
        &mut self,
        path: &Path,
        metadata: &Metadata,
        mut reader: Box<dyn std::io::Read + Send>,
    ) -> RemoteResult<u64> {
        if self.is_connected() {
            let mut stream = self.create(path, metadata)?;
            trace!("Opened remote file");
            let mut bytes: usize = 0;
            let transfer_size = metadata.size as usize;
            let mut buffer: [u8; 65535] = [0; 65535];
            while bytes < transfer_size {
                let bytes_read = reader.read(&mut buffer).map_err(|e| {
                    error!("Failed to read from file: {e}",);
                    RemoteError::new_ex(RemoteErrorType::IoError, e)
                })?;
                let mut delta = 0;
                while delta < bytes_read {
                    delta += stream.write(&buffer[delta..bytes_read]).map_err(|e| {
                        error!("Failed to write to stream: {e}",);
                        RemoteError::new_ex(RemoteErrorType::IoError, e)
                    })?;
                }
                bytes += bytes_read;
            }
            stream.flush().map_err(|e| {
                error!("Failed to flush stream: {e}");
                RemoteError::new_ex(RemoteErrorType::IoError, e)
            })?;
            self.on_written(stream)?;
            trace!("Written {bytes} bytes to destination",);
            Ok(bytes as u64)
        } else {
            Err(RemoteError::new(RemoteErrorType::NotConnected))
        }
    }

    fn open_file(&mut self, src: &Path, mut dest: Box<dyn Write + Send>) -> RemoteResult<u64> {
        if self.is_connected() {
            let transfer_size = self.stat(src)?.metadata().size as usize;
            let mut stream = self.open(src)?;
            trace!("File opened");
            let mut bytes: usize = 0;
            let mut buffer: [u8; 65535] = [0; 65535];
            while bytes < transfer_size {
                let bytes_read = stream.read(&mut buffer).map_err(|e| {
                    error!("Failed to read from stream: {e}");
                    RemoteError::new_ex(RemoteErrorType::IoError, e)
                })?;
                let mut delta = 0;
                while delta < bytes_read {
                    delta += dest.write(&buffer[delta..bytes_read]).map_err(|e| {
                        error!("Failed to write to file: {e}",);
                        RemoteError::new_ex(RemoteErrorType::IoError, e)
                    })?;
                }
                bytes += bytes_read;
            }
            self.on_read(stream)?;
            trace!("Copied {bytes} bytes to destination",);
            Ok(bytes as u64)
        } else {
            Err(RemoteError::new(RemoteErrorType::NotConnected))
        }
    }
}

#[cfg(test)]
mod tests;
