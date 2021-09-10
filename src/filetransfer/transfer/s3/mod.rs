//! ## S3 transfer
//!
//! S3 file transfer module

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// -- mod
mod object;

// Locals
use super::{FileTransfer, FileTransferError, FileTransferErrorType, ProtocolParams};
use crate::fs::{FsDirectory, FsEntry, FsFile};
use crate::utils::path;
use object::S3Object;

// ext
use s3::creds::Credentials;
use s3::serde_types::Object;
use s3::{Bucket, Region};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// ## S3FileTransfer
///
/// Aws s3 file transfer
pub struct S3FileTransfer {
    bucket: Option<Bucket>,
    wrkdir: PathBuf,
}

impl Default for S3FileTransfer {
    fn default() -> Self {
        Self {
            bucket: None,
            wrkdir: PathBuf::from("/"),
        }
    }
}

impl S3FileTransfer {
    /// ### list_objects
    ///
    /// List objects contained in `p` path
    fn list_objects(&self, p: &Path, list_dir: bool) -> Result<Vec<S3Object>, FileTransferError> {
        // Make path relative
        let key: String = Self::fmt_path(p, list_dir);
        debug!("Query list directory {}; key: {}", p.display(), key);
        self.query_objects(key, true)
    }

    /// ### stat_object
    ///
    /// Stat an s3 object
    fn stat_object(&self, p: &Path) -> Result<S3Object, FileTransferError> {
        let key: String = Self::fmt_path(p, false);
        debug!("Query stat object {}; key: {}", p.display(), key);
        let objects = self.query_objects(key, false)?;
        // Absolutize path
        let absol: PathBuf = path::absolutize(Path::new("/"), p);
        // Find associated object
        match objects
            .into_iter()
            .find(|x| x.path.as_path() == absol.as_path())
        {
            Some(obj) => Ok(obj),
            None => Err(FileTransferError::new_ex(
                FileTransferErrorType::NoSuchFileOrDirectory,
                format!("{}: No such file or directory", p.display()),
            )),
        }
    }

    /// ### query_objects
    ///
    /// Query objects at key
    fn query_objects(
        &self,
        key: String,
        only_direct_children: bool,
    ) -> Result<Vec<S3Object>, FileTransferError> {
        let results = self.bucket.as_ref().unwrap().list(key.clone(), None);
        match results {
            Ok(entries) => {
                let mut objects: Vec<S3Object> = Vec::new();
                entries.iter().for_each(|x| {
                    x.contents
                        .iter()
                        .filter(|x| {
                            if only_direct_children {
                                Self::list_object_should_be_kept(x, key.as_str())
                            } else {
                                true
                            }
                        })
                        .for_each(|x| objects.push(S3Object::from(x)))
                });
                debug!("Found objects: {:?}", objects);
                Ok(objects)
            }
            Err(e) => Err(FileTransferError::new_ex(
                FileTransferErrorType::DirStatFailed,
                e.to_string(),
            )),
        }
    }

    /// ### list_object_should_be_kept
    ///
    /// Returns whether object should be kept after list command.
    /// The object won't be kept if:
    ///
    /// 1. is not a direct child of provided dir
    fn list_object_should_be_kept(obj: &Object, dir: &str) -> bool {
        Self::is_direct_child(obj.key.as_str(), dir)
    }

    /// ### is_direct_child
    ///
    /// Checks whether Object's key is direct child of `parent` path.
    fn is_direct_child(key: &str, parent: &str) -> bool {
        key == format!("{}{}", parent, S3Object::object_name(key))
            || key == format!("{}{}/", parent, S3Object::object_name(key))
    }

    /// ### resolve
    ///
    /// Make s3 absolute path from a given path
    fn resolve(&self, p: &Path) -> PathBuf {
        path::diff_paths(path::absolutize(self.wrkdir.as_path(), p), &Path::new("/"))
            .unwrap_or_default()
    }

    /// ### fmt_fs_entry_path
    ///
    /// fmt path for fsentry according to format expected by s3
    fn fmt_fs_file_path(f: &FsFile) -> String {
        Self::fmt_path(f.abs_path.as_path(), false)
    }

    /// ### fmt_path
    ///
    /// fmt path for fsentry according to format expected by s3
    fn fmt_path(p: &Path, is_dir: bool) -> String {
        // prevent root as slash
        if p == Path::new("/") {
            return "".to_string();
        }
        // Remove root only if absolute
        #[cfg(target_family = "unix")]
        let is_absolute: bool = p.is_absolute();
        // NOTE: don't use is_absolute: on windows won't work
        #[cfg(target_family = "windows")]
        let is_absolute: bool = p.display().to_string().starts_with('/');
        let p: PathBuf = match is_absolute {
            true => path::diff_paths(p, &Path::new("/")).unwrap_or_default(),
            false => p.to_path_buf(),
        };
        // NOTE: windows only: resolve paths
        #[cfg(target_family = "windows")]
        let p: PathBuf = PathBuf::from(path_slash::PathExt::to_slash_lossy(p.as_path()).as_str());
        // Fmt
        match is_dir {
            true => {
                let mut p: String = p.display().to_string();
                if !p.ends_with('/') {
                    p.push('/');
                }
                p
            }
            false => p.to_string_lossy().to_string(),
        }
    }
}

impl FileTransfer for S3FileTransfer {
    /// ### connect
    ///
    /// Connect to the remote server
    /// Can return banner / welcome message on success
    fn connect(&mut self, params: &ProtocolParams) -> Result<Option<String>, FileTransferError> {
        // Verify parameters are S3
        let params = match params.s3_params() {
            Some(params) => params,
            None => return Err(FileTransferError::new(FileTransferErrorType::BadAddress)),
        };
        // Load credentials
        debug!("Loading credentials... (profile {:?})", params.profile);
        let credentials: Credentials =
            Credentials::new(None, None, None, None, params.profile.as_deref()).map_err(|e| {
                FileTransferError::new_ex(
                    FileTransferErrorType::AuthenticationFailed,
                    format!("Could not load s3 credentials: {}", e),
                )
            })?;
        // Parse region
        debug!("Parsing region {}", params.region);
        let region: Region = Region::from_str(params.region.as_str()).map_err(|e| {
            FileTransferError::new_ex(
                FileTransferErrorType::AuthenticationFailed,
                format!("Could not parse s3 region: {}", e),
            )
        })?;
        debug!(
            "Credentials loaded! Connecting to bucket {}...",
            params.bucket_name
        );
        self.bucket = Some(
            Bucket::new(params.bucket_name.as_str(), region, credentials).map_err(|e| {
                FileTransferError::new_ex(
                    FileTransferErrorType::AuthenticationFailed,
                    format!("Could not connect to bucket {}: {}", params.bucket_name, e),
                )
            })?,
        );
        info!("Connection successfully established");
        Ok(None)
    }

    /// ### disconnect
    ///
    /// Disconnect from the remote server
    fn disconnect(&mut self) -> Result<(), FileTransferError> {
        info!("Disconnecting from S3 bucket...");
        match self.bucket.take() {
            Some(bucket) => {
                drop(bucket);
                Ok(())
            }
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### is_connected
    ///
    /// Indicates whether the client is connected to remote
    fn is_connected(&self) -> bool {
        self.bucket.is_some()
    }

    /// ### pwd
    ///
    /// Print working directory
    fn pwd(&mut self) -> Result<PathBuf, FileTransferError> {
        info!("PWD");
        match self.is_connected() {
            true => Ok(self.wrkdir.clone()),
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### change_dir
    ///
    /// Change working directory
    fn change_dir(&mut self, dir: &Path) -> Result<PathBuf, FileTransferError> {
        match &self.bucket.is_some() {
            true => {
                // Always allow entering root
                if dir == Path::new("/") {
                    self.wrkdir = dir.to_path_buf();
                    info!("New working directory: {}", self.wrkdir.display());
                    return Ok(self.wrkdir.clone());
                }
                // Check if directory exists
                debug!("Entering directory {}...", dir.display());
                let dir_p: PathBuf = self.resolve(dir);
                let dir_s: String = Self::fmt_path(dir_p.as_path(), true);
                debug!("Searching for key {} (path: {})...", dir_s, dir_p.display());
                // Check if directory already exists
                if self
                    .stat_object(PathBuf::from(dir_s.as_str()).as_path())
                    .is_ok()
                {
                    self.wrkdir = path::absolutize(Path::new("/"), dir_p.as_path());
                    info!("New working directory: {}", self.wrkdir.display());
                    Ok(self.wrkdir.clone())
                } else {
                    Err(FileTransferError::new(
                        FileTransferErrorType::NoSuchFileOrDirectory,
                    ))
                }
            }
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### copy
    ///
    /// Copy file to destination
    fn copy(&mut self, _src: &FsEntry, _dst: &Path) -> Result<(), FileTransferError> {
        Err(FileTransferError::new(
            FileTransferErrorType::UnsupportedFeature,
        ))
    }

    /// ### list_dir
    ///
    /// List directory entries
    fn list_dir(&mut self, path: &Path) -> Result<Vec<FsEntry>, FileTransferError> {
        match self.is_connected() {
            true => self
                .list_objects(path, true)
                .map(|x| x.into_iter().map(|x| x.into()).collect()),
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### mkdir
    ///
    /// Make directory
    /// In case the directory already exists, it must return an Error of kind `FileTransferErrorType::DirectoryAlreadyExists`
    fn mkdir(&mut self, dir: &Path) -> Result<(), FileTransferError> {
        match &self.bucket {
            Some(bucket) => {
                let dir: String = Self::fmt_path(self.resolve(dir).as_path(), true);
                debug!("Making directory {}...", dir);
                // Check if directory already exists
                if self
                    .stat_object(PathBuf::from(dir.as_str()).as_path())
                    .is_ok()
                {
                    error!("Directory {} already exists", dir);
                    return Err(FileTransferError::new(
                        FileTransferErrorType::DirectoryAlreadyExists,
                    ));
                }
                bucket
                    .put_object(dir.as_str(), &[])
                    .map(|_| ())
                    .map_err(|e| {
                        FileTransferError::new_ex(
                            FileTransferErrorType::FileCreateDenied,
                            format!("Could not make directory: {}", e),
                        )
                    })
            }
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### remove
    ///
    /// Remove a file or a directory
    fn remove(&mut self, file: &FsEntry) -> Result<(), FileTransferError> {
        let path = Self::fmt_path(
            path::diff_paths(file.get_abs_path(), &Path::new("/"))
                .unwrap_or_default()
                .as_path(),
            file.is_dir(),
        );
        info!("Removing object {}...", path);
        match &self.bucket {
            Some(bucket) => bucket.delete_object(path).map(|_| ()).map_err(|e| {
                FileTransferError::new_ex(
                    FileTransferErrorType::ProtocolError,
                    format!("Could not remove file: {}", e),
                )
            }),
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### rename
    ///
    /// Rename file or a directory
    fn rename(&mut self, _file: &FsEntry, _dst: &Path) -> Result<(), FileTransferError> {
        Err(FileTransferError::new(
            FileTransferErrorType::UnsupportedFeature,
        ))
    }

    /// ### stat
    ///
    /// Stat file and return FsEntry
    fn stat(&mut self, p: &Path) -> Result<FsEntry, FileTransferError> {
        match self.is_connected() {
            true => {
                // First try as a "file"
                let path: PathBuf = self.resolve(p);
                if let Ok(obj) = self.stat_object(path.as_path()) {
                    return Ok(obj.into());
                }
                // Try as a "directory"
                debug!("Failed to stat object as file; trying as a directory...");
                let path: PathBuf = PathBuf::from(Self::fmt_path(path.as_path(), true));
                self.stat_object(path.as_path()).map(|x| x.into())
            }
            false => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### exec
    ///
    /// Execute a command on remote host
    fn exec(&mut self, _cmd: &str) -> Result<String, FileTransferError> {
        Err(FileTransferError::new(
            FileTransferErrorType::UnsupportedFeature,
        ))
    }

    /// ### send_file_wno_stream
    ///
    /// Send a file to remote WITHOUT using streams.
    /// This method SHOULD be implemented ONLY when streams are not supported by the current file transfer.
    /// The developer implementing the filetransfer user should FIRST try with `send_file` followed by `on_sent`
    /// If the function returns error kind() `UnsupportedFeature`, then he should call this function.
    /// By default this function uses the streams function to copy content from reader to writer
    fn send_file_wno_stream(
        &mut self,
        _src: &FsFile,
        dest: &Path,
        mut reader: Box<dyn Read>,
    ) -> Result<(), FileTransferError> {
        match &mut self.bucket {
            Some(bucket) => {
                let key = Self::fmt_path(dest, false);
                info!("Query PUT for key '{}'", key);
                bucket
                    .put_object_stream(&mut reader, key.as_str())
                    .map(|_| ())
                    .map_err(|e| {
                        FileTransferError::new_ex(
                            FileTransferErrorType::ProtocolError,
                            format!("Could not put file: {}", e),
                        )
                    })
            }
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }

    /// ### recv_file_wno_stream
    ///
    /// Receive a file from remote WITHOUT using streams.
    /// This method SHOULD be implemented ONLY when streams are not supported by the current file transfer.
    /// The developer implementing the filetransfer user should FIRST try with `send_file` followed by `on_sent`
    /// If the function returns error kind() `UnsupportedFeature`, then he should call this function.
    /// By default this function uses the streams function to copy content from reader to writer
    fn recv_file_wno_stream(&mut self, src: &FsFile, dest: &Path) -> Result<(), FileTransferError> {
        match &mut self.bucket {
            Some(bucket) => {
                let mut writer = File::create(dest).map_err(|e| {
                    FileTransferError::new_ex(
                        FileTransferErrorType::FileCreateDenied,
                        format!("Could not open local file: {}", e),
                    )
                })?;
                let key = Self::fmt_fs_file_path(src);
                info!("Query GET for key '{}'", key);
                bucket
                    .get_object_stream(key.as_str(), &mut writer)
                    .map(|_| ())
                    .map_err(|e| {
                        FileTransferError::new_ex(
                            FileTransferErrorType::ProtocolError,
                            format!("Could not get file: {}", e),
                        )
                    })
            }
            None => Err(FileTransferError::new(
                FileTransferErrorType::UninitializedSession,
            )),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[cfg(feature = "with-s3-ci")]
    use crate::filetransfer::params::AwsS3Params;
    #[cfg(feature = "with-s3-ci")]
    use crate::utils::random;
    use crate::utils::test_helpers;

    use pretty_assertions::assert_eq;
    #[cfg(feature = "with-s3-ci")]
    use std::env;
    #[cfg(feature = "with-s3-ci")]
    use tempfile::NamedTempFile;

    #[test]
    fn s3_new() {
        let s3: S3FileTransfer = S3FileTransfer::default();
        assert_eq!(s3.wrkdir.as_path(), Path::new("/"));
        assert!(s3.bucket.is_none());
    }

    #[test]
    fn s3_is_direct_child() {
        assert_eq!(S3FileTransfer::is_direct_child("pippo/", ""), true);
        assert_eq!(
            S3FileTransfer::is_direct_child("pippo/sottocartella/", ""),
            false
        );
        assert_eq!(
            S3FileTransfer::is_direct_child("pippo/sottocartella/", "pippo/"),
            true
        );
        assert_eq!(
            S3FileTransfer::is_direct_child("pippo/sottocartella/", "pippo"), // This case must be handled indeed
            false
        );
        assert_eq!(
            S3FileTransfer::is_direct_child(
                "pippo/sottocartella/readme.md",
                "pippo/sottocartella/"
            ),
            true
        );
        assert_eq!(
            S3FileTransfer::is_direct_child(
                "pippo/sottocartella/readme.md",
                "pippo/sottocartella/"
            ),
            true
        );
    }

    #[test]
    fn s3_resolve() {
        let mut s3: S3FileTransfer = S3FileTransfer::default();
        s3.wrkdir = PathBuf::from("/tmp");
        // Absolute
        assert_eq!(
            s3.resolve(&Path::new("/tmp/sottocartella/")).as_path(),
            Path::new("tmp/sottocartella")
        );
        // Relative
        assert_eq!(
            s3.resolve(&Path::new("subfolder/")).as_path(),
            Path::new("tmp/subfolder")
        );
    }

    #[test]
    fn s3_fmt_fs_file_path() {
        let f: FsFile =
            test_helpers::make_fsentry(&Path::new("/tmp/omar.txt"), false).unwrap_file();
        assert_eq!(
            S3FileTransfer::fmt_fs_file_path(&f).as_str(),
            "tmp/omar.txt"
        );
    }

    #[test]
    fn s3_fmt_path() {
        assert_eq!(
            S3FileTransfer::fmt_path(&Path::new("/tmp/omar.txt"), false).as_str(),
            "tmp/omar.txt"
        );
        assert_eq!(
            S3FileTransfer::fmt_path(&Path::new("omar.txt"), false).as_str(),
            "omar.txt"
        );
        assert_eq!(
            S3FileTransfer::fmt_path(&Path::new("/tmp/subfolder"), true).as_str(),
            "tmp/subfolder/"
        );
        assert_eq!(
            S3FileTransfer::fmt_path(&Path::new("tmp/subfolder"), true).as_str(),
            "tmp/subfolder/"
        );
        assert_eq!(
            S3FileTransfer::fmt_path(&Path::new("tmp"), true).as_str(),
            "tmp/"
        );
        assert_eq!(
            S3FileTransfer::fmt_path(&Path::new("tmp/"), true).as_str(),
            "tmp/"
        );
        assert_eq!(S3FileTransfer::fmt_path(&Path::new("/"), true).as_str(), "");
    }

    // -- test transfer
    #[cfg(feature = "with-s3-ci")]
    #[test]
    fn s3_filetransfer() {
        // Gather s3 environment args
        let bucket: String = env::var("AWS_S3_BUCKET").ok().unwrap();
        let region: String = env::var("AWS_S3_REGION").ok().unwrap();
        let params = get_ftparams(bucket, region);
        // Get transfer
        let mut s3 = S3FileTransfer::default();
        // Connect
        assert!(s3.connect(&params).is_ok());
        // Check is connected
        assert_eq!(s3.is_connected(), true);
        // Pwd
        assert_eq!(s3.pwd().ok().unwrap(), PathBuf::from("/"));
        // Go to github-ci directory
        assert!(s3.change_dir(&Path::new("/github-ci")).is_ok());
        assert_eq!(s3.pwd().ok().unwrap(), PathBuf::from("/github-ci"));
        // Find
        assert_eq!(s3.find("*.jpg").ok().unwrap().len(), 1);
        // List directory (3 entries)
        assert_eq!(s3.list_dir(&Path::new("/github-ci")).ok().unwrap().len(), 3);
        // Go to playground
        assert!(s3.change_dir(&Path::new("/github-ci/playground")).is_ok());
        assert_eq!(
            s3.pwd().ok().unwrap(),
            PathBuf::from("/github-ci/playground")
        );
        // Create directory
        let dir_name: String = format!("{}/", random::random_alphanumeric_with_len(8));
        let mut dir_path: PathBuf = PathBuf::from("/github-ci/playground");
        dir_path.push(dir_name.as_str());
        let dir_entry = test_helpers::make_fsentry(dir_path.as_path(), true);
        assert!(s3.mkdir(dir_path.as_path()).is_ok());
        assert!(s3.change_dir(dir_path.as_path()).is_ok());
        // Copy/rename file is unsupported
        assert!(s3.copy(&dir_entry, &Path::new("/copia")).is_err());
        assert!(s3.rename(&dir_entry, &Path::new("/copia")).is_err());
        // Exec is unsupported
        assert!(s3.exec("omar!").is_err());
        // Stat file
        let entry = s3
            .stat(&Path::new("/github-ci/avril_lavigne.jpg"))
            .ok()
            .unwrap()
            .unwrap_file();
        assert_eq!(entry.name.as_str(), "avril_lavigne.jpg");
        assert_eq!(
            entry.abs_path.as_path(),
            Path::new("/github-ci/avril_lavigne.jpg")
        );
        assert_eq!(entry.ftype.as_deref().unwrap(), "jpg");
        assert_eq!(entry.size, 101738);
        assert_eq!(entry.user, None);
        assert_eq!(entry.group, None);
        assert_eq!(entry.unix_pex, None);
        // Download file
        let (local_file_entry, local_file): (FsFile, NamedTempFile) =
            test_helpers::create_sample_file_entry();
        let remote_entry =
            test_helpers::make_fsentry(&Path::new("/github-ci/avril_lavigne.jpg"), false)
                .unwrap_file();
        assert!(s3
            .recv_file_wno_stream(&remote_entry, local_file.path())
            .is_ok());
        // Upload file
        let mut dest_path = dir_path.clone();
        dest_path.push("aurellia_lavagna.jpg");
        let reader = Box::new(File::open(local_file.path()).ok().unwrap());
        assert!(s3
            .send_file_wno_stream(&local_file_entry, dest_path.as_path(), reader)
            .is_ok());
        // Remove temp dir
        assert!(s3.remove(&dir_entry).is_ok());
        // Disconnect
        assert!(s3.disconnect().is_ok());
    }

    #[cfg(feature = "with-s3-ci")]
    fn get_ftparams(bucket: String, region: String) -> ProtocolParams {
        ProtocolParams::AwsS3(AwsS3Params::new(bucket, region, None))
    }
}
