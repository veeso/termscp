//! ## FileTransferActivity
//!
//! `filetransfer_activiy` is the module which implements the Filetransfer activity, which is the main activity afterall

// locals
use super::{FileTransferActivity, LogLevel};
use remotefs::fs::UnixPex;
use std::path::PathBuf;

impl FileTransferActivity {
    pub(crate) fn action_local_mkdir(&mut self, input: String) {
        match self.host.mkdir(PathBuf::from(input.as_str()).as_path()) {
            Ok(_) => {
                // Reload files
                self.log(LogLevel::Info, format!("Created directory \"{}\"", input));
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not create directory \"{}\": {}", input, err),
                );
            }
        }
    }
    pub(crate) fn action_remote_mkdir(&mut self, input: String) {
        match self.client.as_mut().create_dir(
            PathBuf::from(input.as_str()).as_path(),
            UnixPex::from(0o755),
        ) {
            Ok(_) => {
                // Reload files
                self.log(LogLevel::Info, format!("Created directory \"{}\"", input));
            }
            Err(err) => {
                // Report err
                self.log_and_alert(
                    LogLevel::Error,
                    format!("Could not create directory \"{}\": {}", input, err),
                );
            }
        }
    }
}
