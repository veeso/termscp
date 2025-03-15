//! ## Host
//!
//! `host` is the module which provides functionalities to host file system

mod bridge;
mod localhost;
mod remote_bridged;

use std::path::{Path, PathBuf};

use thiserror::Error;

// Locals
pub use self::bridge::HostBridge;
pub use self::localhost::Localhost;
pub use self::remote_bridged::RemoteBridged;

pub type HostResult<T> = Result<T, HostError>;

/// HostErrorType provides an overview of the specific host error
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum HostErrorType {
    #[error("No such file or directory")]
    NoSuchFileOrDirectory,
    #[error("File is readonly")]
    ReadonlyFile,
    #[error("Could not access directory")]
    DirNotAccessible,
    #[error("Could not access file")]
    FileNotAccessible,
    #[error("File already exists")]
    FileAlreadyExists,
    #[error("Could not create file")]
    CouldNotCreateFile,
    #[error("Command execution failed")]
    ExecutionFailed,
    #[error("Could not delete file")]
    DeleteFailed,
    #[error("Not implemented")]
    NotImplemented,
    #[error("remote fs error: {0}")]
    RemoteFs(#[from] remotefs::RemoteError),
}

/// HostError is a wrapper for the error type and the exact io error
#[derive(Debug, Error)]
pub struct HostError {
    pub error: HostErrorType,
    ioerr: Option<std::io::Error>,
    path: Option<PathBuf>,
}

impl From<remotefs::RemoteError> for HostError {
    fn from(value: remotefs::RemoteError) -> Self {
        HostError::from(HostErrorType::RemoteFs(value))
    }
}

impl HostError {
    /// Instantiates a new HostError
    pub(crate) fn new(error: HostErrorType, errno: Option<std::io::Error>, p: &Path) -> Self {
        HostError {
            error,
            ioerr: errno,
            path: Some(p.to_path_buf()),
        }
    }
}

impl From<HostErrorType> for HostError {
    fn from(error: HostErrorType) -> Self {
        HostError {
            error,
            ioerr: None,
            path: None,
        }
    }
}

impl std::fmt::Display for HostError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let p_str: String = match self.path.as_ref() {
            None => String::new(),
            Some(p) => format!(" ({})", p.display()),
        };
        match &self.ioerr {
            Some(err) => write!(f, "{}: {}{}", self.error, err, p_str),
            None => write!(f, "{}{}", self.error, p_str),
        }
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

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
