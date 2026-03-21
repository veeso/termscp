use std::fs::File;
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};

use tempfile::NamedTempFile;

use crate::host::{HostError, HostErrorType, HostResult};

/// A temporary file mapped to a remote file which has been transferred to local
/// and which supports read/write operations
#[derive(Debug, Clone)]
pub struct TempMappedFile {
    tempfile: Arc<NamedTempFile>,
    handle: Arc<Mutex<Option<File>>>,
}

impl Write for TempMappedFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut handle = self.write_hnd()?;
        handle.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut handle = self.write_hnd()?;
        handle.flush()
    }
}

impl Read for TempMappedFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut handle = self.read_hnd()?;
        handle.read(buf)
    }
}

impl TempMappedFile {
    pub fn new() -> HostResult<Self> {
        NamedTempFile::new()
            .map(|tempfile| TempMappedFile {
                tempfile: Arc::new(tempfile),
                handle: Arc::new(Mutex::new(None)),
            })
            .map_err(|e| {
                HostError::new(
                    HostErrorType::CouldNotCreateFile,
                    Some(e),
                    std::path::Path::new(""),
                )
            })
    }

    /// Syncs the file to disk and frees the file handle.
    ///
    /// Must be called
    pub fn sync(&mut self) -> HostResult<()> {
        {
            let mut lock = self.lock_handle().map_err(|e| {
                HostError::new(
                    HostErrorType::FileNotAccessible,
                    Some(e),
                    self.tempfile.path(),
                )
            })?;

            if let Some(hnd) = lock.take() {
                hnd.sync_all().map_err(|e| {
                    HostError::new(
                        HostErrorType::FileNotAccessible,
                        Some(e),
                        self.tempfile.path(),
                    )
                })?;
            }
        }

        Ok(())
    }

    fn write_hnd(&mut self) -> io::Result<FileHandle<'_>> {
        let mut lock = self.lock_handle()?;
        if lock.is_none() {
            let hnd = File::create(self.tempfile.path())?;
            lock.replace(hnd);
        }

        Ok(FileHandle::new(lock))
    }

    fn read_hnd(&mut self) -> io::Result<FileHandle<'_>> {
        let mut lock = self.lock_handle()?;
        if lock.is_none() {
            let hnd = File::open(self.tempfile.path())?;
            lock.replace(hnd);
        }

        Ok(FileHandle::new(lock))
    }

    fn lock_handle(&self) -> io::Result<std::sync::MutexGuard<'_, Option<File>>> {
        self.handle
            .lock()
            .map_err(|_| io::Error::other("temporary file handle lock poisoned"))
    }
}

struct FileHandle<'a> {
    guard: std::sync::MutexGuard<'a, Option<File>>,
}

impl<'a> FileHandle<'a> {
    fn new(guard: std::sync::MutexGuard<'a, Option<File>>) -> Self {
        Self { guard }
    }

    fn file_mut(&mut self) -> io::Result<&mut File> {
        self.guard
            .as_mut()
            .ok_or_else(|| io::Error::other("temporary file handle is not initialized"))
    }
}

impl Write for FileHandle<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file_mut()?.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file_mut()?.flush()
    }
}

impl Read for FileHandle<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file_mut()?.read(buf)
    }
}

#[cfg(test)]
mod test {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_should_write_and_read_file() {
        let mut file = TempMappedFile::new().unwrap();
        file.write_all(b"Hello, World!").unwrap();

        file.sync().unwrap();

        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();

        assert_eq!(buf, b"Hello, World!");
    }
}
