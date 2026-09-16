//! ## Remote Bridged Host
//!
//! Bridges a `RemoteFs` implementation behind the local host interface used by
//! the file transfer activity.

mod temp_mapped_file;

use std::io::Write;
use std::path::{Component, Path, PathBuf};

use remotefs::fs::{Capabilities, Metadata, ReadOptions, SetMetadata, UnixPex, WriteOptions};
use remotefs::{File, RemoteError, RemoteErrorType, RemoteFs};

use self::temp_mapped_file::TempMappedFile;
use super::{HostBridge, HostError, HostErrorType, HostReader, HostResult, HostWriter};
use crate::utils::path::normalize;

struct WriteStreamOp {
    path: PathBuf,
    options: WriteOptions,
    tempfile: TempMappedFile,
}

/// A remote host bridged over the local host
pub struct RemoteBridged {
    /// Remote fs client
    remote: Box<dyn RemoteFs>,
    /// Consumer-owned remote working directory used to resolve relative paths
    wrkdir: PathBuf,
    /// Reminder used to finalize write stream
    write_stream_op: Option<WriteStreamOp>,
}

impl RemoteBridged {
    fn open_file_from_temp(&mut self, file: &Path) -> HostResult<HostReader> {
        let mut temp_file = TempMappedFile::new()?;

        self.remote
            .read_file(file, &ReadOptions::default(), &mut temp_file)
            .map_err(HostError::from)?;

        // Sync changes
        temp_file.sync()?;

        // now return as read
        Ok(HostReader::io(temp_file))
    }
}

fn resolve_remote_path(wrkdir: &Path, target: &Path) -> HostResult<PathBuf> {
    let resolved = if remotefs::path::ensure_absolute(target).is_ok() {
        target.to_path_buf()
    } else {
        wrkdir.join(target)
    };
    remotefs::path::ensure_absolute(&resolved).map_err(HostError::from)?;
    Ok(resolved)
}

fn write_options(metadata: &Metadata) -> WriteOptions {
    let mut options = WriteOptions::default();
    if let Some(size) = metadata.size {
        options = options.size_hint(size);
    }
    if let Some(mode) = metadata.mode {
        options = options.mode(mode);
    }
    if let Some(modified) = metadata.modified {
        options = options.modified(modified);
    }
    options
}

fn set_metadata_options(metadata: &Metadata) -> SetMetadata {
    let mut options = SetMetadata::default();
    if let Some(mode) = metadata.mode {
        options = options.mode(mode);
    }
    if let Some(uid) = metadata.uid {
        options = options.uid(uid);
    }
    if let Some(gid) = metadata.gid {
        options = options.gid(gid);
    }
    if let Some(accessed) = metadata.accessed {
        options = options.accessed(accessed);
    }
    if let Some(modified) = metadata.modified {
        options = options.modified(modified);
    }
    options
}

fn map_io_error(error: std::io::Error) -> HostError {
    HostError::new(HostErrorType::FileNotAccessible, Some(error), Path::new(""))
}

impl From<Box<dyn RemoteFs>> for RemoteBridged {
    fn from(remote: Box<dyn RemoteFs>) -> Self {
        RemoteBridged {
            remote,
            wrkdir: PathBuf::from("/"),
            write_stream_op: None,
        }
    }
}

impl HostBridge for RemoteBridged {
    fn connect(&mut self) -> HostResult<()> {
        self.remote.connect().map_err(HostError::from)
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
        Ok(self.wrkdir.clone())
    }

    fn change_wrkdir(&mut self, new_dir: &Path) -> HostResult<PathBuf> {
        debug!("Changing working directory to {:?}", new_dir);
        let new_dir = resolve_remote_path(&self.wrkdir, new_dir)?;
        let entry = self.remote.stat(&new_dir).map_err(HostError::from)?;
        if !entry.is_dir() {
            return Err(HostError::from(RemoteError::new(RemoteErrorType::BadFile)));
        }
        self.wrkdir = new_dir;
        Ok(self.wrkdir.clone())
    }

    fn mkdir_ex(&mut self, dir_name: &Path, ignore_existing: bool) -> HostResult<()> {
        debug!("Creating directory {:?}", dir_name);
        let path = resolve_remote_path(&self.wrkdir, dir_name)?;
        match self.remote.create_dir(&path, Some(UnixPex::from(0o755))) {
            Ok(()) => Ok(()),
            Err(error) if ignore_existing && error.kind() == RemoteErrorType::AlreadyExists => {
                Ok(())
            }
            Err(error) => Err(HostError::from(error)),
        }
    }

    fn remove(&mut self, entry: &File) -> HostResult<()> {
        debug!("Removing {:?}", entry.path());
        let path = resolve_remote_path(&self.wrkdir, entry.path())?;
        if entry.is_dir() {
            self.remote.remove_dir_all(&path).map_err(HostError::from)
        } else {
            self.remote.remove_file(&path).map_err(HostError::from)
        }
    }

    fn rename(&mut self, entry: &File, dst_path: &Path) -> HostResult<()> {
        debug!("Renaming {:?} to {:?}", entry.path(), dst_path);
        let source = resolve_remote_path(&self.wrkdir, entry.path())?;
        let destination = resolve_remote_path(&self.wrkdir, dst_path)?;
        self.remote
            .rename(&source, &destination)
            .map_err(HostError::from)
    }

    fn copy(&mut self, entry: &File, dst: &Path) -> HostResult<()> {
        debug!("Copying {:?} to {:?}", entry.path(), dst);
        let source = resolve_remote_path(&self.wrkdir, entry.path())?;
        let destination = resolve_remote_path(&self.wrkdir, dst)?;
        self.remote
            .copy(&source, &destination)
            .map_err(HostError::from)
    }

    fn stat(&mut self, path: &Path) -> HostResult<File> {
        debug!("Statting {:?}", path);
        let path = resolve_remote_path(&self.wrkdir, path)?;
        self.remote.stat(&path).map_err(HostError::from)
    }

    fn exists(&mut self, path: &Path) -> HostResult<bool> {
        debug!("Checking existence of {:?}", path);
        let path = resolve_remote_path(&self.wrkdir, path)?;
        self.remote.exists(&path).map_err(HostError::from)
    }

    fn list_dir(&mut self, path: &Path) -> HostResult<Vec<File>> {
        debug!("Listing directory {:?}", path);
        let path = resolve_remote_path(&self.wrkdir, path)?;
        let entries = self.remote.list_dir(&path).map_err(HostError::from)?;
        Ok(filter_self_refs(&path, entries))
    }

    fn setstat(&mut self, path: &Path, metadata: &Metadata) -> HostResult<()> {
        debug!("Setting metadata for {:?}", path);
        let path = resolve_remote_path(&self.wrkdir, path)?;
        let options = set_metadata_options(metadata);
        self.remote
            .set_metadata(&path, &options)
            .map_err(HostError::from)
    }

    fn exec(&mut self, cmd: &str) -> HostResult<String> {
        debug!("Executing command: {}", cmd);
        self.remote
            .exec(cmd)
            .map(|output| output.stdout)
            .map_err(HostError::from)
    }

    fn symlink(&mut self, src: &Path, dst: &Path) -> HostResult<()> {
        debug!("Creating symlink from {:?} to {:?}", src, dst);
        let path = resolve_remote_path(&self.wrkdir, src)?;
        let target = resolve_remote_path(&self.wrkdir, dst)?;
        self.remote.symlink(&path, &target).map_err(HostError::from)
    }

    fn chmod(&mut self, path: &Path, pex: UnixPex) -> HostResult<()> {
        debug!("Changing permissions of {:?} to {:?}", path, pex);
        let path = resolve_remote_path(&self.wrkdir, path)?;
        self.remote
            .set_metadata(&path, &SetMetadata::default().mode(pex))
            .map_err(HostError::from)
    }

    fn open_file(&mut self, file: &Path) -> HostResult<HostReader> {
        let path = resolve_remote_path(&self.wrkdir, file)?;
        if self
            .remote
            .capabilities()
            .contains(Capabilities::STREAM_READ)
        {
            match self.remote.open(&path, &ReadOptions::default()) {
                Ok(stream) => Ok(HostReader::remote(stream)),
                Err(error) if error.kind() == RemoteErrorType::UnsupportedFeature => {
                    self.open_file_from_temp(&path)
                }
                Err(error) => Err(HostError::from(error)),
            }
        } else {
            self.open_file_from_temp(&path)
        }
    }

    fn create_file(&mut self, file: &Path, metadata: &Metadata) -> HostResult<HostWriter> {
        let path = resolve_remote_path(&self.wrkdir, file)?;
        let options = write_options(metadata);
        self.write_stream_op = None;
        if self
            .remote
            .capabilities()
            .contains(Capabilities::STREAM_WRITE)
        {
            match self.remote.create(&path, &options) {
                Ok(stream) => Ok(HostWriter::remote(stream)),
                Err(error)
                    if matches!(
                        error.kind(),
                        RemoteErrorType::SizeRequired | RemoteErrorType::UnsupportedFeature
                    ) =>
                {
                    self.create_file_from_temp(path, options)
                }
                Err(error) => Err(HostError::from(error)),
            }
        } else {
            self.create_file_from_temp(path, options)
        }
    }

    fn finalize_write(&mut self, mut writer: HostWriter) -> HostResult<()> {
        writer.flush().map_err(map_io_error)?;
        writer.finish()?;
        if let Some(WriteStreamOp {
            path,
            mut options,
            mut tempfile,
        }) = self.write_stream_op.take()
        {
            // sync
            tempfile.sync()?;
            options = options.size_hint(tempfile.len()?);
            // write file
            self.remote
                .write_file(&path, &options, &mut tempfile)
                .map_err(HostError::from)?;
        }
        Ok(())
    }
}

impl RemoteBridged {
    fn create_file_from_temp(
        &mut self,
        path: PathBuf,
        options: WriteOptions,
    ) -> HostResult<HostWriter> {
        let tempfile = TempMappedFile::new()?;
        self.write_stream_op = Some(WriteStreamOp {
            path,
            options,
            tempfile: tempfile.clone(),
        });

        Ok(HostWriter::io(tempfile))
    }
}

/// Drop entries that refer to the directory being listed.
///
/// Some non-compliant FTP servers (e.g. LiteSpeed) include a self-reference
/// to the listed directory in the LIST response, which would otherwise appear
/// as a duplicate entry in the explorer.
fn filter_self_refs(path: &Path, entries: Vec<File>) -> Vec<File> {
    let normalized = normalize(path);
    entries
        .into_iter()
        .filter(|entry| {
            let last = entry.path().components().next_back();
            let is_dot_ref = matches!(last, Some(Component::CurDir | Component::ParentDir));
            !is_dot_ref && normalize(entry.path()) != normalized
        })
        .collect()
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    use pretty_assertions::assert_eq;
    use remotefs::fs::{FileType, Metadata, UnixPex};

    use super::*;

    fn file(path: &str, file_type: FileType) -> File {
        File::new(
            path,
            Metadata::default()
                .accessed(UNIX_EPOCH)
                .created(UNIX_EPOCH)
                .modified(UNIX_EPOCH)
                .file_type(file_type)
                .size(0),
        )
    }

    #[test]
    fn resolves_relative_paths_from_consumer_working_directory() {
        assert_eq!(
            resolve_remote_path(Path::new("/home/user"), Path::new("docs/report.txt")).unwrap(),
            Path::new("/home/user/docs/report.txt")
        );
    }

    #[test]
    fn preserves_absolute_remote_paths() {
        for path in ["/srv/data", r"C:\data\file", r"\\server\share\file"] {
            assert_eq!(
                resolve_remote_path(Path::new("/ignored"), Path::new(path)).unwrap(),
                Path::new(path)
            );
        }
    }

    #[test]
    fn maps_metadata_to_write_options() {
        let modified = SystemTime::UNIX_EPOCH;
        let mode = UnixPex::from(0o640);
        let options = write_options(&Metadata::default().size(42).mode(mode).modified(modified));

        assert_eq!(options.size_hint, Some(42));
        assert_eq!(options.mode, Some(mode));
        assert_eq!(options.modified, Some(modified));
    }

    #[test]
    fn maps_settable_metadata_fields() {
        let accessed = SystemTime::UNIX_EPOCH;
        let modified = accessed + std::time::Duration::from_secs(1);
        let mode = UnixPex::from(0o640);
        let options = set_metadata_options(
            &Metadata::default()
                .accessed(accessed)
                .gid(20)
                .mode(mode)
                .modified(modified)
                .uid(10),
        );

        assert_eq!(options.accessed, Some(accessed));
        assert_eq!(options.gid, Some(20));
        assert_eq!(options.mode, Some(mode));
        assert_eq!(options.modified, Some(modified));
        assert_eq!(options.uid, Some(10));
    }

    #[test]
    fn default_metadata_maps_to_empty_options() {
        assert_eq!(write_options(&Metadata::default()), Default::default());
        assert_eq!(
            set_metadata_options(&Metadata::default()),
            Default::default()
        );
    }

    #[test]
    fn filter_self_refs_drops_entry_matching_listed_dir() {
        let entries = vec![
            file("/wp-content/wp-content", FileType::Directory),
            file("/wp-content/index.php", FileType::File),
        ];

        let filtered = filter_self_refs(Path::new("/wp-content/wp-content"), entries);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].path(), Path::new("/wp-content/index.php"));
    }

    #[test]
    fn filter_self_refs_drops_dot_and_dotdot_entries() {
        let entries = vec![
            file("/foo/.", FileType::Directory),
            file("/foo/..", FileType::Directory),
            file("/foo/bar", FileType::File),
        ];

        let filtered = filter_self_refs(Path::new("/foo"), entries);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].path(), Path::new("/foo/bar"));
    }

    #[test]
    fn filter_self_refs_normalizes_paths() {
        let entries = vec![
            file("/foo/./bar", FileType::File),
            file("/foo/baz/../", FileType::Directory),
        ];

        let filtered = filter_self_refs(Path::new("/foo"), entries);

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].path(), Path::new("/foo/./bar"));
    }

    #[test]
    fn filter_self_refs_preserves_unrelated_entries() {
        let entries = vec![
            file("/home/user/notes.txt", FileType::File),
            file("/home/user/photos", FileType::Directory),
        ];

        let filtered = filter_self_refs(Path::new("/home/user"), entries.clone());

        assert_eq!(filtered.len(), 2);
    }
}
