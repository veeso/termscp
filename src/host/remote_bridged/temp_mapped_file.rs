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
        let rc = self.write_hnd()?;
        let mut ref_mut = rc.lock().unwrap();
        ref_mut.as_mut().unwrap().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let rc = self.write_hnd()?;
        let mut ref_mut = rc.lock().unwrap();
        ref_mut.as_mut().unwrap().flush()
    }
}

impl Read for TempMappedFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let rc = self.read_hnd()?;
        let mut ref_mut = rc.lock().unwrap();
        ref_mut.as_mut().unwrap().read(buf)
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
            let mut lock = self.handle.lock().unwrap();

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

    fn write_hnd(&mut self) -> io::Result<Arc<Mutex<Option<File>>>> {
        {
            let mut lock = self.handle.lock().unwrap();
            if lock.is_none() {
                let hnd = File::create(self.tempfile.path())?;
                lock.replace(hnd);
            }
        }

        Ok(self.handle.clone())
    }

    fn read_hnd(&mut self) -> io::Result<Arc<Mutex<Option<File>>>> {
        {
            let mut lock = self.handle.lock().unwrap();
            if lock.is_none() {
                let hnd = File::open(self.tempfile.path())?;
                lock.replace(hnd);
            }
        }

        Ok(self.handle.clone())
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
