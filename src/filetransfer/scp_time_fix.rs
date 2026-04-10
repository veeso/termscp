use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Local, LocalResult, TimeZone, Utc};
use remotefs::fs::{File, Metadata, ReadStream, RemoteFs, RemoteResult, UnixPex, Welcome, WriteStream};

/// Wrap SCP clients and reinterpret timestamps returned by the dependency bug.
///
/// The upstream SCP parser reads `ls -l` wall-clock timestamps and stores them as
/// if they were UTC instants. We recover the original wall-clock fields from the
/// wrong UTC timestamp and then reinterpret them in local time.
pub struct ScpTimeFix<F> {
    inner: F,
}

impl<F> ScpTimeFix<F> {
    pub fn new(inner: F) -> Self {
        Self { inner }
    }

    fn fix_time(time: SystemTime) -> SystemTime {
        let naive = DateTime::<Utc>::from(time).naive_utc();
        let local = match Local.from_local_datetime(&naive) {
            LocalResult::Single(dt) => dt,
            LocalResult::Ambiguous(dt, _) => dt,
            LocalResult::None => Utc.from_utc_datetime(&naive).with_timezone(&Local),
        };
        local.into()
    }

    fn fix_metadata(metadata: &mut Metadata) {
        if let Some(modified) = metadata.modified {
            metadata.modified = Some(Self::fix_time(modified));
        }
    }

    fn fix_file(mut file: File) -> File {
        Self::fix_metadata(&mut file.metadata);
        file
    }
}

impl<F> RemoteFs for ScpTimeFix<F>
where
    F: RemoteFs,
{
    fn connect(&mut self) -> RemoteResult<Welcome> {
        self.inner.connect()
    }

    fn disconnect(&mut self) -> RemoteResult<()> {
        self.inner.disconnect()
    }

    fn is_connected(&mut self) -> bool {
        self.inner.is_connected()
    }

    fn pwd(&mut self) -> RemoteResult<PathBuf> {
        self.inner.pwd()
    }

    fn change_dir(&mut self, dir: &Path) -> RemoteResult<PathBuf> {
        self.inner.change_dir(dir)
    }

    fn list_dir(&mut self, path: &Path) -> RemoteResult<Vec<File>> {
        self.inner
            .list_dir(path)
            .map(|entries| entries.into_iter().map(Self::fix_file).collect())
    }

    fn stat(&mut self, path: &Path) -> RemoteResult<File> {
        self.inner.stat(path).map(Self::fix_file)
    }

    fn setstat(&mut self, path: &Path, metadata: Metadata) -> RemoteResult<()> {
        self.inner.setstat(path, metadata)
    }

    fn exists(&mut self, path: &Path) -> RemoteResult<bool> {
        self.inner.exists(path)
    }

    fn remove_file(&mut self, path: &Path) -> RemoteResult<()> {
        self.inner.remove_file(path)
    }

    fn remove_dir(&mut self, path: &Path) -> RemoteResult<()> {
        self.inner.remove_dir(path)
    }

    fn remove_dir_all(&mut self, path: &Path) -> RemoteResult<()> {
        self.inner.remove_dir_all(path)
    }

    fn create_dir(&mut self, path: &Path, mode: UnixPex) -> RemoteResult<()> {
        self.inner.create_dir(path, mode)
    }

    fn symlink(&mut self, path: &Path, target: &Path) -> RemoteResult<()> {
        self.inner.symlink(path, target)
    }

    fn copy(&mut self, src: &Path, dest: &Path) -> RemoteResult<()> {
        self.inner.copy(src, dest)
    }

    fn mov(&mut self, src: &Path, dest: &Path) -> RemoteResult<()> {
        self.inner.mov(src, dest)
    }

    fn exec(&mut self, cmd: &str) -> RemoteResult<(u32, String)> {
        self.inner.exec(cmd)
    }

    fn append(&mut self, path: &Path, metadata: &Metadata) -> RemoteResult<WriteStream> {
        self.inner.append(path, metadata)
    }

    fn create(&mut self, path: &Path, metadata: &Metadata) -> RemoteResult<WriteStream> {
        self.inner.create(path, metadata)
    }

    fn open(&mut self, path: &Path) -> RemoteResult<ReadStream> {
        self.inner.open(path)
    }

    fn on_written(&mut self, writable: WriteStream) -> RemoteResult<()> {
        self.inner.on_written(writable)
    }

    fn on_read(&mut self, readable: ReadStream) -> RemoteResult<()> {
        self.inner.on_read(readable)
    }

    fn append_file(
        &mut self,
        path: &Path,
        metadata: &Metadata,
        reader: Box<dyn Read + Send>,
    ) -> RemoteResult<u64> {
        self.inner.append_file(path, metadata, reader)
    }

    fn create_file(
        &mut self,
        path: &Path,
        metadata: &Metadata,
        reader: Box<dyn Read + Send>,
    ) -> RemoteResult<u64> {
        self.inner.create_file(path, metadata, reader)
    }

    fn open_file(&mut self, src: &Path, dest: Box<dyn Write + Send>) -> RemoteResult<u64> {
        self.inner.open_file(src, dest)
    }

    #[cfg(feature = "find")]
    fn find(&mut self, search: &str) -> RemoteResult<Vec<File>> {
        self.inner
            .find(search)
            .map(|entries| entries.into_iter().map(Self::fix_file).collect())
    }
}
