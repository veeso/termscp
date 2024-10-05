//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use std::fs::File as StdFile;
use std::path::PathBuf;

use remotefs::fs::Metadata;

use super::{File, FileTransferActivity, LogLevel};

impl FileTransferActivity {
    pub(crate) fn action_local_newfile(&mut self, input: String) {
        // Check if file exists
        let mut file_exists: bool = false;
        for file in self.local().iter_files_all() {
            if input == file.name() {
                file_exists = true;
            }
        }
        if file_exists {
            self.log_and_alert(LogLevel::Warn, format!("File \"{input}\" already exists",));
            return;
        }

        // Create file
        let file_path: PathBuf = PathBuf::from(input.as_str());
        let writer = match self
            .host
            .create_file(file_path.as_path(), &Metadata::default())
        {
            Ok(f) => f,
            Err(err) => {
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not create file \"{}\": {}", file_path.display(), err),
                );
                return;
            }
        };
        // finalize write
        if let Err(err) = self.host.finalize_write(writer) {
            self.log_and_alert(
                LogLevel::Error,
                format!("Could not write file \"{}\": {}", file_path.display(), err),
            );
            return;
        }

        self.log(
            LogLevel::Info,
            format!("Created file \"{}\"", file_path.display()),
        );
    }

    pub(crate) fn action_remote_newfile(&mut self, input: String) {
        // Check if file exists
        let mut file_exists: bool = false;
        for file in self.remote().iter_files_all() {
            if input == file.name() {
                file_exists = true;
            }
        }
        if file_exists {
            self.log_and_alert(LogLevel::Warn, format!("File \"{input}\" already exists",));
            return;
        }
        // Get path on remote
        let file_path: PathBuf = PathBuf::from(input.as_str());
        // Create file (on local)
        match tempfile::NamedTempFile::new() {
            Err(err) => {
                self.log_and_alert(LogLevel::Error, format!("Could not create tempfile: {err}"))
            }
            Ok(tfile) => {
                // Stat tempfile
                let local_file: File = match self.host.stat(tfile.path()) {
                    Err(err) => {
                        self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not stat tempfile: {err}"),
                        );
                        return;
                    }
                    Ok(f) => f,
                };
                if local_file.is_file() {
                    // Create file
                    let reader = Box::new(match StdFile::open(tfile.path()) {
                        Ok(f) => f,
                        Err(err) => {
                            self.log_and_alert(
                                LogLevel::Error,
                                format!("Could not open tempfile: {err}"),
                            );
                            return;
                        }
                    });
                    match self
                        .client
                        .create_file(file_path.as_path(), &local_file.metadata, reader)
                    {
                        Err(err) => self.log_and_alert(
                            LogLevel::Error,
                            format!("Could not create file \"{}\": {}", file_path.display(), err),
                        ),
                        Ok(_) => {
                            self.log(
                                LogLevel::Info,
                                format!("Created file \"{}\"", file_path.display()),
                            );
                        }
                    }
                }
            }
        }
    }
}
