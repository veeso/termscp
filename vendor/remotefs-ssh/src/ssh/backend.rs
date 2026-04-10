//! Defines the main trait for SSH Backends to be used with the clients and the backend implementations
//! to support different SSH libraries (e.g. libssh2, libssh)

#[cfg(feature = "libssh")]
#[cfg_attr(docsrs, doc(cfg(feature = "libssh")))]
mod libssh;

#[cfg(feature = "libssh2")]
#[cfg_attr(docsrs, doc(cfg(feature = "libssh2")))]
mod libssh2;

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use remotefs::fs::{Metadata, ReadStream, WriteStream};
use remotefs::{File, RemoteResult};

#[cfg(feature = "libssh")]
#[cfg_attr(docsrs, doc(cfg(feature = "libssh")))]
pub use self::libssh::LibSshSession;
#[cfg(feature = "libssh2")]
#[cfg_attr(docsrs, doc(cfg(feature = "libssh2")))]
pub use self::libssh2::LibSsh2Session;
use crate::SshOpts;

/// SSH session trait.
///
/// Provides SSH channel functions
pub trait SshSession: Sized {
    type Sftp: Sftp;

    /// Connects to the SSH server and establishes a new [`SshSession`]
    fn connect(opts: &SshOpts) -> RemoteResult<Self>;

    /// Disconnect from the server
    fn disconnect(&self) -> RemoteResult<()>;

    /// Get the SSH server banner.
    fn banner(&self) -> RemoteResult<Option<String>>;

    /// Check if the session is authenticated.
    fn authenticated(&self) -> RemoteResult<bool>;

    /// Executes a command on the SSH server and returns the exit code and the output.
    fn cmd<S>(&mut self, cmd: S) -> RemoteResult<(u32, String)>
    where
        S: AsRef<str>;

    /// Executes a command on the SSH server at a specific path and returns the exit code and the output.
    fn cmd_at<S>(&mut self, cmd: S, path: &Path) -> RemoteResult<(u32, String)>
    where
        S: AsRef<str>,
    {
        self.cmd(format!("cd \"{}\"; {}", path.display(), cmd.as_ref()))
    }

    /// Receives a file over SCP.
    ///
    /// Returns a channel can be read from server.
    fn scp_recv(&self, path: &Path) -> RemoteResult<Box<dyn Read + Send>>;

    /// Send a file over SCP.
    ///
    /// Returns a channel which can be written to send data
    fn scp_send(
        &self,
        remote_path: &Path,
        mode: i32,
        size: u64,
        times: Option<(u64, u64)>,
    ) -> RemoteResult<Box<dyn Write + Send>>;

    /// Returns a SFTP client
    fn sftp(&self) -> RemoteResult<Self::Sftp>;
}

/// Sftp provider for a [`SshSession`] implementation via the [`SshSession::sftp`] method.
pub trait Sftp {
    /// Creates a new directory at the specified `path` with the given `mode`.
    fn mkdir(&self, path: &Path, mode: i32) -> RemoteResult<()>;

    /// Opens a file for reading at the specified `path`.
    fn open_read(&self, path: &Path) -> RemoteResult<ReadStream>;

    /// Open a file for write at the specified `path` with the given `flags`. If the file is created, set the mode.
    fn open_write(&self, path: &Path, flags: WriteMode, mode: i32) -> RemoteResult<WriteStream>;

    /// Lists the contents of a directory at `dirname` and returns the listed [`File`] for it.
    fn readdir<T>(&self, dirname: T) -> RemoteResult<Vec<File>>
    where
        T: AsRef<Path>;

    /// Resolve the real path for `path`.
    #[allow(dead_code)]
    fn realpath(&self, path: &Path) -> RemoteResult<PathBuf>;

    /// Renames a file from `src` to `dest`.
    fn rename(&self, src: &Path, dest: &Path) -> RemoteResult<()>;

    /// Removes a directory at `path`.
    fn rmdir(&self, path: &Path) -> RemoteResult<()>;

    /// Set the [`Metadata`] for a file at `path`.
    fn setstat(&self, path: &Path, metadata: Metadata) -> RemoteResult<()>;

    /// Get the [`File`] metadata for a file.
    fn stat(&self, filename: &Path) -> RemoteResult<File>;

    /// Creates a symlink at `path` pointing to `target`.
    fn symlink(&self, path: &Path, target: &Path) -> RemoteResult<()>;

    /// Deletes a file at `path`.
    fn unlink(&self, path: &Path) -> RemoteResult<()>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Open modes for reading and writing files.
pub enum WriteMode {
    Append,
    Truncate,
}
