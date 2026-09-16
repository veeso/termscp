//! Runtime-owning wrapper for asynchronous remote filesystem adapters.

use std::io::{Read, Write};
use std::path::Path;

use remotefs::fs::{
    Capabilities, ExecOutput, ReadOptions, ReadStream, SetMetadata, UnixPex, WriteOptions,
    WriteStream,
};
use remotefs::{File, RemoteFs, RemoteResult};
use tokio::runtime::Runtime;

pub(super) struct RuntimeRemoteFs {
    // Fields are dropped in declaration order, so the client cannot outlive its runtime.
    remote: Box<dyn RemoteFs>,
    _runtime: Runtime,
}

impl RuntimeRemoteFs {
    pub(super) fn new(remote: impl RemoteFs + 'static, runtime: Runtime) -> Self {
        Self {
            remote: Box::new(remote),
            _runtime: runtime,
        }
    }
}

impl RemoteFs for RuntimeRemoteFs {
    fn connect(&mut self) -> RemoteResult<()> {
        self.remote.connect()
    }

    fn disconnect(&mut self) -> RemoteResult<()> {
        self.remote.disconnect()
    }

    fn is_connected(&self) -> bool {
        self.remote.is_connected()
    }

    fn capabilities(&self) -> Capabilities {
        self.remote.capabilities()
    }

    fn list_dir(&self, path: &Path) -> RemoteResult<Vec<File>> {
        self.remote.list_dir(path)
    }

    fn stat(&self, path: &Path) -> RemoteResult<File> {
        self.remote.stat(path)
    }

    fn exists(&self, path: &Path) -> RemoteResult<bool> {
        self.remote.exists(path)
    }

    fn set_metadata(&self, path: &Path, metadata: &SetMetadata) -> RemoteResult<()> {
        self.remote.set_metadata(path, metadata)
    }

    fn create_dir(&self, path: &Path, mode: Option<UnixPex>) -> RemoteResult<()> {
        self.remote.create_dir(path, mode)
    }

    fn remove_file(&self, path: &Path) -> RemoteResult<()> {
        self.remote.remove_file(path)
    }

    fn remove_dir(&self, path: &Path) -> RemoteResult<()> {
        self.remote.remove_dir(path)
    }

    fn remove_dir_all(&self, path: &Path) -> RemoteResult<()> {
        self.remote.remove_dir_all(path)
    }

    fn rename(&self, src: &Path, dest: &Path) -> RemoteResult<()> {
        self.remote.rename(src, dest)
    }

    fn copy(&self, src: &Path, dest: &Path) -> RemoteResult<()> {
        self.remote.copy(src, dest)
    }

    fn symlink(&self, path: &Path, target: &Path) -> RemoteResult<()> {
        self.remote.symlink(path, target)
    }

    fn open(&self, path: &Path, opts: &ReadOptions) -> RemoteResult<ReadStream> {
        self.remote.open(path, opts)
    }

    fn create(&self, path: &Path, opts: &WriteOptions) -> RemoteResult<WriteStream> {
        self.remote.create(path, opts)
    }

    fn append(&self, path: &Path, opts: &WriteOptions) -> RemoteResult<WriteStream> {
        self.remote.append(path, opts)
    }

    fn read_file(
        &self,
        path: &Path,
        opts: &ReadOptions,
        dest: &mut (dyn Write + Send),
    ) -> RemoteResult<u64> {
        self.remote.read_file(path, opts, dest)
    }

    fn write_file(
        &self,
        path: &Path,
        opts: &WriteOptions,
        src: &mut (dyn Read + Send),
    ) -> RemoteResult<u64> {
        self.remote.write_file(path, opts, src)
    }

    fn append_file(
        &self,
        path: &Path,
        opts: &WriteOptions,
        src: &mut (dyn Read + Send),
    ) -> RemoteResult<u64> {
        self.remote.append_file(path, opts, src)
    }

    fn exec(&self, cmd: &str) -> RemoteResult<ExecOutput> {
        self.remote.exec(cmd)
    }
}

#[cfg(test)]
mod tests {
    use remotefs_ftp::FtpFs;

    use super::*;

    #[test]
    fn runtime_remote_fs_keeps_runtime_alive() {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();
        let handle = runtime.handle().clone();
        let remote = RuntimeRemoteFs::new(FtpFs::new("127.0.0.1", 21), runtime);

        let task = handle.spawn(async { 42 });

        assert_eq!(handle.block_on(task).unwrap(), 42);
        drop(remote);
    }
}
