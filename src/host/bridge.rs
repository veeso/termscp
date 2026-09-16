//! ## Host Bridge
//!
//! Defines the host abstraction used to expose localhost and bridged remote
//! filesystems through a shared interface.

use std::fmt;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use remotefs::File;
use remotefs::fs::{Metadata, UnixPex};

use super::HostResult;

enum HostReaderInner {
    Io(Box<dyn Read + Send>),
    Remote(remotefs::fs::ReadStream),
}

/// An owned host reader that optionally retains a remote transfer finalizer.
pub struct HostReader(HostReaderInner);

impl HostReader {
    pub(crate) fn io<T>(reader: T) -> Self
    where
        T: Read + Send + 'static,
    {
        Self(HostReaderInner::Io(Box::new(reader)))
    }

    pub(crate) fn remote(reader: remotefs::fs::ReadStream) -> Self {
        Self(HostReaderInner::Remote(reader))
    }

    /// Completes the remote read and consumes this reader.
    ///
    /// # Errors
    ///
    /// Returns the remote stream finalization error, if the reader is backed
    /// by a remote stream.
    pub fn finish(self) -> super::HostResult<()> {
        match self.0 {
            HostReaderInner::Io(_) => Ok(()),
            HostReaderInner::Remote(reader) => reader.finish().map_err(Into::into),
        }
    }
}

impl fmt::Debug for HostReader {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("HostReader").finish_non_exhaustive()
    }
}

impl Read for HostReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        match &mut self.0 {
            HostReaderInner::Io(reader) => reader.read(buffer),
            HostReaderInner::Remote(reader) => reader.read(buffer),
        }
    }
}

enum HostWriterInner {
    Io(Box<dyn Write + Send>),
    Remote(remotefs::fs::WriteStream),
}

/// An owned host writer that optionally retains a remote transfer finalizer.
pub struct HostWriter(HostWriterInner);

impl HostWriter {
    pub(crate) fn io<T>(writer: T) -> Self
    where
        T: Write + Send + 'static,
    {
        Self(HostWriterInner::Io(Box::new(writer)))
    }

    pub(crate) fn remote(writer: remotefs::fs::WriteStream) -> Self {
        Self(HostWriterInner::Remote(writer))
    }

    /// Completes the remote write and consumes this writer.
    ///
    /// # Errors
    ///
    /// Returns the remote stream finalization error, if the writer is backed
    /// by a remote stream.
    pub fn finish(self) -> super::HostResult<()> {
        match self.0 {
            HostWriterInner::Io(_) => Ok(()),
            HostWriterInner::Remote(writer) => writer.finish().map_err(Into::into),
        }
    }
}

impl fmt::Debug for HostWriter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("HostWriter").finish_non_exhaustive()
    }
}

impl Write for HostWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        match &mut self.0 {
            HostWriterInner::Io(writer) => writer.write(buffer),
            HostWriterInner::Remote(writer) => writer.write(buffer),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match &mut self.0 {
            HostWriterInner::Io(writer) => writer.flush(),
            HostWriterInner::Remote(writer) => writer.flush(),
        }
    }
}

/// Trait to bridge a remote filesystem to the host filesystem
///
/// In case of `Localhost` this should be effortless, while for remote hosts this should
/// implement a real bridge when the resource is first loaded on the local
///  filesystem and then processed on the remote.
pub trait HostBridge {
    /// Connect to host
    fn connect(&mut self) -> HostResult<()>;

    /// Disconnect from host
    fn disconnect(&mut self) -> HostResult<()>;

    /// Returns whether the host is connected
    fn is_connected(&mut self) -> bool;

    /// Returns whether the host is localhost
    fn is_localhost(&self) -> bool;

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
    fn open_file(&mut self, file: &Path) -> HostResult<HostReader>;

    /// Open file for writing
    fn create_file(&mut self, file: &Path, metadata: &Metadata) -> HostResult<HostWriter>;

    /// Finalize write operation
    fn finalize_write(&mut self, writer: HostWriter) -> HostResult<()>;
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use remotefs::fs::{ReadStream, RemoteRead, RemoteWrite, WriteStream};

    use super::*;

    struct TrackedReader {
        reader: Cursor<Vec<u8>>,
        finishes: Arc<AtomicUsize>,
    }

    impl Read for TrackedReader {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            self.reader.read(buffer)
        }
    }

    impl RemoteRead for TrackedReader {
        fn finish(self: Box<Self>) -> remotefs::RemoteResult<()> {
            self.finishes.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    struct TrackedWriter {
        writer: Cursor<Vec<u8>>,
        finishes: Arc<AtomicUsize>,
    }

    impl Write for TrackedWriter {
        fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
            self.writer.write(buffer)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.writer.flush()
        }
    }

    impl RemoteWrite for TrackedWriter {
        fn finish(self: Box<Self>) -> remotefs::RemoteResult<()> {
            self.finishes.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[test]
    fn remote_reader_finishes_exactly_once_when_consumed() {
        let finishes = Arc::new(AtomicUsize::new(0));
        let reader = HostReader::remote(ReadStream::new(TrackedReader {
            reader: Cursor::new(Vec::new()),
            finishes: finishes.clone(),
        }));

        reader.finish().unwrap();

        assert_eq!(finishes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn remote_writer_finishes_exactly_once_when_consumed() {
        let finishes = Arc::new(AtomicUsize::new(0));
        let writer = HostWriter::remote(WriteStream::new(TrackedWriter {
            writer: Cursor::new(Vec::new()),
            finishes: finishes.clone(),
        }));

        writer.finish().unwrap();

        assert_eq!(finishes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn unfinished_remote_reader_is_dropped_without_finalizing() {
        let finishes = Arc::new(AtomicUsize::new(0));
        let reader = HostReader::remote(ReadStream::new(TrackedReader {
            reader: Cursor::new(Vec::new()),
            finishes: finishes.clone(),
        }));

        drop(reader);

        assert_eq!(finishes.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn unfinished_remote_writer_is_dropped_without_finalizing() {
        let finishes = Arc::new(AtomicUsize::new(0));
        let writer = HostWriter::remote(WriteStream::new(TrackedWriter {
            writer: Cursor::new(Vec::new()),
            finishes: finishes.clone(),
        }));

        drop(writer);

        assert_eq!(finishes.load(Ordering::SeqCst), 0);
    }
}
